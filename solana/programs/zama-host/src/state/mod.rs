//! State and deterministic helper functions for the ZamaHost program.
//!
//! This module is intentionally reusable from app programs and tests. It
//! exposes the PDA seeds, account layouts, and handle formulas
//! needed to prepare CPI accounts and to verify host-owned ACL state off-chain.
//!
//! Public API surface: app programs preparing host CPIs and off-chain callers checking handle
//! derivation — `runtime-tests`' Mollusk suites and execution contracts read the enum
//! discriminants and handle fields these helpers expose.

use anchor_lang::prelude::*;
use solana_keccak_hasher::hashv as keccak_hashv;
use solana_sysvar::get_sysvar;

use crate::constants::{COMPUTATION_DOMAIN_SEPARATOR, COMPUTED_HANDLE_MARKER};
use crate::errors::ZamaHostError;

pub mod deny_scope_record;
pub mod encrypted_value;
pub mod hcu_block_meter;
pub mod hcu_trusted_app_record;
pub mod host_config;
pub mod kms_context;
pub mod permit_invalidation;
pub mod rand_nonce;
mod type_gate;
pub mod user_decryption_delegation;

pub use deny_scope_record::*;
pub use encrypted_value::*;
pub use hcu_block_meter::*;
pub use hcu_trusted_app_record::*;
pub use host_config::*;
pub use kms_context::*;
pub use permit_invalidation::*;
pub use rand_nonce::*;
pub(crate) use type_gate::assert_reduction_count;
pub use type_gate::{
    assert_binary_operand_types, assert_is_in_operand_types, assert_mul_div_operand_types,
    assert_sum_operand_types, assert_supported_fhe_type, assert_ternary_operand_types,
    assert_unary_operand_type, assert_valid_bounded_rand_upper_bound, binary_output_type_ok,
    is_supported_fhe_type, is_supported_uint_fhe_type, max_reduction_operands,
    scalar_is_zero_for_type, unary_output_type_ok,
};
pub use user_decryption_delegation::*;

pub use crate::constants::*;

/// Initialization arguments for the singleton [`HostConfig`] account.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct InitializeHostConfigArgs {
    /// Host-chain id encoded into newly derived handles.
    pub chain_id: u64,
    /// EVM gateway chain id used in the coprocessor/KMS EIP-712 domain separators.
    pub gateway_chain_id: u64,
    /// EVM `InputVerification` contract address (EIP-712 verifying contract).
    pub input_verification_contract: [u8; 20],
    /// Registered coprocessor EVM signer set for input attestations (EVM `InputVerifier`
    /// parity). Must be non-empty, distinct, and free of the zero address; bounded by
    /// [`HostConfig::MAX_COPROCESSOR_SIGNERS`].
    pub coprocessor_signers: Vec<[u8; 20]>,
    /// Minimum distinct valid signatures (n-of-m) required over an input attestation;
    /// `1 <= coprocessor_threshold <= coprocessor_signers.len()`.
    pub coprocessor_threshold: u8,
    /// EVM `Decryption` contract address (EIP-712 verifying contract for KMS certs).
    pub decryption_contract: [u8; 20],
    /// Whether computing and allowing require the application's deny-list witness.
    pub grant_deny_list_enabled: bool,
}

/// Binary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheBinaryOpCode {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Remainder.
    Rem,
    /// Bitwise AND.
    And,
    /// Bitwise OR.
    Or,
    /// Bitwise XOR.
    Xor,
    /// Shift left.
    Shl,
    /// Shift right.
    Shr,
    /// Rotate left.
    Rotl,
    /// Rotate right.
    Rotr,
    /// Equality comparison.
    Eq,
    /// Inequality comparison.
    Ne,
    /// Greater-than-or-equal comparison.
    Ge,
    /// Greater-than comparison.
    Gt,
    /// Less-than-or-equal comparison.
    Le,
    /// Less-than comparison.
    Lt,
    /// Minimum.
    Min,
    /// Maximum.
    Max,
}

/// Ternary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheTernaryOpCode {
    /// Selects `if_true` when `control` is true, otherwise `if_false`.
    IfThenElse,
}

