//! Persistent-output addressing and ACL policy types.
//!
//! Public API surface: app programs. [`PersistentOutput`] is how an app declares a write — which
//! account it creates or updates, which keys may read the new handle — so it is exported for
//! callers outside this repository.

use crate::types::FheType;

use anchor_lang::prelude::Pubkey;

use zama_host::{encrypted_value_address, PdaSeed};

use crate::validate::{validate_allow_keys, validate_encrypted_value_id};
use crate::{FheExecutionBuildError, Result};

pub use zama_host::AppScope;

/// The encrypted value label: an encrypted value ID's last component.
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

/// App-level key of a stable `EncryptedValue` account.
///
/// Addressing is stable per `(program, encrypted value account authority, scope, label)` — it
/// does not change on handle updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptedValueId {
    pub(crate) app: AppScope,
    pub(crate) encrypted_value_account_authority: Pubkey,
    pub(crate) label: EncryptedValueLabel,
    /// The encrypted value account's PDA and bump, derived once at construction: on-chain the
    /// derivation is a syscall the app pays where it builds the id, so the builder's heap tally
    /// stays exact, and an id used across several steps derives once instead of per use.
    pub(crate) address: Pubkey,
    pub(crate) bump: u8,
}

impl EncryptedValueId {
    /// ```
    /// use anchor_lang::prelude::Pubkey;
    /// use zama_fhe::{AppScope, EncryptedValueId, EncryptedValueLabel};
    ///
    /// let program = Pubkey::new_unique();
    /// let mint = Pubkey::new_unique();
    /// let token_account = Pubkey::new_unique();
    /// let app = AppScope { program, scope: mint.to_bytes() };
    /// let id = EncryptedValueId::new(app, token_account, EncryptedValueLabel::new([1; 32]));
    /// assert_eq!(id.app(), app);
    /// assert_eq!(id.encrypted_value_account_authority(), token_account);
    /// ```
    pub fn new(
        app: AppScope,
        encrypted_value_account_authority: Pubkey,
        label: EncryptedValueLabel,
    ) -> Self {
        let (address, bump) = encrypted_value_address(
            app.program,
            encrypted_value_account_authority,
            app.scope,
            label.bytes(),
        );
        Self {
            app,
            encrypted_value_account_authority,
            label,
            address,
            bump,
        }
    }

    /// The PDA together with its bump, for a caller that signs or re-creates the account and
    /// would otherwise have to run the derivation a second time.
    pub fn address_with_bump(&self) -> (Pubkey, u8) {
        (self.address, self.bump)
    }

    pub fn address(&self) -> Pubkey {
        self.address
    }

    /// The application the value belongs to: the program that controls its authority and the
    /// scope that program declared. Every host policy (metering, deny list, permits) keys on it.
    pub fn app(&self) -> AppScope {
        self.app
    }

    pub fn encrypted_value_account_authority(&self) -> Pubkey {
        self.encrypted_value_account_authority
    }

    pub fn encrypted_value_label(&self) -> EncryptedValueLabel {
        self.label
    }
}

/// Persistent output descriptor: which `EncryptedValue` account a step writes, and who may read
/// the handle it produces.
///
/// Allowing happens on the write and nowhere else (EVM `FHE.allow` after every state change):
/// each [`allow`](Self::allow) seals one historical-access leaf for the new handle. A write with
/// no allows is legal; its handle is then decryptable by nobody until a later write allows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistentOutput {
    key: EncryptedValueId,
    authority_seeds: Vec<PdaSeed>,
    allows: Vec<Pubkey>,
    previous_handle: Option<[u8; 32]>,
    make_public: bool,
}

