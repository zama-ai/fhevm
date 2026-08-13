//! Persistent-output addressing and ACL policy types.
//!
//! Public API surface: app programs. The bounded-randomness constructors and the audience types
//! are how an app declares who may read a persistent output, so they are exported for callers
//! outside this repository.

use crate::types::FheType;

use anchor_lang::prelude::Pubkey;

use zama_host::encrypted_value_address;

use crate::validate::{validate_encrypted_value_id, validate_subjects};
use crate::{FheExecutionBuildError, Result};

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

/// The encrypted value label: an encrypted value ID's third component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncryptedValueLabel([u8; 32]);

impl EncryptedValueLabel {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

/// App-domain key of a stable `EncryptedValue` account.
///
/// Addressing is stable per `(domain, encrypted value account authority, encrypted value
/// label)` — it does not change
/// on handle updates, unlike the old nonce-keyed ACL records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedValueId {
    pub(crate) domain: Domain,
    pub(crate) encrypted_value_account_authority: Pubkey,
    pub(crate) label: EncryptedValueLabel,
    /// The encrypted value account's PDA, derived once at construction: on-chain the derivation
    /// is a syscall the app pays where it builds the id, so the builder's heap tally stays
    /// exact, and an id used across several steps derives once instead of per use.
    pub(crate) address: Pubkey,
}

impl EncryptedValueId {
    /// ```
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{Domain, EncryptedValueId, EncryptedValueLabel};
    ///
    /// let mint = Pubkey::new_unique();
    /// let token_account = Pubkey::new_unique();
    /// let id = EncryptedValueId::new(Domain::new(mint), token_account, EncryptedValueLabel::new([1; 32]));
    /// assert_eq!(id.domain().pubkey(), mint);
    /// assert_eq!(id.encrypted_value_account_authority(), token_account);
    /// ```
    ///
    /// Passing the two pubkeys the other way round does not compile, so a swapped pair can no
    /// longer address a different encrypted value account than the app meant:
    ///
    /// ```compile_fail
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{Domain, EncryptedValueId, EncryptedValueLabel};
    ///
    /// let mint = Pubkey::new_unique();
    /// let token_account = Pubkey::new_unique();
    /// EncryptedValueId::new(token_account, Domain::new(mint), EncryptedValueLabel::new([1; 32]));
    /// ```
    pub fn new(
        domain: Domain,
        encrypted_value_account_authority: Pubkey,
        label: EncryptedValueLabel,
    ) -> Self {
        let address = encrypted_value_address(zama_solana_acl::derive_encrypted_value_id(
            domain.pubkey().to_bytes(),
            encrypted_value_account_authority.to_bytes(),
            label.bytes(),
        ))
        .0;
        Self {
            domain,
            encrypted_value_account_authority,
            label,
            address,
        }
    }

    pub fn address(&self) -> Pubkey {
        self.address
    }

    pub fn domain(&self) -> Domain {
        self.domain
    }

    pub fn encrypted_value_account_authority(&self) -> Pubkey {
        self.encrypted_value_account_authority
    }

    pub fn encrypted_value_label(&self) -> EncryptedValueLabel {
        self.label
    }

    pub fn encrypted_value_id(&self) -> [u8; 32] {
        zama_solana_acl::derive_encrypted_value_id(
            self.domain.pubkey().to_bytes(),
            self.encrypted_value_account_authority.to_bytes(),
            self.label.bytes(),
        )
    }
}

/// Previous on-chain state a persistent output updates. `None` means this bind
/// is the encrypted value account's first (the `EncryptedValue` PDA is created).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreviousEncryptedValueAccountState {
    pub(crate) handle: [u8; 32],
    pub(crate) subjects: Vec<Pubkey>,
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
            encrypted_value_account_authority: self.key.encrypted_value_account_authority,
            label: self.key.label.bytes(),
            subjects: self.subjects.clone(),
            previous: self.previous.clone(),
            make_public: self.make_public,
        })
    }

    /// [`binding`](Self::binding) for the lowering path, which owns the output: the subject list
    /// and previous state move instead of being cloned, so lowering a persistent output
    /// allocates nothing for data the app already built. Same validation as `binding`.
    pub(crate) fn into_binding(self) -> Result<PersistentOutputBinding> {
        validate_encrypted_value_id(&self.key)?;
        validate_subjects(&self.subjects)?;
        Ok(PersistentOutputBinding {
            encrypted_value: self.key.address(),
            domain: self.key.domain,
            encrypted_value_account_authority: self.key.encrypted_value_account_authority,
            label: self.key.label.bytes(),
            subjects: self.subjects,
            previous: self.previous,
            make_public: self.make_public,
        })
    }
}

/// Host-ready metadata for creating or updating a persistent `EncryptedValue` encrypted value account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistentOutputBinding {
    pub(crate) encrypted_value: Pubkey,
    pub(crate) domain: Domain,
    pub(crate) encrypted_value_account_authority: Pubkey,
    pub(crate) label: [u8; 32],
    pub(crate) subjects: Vec<Pubkey>,
    pub(crate) previous: Option<PreviousEncryptedValueAccountState>,
    pub(crate) make_public: bool,
}

impl PersistentOutputBinding {
    pub fn encrypted_value(&self) -> Pubkey {
        self.encrypted_value
    }

    pub fn domain(&self) -> Domain {
        self.domain
    }

    pub fn encrypted_value_account_authority(&self) -> Pubkey {
        self.encrypted_value_account_authority
    }

    pub fn encrypted_value_label(&self) -> [u8; 32] {
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
}

/// Validated power-of-two upper bound for host bounded-random `euint64` creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedU64UpperBound {
    value: [u8; 32],
}

impl BoundedU64UpperBound {
    pub fn power_of_two(value: u64) -> Result<Self> {
        if value == 0 || !value.is_power_of_two() {
            return Err(FheExecutionBuildError::InvalidRandomUpperBound);
        }
        let mut bytes = [0u8; 32];
        bytes[24..].copy_from_slice(&value.to_be_bytes());
        Self::from_be_bytes(bytes)
    }

    pub fn from_be_bytes(value: [u8; 32]) -> Result<Self> {
        zama_host::assert_valid_bounded_rand_upper_bound(value, FheType::UINT64.byte())
            .map_err(|_| FheExecutionBuildError::InvalidRandomUpperBound)?;
        Ok(Self { value })
    }

    pub fn bytes(self) -> [u8; 32] {
        self.value
    }
}

impl TryFrom<u64> for BoundedU64UpperBound {
    type Error = FheExecutionBuildError;

    fn try_from(value: u64) -> Result<Self> {
        Self::power_of_two(value)
    }
}

/// Output policy exposed by the builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output(pub(crate) OutputKind);

#[derive(Debug, Clone, PartialEq, Eq)]
// Passed by value into lowering, which consumes it in place; boxing the persistent variant
// would put one more allocation on the program's never-freeing heap per output.
#[allow(clippy::large_enum_variant)]
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