/// Unary FHE operators.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheUnaryOpCode {
    /// Arithmetic negation.
    Neg,
    /// Bitwise NOT.
    Not,
    /// Type cast.
    Cast,
}

impl FheUnaryOpCode {
    /// Stable byte encoding used in handle derivation and events; mirrors the shared coprocessor
    /// `SupportedFheOperations` discriminants (FheNeg=20, FheNot=21, FheCast=23; 22 is reserved).
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Neg => 20,
            Self::Not => 21,
            Self::Cast => 23,
        }
    }
}

/// Arguments for one composed, instruction-local fhe_execute.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FheExecuteArgs {
    /// Declared `remaining_accounts` length, asserted equal to the actual list. Carried in
    /// instruction data so stateless indexers can validate account references without the
    /// transaction envelope (DD-033 self-description).
    pub account_count: u8,
    /// Interned 32-byte constant dictionary: operand handles, scalar values, output programs,
    /// authorities, scopes, labels, previous handles, allowed keys and PDA seed parts. Steps
    /// reference entries by `u8` index, so a value repeated across steps is paid for once (the
    /// compiled-message / constant-dictionary encoding; fhevm-internal#1853 W7).
    ///
    /// The entries stay raw `[u8; 32]` on purpose, and must not become an enum of typed
    /// variants. Interning is what makes the encoding small, and it only works across roles:
    /// the same 32 bytes can be an operand handle in one step and the previous handle of an
    /// update in another, and both steps then share one entry. A typed dictionary would need one entry
    /// per role, which grows the packet and buys nothing — the step that reads an index
    /// already knows which role it is asking for, and `dictionary_key` /
    /// `dictionary_bytes` are where that reading happens. Types belong at the ends of the
    /// wire, not in the middle of it (fhevm-internal#1859 §2).
    pub dictionary: Vec<[u8; 32]>,
    /// Ordered step list. Each `EarlierStep` operand may only reference an output
    /// produced by an earlier index in this vector.
    pub steps: Vec<FheExecuteStep>,
}

/// Resolves an interned dictionary entry; an out-of-range index fails the execution.
pub fn dictionary_bytes(dictionary: &[[u8; 32]], index: u8) -> Result<[u8; 32]> {
    dictionary
        .get(index as usize)
        .copied()
        .ok_or_else(|| error!(ZamaHostError::FheExecuteDictionaryIndexOutOfBounds))
}

/// Resolves an interned dictionary entry as a public key.
pub fn dictionary_key(dictionary: &[[u8; 32]], index: u8) -> Result<Pubkey> {
    Ok(Pubkey::new_from_array(dictionary_bytes(dictionary, index)?))
}

impl FheExecuteArgs {
    /// Resolves an interned dictionary entry; an out-of-range index fails the execution.
    pub fn dictionary_bytes(&self, index: u8) -> Result<[u8; 32]> {
        dictionary_bytes(&self.dictionary, index)
    }

    /// Resolves an interned dictionary entry as a public key.
    pub fn dictionary_key(&self, index: u8) -> Result<Pubkey> {
        dictionary_key(&self.dictionary, index)
    }
}

