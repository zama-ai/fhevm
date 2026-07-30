//! Durable-output addressing and ACL policy types.

use crate::types::FheType;

use anchor_lang::prelude::Pubkey;

use zama_host::encrypted_value_address;

use crate::validate::{validate_access_policy, validate_durable_slot};
use crate::{EvalBuildError, Result};

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

/// App-domain address of a stable `EncryptedValue` encrypted value account.
///
/// Addressing is stable per `(namespace, account, label)` — it does not rotate
/// on handle updates, unlike the old nonce-keyed ACL records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableSlot {
    pub(crate) namespace: Pubkey,
    pub(crate) account: Pubkey,
    pub(crate) label: DurableLabel,
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
    pub(crate) pubkey: Pubkey,
}

impl AccessSubject {
    /// Owner subject allowed on the durable value.
    pub fn owner(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    pub fn compute(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    pub fn use_only(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    pub fn pubkey(self) -> Pubkey {
        self.pubkey
    }

    pub fn matches_record_entry(self, pubkey: Pubkey) -> bool {
        self.pubkey == pubkey
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
/// is the encrypted value account's first (the `EncryptedValue` PDA is created).
#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviousEncryptedValueAccountState {
    handle: [u8; 32],
    subjects: Vec<Pubkey>,
}

/// Durable output descriptor accepted by durable-only steps such as input bind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableOutput {
    slot: DurableSlot,
    access: AccessPolicy,
    previous: Option<PreviousEncryptedValueAccountState>,
    make_public: bool,
}

impl DurableOutput {
    /// First bind for an encrypted value account: creates the `EncryptedValue` PDA.
    pub fn create(slot: DurableSlot, access: AccessPolicy) -> Self {
        Self {
            slot,
            access,
            previous: None,
            make_public: false,
        }
    }

    /// Supersedes an existing encrypted value account. `previous_handle`/`previous_subjects`
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
            previous: Some(PreviousEncryptedValueAccountState {
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

/// Host-ready metadata for creating or superseding a durable `EncryptedValue` encrypted value account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableOutputBirth {
    encrypted_value: Pubkey,
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
    subjects: Vec<AccessSubject>,
    previous: Option<PreviousEncryptedValueAccountState>,
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

    pub(crate) fn host_subjects(&self) -> Vec<Pubkey> {
        self.subjects
            .iter()
            .map(|subject| subject.pubkey())
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
pub struct Output(pub(crate) OutputKind);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OutputKind {
    Transient,
    Durable(DurableOutput),
}

impl Output {
    pub fn transient() -> Self {
        Self(OutputKind::Transient)
    }

    /// First bind for an encrypted value account (creates the `EncryptedValue` PDA).
    pub fn durable(slot: DurableSlot, access: AccessPolicy) -> Self {
        Self(OutputKind::Durable(DurableOutput::create(slot, access)))
    }

    pub fn durable_output(output: DurableOutput) -> Self {
        Self(OutputKind::Durable(output))
    }
}
