//! App-facing helpers for preparing `zama-host` FHE evaluation requests.
//!
//! This crate targets the role-aware host ABI. App code describes encrypted
//! operands and durable outputs by pubkey; [`EvalBuilder`] validates the frame,
//! assigns host account indices, and records the signer/writable requirements for
//! every dynamic account. With the `cpi` feature, [`EvalPlan::resolve_accounts`]
//! preflights the dynamic account set and [`invoke_eval_signed_resolved`] turns
//! the plan plus resolved accounts into the exact `zama-host` CPI.
//!
//! The builder intentionally targets the current role-aware host eval ABI rather
//! than the older `execute_frame` prototype. Instruction-local intermediate
//! values are returned by builder methods as typed transient [`Encrypted`] values;
//! only [`Output::durable`] creates ACL state. Binary, ternary, trivial-encrypt,
//! rand, and verified input steps can be composed in one eval frame.

#![allow(unexpected_cfgs)]

use anchor_lang::prelude::Pubkey;
use std::marker::PhantomData;
#[cfg(not(target_os = "solana"))]
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "cpi")]
use anchor_lang::{
    prelude::AccountInfo,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program::invoke_signed,
    },
    InstructionData, Key, ToAccountInfos, ToAccountMetas,
};

use zama_host::{
    encrypted_value_address, AclSubjectEntry, CoprocessorInputAttestation, FheBinaryOpCode,
    FheEvalArgs, FheEvalOperand, FheEvalOutput, FheEvalStep, FheTernaryOpCode, FheUnaryOpCode,
    MAX_ACL_SUBJECTS, MAX_FHE_EVAL_OPS,
};

/// Result type used by the builder helpers.
pub type Result<T> = std::result::Result<T, EvalBuildError>;

/// Builder failures that can be detected before invoking the host program.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalBuildError {
    /// More accounts were referenced than fit in the host's `u16` indices.
    TooManyRemainingAccounts,
    /// A transient operand referenced an operation that has not been produced.
    InvalidTransientReference,
    /// More ops were added than the host accepts (`MAX_FHE_EVAL_OPS`).
    TooManyOps,
    /// `finish` was called with no ops; the host rejects empty eval frames.
    EmptyOps,
    /// `finish` was called with an all-zero `context_id`; the host rejects it.
    EmptyContextId,
    /// A scalar was supplied as the left-hand operand. The host invariant is
    /// scalar-RHS-only: the left operand must be an encrypted handle.
    ScalarLhsOperand,
    /// A scalar was supplied where the host requires an encrypted operand.
    ScalarEncryptedOperand,
    /// The declared FHE type is not accepted by the host ABI.
    UnsupportedFheType,
    /// A bounded random upper bound is zero, not a power of two, or too wide for euint64.
    InvalidRandomUpperBound,
    /// The declared binary output type is not valid for the selected operator.
    UnsupportedBinaryOutputType,
    /// Binary operand handle types do not match the selected operator.
    BinaryOperandTypeMismatch,
    /// Ternary operand handle types do not match the selected operator.
    TernaryOperandTypeMismatch,
    /// The encrypted-input proof does not contain the selected handle.
    InvalidInputProof,
    /// A durable output ACL policy would be rejected by the host.
    InvalidAccessPolicy,
    /// A durable slot contains an app-domain pubkey the host would reject.
    InvalidDurableSlot,
    /// The fixed app authority pubkey is not a valid signer identity.
    InvalidAppAuthority,
    /// A durable output's declared previous state is inconsistent (one of
    /// `previous_handle`/`previous_subjects` set without the other).
    InconsistentPreviousState,
    /// A lowered host account index does not match the eval plan account list.
    InvalidRemainingAccountReference,
    /// A verified-input operand referenced an attestation not registered with the builder.
    MissingVerifiedInput,
    /// `sum`/`is_in` exceeded the coprocessor's max operand count for the type.
    TooManyReductionOperands,
    /// `mul_div` was given a zero divisor; the host rejects it (EVM DivisionByZero parity).
    MulDivDivisorZero,
    /// `div`/`rem` require a plaintext scalar divisor (EVM `IsNotScalar`).
    DivisorMustBeScalar,
    /// `div`/`rem` divisor is zero once truncated to the operand type (EVM `DivisionByZero`).
    DivisionByZero,
}

/// Typed FHE handle tag used by the host ABI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FheType(u8);

impl FheType {
    pub const BOOL: Self = Self(0);
    pub const UINT8: Self = Self(2);
    pub const UINT16: Self = Self(3);
    pub const UINT32: Self = Self(4);
    pub const UINT64: Self = Self(5);
    pub const UINT128: Self = Self(6);
    pub const ADDRESS: Self = Self(7);
    pub const BYTES256: Self = Self(8);

    const fn byte(self) -> u8 {
        self.0
    }

    fn from_host_byte(byte: u8) -> Result<Self> {
        validate_supported_fhe_type(byte)?;
        Ok(Self(byte))
    }
}

/// Marker for encrypted bool handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bool;

/// Marker for encrypted unsigned integer handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uint<const BITS: u16>;

pub type BoolHandle = Encrypted<Bool>;
pub type Uint64Handle = Encrypted<Uint<64>>;

/// Marker for encrypted address handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address;

/// Marker for opaque 256-byte encrypted values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bytes256;

mod sealed {
    use super::{Address, Bool, Bytes256, Uint};

    pub trait FheTypedSeal {}
    pub trait FheUintSeal {}
    pub trait FheRandomSeal {}
    pub trait FheNotSeal {}
    pub trait FheBitwiseSeal {}
    pub trait FheShiftSeal {}
    pub trait FheEqSeal {}
    pub trait FheNegSeal {}
    pub trait FheIsInSeal {}

    impl FheTypedSeal for Bool {}
    impl FheTypedSeal for Uint<8> {}
    impl FheTypedSeal for Uint<16> {}
    impl FheTypedSeal for Uint<32> {}
    impl FheTypedSeal for Uint<64> {}
    impl FheTypedSeal for Uint<128> {}
    impl FheTypedSeal for Address {}
    impl FheTypedSeal for Bytes256 {}

    impl FheUintSeal for Uint<8> {}
    impl FheUintSeal for Uint<16> {}
    impl FheUintSeal for Uint<32> {}
    impl FheUintSeal for Uint<64> {}
    impl FheUintSeal for Uint<128> {}

    impl FheRandomSeal for Bool {}
    impl FheRandomSeal for Uint<8> {}
    impl FheRandomSeal for Uint<16> {}
    impl FheRandomSeal for Uint<32> {}
    impl FheRandomSeal for Uint<64> {}
    impl FheRandomSeal for Uint<128> {}
    impl FheRandomSeal for Bytes256 {}

    // NOT / bitwise: Bool + Uint8..Uint128 + Uint256 (host `0 | 2..=6 | 8`).
    impl FheNotSeal for Bool {}
    impl FheNotSeal for Uint<8> {}
    impl FheNotSeal for Uint<16> {}
    impl FheNotSeal for Uint<32> {}
    impl FheNotSeal for Uint<64> {}
    impl FheNotSeal for Uint<128> {}
    impl FheNotSeal for Bytes256 {}

    impl FheBitwiseSeal for Bool {}
    impl FheBitwiseSeal for Uint<8> {}
    impl FheBitwiseSeal for Uint<16> {}
    impl FheBitwiseSeal for Uint<32> {}
    impl FheBitwiseSeal for Uint<64> {}
    impl FheBitwiseSeal for Uint<128> {}
    impl FheBitwiseSeal for Bytes256 {}

    // Shifts/rotations and Neg: Uint8..Uint128 + Uint256 (host `2..=6 | 8`).
    impl FheShiftSeal for Uint<8> {}
    impl FheShiftSeal for Uint<16> {}
    impl FheShiftSeal for Uint<32> {}
    impl FheShiftSeal for Uint<64> {}
    impl FheShiftSeal for Uint<128> {}
    impl FheShiftSeal for Bytes256 {}

    impl FheNegSeal for Uint<8> {}
    impl FheNegSeal for Uint<16> {}
    impl FheNegSeal for Uint<32> {}
    impl FheNegSeal for Uint<64> {}
    impl FheNegSeal for Uint<128> {}
    impl FheNegSeal for Bytes256 {}

    // Eq/Ne: Bool + Uint8..Uint128 + Uint160 + Uint256 (host `0 | 2..=8`).
    impl FheEqSeal for Bool {}
    impl FheEqSeal for Uint<8> {}
    impl FheEqSeal for Uint<16> {}
    impl FheEqSeal for Uint<32> {}
    impl FheEqSeal for Uint<64> {}
    impl FheEqSeal for Uint<128> {}
    impl FheEqSeal for Address {}
    impl FheEqSeal for Bytes256 {}

    // IsIn: Uint8..Uint128 + Uint160 + Uint256 (host/EVM/coprocessor `2..=8`; no ebool).
    impl FheIsInSeal for Uint<8> {}
    impl FheIsInSeal for Uint<16> {}
    impl FheIsInSeal for Uint<32> {}
    impl FheIsInSeal for Uint<64> {}
    impl FheIsInSeal for Uint<128> {}
    impl FheIsInSeal for Address {}
    impl FheIsInSeal for Bytes256 {}
}

/// Compile-time FHE type tag for typed encrypted handles.
pub trait FheTyped: sealed::FheTypedSeal {
    const FHE_TYPE: FheType;
}

impl FheTyped for Bool {
    const FHE_TYPE: FheType = FheType::BOOL;
}

impl FheTyped for Uint<8> {
    const FHE_TYPE: FheType = FheType::UINT8;
}

impl FheTyped for Uint<16> {
    const FHE_TYPE: FheType = FheType::UINT16;
}

impl FheTyped for Uint<32> {
    const FHE_TYPE: FheType = FheType::UINT32;
}

impl FheTyped for Uint<64> {
    const FHE_TYPE: FheType = FheType::UINT64;
}

impl FheTyped for Uint<128> {
    const FHE_TYPE: FheType = FheType::UINT128;
}

impl FheTyped for Address {
    const FHE_TYPE: FheType = FheType::ADDRESS;
}

impl FheTyped for Bytes256 {
    const FHE_TYPE: FheType = FheType::BYTES256;
}

/// Marker trait for integer FHE values accepted by arithmetic/comparison ops.
pub trait FheUint: FheTyped + sealed::FheUintSeal {}

impl FheUint for Uint<8> {}
impl FheUint for Uint<16> {}
impl FheUint for Uint<32> {}
impl FheUint for Uint<64> {}
impl FheUint for Uint<128> {}

/// Marker trait for FHE values accepted by host rand steps.
pub trait FheRandom: FheTyped + sealed::FheRandomSeal {}

impl FheRandom for Bool {}
impl FheRandom for Uint<8> {}
impl FheRandom for Uint<16> {}
impl FheRandom for Uint<32> {}
impl FheRandom for Uint<64> {}
impl FheRandom for Uint<128> {}
impl FheRandom for Bytes256 {}

/// Marker trait for FHE values accepted by bitwise NOT: Bool, Uint8..Uint128, Uint256.
pub trait FheNot: FheTyped + sealed::FheNotSeal {}

impl FheNot for Bool {}
impl FheNot for Uint<8> {}
impl FheNot for Uint<16> {}
impl FheNot for Uint<32> {}
impl FheNot for Uint<64> {}
impl FheNot for Uint<128> {}
impl FheNot for Bytes256 {}

/// Marker trait for values accepted by bitwise And/Or/Xor: Bool, Uint8..Uint128, Uint256.
pub trait FheBitwise: FheTyped + sealed::FheBitwiseSeal {}

impl FheBitwise for Bool {}
impl FheBitwise for Uint<8> {}
impl FheBitwise for Uint<16> {}
impl FheBitwise for Uint<32> {}
impl FheBitwise for Uint<64> {}
impl FheBitwise for Uint<128> {}
impl FheBitwise for Bytes256 {}

/// Marker trait for values accepted by shifts/rotations: Uint8..Uint128, Uint256.
pub trait FheShift: FheTyped + sealed::FheShiftSeal {}

impl FheShift for Uint<8> {}
impl FheShift for Uint<16> {}
impl FheShift for Uint<32> {}
impl FheShift for Uint<64> {}
impl FheShift for Uint<128> {}
impl FheShift for Bytes256 {}

/// Marker trait for values accepted by arithmetic negation: Uint8..Uint128, Uint256.
pub trait FheNeg: FheTyped + sealed::FheNegSeal {}

impl FheNeg for Uint<8> {}
impl FheNeg for Uint<16> {}
impl FheNeg for Uint<32> {}
impl FheNeg for Uint<64> {}
impl FheNeg for Uint<128> {}
impl FheNeg for Bytes256 {}

/// Marker trait for values accepted by Eq/Ne: Bool, Uint8..Uint128, Uint160, Uint256.
pub trait FheEq: FheTyped + sealed::FheEqSeal {}

impl FheEq for Bool {}
impl FheEq for Uint<8> {}
impl FheEq for Uint<16> {}
impl FheEq for Uint<32> {}
impl FheEq for Uint<64> {}
impl FheEq for Uint<128> {}
impl FheEq for Address {}
impl FheEq for Bytes256 {}

/// Marker trait for values accepted by IsIn: Uint8..Uint128, Uint160, Uint256.
pub trait FheIsIn: FheTyped + sealed::FheIsInSeal {}

impl FheIsIn for Uint<8> {}
impl FheIsIn for Uint<16> {}
impl FheIsIn for Uint<32> {}
impl FheIsIn for Uint<64> {}
impl FheIsIn for Uint<128> {}
impl FheIsIn for Address {}
impl FheIsIn for Bytes256 {}

/// Typed encrypted eval value.
///
/// Durable values are constructed from app account state. Transient values are
/// returned by [`EvalBuilder`] methods and can only be fed to later steps in the
/// same builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Encrypted<T> {
    operand: Operand,
    marker: PhantomData<T>,
}

impl<T: FheTyped> Encrypted<T> {
    /// Builds a durable operand from a stable `EncryptedValue` lineage. `handle`
    /// must be that lineage's current handle; the host re-verifies this on-chain.
    pub fn durable(handle: [u8; 32], slot: DurableSlot) -> Result<Self> {
        validate_durable_slot(&slot)?;
        if handle_fhe_type(handle) != T::FHE_TYPE.byte() {
            return Err(EvalBuildError::UnsupportedFheType);
        }
        Ok(Self::from_operand(Operand::durable(handle, slot.address())))
    }
}

impl<T> Encrypted<T> {
    fn from_operand(operand: Operand) -> Self {
        Self {
            operand,
            marker: PhantomData,
        }
    }

    fn operand(self) -> Operand {
        self.operand
    }
}

/// Plaintext scalar bytes tagged by the encrypted type they can be paired with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scalar<T> {
    bytes: [u8; 32],
    marker: PhantomData<T>,
}

impl<T> Scalar<T> {
    fn bytes(self) -> [u8; 32] {
        self.bytes
    }

    fn from_low_bytes(value: &[u8]) -> Self {
        let mut bytes = [0u8; 32];
        bytes[32 - value.len()..].copy_from_slice(value);
        Self {
            bytes,
            marker: PhantomData,
        }
    }
}