/// One step inside a composed fhe_execute.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheExecuteStep {
    /// Binary operator step.
    Binary {
        /// Binary operator.
        op: FheBinaryOpCode,
        /// Left-hand encrypted operand.
        lhs: FheExecuteOperand,
        /// Right-hand encrypted operand or scalar bytes.
        rhs: FheExecuteOperand,
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Ternary operator step.
    Ternary {
        /// Ternary operator.
        op: FheTernaryOpCode,
        /// Encrypted bool control operand.
        control: FheExecuteOperand,
        /// Encrypted branch selected when control is true.
        if_true: FheExecuteOperand,
        /// Encrypted branch selected when control is false.
        if_false: FheExecuteOperand,
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Trivial encryption step.
    TrivialEncrypt {
        /// Plaintext bytes encoded using the host scalar convention.
        plaintext: [u8; 32],
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Random ciphertext step.
    Rand {
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Unary operator step.
    Unary {
        /// Unary operator.
        op: FheUnaryOpCode,
        /// Encrypted operand.
        operand: FheExecuteOperand,
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Bounded random ciphertext step.
    RandBounded {
        /// Exclusive upper bound encoded as a 256-bit big-endian integer.
        upper_bound: [u8; 32],
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Sum step.
    Sum {
        /// Encrypted operands.
        operands: Vec<FheExecuteOperand>,
        /// FHE type byte embedded in the output handle.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Is-in membership test step.
    IsIn {
        /// Encrypted value to test.
        value: FheExecuteOperand,
        /// Encrypted set operands.
        set: Vec<FheExecuteOperand>,
        /// FHE type byte of the value and set elements.
        fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
    /// Multiply-then-divide step.
    MulDiv {
        /// Left-hand encrypted factor.
        factor1: FheExecuteOperand,
        /// Right-hand factor, encrypted or scalar bytes.
        factor2: FheExecuteOperand,
        /// Divisor encoded as a 256-bit big-endian integer.
        divisor: [u8; 32],
        /// FHE type byte embedded in the output handle.
        output_fhe_type: u8,
        /// Whether this output remains instruction-local or is bound into persistent ACL state.
        output: FheExecuteOutput,
    },
}

/// A coprocessor input attestation carried inline by a [`FheExecuteOperand::VerifiedInput`], re-verified
/// in-execution (no account, no PDA) — the instruction-local analog of EVM `allowTransient(input, contract)`.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorInputAttestation {
    /// The verified input handle used as the operand.
    pub input_handle: [u8; 32],
    /// All ciphertext handles covered by the proof.
    pub ct_handles: Vec<[u8; 32]>,
    /// Index of `input_handle` within `ct_handles`.
    pub handle_index: u8,
    /// Attested user identity (bytes32).
    pub user_address: [u8; 32],
    /// Attested contract identity — the input's ACL domain key (bytes32).
    pub contract_address: [u8; 32],
    /// Gateway-side contract chain id the attestation binds.
    pub contract_chain_id: u64,
    /// Opaque extra data covered by the attestation.
    pub extra_data: Vec<u8>,
    /// Coprocessor EIP-712 signatures (65-byte secp256k1).
    pub signatures: Vec<[u8; 65]>,
}

/// Operand source for a composed fhe_execute operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheExecuteOperand {
    /// A value read out of persistent ACL state: a canonical `EncryptedValue` account in
    /// `remaining_accounts` whose current handle matches the interned one. Admission is the
    /// signature of the value's authority, found among the execution's signers.
    StoredValue {
        /// Dictionary index of the handle expected as the encrypted value's current handle.
        handle_index: u8,
        /// Index into `remaining_accounts` for the `EncryptedValue` account.
        encrypted_value_index: u8,
    },
    /// The output of an earlier step of this same `fhe_execute`: usable only inside the current
    /// evaluation scope and never stored.
    EarlierStep {
        /// Producer operation index.
        producer_index: u8,
    },
    /// Plaintext scalar bytes (dictionary index). Scalar operands are only valid on the RHS.
    Scalar {
        /// Dictionary index of the scalar value.
        value_index: u8,
    },
    /// External encrypted input verified in-execution by re-running the coprocessor attestation.
    /// The "allow" is instruction-local (no ACL record, no session, no PDA): the input is usable
    /// only where it is consumed in the same `fhe_execute`. Valid as an encrypted operand, not a scalar.
    VerifiedInput {
        /// The inline attestation re-verified to authorize this operand.
        // Boxed so the ~190-byte attestation is paid only by operands that carry one, not
        // inlined into every `FheExecuteOperand` slot of every step (a Rust enum is as large as
        // its fattest variant, and executions live in `Vec<FheExecuteStep>` on the 32KB SBF bump heap
        // on both sides of the CPI). `Box<T>` is borsh- and IDL-transparent: the wire format
        // is unchanged.
        attestation: Box<CoprocessorInputAttestation>,
    },
}

/// One seed of the authority PDA a create proves, so the host can check the authority belongs
/// to the declared program. The bump is the last seed, as a one-byte literal.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum PdaSeed {
    /// A 32-byte seed already in the dictionary (a mint, an owner, the program itself).
    Interned {
        /// Dictionary index.
        index: u8,
    },
    /// Any other seed bytes: a tag such as `b"token-account"`, or the bump.
    Literal {
        /// The seed bytes, at most 32.
        bytes: Vec<u8>,
    },
}

/// Output policy for a composed fhe_execute operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheExecuteOutput {
    /// The result stays inside the current `fhe_execute` scope; no persistent ACL record.
    Transient,
    /// The result is bound into persistent ACL state: the `EncryptedValue` account PDA is created
    /// when absent, or its handle replaced when it exists.
    StoredValue {
        /// Index into `remaining_accounts` for the output `EncryptedValue` PDA.
        output_encrypted_value_index: u8,
        /// Optional index into `remaining_accounts` for the encrypted value account authority
        /// signer.
        ///
        /// `None` uses the fixed `encrypted_value_account_authority` account in the execution
        /// context. `Some(index)` requires that remaining account to be a signer
        /// and to match the declared output authority.
        output_authority_index: Option<u8>,
        /// Dictionary index of the application program the output value belongs to.
        output_program_index: u8,
        /// Dictionary index of the encrypted value account authority declared for the output.
        output_authority_key_index: u8,
        /// Dictionary index of the program-declared scope of the output value.
        output_scope_index: u8,
        /// Dictionary index of the encrypted value label for the output.
        output_label_index: u8,
        /// On create, the seeds proving the authority is a PDA of the program (bump last). The
        /// host recomputes the address and refuses a create whose authority another program
        /// controls. Empty on update: the stored program was proven when the value was created,
        /// and the canonical address check binds the declared program to it.
        output_authority_seeds: Vec<PdaSeed>,
        /// Dictionary indexes of the keys allowed to decrypt the NEW handle, sealed as one
        /// historical-access leaf each, in this order, right after the handle is written. Empty
        /// is legal: nobody can decrypt that handle.
        output_allow_indexes: Vec<u8>,
        /// Dictionary index of the handle being replaced: `None` on create, the stored current
        /// handle on update (validated against the account, so an execution built on stale state
        /// fails instead of overwriting a newer handle).
        previous_handle_index: Option<u8>,
        /// When true, the new handle is sealed publicly decryptable after its allow leaves
        /// (byte-identical to `make_handle_public`). Carried in instruction data so indexers
        /// reconstruct that leaf without reading the account (DD-036).
        make_public: bool,
    },
}

impl FheBinaryOpCode {
    /// Stable byte encoding used in handle derivation and events.
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Div => 3,
            Self::Rem => 4,
            Self::And => 5,
            Self::Or => 6,
            Self::Xor => 7,
            Self::Shl => 8,
            Self::Shr => 9,
            Self::Rotl => 10,
            Self::Rotr => 11,
            Self::Eq => 12,
            Self::Ne => 13,
            Self::Ge => 14,
            Self::Gt => 15,
            Self::Le => 16,
            Self::Lt => 17,
            Self::Min => 18,
            Self::Max => 19,
        }
    }
}

impl FheTernaryOpCode {
    /// Stable byte encoding used in handle derivation and events.
    pub fn as_u8(self) -> u8 {
        match self {
            Self::IfThenElse => 25,
        }
    }
}

/// Returns the chain id embedded in a handle.
pub fn handle_chain_id(handle: [u8; 32]) -> u64 {
    let mut chain_id = [0u8; 8];
    chain_id.copy_from_slice(&handle[22..30]);
    u64::from_be_bytes(chain_id)
}

/// Returns the FHE type id embedded in a handle.
pub fn handle_fhe_type(handle: [u8; 32]) -> u8 {
    handle[30]
}

/// Checks that a handle targets this host chain and uses supported metadata.
pub fn assert_handle_for_chain(handle: [u8; 32], chain_id: u64) -> Result<()> {
    require!(
        handle_chain_id(handle) == chain_id,
        ZamaHostError::InvalidInputHandleChain
    );
    require!(
        handle[31] == HANDLE_VERSION,
        ZamaHostError::InvalidInputHandleVersion
    );
    require!(
        is_supported_fhe_type(handle_fhe_type(handle)),
        ZamaHostError::InvalidInputHandleType
    );
    Ok(())
}

/// Checks that an external encrypted-input handle targets this host chain.
fn assert_input_handle_for_chain(handle: [u8; 32], chain_id: u64) -> Result<()> {
    assert_handle_for_chain(handle, chain_id)?;
    require!(
        handle[21] != COMPUTED_HANDLE_MARKER,
        ZamaHostError::InvalidInputHandle
    );
    Ok(())
}

/// Checks that an external encrypted-input handle is in the selected proof slot.
pub fn assert_input_handle_metadata(
    handle: [u8; 32],
    chain_id: u64,
    handle_index: u8,
) -> Result<()> {
    assert_input_handle_for_chain(handle, chain_id)?;
    require!(
        handle[21] == handle_index,
        ZamaHostError::InvalidInputHandleIndex
    );
    Ok(())
}

/// Returns the canonical singleton host config address.
pub fn host_config_address() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[HOST_CONFIG_SEED], &crate::ID)
}