impl PersistentOutput {
    /// First write to an encrypted value account: creates the `EncryptedValue` PDA.
    ///
    /// `authority_seeds` are the seeds (bump last) that derive the account's authority under
    /// the id's program — the same slices the program signs the CPI with. The host re-derives
    /// the authority from them, which is what makes `(program, scope)` unforgeable.
    pub fn create(key: EncryptedValueId, authority_seeds: &[&[u8]]) -> Self {
        Self {
            key,
            authority_seeds: authority_seeds
                .iter()
                .map(|seed| PdaSeed::Literal {
                    bytes: seed.to_vec(),
                })
                .collect(),
            allows: Vec::new(),
            previous_handle: None,
            make_public: false,
        }
    }

    /// Updates an existing encrypted value account. `current_handle` must be the handle the
    /// account holds when the instruction runs; the host refuses a stale echo, so an execution
    /// built on outdated state fails instead of overwriting a newer handle.
    pub fn update(key: EncryptedValueId, current_handle: [u8; 32]) -> Self {
        Self {
            key,
            authority_seeds: Vec::new(),
            allows: Vec::new(),
            previous_handle: Some(current_handle),
            make_public: false,
        }
    }

    /// Allows `key` to decrypt the handle this write produces.
    pub fn allow(mut self, key: Pubkey) -> Self {
        self.allows.push(key);
        self
    }

    /// Seals the produced handle publicly decryptable inside the same fhe_execute CPI (EVM
    /// `makePubliclyDecryptable` parity; DD-036).
    pub fn make_public(mut self) -> Self {
        self.make_public = true;
        self
    }

    /// The exact checks [`binding`](Self::binding) and the lowering path run, without
    /// allocating. On-chain callers that only need the verdict should call this: on the
    /// never-freeing program heap, the binding they would discard costs real reserve bytes.
    pub fn validate(&self) -> Result<()> {
        validate_encrypted_value_id(&self.key)?;
        validate_allow_keys(&self.allows)
    }

    /// Validate first — the rejecting path allocates nothing — then one clone into the moving
    /// path below, so the two spellings cannot validate or bind differently.
    pub fn binding(&self) -> Result<PersistentOutputBinding> {
        self.validate()?;
        self.clone().into_binding()
    }

    /// [`binding`](Self::binding) for the lowering path, which owns the output: the seed and
    /// allow lists move instead of being cloned, so lowering a persistent output allocates
    /// nothing for data the app already built.
    pub(crate) fn into_binding(self) -> Result<PersistentOutputBinding> {
        self.validate()?;
        Ok(PersistentOutputBinding {
            encrypted_value: self.key.address(),
            app: self.key.app,
            encrypted_value_account_authority: self.key.encrypted_value_account_authority,
            label: self.key.label.bytes(),
            authority_seeds: self.authority_seeds,
            allows: self.allows,
            previous_handle: self.previous_handle,
            make_public: self.make_public,
        })
    }

    pub(crate) fn app(&self) -> AppScope {
        self.key.app
    }
}

/// Host-ready metadata for creating or updating a persistent `EncryptedValue` account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistentOutputBinding {
    pub(crate) encrypted_value: Pubkey,
    pub(crate) app: AppScope,
    pub(crate) encrypted_value_account_authority: Pubkey,
    pub(crate) label: [u8; 32],
    pub(crate) authority_seeds: Vec<PdaSeed>,
    pub(crate) allows: Vec<Pubkey>,
    pub(crate) previous_handle: Option<[u8; 32]>,
    pub(crate) make_public: bool,
}

impl PersistentOutputBinding {
    pub fn encrypted_value(&self) -> Pubkey {
        self.encrypted_value
    }

    pub fn app(&self) -> AppScope {
        self.app
    }

    pub fn encrypted_value_account_authority(&self) -> Pubkey {
        self.encrypted_value_account_authority
    }

    pub fn encrypted_value_label(&self) -> [u8; 32] {
        self.label
    }

    pub fn allows(&self) -> &[Pubkey] {
        &self.allows
    }

    pub fn previous_handle(&self) -> Option<[u8; 32]> {
        self.previous_handle
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