impl Scalar<Uint<8>> {
    pub fn u8(value: u8) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<16>> {
    pub fn u16(value: u16) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<32>> {
    pub fn u32(value: u32) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<64>> {
    pub fn u64(value: u64) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Uint<128>> {
    pub fn u128(value: u128) -> Self {
        Self::from_low_bytes(&value.to_be_bytes())
    }
}

impl Scalar<Bool> {
    pub fn bool(value: bool) -> Self {
        let mut bytes = [0u8; 32];
        bytes[31] = u8::from(value);
        Self {
            bytes,
            marker: PhantomData,
        }
    }
}

impl Scalar<Address> {
    pub fn pubkey(value: Pubkey) -> Self {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(value.as_ref());
        Self {
            bytes,
            marker: PhantomData,
        }
    }
}

impl Scalar<Bytes256> {
    pub fn from_bytes(value: [u8; 32]) -> Self {
        Self {
            bytes: value,
            marker: PhantomData,
        }
    }
}

/// Typed right-hand side accepted by binary eval ops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryRhs<T> {
    Encrypted(Encrypted<T>),
    Scalar(Scalar<T>),
}

impl<T> From<Encrypted<T>> for BinaryRhs<T> {
    fn from(value: Encrypted<T>) -> Self {
        Self::Encrypted(value)
    }
}

impl<T> From<Scalar<T>> for BinaryRhs<T> {
    fn from(value: Scalar<T>) -> Self {
        Self::Scalar(value)
    }
}

fn binary_rhs_operand<T>(rhs: impl Into<BinaryRhs<T>>) -> Operand {
    match rhs.into() {
        BinaryRhs::Encrypted(value) => value.operand(),
        BinaryRhs::Scalar(value) => Operand::scalar(value.bytes()),
    }
}

/// Durable host operand identified by its `EncryptedValue` PDA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DurableOperand {
    handle: [u8; 32],
    encrypted_value: Pubkey,
}

/// Raw operand used by the lowering implementation.
///
/// Public builders expose typed [`Encrypted`] values instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Operand(OperandKind);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperandKind {
    Durable(DurableOperand),
    Transient {
        producer_index: u16,
        context_id: EvalContextId,
        builder_scope: EvalBuilderScope,
    },
    /// External input verified in-frame via a coprocessor attestation (EVM `fromExternal`). The
    /// `Vec`-bearing attestation is held by the [`EvalBuilder`] and referenced by index; keeping
    /// only the index + `input_handle` here leaves the operand `Copy`. `input_handle` carries the
    /// FHE type for operand type-checks without touching the attestation.
    VerifiedInput {
        input_handle: [u8; 32],
        attestation_index: u16,
    },
    Scalar([u8; 32]),
}

impl Operand {
    fn durable(handle: [u8; 32], encrypted_value: Pubkey) -> Self {
        Self(OperandKind::Durable(DurableOperand {
            handle,
            encrypted_value,
        }))
    }

    fn transient(
        producer_index: u16,
        context_id: EvalContextId,
        builder_scope: EvalBuilderScope,
    ) -> Self {
        Self(OperandKind::Transient {
            producer_index,
            context_id,
            builder_scope,
        })
    }

    fn scalar(value: [u8; 32]) -> Self {
        Self(OperandKind::Scalar(value))
    }

    fn verified_input(input_handle: [u8; 32], attestation_index: u16) -> Self {
        Self(OperandKind::VerifiedInput {
            input_handle,
            attestation_index,
        })
    }
}

/// Non-zero frame identifier for one host eval request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvalContextId([u8; 32]);

impl EvalContextId {
    pub fn new(bytes: [u8; 32]) -> Result<Self> {
        if bytes == [0u8; 32] {
            return Err(EvalBuildError::EmptyContextId);
        }
        Ok(Self(bytes))
    }

    pub fn bytes(self) -> [u8; 32] {
        self.0
    }
}

impl TryFrom<[u8; 32]> for EvalContextId {
    type Error = EvalBuildError;

    fn try_from(value: [u8; 32]) -> Result<Self> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EvalBuilderScope(u64);

#[cfg(not(target_os = "solana"))]
static NEXT_EVAL_BUILDER_SCOPE: AtomicU64 = AtomicU64::new(1);

#[cfg(not(target_os = "solana"))]
fn next_eval_builder_scope() -> EvalBuilderScope {
    EvalBuilderScope(NEXT_EVAL_BUILDER_SCOPE.fetch_add(1, Ordering::Relaxed))
}

#[cfg(target_os = "solana")]
fn next_eval_builder_scope() -> EvalBuilderScope {
    EvalBuilderScope(1)
}

/// App-domain encrypted field label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DurableLabel([u8; 32]);

impl DurableLabel {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

/// App-domain address of a stable `EncryptedValue` lineage.
///
/// Addressing is stable per `(namespace, account, label)` — it does not rotate
/// on handle updates, unlike the old nonce-keyed ACL records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableSlot {
    namespace: Pubkey,
    account: Pubkey,
    label: DurableLabel,
}

impl DurableSlot {
    pub fn new(namespace: Pubkey, account: Pubkey, label: DurableLabel) -> Self {
        Self {
            namespace,
            account,
            label,
        }
    }

    pub fn address(&self) -> Pubkey {
        encrypted_value_address(self.value_key()).0
    }

    pub fn namespace(&self) -> Pubkey {
        self.namespace
    }

    pub fn account(&self) -> Pubkey {
        self.account
    }

    pub fn label(&self) -> DurableLabel {
        self.label
    }

    pub fn value_key(&self) -> [u8; 32] {
        zama_solana_acl::derive_value_key(
            self.namespace.to_bytes(),
            self.account.to_bytes(),
            self.label.bytes(),
        )
    }
}

/// Subject granted access to a durable eval output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessSubject {
    pubkey: Pubkey,
}

impl AccessSubject {
    /// Owner subject allowed on the durable value.
    pub fn owner(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    pub fn compute(pubkey: Pubkey) -> Self {
        Self::from_host(AclSubjectEntry::compute(pubkey))
    }

    pub fn use_only(pubkey: Pubkey) -> Self {
        Self::from_host(AclSubjectEntry::use_only(pubkey))
    }

    pub fn pubkey(self) -> Pubkey {
        self.pubkey
    }

    pub fn matches_record_entry(self, pubkey: Pubkey) -> bool {
        self.pubkey == pubkey
    }

    fn from_host(subject: AclSubjectEntry) -> Self {
        Self {
            pubkey: subject.pubkey,
        }
    }

    fn host_entry(self) -> AclSubjectEntry {
        AclSubjectEntry {
            pubkey: self.pubkey,
        }
    }
}

/// ACL policy for a durable eval output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessPolicy {
    subjects: Vec<AccessSubject>,
}

impl AccessPolicy {
    pub fn from_subjects(subjects: Vec<AccessSubject>) -> Result<Self> {
        validate_access_policy(&subjects)?;
        Ok(Self { subjects })
    }

    pub fn for_owner(pubkey: Pubkey) -> Result<Self> {
        Self::from_subjects(vec![AccessSubject::owner(pubkey)])
    }

    pub fn for_compute(pubkey: Pubkey) -> Result<Self> {
        Self::from_subjects(vec![AccessSubject::compute(pubkey)])
    }

    pub fn for_use_only(pubkey: Pubkey) -> Result<Self> {
        Self::from_subjects(vec![AccessSubject::use_only(pubkey)])
    }

    pub fn for_owner_and_compute(owner: Pubkey, compute: Pubkey) -> Result<Self> {
        Self::for_owner(owner)?.with_compute(compute)
    }

    pub fn with_owner(self, pubkey: Pubkey) -> Result<Self> {
        self.with_subject(AccessSubject::owner(pubkey))
    }

    pub fn with_compute(self, pubkey: Pubkey) -> Result<Self> {
        self.with_subject(AccessSubject::compute(pubkey))
    }

    pub fn with_use_only(self, pubkey: Pubkey) -> Result<Self> {
        self.with_subject(AccessSubject::use_only(pubkey))
    }

    pub fn subjects(&self) -> &[AccessSubject] {
        &self.subjects
    }

    fn with_subject(mut self, subject: AccessSubject) -> Result<Self> {
        self.subjects.push(subject);
        validate_access_policy(&self.subjects)?;
        Ok(self)
    }
}

/// Previous on-chain state a durable output supersedes. `None` means this bind
/// is the lineage's first (the `EncryptedValue` PDA is created).
#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviousLineageState {
    handle: [u8; 32],
    subjects: Vec<Pubkey>,
}

/// Durable output descriptor accepted by durable-only steps such as input bind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableOutput {
    slot: DurableSlot,
    access: AccessPolicy,
    previous: Option<PreviousLineageState>,
    make_public: bool,
}

impl DurableOutput {
    /// First bind for a lineage: creates the `EncryptedValue` PDA.
    pub fn create(slot: DurableSlot, access: AccessPolicy) -> Self {
        Self {
            slot,
            access,
            previous: None,
            make_public: false,
        }
    }

    /// Supersedes an existing lineage. `previous_handle`/`previous_subjects`
    /// must be read from the on-chain `EncryptedValue` account in the same
    /// instruction; the host verifies them exactly.
    pub fn supersede(
        slot: DurableSlot,
        access: AccessPolicy,
        previous_handle: [u8; 32],
        previous_subjects: Vec<Pubkey>,
    ) -> Self {
        Self {
            slot,
            access,
            previous: Some(PreviousLineageState {
                handle: previous_handle,
                subjects: previous_subjects,
            }),
            make_public: false,
        }
    }

    /// Opts this output into being born publicly decryptable: the host seals a
    /// public-decrypt leaf for the newly bound handle inside the same eval CPI
    /// (EVM `unwrap`'s `makePubliclyDecryptable` parity; DD-036).
    pub fn with_make_public(mut self, make_public: bool) -> Self {
        self.make_public = make_public;
        self
    }

    pub fn birth(&self) -> Result<DurableOutputBirth> {
        validate_durable_slot(&self.slot)?;
        validate_access_policy(self.access.subjects())?;
        Ok(DurableOutputBirth {
            encrypted_value: self.slot.address(),
            acl_domain_key: self.slot.namespace,
            app_account: self.slot.account,
            encrypted_value_label: self.slot.label.bytes(),
            subjects: self.access.subjects.clone(),
            previous: self.previous.clone(),
            make_public: self.make_public,
        })
    }
}

/// Host-ready metadata for creating or superseding a durable `EncryptedValue` lineage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableOutputBirth {
    encrypted_value: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    subjects: Vec<AccessSubject>,
    previous: Option<PreviousLineageState>,
    make_public: bool,
}

impl DurableOutputBirth {
    pub fn encrypted_value(&self) -> Pubkey {
        self.encrypted_value
    }

    pub fn acl_domain_key(&self) -> Pubkey {
        self.acl_domain_key
    }

    pub fn app_account(&self) -> Pubkey {
        self.app_account
    }

    pub fn encrypted_value_label(&self) -> [u8; 32] {
        self.encrypted_value_label
    }

    pub fn subjects(&self) -> &[AccessSubject] {
        &self.subjects
    }

    pub fn previous_handle(&self) -> Option<[u8; 32]> {
        self.previous.as_ref().map(|previous| previous.handle)
    }

    pub fn previous_subjects(&self) -> Option<&[Pubkey]> {
        self.previous
            .as_ref()
            .map(|previous| previous.subjects.as_slice())
    }

    pub fn make_public(&self) -> bool {
        self.make_public
    }

    fn host_subjects(&self) -> Vec<AclSubjectEntry> {
        self.subjects
            .iter()
            .copied()
            .map(AccessSubject::host_entry)
            .collect()
    }
}

/// Validated power-of-two upper bound for host bounded-random `euint64` creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedU64UpperBound {
    value: [u8; 32],
}

impl BoundedU64UpperBound {
    pub fn power_of_two(value: u64) -> Result<Self> {
        if value == 0 || !value.is_power_of_two() {
            return Err(EvalBuildError::InvalidRandomUpperBound);
        }
        let mut bytes = [0u8; 32];
        bytes[24..].copy_from_slice(&value.to_be_bytes());
        Self::from_be_bytes(bytes)
    }

    pub fn full_width() -> Self {
        let mut value = [0u8; 32];
        value[23] = 1;
        debug_assert!(zama_host::assert_valid_bounded_rand_upper_bound(
            value,
            FheType::UINT64.byte()
        )
        .is_ok());
        Self { value }
    }

    pub fn from_be_bytes(value: [u8; 32]) -> Result<Self> {
        zama_host::assert_valid_bounded_rand_upper_bound(value, FheType::UINT64.byte())
            .map_err(|_| EvalBuildError::InvalidRandomUpperBound)?;
        Ok(Self { value })
    }

    pub fn bytes(self) -> [u8; 32] {
        self.value
    }
}

impl TryFrom<u64> for BoundedU64UpperBound {
    type Error = EvalBuildError;

    fn try_from(value: u64) -> Result<Self> {
        Self::power_of_two(value)
    }
}

/// Output policy exposed by the builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output(OutputKind);

#[derive(Debug, Clone, PartialEq, Eq)]
enum OutputKind {
    Transient,
    Durable(DurableOutput),
}

impl Output {
    pub fn transient() -> Self {
        Self(OutputKind::Transient)
    }

    /// First bind for a lineage (creates the `EncryptedValue` PDA).
    pub fn durable(slot: DurableSlot, access: AccessPolicy) -> Self {
        Self(OutputKind::Durable(DurableOutput::create(slot, access)))
    }

    pub fn durable_output(output: DurableOutput) -> Self {
        Self(OutputKind::Durable(output))
    }
}

/// Why an eval plan needs a dynamic account.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalAccountPurpose {
    DurableInputAcl,
    DurableOutputAcl,
    DurableOutputAuthority,
}

/// Public view of one dynamic account required by an eval plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalAccountRequirement {
    pubkey: Pubkey,
    is_writable: bool,
    is_signer: bool,
    purposes: Vec<EvalAccountPurpose>,
}

impl EvalAccountRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn is_writable(&self) -> bool {
        self.is_writable
    }

    pub fn is_signer(&self) -> bool {
        self.is_signer
    }

    pub fn has_purpose(&self, purpose: EvalAccountPurpose) -> bool {
        self.purposes.contains(&purpose)
    }

    pub fn purposes(&self) -> &[EvalAccountPurpose] {
        &self.purposes
    }

    pub fn requires_dynamic_account(&self) -> bool {
        self.purposes
            .iter()
            .any(|purpose| *purpose != EvalAccountPurpose::DurableOutputAuthority)
    }

    pub fn requires_output_authority(&self) -> bool {
        self.has_purpose(EvalAccountPurpose::DurableOutputAuthority)
    }
}

/// Dynamic account role required by an eval plan.
#[derive(Debug, Clone, PartialEq, Eq)]
struct EvalAccountMeta {
    pubkey: Pubkey,
    is_writable: bool,
    is_signer: bool,
    purposes: Vec<EvalAccountPurpose>,
}

impl EvalAccountMeta {
    fn readonly(pubkey: Pubkey, purpose: EvalAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    fn writable(pubkey: Pubkey, purpose: EvalAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: true,
            is_signer: false,
            purposes: vec![purpose],
        }
    }

    fn readonly_signer(pubkey: Pubkey, purpose: EvalAccountPurpose) -> Self {
        Self {
            pubkey,
            is_writable: false,
            is_signer: true,
            purposes: vec![purpose],
        }
    }