/// Returns the canonical KMS context address for a context id.
pub fn kms_context_address(context_id: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[KMS_CONTEXT_SEED, &context_id], &crate::ID)
}

/// The application identity every host policy keys on: the program that proved it controls a
/// value's authority, and the scope that program declared for it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppScope {
    pub program: Pubkey,
    pub scope: [u8; 32],
}

impl AppScope {
    fn address(&self, prefix: &[u8]) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[prefix, self.program.as_ref(), &self.scope], &crate::ID)
    }
}

/// Returns the canonical deny-list address for an application.
pub fn deny_scope_address(app: AppScope) -> (Pubkey, u8) {
    app.address(DENY_SCOPE_SEED)
}

/// Returns the canonical HCU trust-registry record address for an application.
pub fn hcu_trusted_app_address(app: AppScope) -> (Pubkey, u8) {
    app.address(HCU_TRUSTED_APP_SEED)
}

/// Returns the canonical HCU block meter address for an application.
pub fn hcu_block_meter_address(app: AppScope) -> (Pubkey, u8) {
    app.address(HCU_BLOCK_METER_SEED)
}

/// Returns the canonical singleton random-seed nonce address.
pub fn rand_nonce_address() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[RAND_NONCE_SEED], &crate::ID)
}

/// Returns the canonical permit-invalidation watermark address for a user.
pub fn permit_invalidation_address(user: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PERMIT_INVALIDATION_SEED, user.as_ref()], &crate::ID)
}

