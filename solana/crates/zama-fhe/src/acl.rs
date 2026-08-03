//! Persistent-output addressing and ACL policy types.
//!
//! Public API surface: app programs. The bounded-randomness constructors and the audience types
//! are how an app declares who may read a persistent output, so they are exported for callers
//! outside this repository.

use crate::types::FheType;

use anchor_lang::prelude::Pubkey;

use zama_host::encrypted_value_address;

use crate::validate::{validate_encrypted_value_id, validate_subjects};
use crate::{BatchBuildError, Result};

/// App-level ACL domain of an `EncryptedValue` account, such as a confidential token mint.
///
/// A domain and the account it scopes are both plain pubkeys, so the two used to be
/// interchangeable at every derivation site; this type is what makes swapping them a compile
/// error instead of a wrong PDA.
///
/// `repr(transparent)`, so a domain is passed exactly like the pubkey it wraps. What the wrapper
/// still costs is the copy its constructor makes: measured across the snapshotted instructions,
/// between 0 and 16 CU each depending on how the surrounding code inlines it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Domain(Pubkey);

impl Domain {
    pub const fn new(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }

    pub const fn pubkey(self) -> Pubkey {
        self.0
    }
}

/// App-domain encrypted field label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersistentLabel([u8; 32]);

impl PersistentLabel {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

/// App-domain key of a stable `EncryptedValue` account.
///
/// Addressing is stable per `(domain, account, label)` — it does not change
/// on handle updates, unlike the old nonce-keyed ACL records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedValueId {
    pub(crate) domain: Domain,
    pub(crate) account: Pubkey,
    pub(crate) label: PersistentLabel,
}

impl EncryptedValueId {
    /// ```
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{Domain, EncryptedValueId, PersistentLabel};
    ///
    /// let mint = Pubkey::new_unique();
    /// let token_account = Pubkey::new_unique();
    /// let id = EncryptedValueId::new(Domain::new(mint), token_account, PersistentLabel::new([1; 32]));
    /// assert_eq!(id.domain().pubkey(), mint);
    /// assert_eq!(id.account(), token_account);
    /// ```
    ///
    /// Passing the two pubkeys the other way round does not compile, so a swapped pair can no
    /// longer address a different value account than the app meant:
    ///
    /// ```compile_fail
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{Domain, EncryptedValueId, PersistentLabel};
    ///
    /// let mint = Pubkey::new_unique();
    /// let token_account = Pubkey::new_unique();
    /// EncryptedValueId::new(token_account, Domain::new(mint), PersistentLabel::new([1; 32]));
    /// ```
    pub fn new(domain: Domain, account: Pubkey, label: PersistentLabel) -> Self {
        Self {
            domain,
            account,
            label,
        }
    }

    pub fn address(&self) -> Pubkey {
        encrypted_value_address(self.encrypted_value_id()).0
    }

    pub fn domain(&self) -> Domain {
        self.domain
    }

    pub fn account(&self) -> Pubkey {
        self.account
    }

    pub fn label(&self) -> PersistentLabel {
        self.label
    }

    pub fn encrypted_value_id(&self) -> [u8; 32] {
        zama_solana_acl::derive_encrypted_value_id(
            self.domain.pubkey().to_bytes(),
            self.account.to_bytes(),
            self.label.bytes(),
        )
    }
}

/// Previous on-chain state a persistent output updates. `None` means this bind
/// is the encrypted value account's first (the `EncryptedValue` PDA is created).
#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviousEncryptedValueAccountState {
    handle: [u8; 32],
    subjects: Vec<Pubkey>,
}

/// Persistent output descriptor accepted by persistent-only steps such as input bind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistentOutput {
    key: EncryptedValueId,
    subjects: Vec<Pubkey>,
    previous: Option<PreviousEncryptedValueAccountState>,
    make_public: bool,
}