    fn promote(&mut self, required: Self) {
        self.is_writable |= required.is_writable;
        self.is_signer |= required.is_signer;
        for purpose in required.purposes {
            if !self.purposes.contains(&purpose) {
                self.purposes.push(purpose);
            }
        }
    }
}

impl From<&EvalAccountMeta> for EvalAccountRequirement {
    fn from(meta: &EvalAccountMeta) -> Self {
        Self {
            pubkey: meta.pubkey,
            is_writable: meta.is_writable,
            is_signer: meta.is_signer,
            purposes: meta.purposes.clone(),
        }
    }
}

/// App authority that signs the fixed ZamaHost eval CPI account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvalAppAuthority(Pubkey);

impl EvalAppAuthority {
    pub fn new(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }

    pub fn pubkey(self) -> Pubkey {
        self.0
    }
}

/// Output authority required by an eval plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvalOutputAuthorityRequirement {
    pubkey: Pubkey,
    cpi_account_authority: bool,
}

impl EvalOutputAuthorityRequirement {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    pub fn signs_cpi_account(&self) -> bool {
        self.cpi_account_authority
    }
}

/// Account-list resolution failure for an [`EvalPlan`].
#[cfg(feature = "cpi")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalAccountResolutionError {
    /// The same dynamic account pubkey was supplied more than once.
    DuplicateDynamicAccount { pubkey: Pubkey },
    /// A supplied dynamic account is not required by this plan's non-authority
    /// remaining-account slots.
    UnexpectedDynamicAccount { pubkey: Pubkey },
    /// A non-authority remaining-account slot could not be resolved.
    MissingDynamicAccount { requirement: EvalAccountRequirement },
    /// A writable remaining-account slot was supplied as readonly.
    DynamicAccountNotWritable { requirement: EvalAccountRequirement },
    /// The same durable output authority witness was supplied more than once.
    DuplicateOutputAuthority { pubkey: Pubkey },
    /// A supplied output authority is not required by this plan.
    UnexpectedOutputAuthority { pubkey: Pubkey },
    /// A required durable output authority witness could not be resolved.
    MissingOutputAuthority {
        authority: EvalOutputAuthorityRequirement,
    },
}

#[cfg(feature = "cpi")]
impl EvalAccountResolutionError {
    pub fn pubkey(&self) -> Pubkey {
        match self {
            Self::DuplicateDynamicAccount { pubkey }
            | Self::UnexpectedDynamicAccount { pubkey }
            | Self::DuplicateOutputAuthority { pubkey }
            | Self::UnexpectedOutputAuthority { pubkey } => *pubkey,
            Self::MissingDynamicAccount { requirement }
            | Self::DynamicAccountNotWritable { requirement } => requirement.pubkey(),
            Self::MissingOutputAuthority { authority } => authority.pubkey(),
        }
    }
}

/// Ordered dynamic accounts resolved from an [`EvalPlan`].
#[cfg(feature = "cpi")]
#[derive(Debug)]
pub struct ResolvedEvalAccounts<'info> {
    accounts: Vec<AccountInfo<'info>>,
}

#[cfg(feature = "cpi")]
impl<'info> ResolvedEvalAccounts<'info> {
    pub fn account_infos(&self) -> &[AccountInfo<'info>] {
        &self.accounts
    }

    pub fn resolve(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>> {
        self.accounts
            .iter()
            .find(|candidate| candidate.key() == pubkey)
            .cloned()
    }
}

/// Opaque lowered eval request produced by [`EvalBuilder::finish`] or
/// [`EvalPlan::build`].
///
/// App code passes this to [`invoke_eval_signed_resolved`] instead of editing
/// raw host args or dynamic account roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalPlan {
    app_authority: EvalAppAuthority,
    args: FheEvalArgs,
    /// Exact dynamic `remaining_accounts` order referenced by the `u16` indices
    /// inside `args`. Keep this coupled to `args`; `finish` validates every
    /// index before constructing the plan.
    remaining_accounts: Vec<EvalAccountMeta>,
}

impl EvalPlan {
    /// Builds and validates an eval plan through a closure.
    ///
    /// This keeps transient values scoped to one builder while removing the
    /// need for app code to call [`EvalBuilder::finish`] explicitly.
    pub fn build<T, F>(
        context_id: EvalContextId,
        app_authority: EvalAppAuthority,
        build: F,
    ) -> Result<Self>
    where
        F: FnOnce(&mut EvalBuilder) -> Result<T>,
    {
        let mut builder = EvalBuilder::new(context_id, app_authority);
        build(&mut builder)?;
        builder.finish()
    }

    pub fn app_authority(&self) -> EvalAppAuthority {
        self.app_authority
    }

    pub fn dynamic_account_requirements(
        &self,
    ) -> impl ExactSizeIterator<Item = EvalAccountRequirement> + '_ {
        self.remaining_accounts
            .iter()
            .map(EvalAccountRequirement::from)
    }

    #[cfg(feature = "cpi")]
    /// Resolves unordered app-supplied accounts into the exact host
    /// `remaining_accounts` order for this plan.
    ///
    /// `dynamic_accounts` must contain only non-authority plan accounts such as
    /// durable input ACLs, permission records, transient sessions, and writable
    /// durable output ACL records. `output_authorities` must contain signer
    /// witnesses for durable outputs whose app account is not the fixed CPI
    /// `app_account_authority`.
    pub fn resolve_accounts<'info>(
        &self,
        dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
        output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
    ) -> std::result::Result<ResolvedEvalAccounts<'info>, EvalAccountResolutionError> {
        resolve_eval_accounts(self, dynamic_accounts, output_authorities)
    }

    pub fn output_authority_requirements(
        &self,
    ) -> impl Iterator<Item = EvalOutputAuthorityRequirement> + '_ {
        std::iter::once(EvalOutputAuthorityRequirement {
            pubkey: self.app_authority.pubkey(),
            cpi_account_authority: true,
        })
        .chain(self.additional_output_authorities().map(|pubkey| {
            EvalOutputAuthorityRequirement {
                pubkey,
                cpi_account_authority: false,
            }
        }))
    }

    pub fn output_authorities(&self) -> impl Iterator<Item = Pubkey> + '_ {
        self.output_authority_requirements()
            .map(|requirement| requirement.pubkey())
    }

    pub fn additional_output_authorities(&self) -> impl Iterator<Item = Pubkey> + '_ {
        self.remaining_accounts
            .iter()
            .filter(|account| {
                account
                    .purposes
                    .contains(&EvalAccountPurpose::DurableOutputAuthority)
            })
            .map(|account| account.pubkey)
    }

    /// Subjects this plan grants through a supersede that were not already on the
    /// stored lineage — `output_subjects \ previous_subjects` for each durable
    /// output that rotates its audience. The host deny-list-checks each of these
    /// exactly like `allow_subjects`, so an app forwarding deny-record witnesses
    /// must cover them alongside the output authorities.
    pub fn rotation_added_subjects(&self) -> Vec<Pubkey> {
        let mut added = Vec::new();
        for step in &self.args.steps {
            let FheEvalOutput::AllowedDurable {
                output_subjects,
                previous_subjects: Some(previous_subjects),
                ..
            } = fhe_eval_step_output(step)
            else {
                continue;
            };
            for entry in output_subjects {
                if !previous_subjects.contains(&entry.pubkey) && !added.contains(&entry.pubkey) {
                    added.push(entry.pubkey);
                }
            }
        }
        added
    }
}

/// The output policy of an eval step, independent of step kind.
fn fhe_eval_step_output(step: &FheEvalStep) -> &FheEvalOutput {
    match step {
        FheEvalStep::Binary { output, .. }
        | FheEvalStep::Ternary { output, .. }
        | FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. }
        | FheEvalStep::Unary { output, .. }
        | FheEvalStep::RandBounded { output, .. }
        | FheEvalStep::Sum { output, .. }
        | FheEvalStep::IsIn { output, .. }
        | FheEvalStep::MulDiv { output, .. } => output,
    }
}

#[cfg(feature = "cpi")]
fn resolve_eval_accounts<'info>(
    plan: &EvalPlan,
    dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
    output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
) -> std::result::Result<ResolvedEvalAccounts<'info>, EvalAccountResolutionError> {
    let dynamic_accounts = dynamic_accounts.into_iter().collect::<Vec<_>>();
    let output_authorities = output_authorities.into_iter().collect::<Vec<_>>();

    for (index, account) in dynamic_accounts.iter().enumerate() {
        let pubkey = account.key();
        if dynamic_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(EvalAccountResolutionError::DuplicateDynamicAccount { pubkey });
        }
        let Some(required) = plan
            .dynamic_account_requirements()
            .find(|required| required.pubkey() == pubkey)
        else {
            return Err(EvalAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        };
        if !required.requires_dynamic_account() {
            return Err(EvalAccountResolutionError::UnexpectedDynamicAccount { pubkey });
        }
    }

    for (index, authority) in output_authorities.iter().enumerate() {
        let pubkey = authority.key();
        if output_authorities[index + 1..]
            .iter()
            .any(|candidate| candidate.key() == pubkey)
        {
            return Err(EvalAccountResolutionError::DuplicateOutputAuthority { pubkey });
        }
        if !plan.output_authorities().any(|required| required == pubkey) {
            return Err(EvalAccountResolutionError::UnexpectedOutputAuthority { pubkey });
        }
    }

    for authority in plan.output_authority_requirements() {
        if !output_authorities
            .iter()
            .any(|candidate| candidate.key() == authority.pubkey())
        {
            return Err(EvalAccountResolutionError::MissingOutputAuthority { authority });
        }
    }

    let mut accounts = Vec::new();
    for required in plan.dynamic_account_requirements() {
        let account = if required.requires_output_authority() {
            output_authorities
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or(EvalAccountResolutionError::MissingOutputAuthority {
                    authority: EvalOutputAuthorityRequirement {
                        pubkey: required.pubkey(),
                        cpi_account_authority: false,
                    },
                })?
        } else if required.requires_dynamic_account() {
            dynamic_accounts
                .iter()
                .find(|candidate| candidate.key() == required.pubkey())
                .cloned()
                .ok_or_else(|| EvalAccountResolutionError::MissingDynamicAccount {
                    requirement: required.clone(),
                })?
        } else {
            continue;
        };
        if required.is_writable() && !account.is_writable {
            return Err(EvalAccountResolutionError::DynamicAccountNotWritable {
                requirement: required,
            });
        }
        accounts.push(account);
    }

    Ok(ResolvedEvalAccounts { accounts })
}

/// Pubkey-oriented builder for `FheEvalArgs`.
#[derive(Debug)]
pub struct EvalBuilder {
    context_id: EvalContextId,
    scope: EvalBuilderScope,
    app_authority: EvalAppAuthority,
    steps: Vec<FheEvalStep>,
    produced_types: Vec<u8>,
    remaining_accounts: Vec<EvalAccountMeta>,
    /// Coprocessor attestations backing `VerifiedInput` operands, referenced by index. Held here
    /// (rather than inline in the operand) so `Operand` stays `Copy`.
    verified_inputs: Vec<CoprocessorInputAttestation>,
}

impl Clone for EvalBuilder {
    fn clone(&self) -> Self {
        Self {
            context_id: self.context_id,
            scope: next_eval_builder_scope(),
            app_authority: self.app_authority,
            steps: self.steps.clone(),
            produced_types: self.produced_types.clone(),
            remaining_accounts: self.remaining_accounts.clone(),
            verified_inputs: self.verified_inputs.clone(),
        }
    }
}

impl EvalBuilder {
    pub fn new(context_id: EvalContextId, app_authority: EvalAppAuthority) -> Self {
        Self {
            context_id,
            scope: next_eval_builder_scope(),
            app_authority,
            steps: Vec::new(),
            produced_types: Vec::new(),
            remaining_accounts: Vec::new(),
            verified_inputs: Vec::new(),
        }
    }