/// Returns the canonical user-decryption delegation address.
pub fn user_decryption_delegation_address(
    delegator: Pubkey,
    delegate: Pubkey,
    encrypted_value_account_authority: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            encrypted_value_account_authority.as_ref(),
        ],
        &crate::ID,
    )
}

fn finish_computed_handle(result: &mut [u8; 32], chain_id_bytes: &[u8; 8], fhe_type: u8) {
    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
}

/// Slot and chain context bound into every content-addressed handle
/// derivation. Passing it as one value keeps `previous_bank_hash` unswappable
/// with operand handles — three same-repr `[u8; 32]` values meet at these call
/// sites otherwise.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandleDerivationContext {
    pub chain_id: u64,
    pub previous_bank_hash: [u8; 32],
    pub unix_timestamp: i64,
}

/// Derives a content-addressed binary handle (EVM `FHEVMExecutor` shape):
/// no salt beyond slot entropy, so an identical computation derives the
/// identical handle — the same value, by construction.
pub fn computed_eval_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    ctx: &HandleDerivationContext,
) -> [u8; 32] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let op_byte = [op.as_u8()];
    let scalar_byte = [u8::from(scalar)];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = keccak_hashv(&[
        b"FHE_eval",
        &op_byte,
        &lhs,
        &rhs,
        &scalar_byte,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives a content-addressed ternary handle (see [`computed_eval_handle`]).
pub fn computed_eval_ternary_handle(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    ctx: &HandleDerivationContext,
) -> [u8; 32] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let op_byte = [op.as_u8()];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = keccak_hashv(&[
        b"FHE_eval_ternary",
        &op_byte,
        &control,
        &if_true,
        &if_false,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives a content-addressed trivial-encrypt handle (see [`computed_eval_handle`]).
pub fn computed_eval_trivial_handle(
    plaintext: [u8; 32],
    fhe_type: u8,
    ctx: &HandleDerivationContext,
) -> [u8; 32] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = keccak_hashv(&[
        b"FHE_eval_trivial",
        &plaintext,
        &fhe_type_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives the compulsorily fresh seed for an instruction-local execution random handle.
///
/// Freshness is anchored, never caller-advised: `rand_nonce` is the host's global counter,
/// consumed by this execution and never seen again, so two executions in one slot cannot share
/// a seed. `op_index` separates rand steps within one execution; slot entropy separates slots;
/// the application identity binds the seed to the values it will land in.
pub fn computed_eval_rand_seed(
    rand_nonce: u64,
    app: AppScope,
    op_index: u16,
    ctx: &HandleDerivationContext,
) -> [u8; 16] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let chain_id_bytes = chain_id.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let nonce_bytes = rand_nonce.to_be_bytes();
    let hash = keccak_hashv(&[
        b"FHE_eval_seed",
        &nonce_bytes,
        &op_index_bytes,
        app.program.as_ref(),
        &app.scope,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();
    let mut seed = [0; 16];
    seed.copy_from_slice(&hash[..16]);
    seed
}

/// Derives a content-addressed sum handle (see [`computed_eval_handle`]).
pub fn computed_eval_sum_handle(
    operand_handles: &[[u8; 32]],
    fhe_type: u8,
    ctx: &HandleDerivationContext,
) -> [u8; 32] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut preimage: Vec<&[u8]> = vec![b"FHE_eval_sum", &fhe_type_bytes];
    for h in operand_handles {
        preimage.push(h.as_ref());
    }
    preimage.push(crate::ID.as_ref());
    preimage.push(&chain_id_bytes);
    preimage.push(&previous_bank_hash);
    preimage.push(&timestamp_bytes);
    let mut result = keccak_hashv(preimage.as_slice()).to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives a content-addressed is-in handle (see [`computed_eval_handle`]).
pub fn computed_eval_is_in_handle(
    value_handle: [u8; 32],
    set_handles: &[[u8; 32]],
    fhe_type: u8,
    ctx: &HandleDerivationContext,
) -> [u8; 32] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut preimage: Vec<&[u8]> = vec![b"FHE_eval_is_in", &fhe_type_bytes, &value_handle];
    for h in set_handles {
        preimage.push(h.as_ref());
    }
    preimage.push(crate::ID.as_ref());
    preimage.push(&chain_id_bytes);
    preimage.push(&previous_bank_hash);
    preimage.push(&timestamp_bytes);
    let mut result = keccak_hashv(preimage.as_slice()).to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, 0 /* ebool */);
    result
}

/// Derives a content-addressed mul-div handle (see [`computed_eval_handle`]).
pub fn computed_eval_mul_div_handle(
    factor1: [u8; 32],
    factor2: [u8; 32],
    divisor: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
    ctx: &HandleDerivationContext,
) -> [u8; 32] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let scalar_byte = [u8::from(scalar)];
    let mut result = keccak_hashv(&[
        b"FHE_eval_mul_div",
        &factor1,
        &factor2,
        &divisor,
        &scalar_byte,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ])
    .to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, output_fhe_type);
    result
}

/// Deterministically derives a random-ciphertext handle from the emitted seed.
pub fn computed_rand_handle(seed: [u8; 16], fhe_type: u8, chain_id: u64) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = keccak_hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[3],
        &fhe_type_bytes,
        &seed,
        crate::ID.as_ref(),
        &chain_id_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Deterministically derives a bounded-random ciphertext handle from the emitted seed.
pub fn computed_rand_bounded_handle(
    upper_bound: [u8; 32],
    seed: [u8; 16],
    fhe_type: u8,
    chain_id: u64,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = keccak_hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[4],
        &upper_bound,
        &fhe_type_bytes,
        &seed,
        crate::ID.as_ref(),
        &chain_id_bytes,
    ])
    .to_bytes();

    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Derives a content-addressed unary handle (see [`computed_eval_handle`]).
pub fn computed_eval_unary_handle(
    op: FheUnaryOpCode,
    operand: [u8; 32],
    fhe_type: u8,
    ctx: &HandleDerivationContext,
) -> [u8; 32] {
    let HandleDerivationContext {
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    } = *ctx;
    let op_byte = [op.as_u8()];
    let type_byte = [fhe_type];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    // Cast binds its target type into the prehandle (EVM `FHEVMExecutor.cast`); Neg/Not take it from the operand.
    let mut parts: Vec<&[u8]> = vec![b"FHE_eval_unary", &op_byte, &operand];
    if matches!(op, FheUnaryOpCode::Cast) {
        parts.push(&type_byte);
    }
    parts.extend_from_slice(&[
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
    ]);
    let mut result = keccak_hashv(&parts).to_bytes();
    finish_computed_handle(&mut result, &chain_id_bytes, fhe_type);
    result
}

/// Returns the latest prior bank hash.
///
/// Handle derivation must fail closed when the runtime cannot provide the
/// prior bank hash. Solana can skip slots, so this uses the most recent
/// `SlotHashes` entry below `current_slot` rather than requiring
/// `current_slot - 1` to exist.
pub fn previous_bank_hash(current_slot: u64) -> Result<[u8; 32]> {
    if current_slot == 0 {
        return Err(error!(ZamaHostError::PreviousBankHashUnavailable));
    }
    // Read the `SlotHashes` sysvar via the `sol_get_sysvar` syscall instead of
    // `PodSlotHashes::fetch()` (broken as of solana-sysvar <= 4.2.0: it allocates an align-1
    // buffer, then errors unless the bump allocator happened to leave it 8-aligned). The
    // syscall copies raw bytes — `[u64 count][ (u64 slot, [u8;32] hash) ...]` — with no
    // alignment requirement, and entries are parsed with `from_le_bytes`.
    //
    // Entries are ordered newest-first, so the answer is in the first few. Read only a small
    // window: the full 20_488-byte sysvar would burn 2/3 of the default 32KB bump heap
    // (never freed) per call.
    const ENTRY_LEN: usize = 40; // u64 slot + 32-byte hash
    const MAX_SCAN_ENTRIES: usize = 16;

    let mut count_bytes = [0u8; 8];
    get_sysvar(&mut count_bytes, &solana_sysvar::slot_hashes::id(), 0, 8)
        .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;
    let count = u64::from_le_bytes(count_bytes) as usize;
    if count == 0 {
        return Err(error!(ZamaHostError::PreviousBankHashUnavailable));
    }

    let scan = count.min(MAX_SCAN_ENTRIES);
    let mut data = vec![0u8; scan * ENTRY_LEN];
    get_sysvar(
        &mut data,
        &solana_sysvar::slot_hashes::id(),
        8,
        (scan * ENTRY_LEN) as u64,
    )
    .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;

    let entries = (0..scan).filter_map(|index| {
        let offset = index * ENTRY_LEN;
        let slot = u64::from_le_bytes(data[offset..offset + 8].try_into().ok()?);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&data[offset + 8..offset + ENTRY_LEN]);
        Some((slot, hash))
    });
    latest_prior_bank_hash_from_entries(current_slot, entries)
        .ok_or_else(|| error!(ZamaHostError::PreviousBankHashUnavailable))
}

fn latest_prior_bank_hash_from_entries<I>(current_slot: u64, entries: I) -> Option<[u8; 32]>
where
    I: IntoIterator<Item = (u64, [u8; 32])>,
{
    entries
        .into_iter()
        .find_map(|(slot, hash)| (slot < current_slot).then_some(hash))
}

#[cfg(test)]
mod tests;