impl PersistentOutput {
    /// First bind for an encrypted value account: creates the `EncryptedValue` PDA.
    pub fn create(key: EncryptedValueId, subjects: Vec<Pubkey>) -> Self {
        Self {
            key,
            subjects,
            previous: None,
            make_public: false,
        }
    }

    /// Updates an existing encrypted value account. `current` must be read from
    /// the on-chain account in the same instruction; the host verifies its
    /// handle and subjects exactly.
    pub fn update(
        key: EncryptedValueId,
        subjects: Vec<Pubkey>,
        current: &zama_host::EncryptedValue,
    ) -> Self {
        Self {
            key,
            subjects,
            previous: Some(PreviousEncryptedValueAccountState {
                handle: current.current_handle,
                subjects: current.subjects.clone(),
            }),
            make_public: false,
        }
    }

    /// Opts this output into being created publicly decryptable: the host seals a
    /// public-decrypt leaf for the newly bound handle inside the same fhe_execute CPI
    /// (EVM `unwrap`'s `makePubliclyDecryptable` parity; DD-036).
    pub fn with_make_public(mut self, make_public: bool) -> Self {
        self.make_public = make_public;
        self
    }

    pub fn binding(&self) -> Result<PersistentOutputBinding> {
        validate_encrypted_value_id(&self.key)?;
        validate_subjects(&self.subjects)?;
        Ok(PersistentOutputBinding {
            encrypted_value: self.key.address(),
            domain: self.key.domain,
            account: self.key.account,
            label: self.key.label.bytes(),
            subjects: self.subjects.clone(),
            previous: self.previous.clone(),
            make_public: self.make_public,
        })
    }
}

/// Host-ready metadata for creating or updating a persistent `EncryptedValue` encrypted value account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistentOutputBinding {
    encrypted_value: Pubkey,
    domain: Domain,
    account: Pubkey,
    label: [u8; 32],
    subjects: Vec<Pubkey>,
    previous: Option<PreviousEncryptedValueAccountState>,
    make_public: bool,
}

impl PersistentOutputBinding {
    pub fn encrypted_value(&self) -> Pubkey {
        self.encrypted_value
    }

    pub fn domain(&self) -> Domain {
        self.domain
    }

    pub fn account(&self) -> Pubkey {
        self.account
    }

    pub fn label(&self) -> [u8; 32] {
        self.label
    }

    pub fn subjects(&self) -> &[Pubkey] {
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

    /// The declared previous state in the host's wire shape (`None` on create).
    pub fn previous_state(&self) -> Option<zama_host::PreviousState> {
        self.previous
            .as_ref()
            .map(|previous| zama_host::PreviousState {
                handle: previous.handle,
                subjects: previous.subjects.clone(),
            })
    }

    pub fn make_public(&self) -> bool {
        self.make_public
    }

    pub(crate) fn host_subjects(&self) -> Vec<Pubkey> {
        self.subjects.clone()
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
            return Err(BatchBuildError::InvalidRandomUpperBound);
        }
        let mut bytes = [0u8; 32];
        bytes[24..].copy_from_slice(&value.to_be_bytes());
        Self::from_be_bytes(bytes)
    }

    pub fn from_be_bytes(value: [u8; 32]) -> Result<Self> {
        zama_host::assert_valid_bounded_rand_upper_bound(value, FheType::UINT64.byte())
            .map_err(|_| BatchBuildError::InvalidRandomUpperBound)?;
        Ok(Self { value })
    }

    pub fn bytes(self) -> [u8; 32] {
        self.value
    }
}

impl TryFrom<u64> for BoundedU64UpperBound {
    type Error = BatchBuildError;

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
    Persistent(PersistentOutput),
}

impl Output {
    pub fn transient() -> Self {
        Self(OutputKind::Transient)
    }

    /// Binds the step output persistently. Whether the output creates or updates its
    /// `EncryptedValue` PDA is said at the call site through [`PersistentOutput::create`] /
    /// [`PersistentOutput::update`].
    pub fn persistent(output: PersistentOutput) -> Self {
        Self(OutputKind::Persistent(output))
    }
}