    /// Introduces a coprocessor-attested external input as a transient operand — the Solana analog
    /// of EVM `FHE.fromExternal`. The host re-verifies the attestation in-frame and requires the
    /// caller to be the attested contract (`compute_subject == contract_address`); derived outputs
    /// are then unconstrained, exactly like EVM `allowTransient(input, msg.sender)`. The returned
    /// value is an operand usable only in later steps of this builder.
    pub fn verified_input<T: FheTyped>(
        &mut self,
        attestation: CoprocessorInputAttestation,
    ) -> Result<Encrypted<T>> {
        if handle_fhe_type(attestation.input_handle) != T::FHE_TYPE.byte() {
            return Err(EvalBuildError::UnsupportedFheType);
        }
        let attestation_index =
            u16::try_from(self.verified_inputs.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let input_handle = attestation.input_handle;
        self.verified_inputs.push(attestation);
        Ok(Encrypted::from_operand(Operand::verified_input(
            input_handle,
            attestation_index,
        )))
    }

    pub fn add<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Add,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn sub<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Sub,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn ge<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Ge,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    fn binary_op(
        &mut self,
        op: FheBinaryOpCode,
        lhs: Operand,
        rhs: Operand,
        output_fhe_type: FheType,
        output: Output,
    ) -> Result<Operand> {
        let output_fhe_type = output_fhe_type.byte();
        // The host requires the left operand to be an encrypted handle; only the
        // RHS may be a plaintext scalar. Catch this before the CPI.
        if matches!(lhs.0, OperandKind::Scalar(_)) {
            return Err(EvalBuildError::ScalarLhsOperand);
        }
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        validate_binary_step(
            op,
            &lhs,
            &rhs,
            output_fhe_type,
            self.steps.len(),
            self.context_id,
            self.scope,
            |index| self.produced_types.get(index as usize).copied(),
        )?;
        let op_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let lhs = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            lhs,
        )?;
        let rhs = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            rhs,
        )?;
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::Binary {
            op,
            lhs,
            rhs,
            output_fhe_type,
            output,
        });
        self.produced_types.push(output_fhe_type);
        Ok(Operand::transient(op_index, self.context_id, self.scope))
    }

    pub fn if_then_else<T: FheTyped>(
        &mut self,
        control: Encrypted<Bool>,
        if_true: Encrypted<T>,
        if_false: Encrypted<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        let control = control.operand();
        let if_true = if_true.operand();
        let if_false = if_false.operand();
        let output_fhe_type =
            self.encrypted_operand_type(&if_true, EvalBuildError::ScalarEncryptedOperand)?;
        let output_fhe_type = output_fhe_type.byte();
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        validate_ternary_step(
            &control,
            &if_true,
            &if_false,
            output_fhe_type,
            self.steps.len(),
            |index| self.produced_types.get(index as usize).copied(),
            self.context_id,
            self.scope,
        )?;
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let control = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            control,
        )?;
        let if_true = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            if_true,
        )?;
        let if_false = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            if_false,
        )?;
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::Ternary {
            op: FheTernaryOpCode::IfThenElse,
            control,
            if_true,
            if_false,
            output_fhe_type,
            output,
        });
        self.produced_types.push(output_fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index,
            self.context_id,
            self.scope,
        )))
    }

    pub fn trivial_encrypt<T: FheTyped>(
        &mut self,
        plaintext: Scalar<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.trivial_encrypt_raw(plaintext.bytes(), T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    fn trivial_encrypt_raw(
        &mut self,
        plaintext: [u8; 32],
        fhe_type: FheType,
        output: Output,
    ) -> Result<Operand> {
        let fhe_type = fhe_type.byte();
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        validate_supported_fhe_type(fhe_type)?;
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::TrivialEncrypt {
            plaintext,
            fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Operand::transient(step_index, self.context_id, self.scope))
    }

    pub fn trivial_encrypt_u64(
        &mut self,
        plaintext: u64,
        output: Output,
    ) -> Result<Encrypted<Uint<64>>> {
        self.trivial_encrypt(Scalar::<Uint<64>>::u64(plaintext), output)
    }

    pub fn rand<T: FheRandom>(&mut self, output: Output) -> Result<Encrypted<T>> {
        self.rand_raw(T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    fn rand_raw(&mut self, fhe_type: FheType, output: Output) -> Result<Operand> {
        let fhe_type = fhe_type.byte();
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        validate_supported_rand_type(fhe_type)?;
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::Rand { fhe_type, output });
        self.produced_types.push(fhe_type);
        Ok(Operand::transient(step_index, self.context_id, self.scope))
    }

    pub fn rand_u64(&mut self, output: Output) -> Result<Encrypted<Uint<64>>> {
        self.rand::<Uint<64>>(output)
    }

    pub fn rand_bounded_u64(
        &mut self,
        upper_bound: BoundedU64UpperBound,
        output: Output,
    ) -> Result<Encrypted<Uint<64>>> {
        let fhe_type = FheType::UINT64.byte();
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::RandBounded {
            upper_bound: upper_bound.bytes(),
            fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index,
            self.context_id,
            self.scope,
        )))
    }

    // --- Binary ops not yet exposed as named methods ---

    pub fn mul<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Mul,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn div<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Div,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rem<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Rem,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn and<T: FheBitwise>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::And,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn or<T: FheBitwise>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Or,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn xor<T: FheBitwise>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Xor,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn shl<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Shl,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn shr<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Shr,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rotl<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Rotl,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn rotr<T: FheShift>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Rotr,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn eq<T: FheEq>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Eq,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn ne<T: FheEq>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Ne,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn gt<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Gt,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn le<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Le,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn lt<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        self.binary_op(
            FheBinaryOpCode::Lt,
            lhs.operand(),
            binary_rhs_operand(rhs),
            FheType::BOOL,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn min<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Min,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn max<T: FheUint>(
        &mut self,
        lhs: Encrypted<T>,
        rhs: impl Into<BinaryRhs<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.binary_op(
            FheBinaryOpCode::Max,
            lhs.operand(),
            binary_rhs_operand(rhs),
            T::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    // --- Unary ops ---

    pub fn neg<T: FheNeg>(
        &mut self,
        operand: Encrypted<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.unary_op(FheUnaryOpCode::Neg, operand.operand(), T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    pub fn not<T: FheNot>(
        &mut self,
        operand: Encrypted<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        self.unary_op(FheUnaryOpCode::Not, operand.operand(), T::FHE_TYPE, output)
            .map(Encrypted::from_operand)
    }

    pub fn cast<FROM: FheTyped, TO: FheTyped>(
        &mut self,
        operand: Encrypted<FROM>,
        output: Output,
    ) -> Result<Encrypted<TO>> {
        self.unary_op(
            FheUnaryOpCode::Cast,
            operand.operand(),
            TO::FHE_TYPE,
            output,
        )
        .map(Encrypted::from_operand)
    }

    pub fn sum<T: FheUint>(
        &mut self,
        operands: impl IntoIterator<Item = Encrypted<T>>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        // EVM `fheSum` and the coprocessor enforce no minimum: a zero/single-operand sum is valid.
        let operand_ops: Vec<Operand> = operands.into_iter().map(|e| e.operand()).collect();
        for op in &operand_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(EvalBuildError::ScalarEncryptedOperand);
            }
        }
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_uint_fhe_type(fhe_type)?;
        if operand_ops.len() > max_reduction_operands(fhe_type) {
            return Err(EvalBuildError::TooManyReductionOperands);
        }
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let mut lowered: Vec<FheEvalOperand> = Vec::with_capacity(operand_ops.len());
        for op in operand_ops {
            lowered.push(lower_operand(
                &mut remaining_accounts,
                self.steps.len(),
                self.context_id,
                self.scope,
                &self.verified_inputs,
                op,
            )?);
        }
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::Sum {
            operands: lowered,
            fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index,
            self.context_id,
            self.scope,
        )))
    }

    pub fn is_in<T: FheIsIn>(
        &mut self,
        value: Encrypted<T>,
        set: impl IntoIterator<Item = Encrypted<T>>,
        output: Output,
    ) -> Result<Encrypted<Bool>> {
        // EVM `fheIsIn` and the coprocessor enforce no minimum: an empty set is valid (false result).
        let set_ops: Vec<Operand> = set.into_iter().map(|e| e.operand()).collect();
        let value_op = value.operand();
        if matches!(value_op.0, OperandKind::Scalar(_)) {
            return Err(EvalBuildError::ScalarEncryptedOperand);
        }
        for op in &set_ops {
            if matches!(op.0, OperandKind::Scalar(_)) {
                return Err(EvalBuildError::ScalarEncryptedOperand);
            }
        }
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_supported_fhe_type(fhe_type)?;
        if set_ops.len() > max_reduction_operands(fhe_type) {
            return Err(EvalBuildError::TooManyReductionOperands);
        }
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let value_lowered = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            value_op,
        )?;
        let mut set_lowered: Vec<FheEvalOperand> = Vec::with_capacity(set_ops.len());
        for op in set_ops {
            set_lowered.push(lower_operand(
                &mut remaining_accounts,
                self.steps.len(),
                self.context_id,
                self.scope,
                &self.verified_inputs,
                op,
            )?);
        }
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        let bool_type = FheType::BOOL.byte();
        self.steps.push(FheEvalStep::IsIn {
            value: value_lowered,
            set: set_lowered,
            fhe_type,
            output,
        });
        self.produced_types.push(bool_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index,
            self.context_id,
            self.scope,
        )))
    }

    pub fn mul_div<T: FheUint>(
        &mut self,
        factor1: Encrypted<T>,
        factor2: impl Into<BinaryRhs<T>>,
        divisor: Scalar<T>,
        output: Output,
    ) -> Result<Encrypted<T>> {
        let lhs = factor1.operand();
        let rhs = binary_rhs_operand(factor2);
        if matches!(lhs.0, OperandKind::Scalar(_)) {
            return Err(EvalBuildError::ScalarLhsOperand);
        }
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        let fhe_type = T::FHE_TYPE.byte();
        validate_uint_fhe_type(fhe_type)?;
        // fheMulDiv factor1 caps at Uint64 (EVM + coprocessor); reject Uint128.
        if !matches!(fhe_type, 2..=5) {
            return Err(EvalBuildError::UnsupportedFheType);
        }
        // Divisor must be non-zero once truncated to the operand type (EVM DivisionByZero parity).
        let divisor_bytes = divisor.bytes();
        if scalar_is_zero_for_type(divisor_bytes, fhe_type) {
            return Err(EvalBuildError::MulDivDivisorZero);
        }
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let factor1 = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            lhs,
        )?;
        let factor2 = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            rhs,
        )?;
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::MulDiv {
            factor1,
            factor2,
            divisor: divisor_bytes,
            output_fhe_type: fhe_type,
            output,
        });
        self.produced_types.push(fhe_type);
        Ok(Encrypted::from_operand(Operand::transient(
            step_index,
            self.context_id,
            self.scope,
        )))
    }

    fn unary_op(
        &mut self,
        op: FheUnaryOpCode,
        operand: Operand,
        output_fhe_type: FheType,
        output: Output,
    ) -> Result<Operand> {
        let output_fhe_type = output_fhe_type.byte();
        if matches!(operand.0, OperandKind::Scalar(_)) {
            return Err(EvalBuildError::ScalarEncryptedOperand);
        }
        if self.steps.len() >= MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        validate_unary_step(
            op,
            &operand,
            output_fhe_type,
            self.steps.len(),
            self.context_id,
            self.scope,
            |index| self.produced_types.get(index as usize).copied(),
        )?;
        let step_index = u16::try_from(self.steps.len()).map_err(|_| EvalBuildError::TooManyOps)?;
        let mut remaining_accounts = self.remaining_accounts.clone();
        let operand = lower_operand(
            &mut remaining_accounts,
            self.steps.len(),
            self.context_id,
            self.scope,
            &self.verified_inputs,
            operand,
        )?;
        let output = lower_output(&mut remaining_accounts, self.app_authority, output)?;
        self.remaining_accounts = remaining_accounts;
        self.steps.push(FheEvalStep::Unary {
            op,
            operand,
            output_fhe_type,
            output,
        });
        self.produced_types.push(output_fhe_type);
        Ok(Operand::transient(step_index, self.context_id, self.scope))
    }

    fn encrypted_operand_type(
        &self,
        operand: &Operand,
        scalar_error: EvalBuildError,
    ) -> Result<FheType> {
        let fhe_type = operand_fhe_type(
            operand,
            self.steps.len(),
            self.context_id,
            self.scope,
            &|index| self.produced_types.get(index as usize).copied(),
        )?
        .ok_or(scalar_error)?;
        FheType::from_host_byte(fhe_type)
    }

    /// Validates the accumulated frame and lowers it to an [`EvalPlan`].
    ///
    /// Mirrors the host admission checks (`context_id != 0`, non-empty steps,
    /// `steps.len() <= MAX_FHE_EVAL_OPS`) so a malformed frame fails locally
    /// instead of on-chain.
    ///
    /// Not mirrored (it depends on the deployed `hcu_block_cap_per_app`, unknown here): under a
    /// finite block cap the host rejects a persist-nothing frame — one binding no durable input, no
    /// verified input, and no durable output — with `FheEvalUnanchoredUnderBlockCap`
    /// (fhevm-internal#1744). Give such a frame a durable output (the bootstrap/mint path) or a
    /// verified input if it must run under a finite cap.
    pub fn finish(self) -> Result<EvalPlan> {
        validate_app_authority(self.app_authority)?;
        if self.context_id.bytes() == [0u8; 32] {
            return Err(EvalBuildError::EmptyContextId);
        }
        if self.steps.is_empty() {
            return Err(EvalBuildError::EmptyOps);
        }
        if self.steps.len() > MAX_FHE_EVAL_OPS {
            return Err(EvalBuildError::TooManyOps);
        }
        validate_lowered_eval_plan(&self.steps, &self.remaining_accounts)?;
        Ok(EvalPlan {
            app_authority: self.app_authority,
            args: FheEvalArgs {
                context_id: self.context_id.bytes(),
                steps: self.steps,
            },
            remaining_accounts: self.remaining_accounts,
        })
    }
}

fn validate_lowered_eval_plan(
    steps: &[FheEvalStep],
    remaining_accounts: &[EvalAccountMeta],
) -> Result<()> {
    if u16::try_from(remaining_accounts.len()).is_err() {
        return Err(EvalBuildError::TooManyRemainingAccounts);
    }
    for (index, account) in remaining_accounts.iter().enumerate() {
        if account.pubkey == Pubkey::default() || account.purposes.is_empty() {
            return Err(EvalBuildError::InvalidRemainingAccountReference);
        }
        if remaining_accounts[index + 1..]
            .iter()
            .any(|candidate| candidate.pubkey == account.pubkey)
        {
            return Err(EvalBuildError::InvalidRemainingAccountReference);
        }
    }

    let mut used_accounts = vec![false; remaining_accounts.len()];
    for (step_index, step) in steps.iter().enumerate() {
        validate_lowered_step(step, step_index, &mut used_accounts)?;
    }
    if used_accounts.iter().any(|used| !*used) {
        return Err(EvalBuildError::InvalidRemainingAccountReference);
    }
    Ok(())
}

fn validate_lowered_step(
    step: &FheEvalStep,
    step_index: usize,
    used_accounts: &mut [bool],
) -> Result<()> {
    match step {
        FheEvalStep::Binary {
            lhs, rhs, output, ..
        } => {
            validate_lowered_encrypted_operand(lhs, step_index, used_accounts)?;
            validate_lowered_rhs_operand(rhs, step_index, used_accounts)?;
            validate_lowered_output(output, used_accounts)?;
        }
        FheEvalStep::Ternary {
            control,
            if_true,
            if_false,
            output,
            ..
        } => {
            validate_lowered_encrypted_operand(control, step_index, used_accounts)?;
            validate_lowered_encrypted_operand(if_true, step_index, used_accounts)?;
            validate_lowered_encrypted_operand(if_false, step_index, used_accounts)?;
            validate_lowered_output(output, used_accounts)?;
        }
        FheEvalStep::TrivialEncrypt { output, .. }
        | FheEvalStep::Rand { output, .. }
        | FheEvalStep::RandBounded { output, .. } => {
            validate_lowered_output(output, used_accounts)?
        }
        FheEvalStep::Unary {
            operand, output, ..
        } => {
            validate_lowered_encrypted_operand(operand, step_index, used_accounts)?;
            validate_lowered_output(output, used_accounts)?;
        }
        FheEvalStep::Sum {
            operands, output, ..
        } => {
            for operand in operands {
                validate_lowered_encrypted_operand(operand, step_index, used_accounts)?;
            }
            validate_lowered_output(output, used_accounts)?;
        }
        FheEvalStep::IsIn {
            value, set, output, ..
        } => {
            validate_lowered_encrypted_operand(value, step_index, used_accounts)?;
            for operand in set {
                validate_lowered_encrypted_operand(operand, step_index, used_accounts)?;
            }
            validate_lowered_output(output, used_accounts)?;
        }
        FheEvalStep::MulDiv {
            factor1,
            factor2,
            output,
            ..
        } => {
            validate_lowered_encrypted_operand(factor1, step_index, used_accounts)?;
            validate_lowered_rhs_operand(factor2, step_index, used_accounts)?;
            validate_lowered_output(output, used_accounts)?;
        }
    }
    Ok(())
}

fn validate_lowered_rhs_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    used_accounts: &mut [bool],
) -> Result<()> {
    match operand {
        FheEvalOperand::Scalar(_) => Ok(()),
        _ => validate_lowered_encrypted_operand(operand, step_index, used_accounts),
    }
}

fn validate_lowered_encrypted_operand(
    operand: &FheEvalOperand,
    step_index: usize,
    used_accounts: &mut [bool],
) -> Result<()> {
    match operand {
        FheEvalOperand::AllowedDurable {
            encrypted_value_index,
            ..
        } => {
            mark_lowered_account(used_accounts, *encrypted_value_index)?;
        }
        FheEvalOperand::AllowedLocal { producer_index } => {
            if *producer_index as usize >= step_index {
                return Err(EvalBuildError::InvalidTransientReference);
            }
        }
        FheEvalOperand::VerifiedInput { .. } => {
            // No remaining account: the attestation is carried inline and verified in-frame.
        }
        FheEvalOperand::Scalar(_) => return Err(EvalBuildError::ScalarEncryptedOperand),
    }
    Ok(())
}

fn validate_lowered_output(output: &FheEvalOutput, used_accounts: &mut [bool]) -> Result<()> {
    match output {
        FheEvalOutput::AllowedLocal => {}
        FheEvalOutput::AllowedDurable {
            output_encrypted_value_index,
            output_app_account_authority_index,
            ..
        } => {
            mark_lowered_account(used_accounts, *output_encrypted_value_index)?;
            if let Some(index) = output_app_account_authority_index {
                mark_lowered_account(used_accounts, *index)?;
            }
        }
    }
    Ok(())
}

fn mark_lowered_account(used_accounts: &mut [bool], index: u16) -> Result<()> {
    let used = used_accounts
        .get_mut(index as usize)
        .ok_or(EvalBuildError::InvalidRemainingAccountReference)?;
    *used = true;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_binary_step<F>(
    op: FheBinaryOpCode,
    lhs: &Operand,
    rhs: &Operand,
    output_fhe_type: u8,
    produced_count: usize,
    context_id: EvalContextId,
    builder_scope: EvalBuilderScope,
    produced_type: F,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_binary_output_type(op, output_fhe_type)?;

    let lhs_type = operand_fhe_type(
        lhs,
        produced_count,
        context_id,
        builder_scope,
        &produced_type,
    )?
    .ok_or(EvalBuildError::ScalarLhsOperand)?;
    match op {
        // Eq/Ne accept the widest operand set (Bool..Uint256); ordered comparisons Uint8..Uint128.
        FheBinaryOpCode::Eq | FheBinaryOpCode::Ne => {
            if !matches!(lhs_type, 0 | 2..=8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
        }
        FheBinaryOpCode::Ge | FheBinaryOpCode::Gt | FheBinaryOpCode::Le | FheBinaryOpCode::Lt => {
            if !matches!(lhs_type, 2..=6) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
        }
        // Div/Rem: divisor must be a plaintext scalar (EVM `IsNotScalar`), non-zero after truncation.
        FheBinaryOpCode::Div | FheBinaryOpCode::Rem => {
            if lhs_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
            match &rhs.0 {
                OperandKind::Scalar(value) => {
                    if scalar_is_zero_for_type(*value, output_fhe_type) {
                        return Err(EvalBuildError::DivisionByZero);
                    }
                }
                OperandKind::Durable(_)
                | OperandKind::Transient { .. }
                | OperandKind::VerifiedInput { .. } => {
                    return Err(EvalBuildError::DivisorMustBeScalar)
                }
            }
        }
        // Remaining ops: operand type must equal the (op-gated) output type.
        FheBinaryOpCode::Add
        | FheBinaryOpCode::Sub
        | FheBinaryOpCode::Mul
        | FheBinaryOpCode::And
        | FheBinaryOpCode::Or
        | FheBinaryOpCode::Xor
        | FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr
        | FheBinaryOpCode::Min
        | FheBinaryOpCode::Max => {
            if lhs_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
        }
    }
    if let Some(rhs_type) = operand_fhe_type(
        rhs,
        produced_count,
        context_id,
        builder_scope,
        &produced_type,
    )? {
        if rhs_type != lhs_type {
            return Err(EvalBuildError::BinaryOperandTypeMismatch);
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_unary_step<F>(
    op: FheUnaryOpCode,
    operand: &Operand,
    output_fhe_type: u8,
    produced_count: usize,
    context_id: EvalContextId,
    builder_scope: EvalBuilderScope,
    produced_type: F,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_fhe_type(output_fhe_type)?;
    let valid_output = match op {
        FheUnaryOpCode::Neg => matches!(output_fhe_type, 2..=6 | 8),
        FheUnaryOpCode::Not => matches!(output_fhe_type, 0 | 2..=6 | 8),
        // EVM `cast` output set: Uint8..Uint128 | Uint256 (no ebool, no eaddress/Uint160).
        FheUnaryOpCode::Cast => matches!(output_fhe_type, 2..=6 | 8),
    };
    if !valid_output {
        return Err(EvalBuildError::UnsupportedFheType);
    }
    let operand_type = operand_fhe_type(
        operand,
        produced_count,
        context_id,
        builder_scope,
        &produced_type,
    )?
    .ok_or(EvalBuildError::ScalarEncryptedOperand)?;
    validate_supported_fhe_type(operand_type)?;
    match op {
        FheUnaryOpCode::Neg => {
            if !matches!(operand_type, 2..=6 | 8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Not => {
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
            if operand_type != output_fhe_type {
                return Err(EvalBuildError::BinaryOperandTypeMismatch);
            }
        }
        FheUnaryOpCode::Cast => {
            // EVM `cast` input set: Bool | Uint8..Uint128 | Uint256 (no eaddress/Uint160).
            if !matches!(operand_type, 0 | 2..=6 | 8) {
                return Err(EvalBuildError::UnsupportedFheType);
            }
            // Same-type cast is rejected (EVM InvalidType parity).
            if operand_type == output_fhe_type {
                return Err(EvalBuildError::UnsupportedFheType);
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_ternary_step<F>(
    control: &Operand,
    if_true: &Operand,
    if_false: &Operand,
    output_fhe_type: u8,
    produced_count: usize,
    produced_type: F,
    context_id: EvalContextId,
    builder_scope: EvalBuilderScope,
) -> Result<()>
where
    F: Fn(u16) -> Option<u8>,
{
    validate_supported_fhe_type(output_fhe_type)?;

    let control_type = operand_fhe_type(
        control,
        produced_count,
        context_id,
        builder_scope,
        &produced_type,
    )?
    .ok_or(EvalBuildError::ScalarEncryptedOperand)?;
    let true_type = operand_fhe_type(
        if_true,
        produced_count,
        context_id,
        builder_scope,
        &produced_type,
    )?
    .ok_or(EvalBuildError::ScalarEncryptedOperand)?;
    let false_type = operand_fhe_type(
        if_false,
        produced_count,
        context_id,
        builder_scope,
        &produced_type,
    )?
    .ok_or(EvalBuildError::ScalarEncryptedOperand)?;

    if control_type != 0 || true_type != output_fhe_type || false_type != output_fhe_type {
        return Err(EvalBuildError::TernaryOperandTypeMismatch);
    }
    Ok(())
}

fn operand_fhe_type<F>(
    operand: &Operand,
    produced_count: usize,
    context_id: EvalContextId,
    builder_scope: EvalBuilderScope,
    produced_type: &F,
) -> Result<Option<u8>>
where
    F: Fn(u16) -> Option<u8>,
{
    match &operand.0 {
        OperandKind::Durable(durable) => Ok(Some(handle_fhe_type(durable.handle))),
        OperandKind::Transient {
            producer_index,
            context_id: operand_context_id,
            builder_scope: operand_builder_scope,
        } => {
            if *operand_context_id != context_id || *operand_builder_scope != builder_scope {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            if *producer_index as usize >= produced_count {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            produced_type(*producer_index)
                .map(Some)
                .ok_or(EvalBuildError::InvalidTransientReference)
        }
        OperandKind::VerifiedInput { input_handle, .. } => Ok(Some(handle_fhe_type(*input_handle))),
        OperandKind::Scalar(_) => Ok(None),
    }
}

fn validate_supported_binary_output_type(op: FheBinaryOpCode, output_fhe_type: u8) -> Result<()> {
    validate_supported_fhe_type(output_fhe_type)?;
    let valid = match op {
        FheBinaryOpCode::Add
        | FheBinaryOpCode::Sub
        | FheBinaryOpCode::Mul
        | FheBinaryOpCode::Div
        | FheBinaryOpCode::Rem
        | FheBinaryOpCode::Min
        | FheBinaryOpCode::Max => matches!(output_fhe_type, 2..=6),
        FheBinaryOpCode::And | FheBinaryOpCode::Or | FheBinaryOpCode::Xor => {
            matches!(output_fhe_type, 0 | 2..=6 | 8)
        }
        FheBinaryOpCode::Shl
        | FheBinaryOpCode::Shr
        | FheBinaryOpCode::Rotl
        | FheBinaryOpCode::Rotr => matches!(output_fhe_type, 2..=6 | 8),
        FheBinaryOpCode::Eq
        | FheBinaryOpCode::Ne
        | FheBinaryOpCode::Ge
        | FheBinaryOpCode::Gt
        | FheBinaryOpCode::Le
        | FheBinaryOpCode::Lt => output_fhe_type == 0,
    };
    if !valid {
        return Err(EvalBuildError::UnsupportedBinaryOutputType);
    }
    Ok(())
}

fn validate_supported_fhe_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8) {
        Ok(())
    } else {
        Err(EvalBuildError::UnsupportedFheType)
    }
}

/// Mirrors the host `scalar_is_zero_for_type` (EVM `_isScalarZeroForType`): zero after width truncation.
fn scalar_is_zero_for_type(scalar: [u8; 32], fhe_type: u8) -> bool {
    let width = match fhe_type {
        2 => 1,
        3 => 2,
        4 => 4,
        5 => 8,
        6 => 16,
        _ => 32,
    };
    scalar[32 - width..].iter().all(|byte| *byte == 0)
}

/// Coprocessor max operand count for FheSum / FheIsIn: 100 for narrow types, 60 for wider ones.
fn max_reduction_operands(fhe_type: u8) -> usize {
    match fhe_type {
        2..=4 => 100,
        _ => 60,
    }
}

fn validate_uint_fhe_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 2..=6) {
        Ok(())
    } else {
        Err(EvalBuildError::UnsupportedFheType)
    }
}

fn validate_supported_rand_type(fhe_type: u8) -> Result<()> {
    if matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 8) {
        Ok(())
    } else {
        Err(EvalBuildError::UnsupportedFheType)
    }
}

fn validate_access_policy(subjects: &[AccessSubject]) -> Result<()> {
    if subjects.is_empty() || subjects.len() > MAX_ACL_SUBJECTS {
        return Err(EvalBuildError::InvalidAccessPolicy);
    }
    for (index, subject) in subjects.iter().enumerate() {
        if subject.pubkey == Pubkey::default() {
            return Err(EvalBuildError::InvalidAccessPolicy);
        }
        if subjects[..index]
            .iter()
            .any(|previous| previous.pubkey == subject.pubkey)
        {
            return Err(EvalBuildError::InvalidAccessPolicy);
        }
    }
    Ok(())
}

fn validate_durable_slot(slot: &DurableSlot) -> Result<()> {
    if slot.namespace == Pubkey::default() || slot.account == Pubkey::default() {
        return Err(EvalBuildError::InvalidDurableSlot);
    }
    Ok(())
}

fn validate_app_authority(authority: EvalAppAuthority) -> Result<()> {
    if authority.pubkey() == Pubkey::default() {
        return Err(EvalBuildError::InvalidAppAuthority);
    }
    Ok(())
}

fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}

fn lower_operand(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    produced_count: usize,
    context_id: EvalContextId,
    builder_scope: EvalBuilderScope,
    verified_inputs: &[CoprocessorInputAttestation],
    operand: Operand,
) -> Result<FheEvalOperand> {
    match operand.0 {
        OperandKind::Durable(durable) => {
            let encrypted_value_index = account_index(
                remaining_accounts,
                EvalAccountMeta::readonly(
                    durable.encrypted_value,
                    EvalAccountPurpose::DurableInputAcl,
                ),
            )?;
            Ok(FheEvalOperand::AllowedDurable {
                handle: durable.handle,
                encrypted_value_index,
            })
        }
        OperandKind::Transient {
            producer_index,
            context_id: operand_context_id,
            builder_scope: operand_builder_scope,
        } => {
            if operand_context_id != context_id || operand_builder_scope != builder_scope {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            if producer_index as usize >= produced_count {
                return Err(EvalBuildError::InvalidTransientReference);
            }
            Ok(FheEvalOperand::AllowedLocal { producer_index })
        }
        OperandKind::VerifiedInput {
            attestation_index, ..
        } => {
            let attestation = verified_inputs
                .get(attestation_index as usize)
                .ok_or(EvalBuildError::MissingVerifiedInput)?
                .clone();
            Ok(FheEvalOperand::VerifiedInput {
                attestation: Box::new(attestation),
            })
        }
        OperandKind::Scalar(value) => Ok(FheEvalOperand::Scalar(value)),
    }
}

fn lower_output(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    app_authority: EvalAppAuthority,
    output: Output,
) -> Result<FheEvalOutput> {
    match output.0 {
        OutputKind::Transient => Ok(FheEvalOutput::AllowedLocal),
        OutputKind::Durable(output) => {
            let birth = output.birth()?;
            let output_encrypted_value_index = account_index(
                remaining_accounts,
                EvalAccountMeta::writable(
                    birth.encrypted_value(),
                    EvalAccountPurpose::DurableOutputAcl,
                ),
            )?;
            let output_app_account_authority_index =
                if birth.app_account() == app_authority.pubkey() {
                    None
                } else {
                    Some(account_index(
                        remaining_accounts,
                        EvalAccountMeta::readonly_signer(
                            birth.app_account(),
                            EvalAccountPurpose::DurableOutputAuthority,
                        ),
                    )?)
                };
            Ok(FheEvalOutput::AllowedDurable {
                output_encrypted_value_index,
                output_app_account_authority_index,
                output_acl_domain_key: birth.acl_domain_key(),
                output_app_account: birth.app_account(),
                output_encrypted_value_label: birth.encrypted_value_label(),
                output_subjects: birth.host_subjects(),
                previous_handle: birth.previous_handle(),
                previous_subjects: birth.previous_subjects().map(|s| s.to_vec()),
                make_public: birth.make_public(),
            })
        }
    }
}

fn account_index(
    remaining_accounts: &mut Vec<EvalAccountMeta>,
    required: EvalAccountMeta,
) -> Result<u16> {
    if let Some(index) = remaining_accounts
        .iter()
        .position(|candidate| candidate.pubkey == required.pubkey)
    {
        remaining_accounts[index].promote(required);
        return u16::try_from(index).map_err(|_| EvalBuildError::TooManyRemainingAccounts);
    }
    let index = u16::try_from(remaining_accounts.len())
        .map_err(|_| EvalBuildError::TooManyRemainingAccounts)?;
    remaining_accounts.push(required);
    Ok(index)
}

#[cfg(feature = "cpi")]
pub struct EvalCpiAccounts<'a, 'info> {
    pub payer: AccountInfo<'info>,
    pub compute_subject: AccountInfo<'info>,
    pub app_account_authority: AccountInfo<'info>,
    pub host_config: AccountInfo<'info>,
    pub deny_subject_records: &'a [AccountInfo<'info>],
    pub system_program: AccountInfo<'info>,
    /// Per-`compute_subject` HCU block meter (mut). The host keys the meter on `compute_subject`, so
    /// untrusted subjects in the metering band supply it; trusted subjects and the unrestricted
    /// default pass `None`.
    pub hcu_block_meter: Option<AccountInfo<'info>>,
    /// HCU trust witness (read-only), keyed on `compute_subject`. `Some` + valid ⇒ bypass; `None` ⇒
    /// untrusted (metered).
    pub hcu_trusted_app_record: Option<AccountInfo<'info>>,
    pub event_authority: AccountInfo<'info>,
    pub program: AccountInfo<'info>,
}

#[cfg(feature = "cpi")]
trait EvalAccountResolver<'info> {
    fn resolve_eval_account(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>>;
}

#[cfg(feature = "cpi")]
impl<'info> EvalAccountResolver<'info> for ResolvedEvalAccounts<'info> {
    fn resolve_eval_account(&self, pubkey: Pubkey) -> Option<AccountInfo<'info>> {
        self.resolve(pubkey)
    }
}

/// Failure returned by the closure-based CPI eval helper.
#[cfg(feature = "cpi")]
#[derive(Debug)]
pub enum EvalInvokeError {
    /// The closure produced an invalid eval frame.
    Build(EvalBuildError),
    /// The supplied dynamic accounts or output authority witnesses do not
    /// satisfy the built plan.
    AccountResolution(EvalAccountResolutionError),
    /// The host CPI returned an Anchor error.
    Cpi(anchor_lang::error::Error),
}

#[cfg(feature = "cpi")]
impl From<EvalBuildError> for EvalInvokeError {
    fn from(error: EvalBuildError) -> Self {
        Self::Build(error)
    }
}

#[cfg(feature = "cpi")]
impl From<EvalAccountResolutionError> for EvalInvokeError {
    fn from(error: EvalAccountResolutionError) -> Self {
        Self::AccountResolution(error)
    }
}

#[cfg(feature = "cpi")]
impl From<anchor_lang::error::Error> for EvalInvokeError {
    fn from(error: anchor_lang::error::Error) -> Self {
        Self::Cpi(error)
    }
}

/// Builds an eval plan with a closure, resolves its dynamic accounts, and
/// invokes `zama-host::fhe_eval`.
///
/// `dynamic_accounts` and additional `output_authorities` may be in any order.
/// The fixed CPI `app_account_authority` is included automatically. The SDK
/// validates the supplied accounts against the plan produced by the closure
/// before constructing the ordered host account list used by
/// [`invoke_eval_signed_resolved`].
#[cfg(feature = "cpi")]
pub fn invoke_eval_signed_with_builder<'a, 'info, T, F>(
    context_id: EvalContextId,
    app_authority: EvalAppAuthority,
    accounts: EvalCpiAccounts<'a, 'info>,
    dynamic_accounts: impl IntoIterator<Item = AccountInfo<'info>>,
    output_authorities: impl IntoIterator<Item = AccountInfo<'info>>,
    signer_seeds: &[&[&[u8]]],
    build: F,
) -> std::result::Result<(), EvalInvokeError>
where
    F: FnOnce(&mut EvalBuilder) -> Result<T>,
{
    let plan = EvalPlan::build(context_id, app_authority, build)?;
    let mut output_authorities = output_authorities.into_iter().collect::<Vec<_>>();
    output_authorities.insert(0, accounts.app_account_authority.clone());
    let resolved_accounts = plan.resolve_accounts(dynamic_accounts, output_authorities)?;
    invoke_eval_signed_resolved(&plan, accounts, &resolved_accounts, signer_seeds)?;
    Ok(())
}

/// Invokes `zama-host::fhe_eval` with accounts pre-resolved from an [`EvalPlan`].
#[cfg(feature = "cpi")]
pub fn invoke_eval_signed_resolved<'a, 'info>(
    plan: &EvalPlan,
    accounts: EvalCpiAccounts<'a, 'info>,
    resolved_accounts: &ResolvedEvalAccounts<'info>,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()> {
    invoke_eval_signed_with_resolver(plan, accounts, resolved_accounts, signer_seeds)
}

#[cfg(feature = "cpi")]
fn invoke_eval_signed_with_resolver<'a, 'info, R>(
    plan: &EvalPlan,
    accounts: EvalCpiAccounts<'a, 'info>,
    resolver: &R,
    signer_seeds: &[&[&[u8]]],
) -> anchor_lang::prelude::Result<()>
where
    R: EvalAccountResolver<'info> + ?Sized,
{
    if accounts.app_account_authority.key() != plan.app_authority.pubkey() {
        return Err(anchor_lang::error::ErrorCode::ConstraintAddress.into());
    }
    let deny_subject_records = accounts.deny_subject_records;
    let fixed_accounts = zama_host::cpi::accounts::FheEval {
        payer: accounts.payer,
        compute_subject: accounts.compute_subject,
        app_account_authority: accounts.app_account_authority,
        host_config: accounts.host_config,
        system_program: accounts.system_program,
        hcu_block_meter: accounts.hcu_block_meter,
        hcu_trusted_app_record: accounts.hcu_trusted_app_record,
        event_authority: accounts.event_authority,
        program: accounts.program,
    };
    let mut account_metas = fixed_accounts.to_account_metas(None);
    let mut account_infos = fixed_accounts.to_account_infos();
    for required in &plan.remaining_accounts {
        let account = resolver
            .resolve_eval_account(required.pubkey)
            .ok_or(anchor_lang::error::ErrorCode::AccountNotEnoughKeys)?;
        let meta = if required.is_writable {
            AccountMeta::new(required.pubkey, required.is_signer)
        } else {
            AccountMeta::new_readonly(required.pubkey, required.is_signer)
        };
        account_metas.push(meta);
        account_infos.push(account);
    }
    for record in deny_subject_records.iter().cloned() {
        account_metas.push(AccountMeta::new_readonly(record.key(), false));
        account_infos.push(record);
    }

    let instruction = Instruction {
        program_id: fixed_accounts.program.key(),
        accounts: account_metas,
        data: zama_host::instruction::FheEval {
            args: plan.args.clone(),
        }
        .data(),
    };

    invoke_signed(&instruction, &account_infos, signer_seeds)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handle(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn typed_handle(tag: u8, fhe_type: u8) -> [u8; 32] {
        let mut handle = [tag; 32];
        handle[30] = fhe_type;
        handle
    }

    fn balance_handle(tag: u8) -> [u8; 32] {
        typed_handle(tag, 5)
    }

    fn context_id(tag: u8) -> EvalContextId {
        EvalContextId::new(handle(tag)).unwrap()
    }

    fn app_authority(pubkey: Pubkey) -> EvalAppAuthority {
        EvalAppAuthority::new(pubkey)
    }

    #[cfg(feature = "cpi")]
    fn account_info(pubkey: Pubkey, is_writable: bool) -> AccountInfo<'static> {
        let key = Box::leak(Box::new(pubkey));
        let owner = Box::leak(Box::new(Pubkey::new_unique()));
        let lamports = Box::leak(Box::new(0));
        let data = Box::leak(Vec::new().into_boxed_slice());
        AccountInfo::new(key, false, is_writable, lamports, data, owner, false)
    }

    fn durable_slot(account: Pubkey, label_tag: u8) -> DurableSlot {
        DurableSlot::new(
            Pubkey::new_unique(),
            account,
            DurableLabel::new(handle(label_tag)),
        )
    }

    fn access_policy(subject: Pubkey) -> AccessPolicy {
        AccessPolicy::for_owner(subject).unwrap()
    }

    fn scalar_operand_u64(value: u64) -> Operand {
        Operand::scalar(Scalar::<Uint<64>>::u64(value).bytes())
    }

    fn dummy_attestation(input_handle: [u8; 32], contract: Pubkey) -> CoprocessorInputAttestation {
        CoprocessorInputAttestation {
            input_handle,
            ct_handles: vec![input_handle],
            handle_index: 0,
            user_address: Pubkey::new_unique().to_bytes(),
            contract_address: contract.to_bytes(),
            contract_chain_id: 1,
            extra_data: vec![],
            signatures: vec![[0u8; 65]],
        }
    }

    #[cfg(feature = "cpi")]
    fn cpi_accounts(app_authority: Pubkey) -> EvalCpiAccounts<'static, 'static> {
        EvalCpiAccounts {
            payer: account_info(Pubkey::new_unique(), true),
            compute_subject: account_info(Pubkey::new_unique(), false),
            app_account_authority: account_info(app_authority, false),
            host_config: account_info(Pubkey::new_unique(), false),
            deny_subject_records: &[],
            system_program: account_info(Pubkey::new_unique(), false),
            hcu_block_meter: None,
            hcu_trusted_app_record: None,
            event_authority: account_info(Pubkey::new_unique(), false),
            program: account_info(Pubkey::new_unique(), false),
        }
    }

    #[test]
    fn eval_plan_build_runs_closure_and_finishes_plan() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let input_acl = input_slot.address();
        let output_slot = durable_slot(primary_authority, 7);
        let output_acl = output_slot.address();
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

        let plan = EvalPlan::build(context_id(9), app_authority(primary_authority), |builder| {
            let incremented =
                builder.add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())?;
            builder.add(
                incremented,
                Scalar::<Uint<64>>::u64(2),
                Output::durable(output_slot, access_policy(primary_authority)),
            )
        })
        .unwrap();

        assert_eq!(plan.app_authority().pubkey(), primary_authority);
        assert_eq!(
            plan.remaining_accounts,
            vec![
                EvalAccountMeta::readonly(input_acl, EvalAccountPurpose::DurableInputAcl),
                EvalAccountMeta::writable(output_acl, EvalAccountPurpose::DurableOutputAcl),
            ]
        );
        assert_eq!(plan.args.steps.len(), 2);
        match &plan.args.steps[1] {
            FheEvalStep::Binary { lhs, output, .. } => {
                assert_eq!(*lhs, FheEvalOperand::AllowedLocal { producer_index: 0 });
                match output {
                    FheEvalOutput::AllowedDurable {
                        output_app_account_authority_index,
                        ..
                    } => {
                        assert_eq!(*output_app_account_authority_index, None);
                    }
                    other => panic!("unexpected output: {other:?}"),
                }
            }
            other => panic!("unexpected step: {other:?}"),
        }
    }

    #[test]
    fn eval_plan_build_lowers_verified_input_operand() {
        let primary_authority = Pubkey::new_unique();
        let output_slot = durable_slot(primary_authority, 7);
        let output_acl = output_slot.address();
        let input_handle = balance_handle(2);
        let attestation = dummy_attestation(input_handle, primary_authority);

        let plan = EvalPlan::build(context_id(9), app_authority(primary_authority), |builder| {
            let amount: Uint64Handle = builder.verified_input(attestation.clone())?;
            builder.add(
                amount,
                Scalar::<Uint<64>>::u64(1),
                Output::durable(output_slot, access_policy(primary_authority)),
            )
        })
        .unwrap();

        assert_eq!(plan.args.steps.len(), 1);
        match &plan.args.steps[0] {
            FheEvalStep::Binary { lhs, rhs, .. } => {
                assert_eq!(
                    *lhs,
                    FheEvalOperand::VerifiedInput {
                        attestation: Box::new(attestation.clone())
                    }
                );
                assert_eq!(
                    *rhs,
                    FheEvalOperand::Scalar(Scalar::<Uint<64>>::u64(1).bytes())
                );
            }
            other => panic!("unexpected step: {other:?}"),
        }
        // A verified input carries no remaining account: the attestation is inline in the operand.
        assert_eq!(
            plan.remaining_accounts,
            vec![EvalAccountMeta::writable(
                output_acl,
                EvalAccountPurpose::DurableOutputAcl
            )]
        );
    }

    #[test]
    fn verified_input_rejects_type_mismatch() {
        let primary_authority = Pubkey::new_unique();
        // Input handle typed as BOOL (0) but requested as Uint64: caught at build time.
        let attestation = dummy_attestation(typed_handle(2, 0), primary_authority);
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        assert_eq!(
            builder.verified_input::<Uint<64>>(attestation).unwrap_err(),
            EvalBuildError::UnsupportedFheType
        );
    }

    #[test]
    fn eval_plan_build_propagates_closure_and_finish_errors() {
        let primary_authority = Pubkey::new_unique();
        let error =
            match EvalPlan::build(context_id(9), app_authority(primary_authority), |builder| {
                builder.binary_op(
                    FheBinaryOpCode::Ge,
                    Operand::durable(balance_handle(1), Pubkey::new_unique()),
                    scalar_operand_u64(2),
                    FheType::UINT64,
                    Output::transient(),
                )
            }) {
                Ok(_) => panic!("invalid frame unexpectedly built"),
                Err(error) => error,
            };
        assert_eq!(error, EvalBuildError::UnsupportedBinaryOutputType);

        let error = match EvalPlan::build(
            context_id(9),
            app_authority(primary_authority),
            |_builder| Ok(()),
        ) {
            Ok(_) => panic!("empty frame unexpectedly built"),
            Err(error) => error,
        };
        assert_eq!(error, EvalBuildError::EmptyOps);
    }

    #[test]
    fn finish_preflights_lowered_remaining_account_indices() {
        let primary_authority = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder.steps.push(FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedDurable {
                handle: balance_handle(1),
                encrypted_value_index: 0,
            },
            rhs: FheEvalOperand::Scalar(Scalar::<Uint<64>>::u64(1).bytes()),
            output_fhe_type: FheType::UINT64.byte(),
            output: FheEvalOutput::AllowedLocal,
        });
        builder.produced_types.push(FheType::UINT64.byte());

        assert_eq!(
            builder.finish().unwrap_err(),
            EvalBuildError::InvalidRemainingAccountReference
        );
    }

    #[test]
    fn finish_preflights_lowered_transient_order_and_account_uniqueness() {
        let primary_authority = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder.steps.push(FheEvalStep::TrivialEncrypt {
            plaintext: Scalar::<Uint<64>>::u64(1).bytes(),
            fhe_type: FheType::UINT64.byte(),
            output: FheEvalOutput::AllowedLocal,
        });
        builder.steps.push(FheEvalStep::Binary {
            op: FheBinaryOpCode::Add,
            lhs: FheEvalOperand::AllowedLocal { producer_index: 1 },
            rhs: FheEvalOperand::Scalar(Scalar::<Uint<64>>::u64(1).bytes()),
            output_fhe_type: FheType::UINT64.byte(),
            output: FheEvalOutput::AllowedLocal,
        });
        builder.produced_types = vec![FheType::UINT64.byte(), FheType::UINT64.byte()];

        assert_eq!(
            builder.finish().unwrap_err(),
            EvalBuildError::InvalidTransientReference
        );

        let input_slot = durable_slot(primary_authority, 1);
        let input_acl = input_slot.address();
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder
            .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
            .unwrap();
        builder.remaining_accounts.push(EvalAccountMeta::readonly(
            input_acl,
            EvalAccountPurpose::DurableInputAcl,
        ));

        assert_eq!(
            builder.finish().unwrap_err(),
            EvalBuildError::InvalidRemainingAccountReference
        );
    }

    #[cfg(feature = "cpi")]
    #[test]
    fn invoke_eval_signed_with_builder_reports_build_errors_before_resolution() {
        let primary_authority = Pubkey::new_unique();
        let error = invoke_eval_signed_with_builder(
            context_id(9),
            app_authority(primary_authority),
            cpi_accounts(primary_authority),
            Vec::<AccountInfo<'static>>::new(),
            Vec::<AccountInfo<'static>>::new(),
            &[],
            |_builder| Ok(()),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            EvalInvokeError::Build(EvalBuildError::EmptyOps)
        ));
    }

    #[cfg(feature = "cpi")]
    #[test]
    fn invoke_eval_signed_with_builder_adds_fixed_authority_before_resolution() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let input_acl = input_slot.address();
        let output_slot = durable_slot(primary_authority, 7);
        let output_acl = output_slot.address();
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

        let error = invoke_eval_signed_with_builder(
            context_id(9),
            app_authority(primary_authority),
            cpi_accounts(primary_authority),
            vec![account_info(input_acl, false)],
            Vec::<AccountInfo<'static>>::new(),
            &[],
            |builder| {
                builder.add(
                    balance,
                    Scalar::<Uint<64>>::u64(1),
                    Output::durable(output_slot, access_policy(primary_authority)),
                )
            },
        )
        .unwrap_err();

        assert!(matches!(
            error,
            EvalInvokeError::AccountResolution(
                EvalAccountResolutionError::MissingDynamicAccount { requirement }
            ) if requirement.pubkey() == output_acl
        ));
    }

    #[cfg(feature = "cpi")]
    #[test]
    fn invoke_eval_signed_with_builder_requires_additional_output_authorities() {
        let primary_authority = Pubkey::new_unique();
        let extra_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let input_acl = input_slot.address();
        let output_slot = durable_slot(extra_authority, 7);
        let output_acl = output_slot.address();
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

        let error = invoke_eval_signed_with_builder(
            context_id(9),
            app_authority(primary_authority),
            cpi_accounts(primary_authority),
            vec![
                account_info(input_acl, false),
                account_info(output_acl, true),
            ],
            Vec::<AccountInfo<'static>>::new(),
            &[],
            |builder| {
                builder.add(
                    balance,
                    Scalar::<Uint<64>>::u64(1),
                    Output::durable(output_slot, access_policy(extra_authority)),
                )
            },
        )
        .unwrap_err();

        assert!(matches!(
            error,
            EvalInvokeError::AccountResolution(
                EvalAccountResolutionError::MissingOutputAuthority { authority }
            ) if authority.pubkey() == extra_authority
        ));
    }

    #[test]
    fn lowers_mixed_eval_to_stable_remaining_account_indices() {
        let primary_authority = Pubkey::new_unique();
        let balance_slot = durable_slot(primary_authority, 1);
        let amount_slot = durable_slot(primary_authority, 2);
        let balance_acl = balance_slot.address();
        let amount_acl = amount_slot.address();
        let output_slot = durable_slot(primary_authority, 7);
        let output_acl = output_slot.address();
        let balance = Uint64Handle::durable(balance_handle(1), balance_slot).unwrap();
        let amount = Uint64Handle::durable(balance_handle(2), amount_slot).unwrap();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let success = builder.ge(balance, amount, Output::transient()).unwrap();
        let debit_candidate = builder.sub(balance, amount, Output::transient()).unwrap();
        builder
            .if_then_else(
                success,
                debit_candidate,
                balance,
                Output::durable(output_slot, access_policy(primary_authority)),
            )
            .unwrap();

        let plan = builder.finish().unwrap();
        assert_eq!(plan.app_authority().pubkey(), primary_authority);

        assert_eq!(
            plan.remaining_accounts,
            vec![
                EvalAccountMeta::readonly(balance_acl, EvalAccountPurpose::DurableInputAcl),
                EvalAccountMeta::readonly(amount_acl, EvalAccountPurpose::DurableInputAcl),
                EvalAccountMeta::writable(output_acl, EvalAccountPurpose::DurableOutputAcl),
            ]
        );
        assert_eq!(plan.args.steps.len(), 3);
        match &plan.args.steps[0] {
            FheEvalStep::Binary { op, output, .. } => {
                assert_eq!(*op, FheBinaryOpCode::Ge);
                assert_eq!(*output, FheEvalOutput::AllowedLocal);
            }
            other => panic!("unexpected step: {other:?}"),
        }
        match &plan.args.steps[2] {
            FheEvalStep::Ternary {
                control,
                if_true,
                if_false,
                output,
                ..
            } => {
                assert_eq!(*control, FheEvalOperand::AllowedLocal { producer_index: 0 });
                assert_eq!(*if_true, FheEvalOperand::AllowedLocal { producer_index: 1 });
                match if_false {
                    FheEvalOperand::AllowedDurable {
                        encrypted_value_index,
                        ..
                    } => {
                        assert_eq!(*encrypted_value_index, 0)
                    }
                    other => panic!("unexpected if_false: {other:?}"),
                }
                match output {
                    FheEvalOutput::AllowedDurable {
                        output_encrypted_value_index,
                        ..
                    } => {
                        assert_eq!(*output_encrypted_value_index, 2)
                    }
                    other => panic!("unexpected output: {other:?}"),
                }
            }
            other => panic!("unexpected step: {other:?}"),
        }
    }

    #[test]
    fn dynamic_account_requirements_expose_order_roles_and_purposes() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let input_acl = input_slot.address();
        let extra_authority = Pubkey::new_unique();
        let output_slot = durable_slot(extra_authority, 7);
        let output_acl = output_slot.address();
        let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

        let plan = EvalPlan::build(context_id(9), app_authority(primary_authority), |builder| {
            builder.add(
                input,
                Scalar::<Uint<64>>::u64(2),
                Output::durable(output_slot, access_policy(extra_authority)),
            )
        })
        .unwrap();

        let requirements = plan.dynamic_account_requirements().collect::<Vec<_>>();
        assert_eq!(
            requirements
                .iter()
                .map(EvalAccountRequirement::pubkey)
                .collect::<Vec<_>>(),
            vec![input_acl, output_acl, extra_authority]
        );
        assert_eq!(
            requirements[0].purposes(),
            &[EvalAccountPurpose::DurableInputAcl]
        );
        assert_eq!(
            requirements[1].purposes(),
            &[EvalAccountPurpose::DurableOutputAcl]
        );
        assert_eq!(
            requirements[2].purposes(),
            &[EvalAccountPurpose::DurableOutputAuthority]
        );
        assert!(requirements[1].is_writable());
        assert!(requirements[2].is_signer());
        assert!(!requirements[2].requires_dynamic_account());
        assert!(requirements[2].requires_output_authority());
    }

    #[test]
    fn lowers_explicit_output_authority_witness() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let acl_record = input_slot.address();
        let authority = Pubkey::new_unique();
        let output_slot = durable_slot(authority, 7);
        let output_acl = output_slot.address();
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder
            .add(
                balance,
                Scalar::<Uint<64>>::u64(2),
                Output::durable(output_slot, access_policy(authority)),
            )
            .unwrap();

        let plan = builder.finish().unwrap();
        assert_eq!(plan.app_authority().pubkey(), primary_authority);
        assert_eq!(
            plan.remaining_accounts,
            vec![
                EvalAccountMeta::readonly(acl_record, EvalAccountPurpose::DurableInputAcl),
                EvalAccountMeta::writable(output_acl, EvalAccountPurpose::DurableOutputAcl),
                EvalAccountMeta::readonly_signer(
                    authority,
                    EvalAccountPurpose::DurableOutputAuthority,
                ),
            ]
        );
        assert_eq!(
            plan.additional_output_authorities().collect::<Vec<_>>(),
            vec![authority]
        );
        let authority_requirements = plan.output_authority_requirements().collect::<Vec<_>>();
        assert_eq!(
            authority_requirements,
            vec![
                EvalOutputAuthorityRequirement {
                    pubkey: primary_authority,
                    cpi_account_authority: true,
                },
                EvalOutputAuthorityRequirement {
                    pubkey: authority,
                    cpi_account_authority: false,
                },
            ]
        );
        match &plan.args.steps[0] {
            FheEvalStep::Binary { output, .. } => match output {
                FheEvalOutput::AllowedDurable {
                    output_encrypted_value_index,
                    output_app_account_authority_index,
                    ..
                } => {
                    assert_eq!(*output_encrypted_value_index, 1);
                    assert_eq!(*output_app_account_authority_index, Some(2));
                }
                other => panic!("unexpected output: {other:?}"),
            },
            other => panic!("unexpected step: {other:?}"),
        }
    }

    #[cfg(feature = "cpi")]
    #[test]
    fn resolve_accounts_orders_and_validates_plan_requirements() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let input_acl = input_slot.address();
        let extra_authority = Pubkey::new_unique();
        let output_slot = durable_slot(extra_authority, 7);
        let output_acl = output_slot.address();
        let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder
            .add(
                input,
                Scalar::<Uint<64>>::u64(2),
                Output::durable(output_slot, access_policy(extra_authority)),
            )
            .unwrap();
        let plan = builder.finish().unwrap();

        let resolved = plan
            .resolve_accounts(
                vec![
                    account_info(output_acl, true),
                    account_info(input_acl, false),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                ],
            )
            .unwrap();
        assert_eq!(
            resolved
                .account_infos()
                .iter()
                .map(|account| account.key())
                .collect::<Vec<_>>(),
            vec![input_acl, output_acl, extra_authority]
        );

        let duplicate = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(input_acl, false),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                ],
            )
            .unwrap_err();
        assert_eq!(
            duplicate,
            EvalAccountResolutionError::DuplicateDynamicAccount { pubkey: input_acl }
        );

        let unexpected = plan
            .resolve_accounts(
                vec![account_info(Pubkey::new_unique(), false)],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                ],
            )
            .unwrap_err();
        assert!(matches!(
            unexpected,
            EvalAccountResolutionError::UnexpectedDynamicAccount { .. }
        ));

        let missing = plan
            .resolve_accounts(
                vec![account_info(output_acl, true)],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                ],
            )
            .unwrap_err();
        assert!(matches!(
            missing,
            EvalAccountResolutionError::MissingDynamicAccount { requirement }
                if requirement.pubkey() == input_acl
        ));

        let readonly = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(output_acl, false),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                ],
            )
            .unwrap_err();
        assert!(matches!(
            readonly,
            EvalAccountResolutionError::DynamicAccountNotWritable { requirement }
                if requirement.pubkey() == output_acl
        ));

        let duplicate_authority = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(output_acl, true),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                    account_info(extra_authority, false),
                ],
            )
            .unwrap_err();
        assert_eq!(
            duplicate_authority,
            EvalAccountResolutionError::DuplicateOutputAuthority {
                pubkey: extra_authority
            }
        );

        let unexpected_authority = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(output_acl, true),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                    account_info(Pubkey::new_unique(), false),
                ],
            )
            .unwrap_err();
        assert!(matches!(
            unexpected_authority,
            EvalAccountResolutionError::UnexpectedOutputAuthority { .. }
        ));

        let missing_authority = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(output_acl, true),
                ],
                vec![account_info(primary_authority, false)],
            )
            .unwrap_err();
        assert_eq!(
            missing_authority,
            EvalAccountResolutionError::MissingOutputAuthority {
                authority: EvalOutputAuthorityRequirement {
                    pubkey: extra_authority,
                    cpi_account_authority: false,
                }
            }
        );
    }

    #[cfg(feature = "cpi")]
    #[test]
    fn resolve_accounts_rejects_known_accounts_in_wrong_bucket() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let input_acl = input_slot.address();
        let extra_authority = Pubkey::new_unique();
        let output_slot = durable_slot(extra_authority, 7);
        let output_acl = output_slot.address();
        let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder
            .add(
                input,
                Scalar::<Uint<64>>::u64(2),
                Output::durable(output_slot, access_policy(extra_authority)),
            )
            .unwrap();
        let plan = builder.finish().unwrap();

        let authority_in_dynamic_bucket = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(output_acl, true),
                    account_info(extra_authority, false),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                ],
            )
            .unwrap_err();
        assert_eq!(
            authority_in_dynamic_bucket,
            EvalAccountResolutionError::UnexpectedDynamicAccount {
                pubkey: extra_authority
            }
        );

        let input_acl_in_authority_bucket = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(output_acl, true),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                    account_info(input_acl, false),
                ],
            )
            .unwrap_err();
        assert_eq!(
            input_acl_in_authority_bucket,
            EvalAccountResolutionError::UnexpectedOutputAuthority { pubkey: input_acl }
        );

        let output_acl_in_authority_bucket = plan
            .resolve_accounts(
                vec![
                    account_info(input_acl, false),
                    account_info(output_acl, true),
                ],
                vec![
                    account_info(primary_authority, false),
                    account_info(extra_authority, false),
                    account_info(output_acl, false),
                ],
            )
            .unwrap_err();
        assert_eq!(
            output_acl_in_authority_bucket,
            EvalAccountResolutionError::UnexpectedOutputAuthority { pubkey: output_acl }
        );
    }

    #[test]
    fn lowers_birth_steps() {
        let primary_authority = Pubkey::new_unique();
        let output_slot = durable_slot(primary_authority, 7);
        let output_acl = output_slot.address();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let trivial = builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
        builder
            .rand_u64(Output::durable(
                output_slot,
                access_policy(primary_authority),
            ))
            .unwrap();
        builder
            .add(trivial, Scalar::<Uint<64>>::u64(1), Output::transient())
            .unwrap();

        let plan = builder.finish().unwrap();
        assert_eq!(
            plan.remaining_accounts,
            vec![EvalAccountMeta::writable(
                output_acl,
                EvalAccountPurpose::DurableOutputAcl
            )]
        );
        assert!(matches!(
            plan.args.steps[0],
            FheEvalStep::TrivialEncrypt { .. }
        ));
        assert!(matches!(plan.args.steps[1], FheEvalStep::Rand { .. }));
    }

    #[test]
    fn rejects_invalid_references_and_types() {
        let primary_authority = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let error = builder
            .binary_op(
                FheBinaryOpCode::Add,
                Operand::transient(0, builder.context_id, builder.scope),
                scalar_operand_u64(1),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err();
        assert_eq!(error, EvalBuildError::InvalidTransientReference);

        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let error = builder
            .binary_op(
                FheBinaryOpCode::Ge,
                Operand::durable(balance_handle(1), Pubkey::new_unique()),
                scalar_operand_u64(2),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err();
        assert_eq!(error, EvalBuildError::UnsupportedBinaryOutputType);

        let input_slot = durable_slot(primary_authority, 1);
        let input = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
        let current_index = builder
            .binary_op(
                FheBinaryOpCode::Add,
                Operand::transient(1, builder.context_id, builder.scope),
                scalar_operand_u64(1),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err();
        assert_eq!(current_index, EvalBuildError::InvalidTransientReference);

        let future_index = builder
            .binary_op(
                FheBinaryOpCode::Add,
                Operand::transient(9, builder.context_id, builder.scope),
                scalar_operand_u64(1),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err();
        assert_eq!(future_index, EvalBuildError::InvalidTransientReference);

        let invalid_rhs = builder
            .binary_op(
                FheBinaryOpCode::Add,
                input.operand(),
                Operand::transient(1, builder.context_id, builder.scope),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err();
        assert_eq!(invalid_rhs, EvalBuildError::InvalidTransientReference);
    }

    #[test]
    fn rejects_transients_from_another_builder_with_same_context_id() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let context = context_id(9);

        let mut first = EvalBuilder::new(context, app_authority(primary_authority));
        let foreign = first
            .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
            .unwrap();

        let mut second = EvalBuilder::new(context, app_authority(primary_authority));
        second.trivial_encrypt_u64(1, Output::transient()).unwrap();
        let error = second
            .add(foreign, Scalar::<Uint<64>>::u64(1), Output::transient())
            .unwrap_err();

        assert_eq!(error, EvalBuildError::InvalidTransientReference);
    }

    #[test]
    fn validates_app_authority_and_durable_account_pubkeys() {
        let mut builder = EvalBuilder::new(context_id(9), app_authority(Pubkey::default()));
        builder.trivial_encrypt_u64(1, Output::transient()).unwrap();
        let error = match builder.finish() {
            Ok(_) => panic!("invalid app authority unexpectedly built"),
            Err(error) => error,
        };
        assert_eq!(error, EvalBuildError::InvalidAppAuthority);

        let invalid_namespace_slot = DurableSlot::new(
            Pubkey::default(),
            Pubkey::new_unique(),
            DurableLabel::new(handle(5)),
        );
        assert_eq!(
            Uint64Handle::durable(balance_handle(1), invalid_namespace_slot.clone()).unwrap_err(),
            EvalBuildError::InvalidDurableSlot
        );
        assert_eq!(
            DurableOutput::create(invalid_namespace_slot, access_policy(Pubkey::new_unique()))
                .birth()
                .unwrap_err(),
            EvalBuildError::InvalidDurableSlot
        );

        let invalid_account_slot = DurableSlot::new(
            Pubkey::new_unique(),
            Pubkey::default(),
            DurableLabel::new(handle(5)),
        );
        assert_eq!(
            Uint64Handle::durable(balance_handle(1), invalid_account_slot.clone()).unwrap_err(),
            EvalBuildError::InvalidDurableSlot
        );
        assert_eq!(
            DurableOutput::create(invalid_account_slot, access_policy(Pubkey::new_unique()))
                .birth()
                .unwrap_err(),
            EvalBuildError::InvalidDurableSlot
        );
    }

    #[test]
    fn binary_validation_rejects_host_type_mismatches() {
        let primary_authority = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let bool_lhs =
            Operand::durable(typed_handle(1, FheType::BOOL.byte()), Pubkey::new_unique());
        let error = builder
            .binary_op(
                FheBinaryOpCode::Add,
                bool_lhs,
                scalar_operand_u64(1),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err();
        // Add gates its output to uint types, and the operand must equal that output type, so a
        // Bool lhs against a Uint64 output is a type mismatch (host + client agree).
        assert_eq!(error, EvalBuildError::BinaryOperandTypeMismatch);

        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let error = builder
            .binary_op(
                FheBinaryOpCode::Add,
                Operand::durable(balance_handle(1), Pubkey::new_unique()),
                Operand::durable(
                    typed_handle(2, FheType::UINT32.byte()),
                    Pubkey::new_unique(),
                ),
                FheType::UINT64,
                Output::transient(),
            )
            .unwrap_err();
        assert_eq!(error, EvalBuildError::BinaryOperandTypeMismatch);
    }

    #[test]
    fn unary_validation_rejects_same_type_cast_and_bad_operand_types() {
        let primary_authority = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        // A cast to a different type is accepted.
        assert!(builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::durable(balance_handle(1), Pubkey::new_unique()),
                FheType::UINT32,
                Output::transient(),
            )
            .is_ok());
        // A same-type cast is rejected (EVM InvalidType parity).
        assert_eq!(
            builder
                .unary_op(
                    FheUnaryOpCode::Cast,
                    Operand::durable(balance_handle(1), Pubkey::new_unique()),
                    FheType::UINT64,
                    Output::transient(),
                )
                .unwrap_err(),
            EvalBuildError::UnsupportedFheType
        );
        // EVM cast type sets: a Bool input casts to a uint (Bool -> Uint32) is accepted...
        assert!(builder
            .unary_op(
                FheUnaryOpCode::Cast,
                Operand::durable(typed_handle(2, FheType::BOOL.byte()), Pubkey::new_unique()),
                FheType::UINT32,
                Output::transient(),
            )
            .is_ok());
        // ...but casting TO ebool, TO eaddress, or FROM eaddress is rejected.
        assert_eq!(
            builder
                .unary_op(
                    FheUnaryOpCode::Cast,
                    Operand::durable(balance_handle(1), Pubkey::new_unique()),
                    FheType::BOOL,
                    Output::transient(),
                )
                .unwrap_err(),
            EvalBuildError::UnsupportedFheType
        );
        assert_eq!(
            builder
                .unary_op(
                    FheUnaryOpCode::Cast,
                    Operand::durable(balance_handle(1), Pubkey::new_unique()),
                    FheType::ADDRESS,
                    Output::transient(),
                )
                .unwrap_err(),
            EvalBuildError::UnsupportedFheType
        );
        assert_eq!(
            builder
                .unary_op(
                    FheUnaryOpCode::Cast,
                    Operand::durable(
                        typed_handle(3, FheType::ADDRESS.byte()),
                        Pubkey::new_unique()
                    ),
                    FheType::UINT64,
                    Output::transient(),
                )
                .unwrap_err(),
            EvalBuildError::UnsupportedFheType
        );
        // Neg rejects a Bool operand (EVM fheNeg supportedTypes = Uint8..Uint128 + Uint256).
        assert_eq!(
            builder
                .unary_op(
                    FheUnaryOpCode::Neg,
                    Operand::durable(typed_handle(1, FheType::BOOL.byte()), Pubkey::new_unique()),
                    FheType::BOOL,
                    Output::transient(),
                )
                .unwrap_err(),
            EvalBuildError::UnsupportedFheType
        );
    }

    #[test]
    fn mul_div_rejects_zero_divisor() {
        let primary_authority = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let balance =
            Uint64Handle::durable(balance_handle(1), durable_slot(primary_authority, 1)).unwrap();
        assert_eq!(
            builder
                .mul_div(
                    balance,
                    Scalar::<Uint<64>>::u64(3),
                    Scalar::<Uint<64>>::u64(0),
                    Output::transient(),
                )
                .unwrap_err(),
            EvalBuildError::MulDivDivisorZero
        );
    }

    #[test]
    fn div_rem_require_nonzero_scalar_divisor() {
        let auth = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(auth));
        // Encrypted divisor is rejected — division is scalar-only (EVM `IsNotScalar`).
        let lhs = Uint64Handle::durable(balance_handle(1), durable_slot(auth, 1)).unwrap();
        let enc_divisor = Uint64Handle::durable(balance_handle(2), durable_slot(auth, 2)).unwrap();
        assert_eq!(
            builder
                .div(lhs, enc_divisor, Output::transient())
                .unwrap_err(),
            EvalBuildError::DivisorMustBeScalar
        );
        // A zero scalar divisor is rejected.
        let lhs2 = Uint64Handle::durable(balance_handle(1), durable_slot(auth, 1)).unwrap();
        assert_eq!(
            builder
                .rem(lhs2, Scalar::<Uint<64>>::u64(0), Output::transient())
                .unwrap_err(),
            EvalBuildError::DivisionByZero
        );
        // A non-zero scalar divisor is accepted.
        let lhs3 = Uint64Handle::durable(balance_handle(1), durable_slot(auth, 1)).unwrap();
        assert!(builder
            .div(lhs3, Scalar::<Uint<64>>::u64(3), Output::transient())
            .is_ok());
    }

    #[test]
    fn builder_exposes_the_host_operator_type_surface() {
        // The typed builder must express the host's type matrix: bitwise on Bool/Uint256, neg on Uint256, eq on Bool, is_in on Uint160.
        let auth = Pubkey::new_unique();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(auth));

        let bool_a = Encrypted::<Bool>::durable(
            typed_handle(1, FheType::BOOL.byte()),
            durable_slot(auth, 1),
        )
        .unwrap();
        let bool_b = Encrypted::<Bool>::durable(
            typed_handle(2, FheType::BOOL.byte()),
            durable_slot(auth, 2),
        )
        .unwrap();
        assert!(builder.and(bool_a, bool_b, Output::transient()).is_ok());

        let u256_a = Encrypted::<Bytes256>::durable(
            typed_handle(3, FheType::BYTES256.byte()),
            durable_slot(auth, 3),
        )
        .unwrap();
        let u256_b = Encrypted::<Bytes256>::durable(
            typed_handle(4, FheType::BYTES256.byte()),
            durable_slot(auth, 4),
        )
        .unwrap();
        assert!(builder.xor(u256_a, u256_b, Output::transient()).is_ok());

        let u256_c = Encrypted::<Bytes256>::durable(
            typed_handle(5, FheType::BYTES256.byte()),
            durable_slot(auth, 5),
        )
        .unwrap();
        assert!(builder.neg(u256_c, Output::transient()).is_ok());

        let bool_c = Encrypted::<Bool>::durable(
            typed_handle(6, FheType::BOOL.byte()),
            durable_slot(auth, 6),
        )
        .unwrap();
        let bool_d = Encrypted::<Bool>::durable(
            typed_handle(7, FheType::BOOL.byte()),
            durable_slot(auth, 7),
        )
        .unwrap();
        assert!(builder.eq(bool_c, bool_d, Output::transient()).is_ok());

        let addr_v = Encrypted::<Address>::durable(
            typed_handle(8, FheType::ADDRESS.byte()),
            durable_slot(auth, 8),
        )
        .unwrap();
        let addr_s = Encrypted::<Address>::durable(
            typed_handle(9, FheType::ADDRESS.byte()),
            durable_slot(auth, 9),
        )
        .unwrap();
        assert!(builder.is_in(addr_v, [addr_s], Output::transient()).is_ok());
    }

    #[test]
    fn access_policy_constructors_validate_immediately() {
        assert_eq!(
            AccessPolicy::for_owner(Pubkey::default()).unwrap_err(),
            EvalBuildError::InvalidAccessPolicy
        );
        let subject = Pubkey::new_unique();
        assert_eq!(
            AccessPolicy::for_owner(subject)
                .unwrap()
                .with_compute(subject)
                .unwrap_err(),
            EvalBuildError::InvalidAccessPolicy
        );

        let mut policy = AccessPolicy::for_owner(Pubkey::new_unique()).unwrap();
        for _ in 1..MAX_ACL_SUBJECTS {
            policy = policy.with_use_only(Pubkey::new_unique()).unwrap();
        }
        assert_eq!(
            policy.with_use_only(Pubkey::new_unique()).unwrap_err(),
            EvalBuildError::InvalidAccessPolicy
        );

        assert_eq!(
            AccessPolicy::from_subjects(Vec::<AccessSubject>::new()).unwrap_err(),
            EvalBuildError::InvalidAccessPolicy
        );
    }

    #[test]
    fn durable_output_birth_matches_eval_lowering() {
        let primary_authority = Pubkey::new_unique();
        let subject = Pubkey::new_unique();
        let output_slot = durable_slot(primary_authority, 42);
        let output = DurableOutput::create(output_slot.clone(), access_policy(subject));
        let birth = output.birth().unwrap();

        assert_eq!(birth.encrypted_value(), output_slot.address());
        assert_eq!(birth.acl_domain_key(), output_slot.namespace());
        assert_eq!(birth.app_account(), output_slot.account());
        assert_eq!(birth.encrypted_value_label(), output_slot.label().bytes());
        assert_eq!(birth.subjects(), access_policy(subject).subjects());
        assert_eq!(birth.previous_handle(), None);
        assert_eq!(birth.previous_subjects(), None);

        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        builder
            .trivial_encrypt_u64(7, Output::durable(output_slot, access_policy(subject)))
            .unwrap();
        let plan = builder.finish().unwrap();
        match &plan.args.steps[0] {
            FheEvalStep::TrivialEncrypt {
                output:
                    FheEvalOutput::AllowedDurable {
                        output_encrypted_value_index,
                        output_acl_domain_key,
                        output_app_account,
                        output_encrypted_value_label,
                        output_subjects,
                        previous_handle,
                        previous_subjects,
                        ..
                    },
                ..
            } => {
                let output_encrypted_value =
                    plan.remaining_accounts[*output_encrypted_value_index as usize].pubkey;
                assert_eq!(output_encrypted_value, birth.encrypted_value());
                assert_eq!(*output_acl_domain_key, birth.acl_domain_key());
                assert_eq!(*output_app_account, birth.app_account());
                assert_eq!(*output_encrypted_value_label, birth.encrypted_value_label());
                assert_eq!(*output_subjects, birth.host_subjects());
                assert_eq!(*previous_handle, birth.previous_handle());
                assert_eq!(previous_subjects.as_deref(), birth.previous_subjects());
            }
            other => panic!("unexpected step: {other:?}"),
        }
    }

    #[test]
    fn durable_output_supersede_carries_previous_state() {
        let primary_authority = Pubkey::new_unique();
        let subject = Pubkey::new_unique();
        let output_slot = durable_slot(primary_authority, 42);
        let previous_handle = balance_handle(1);
        let previous_subjects = vec![subject];
        let output = DurableOutput::supersede(
            output_slot,
            access_policy(subject),
            previous_handle,
            previous_subjects.clone(),
        );
        let birth = output.birth().unwrap();

        assert_eq!(birth.previous_handle(), Some(previous_handle));
        assert_eq!(
            birth.previous_subjects(),
            Some(previous_subjects.as_slice())
        );
    }

    #[test]
    fn rejects_transients_from_another_frame() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();

        let mut first = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        let foreign = first
            .add(balance, Scalar::<Uint<64>>::u64(1), Output::transient())
            .unwrap();

        let mut second = EvalBuilder::new(context_id(10), app_authority(primary_authority));
        second.trivial_encrypt_u64(1, Output::transient()).unwrap();
        let error = second
            .add(foreign, Scalar::<Uint<64>>::u64(1), Output::transient())
            .unwrap_err();

        assert_eq!(error, EvalBuildError::InvalidTransientReference);
    }

    #[test]
    fn typed_handle_constructor_rejects_mismatched_handle_tag() {
        let error = Uint64Handle::durable(
            typed_handle(1, FheType::UINT32.byte()),
            durable_slot(Pubkey::new_unique(), 7),
        )
        .unwrap_err();
        assert_eq!(error, EvalBuildError::UnsupportedFheType);
    }

    #[test]
    fn rand_rejects_address_type_like_host() {
        let mut builder = EvalBuilder::new(context_id(9), app_authority(Pubkey::new_unique()));
        let error = builder
            .rand_raw(FheType::ADDRESS, Output::transient())
            .unwrap_err();
        assert_eq!(error, EvalBuildError::UnsupportedFheType);
    }

    #[test]
    fn finish_rejects_empty_context_id_and_empty_steps() {
        let primary_authority = Pubkey::new_unique();
        assert!(matches!(
            EvalContextId::new([0u8; 32]),
            Err(EvalBuildError::EmptyContextId)
        ));
        assert!(matches!(
            EvalBuilder::new(context_id(9), app_authority(primary_authority)).finish(),
            Err(EvalBuildError::EmptyOps)
        ));
    }

    #[test]
    fn rejects_more_than_max_ops() {
        let primary_authority = Pubkey::new_unique();
        let input_slot = durable_slot(primary_authority, 1);
        let balance = Uint64Handle::durable(balance_handle(1), input_slot).unwrap();
        let mut builder = EvalBuilder::new(context_id(9), app_authority(primary_authority));
        for index in 0..MAX_FHE_EVAL_OPS {
            builder
                .add(
                    balance,
                    Scalar::<Uint<64>>::u64(index as u64),
                    Output::transient(),
                )
                .unwrap();
        }
        let error = builder
            .add(balance, Scalar::<Uint<64>>::u64(99), Output::transient())
            .unwrap_err();
        assert_eq!(error, EvalBuildError::TooManyOps);
    }

    #[test]
    fn scalar_u64_uses_big_endian_low_bytes() {
        let mut expected = [0u8; 32];
        expected[24..].copy_from_slice(&0x0102_0304_0506_0708u64.to_be_bytes());
        assert_eq!(
            Scalar::<Uint<64>>::u64(0x0102_0304_0506_0708).bytes(),
            expected
        );
    }
}
