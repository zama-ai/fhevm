//! State and deterministic helper functions for the ZamaHost program.
//!
//! This module is intentionally reusable from app programs and tests. It
//! exposes the PDA seeds, role flags, account layouts, and handle formulas
//! needed to prepare CPI accounts and to verify host-owned ACL state off-chain.

use anchor_lang::prelude::*;
use solana_sha256_hasher::hashv;
use solana_sysvar::slot_hashes::PodSlotHashes;

use crate::errors::ZamaHostError;

/// Version byte written to host protocol events.
pub const EVENT_VERSION: u8 = 0;
/// Number of subjects embedded directly in an [`AclRecord`].
///
/// Logical ACL sets are not capped at this value. Additional subjects are
/// represented by [`AclPermission`] witness PDAs.
pub const MAX_ACL_SUBJECTS: usize = 8;
/// PoC chain id used by tests and helpers that do not receive [`HostConfig`].
pub const SOLANA_POC_CHAIN_ID: u64 = 12345;

/// Seed for the singleton [`HostConfig`] PDA.
pub const HOST_CONFIG_SEED: &[u8] = b"host-config";
/// Seed prefix for canonical [`AclRecord`] PDAs.
pub const ACL_RECORD_SEED: &[u8] = b"acl-record";
/// Seed prefix for overflow [`AclPermission`] PDAs.
pub const ACL_PERMISSION_SEED: &[u8] = b"acl-permission";
/// Seed prefix for grant deny-list records.
pub const DENY_SUBJECT_SEED: &[u8] = b"deny-subject";
/// Seed prefix for user-decryption delegation records.
pub const DELEGATION_SEED: &[u8] = b"user-decryption-delegation";
/// Reserved app-context sentinel for wildcard user-decryption delegation rows.
pub const WILDCARD_APP_CONTEXT_BYTES: [u8; 32] = [0xff; 32];
/// Seed prefix for one-shot transient compute capability sessions.
pub const TRANSIENT_SESSION_SEED: &[u8] = b"transient-session";
/// Seed prefix for host-owned material commitment records.
pub const HANDLE_MATERIAL_SEED: &[u8] = b"handle-material";

/// Subject may use a handle in host operations and decrypt checks.
pub const ACL_ROLE_USE: u8 = 0x01;
/// Subject may extend persistent ACL membership for a handle.
pub const ACL_ROLE_GRANT: u8 = 0x02;
/// Subject may mark a handle as publicly decryptable.
pub const ACL_ROLE_PUBLIC_DECRYPT: u8 = 0x04;
/// Subject is intended for FHE compute execution.
pub const ACL_ROLE_COMPUTE: u8 = 0x08;
/// All role bits currently understood by the host and KMS verifier.
pub const ACL_ROLE_KNOWN: u8 =
    ACL_ROLE_USE | ACL_ROLE_GRANT | ACL_ROLE_PUBLIC_DECRYPT | ACL_ROLE_COMPUTE;
/// Convenience role set for a full user/owner subject.
pub const ACL_ROLE_ALL: u8 = ACL_ROLE_USE | ACL_ROLE_GRANT | ACL_ROLE_PUBLIC_DECRYPT;
/// Convenience role set for the PoC compute signer.
///
/// This grants handle use and compute authority, but not persistent grant or
/// public decrypt authority.
pub const ACL_ROLE_COMPUTE_SUBJECT: u8 = ACL_ROLE_USE | ACL_ROLE_COMPUTE;
/// Convenience role set used for owner/user subjects in the token PoC.
pub const ACL_ROLE_USER: u8 = ACL_ROLE_ALL;
/// Maximum number of FHE operations accepted by one composed eval.
pub const MAX_FHE_EVAL_OPS: usize = 16;
/// Maximum number of external encrypted-input handles in one signed proof.
pub const MAX_INPUT_PROOF_HANDLES: usize = 16;
/// Maximum opaque verifier payload bytes carried in one signed input proof.
pub const MAX_INPUT_PROOF_EXTRA_DATA: usize = 256;
/// Maximum number of capability entries stored in one transient session.
///
/// Transient sessions intentionally follow the one-shot capability model from
/// `solana/docs/TRANSIENT_ALLOW.md`: one account, one capability, one
/// successful consume.
pub const MAX_TRANSIENT_CAPABILITIES: usize = 1;
/// Transient session accepts new capabilities.
pub const TRANSIENT_SESSION_STATE_OPEN: u8 = 0;
/// Transient session may be consumed but no longer mutated by append APIs.
pub const TRANSIENT_SESSION_STATE_SEALED: u8 = 1;
/// Material commitment state for committed/decryptable material.
pub const HANDLE_MATERIAL_STATE_COMMITTED: u8 = 1;

const COMPUTATION_DOMAIN_SEPARATOR: &[u8] = b"FHE_comp";
const RANDOM_SEED_DOMAIN_SEPARATOR: &[u8] = b"FHE_seed";
const COMPUTED_HANDLE_MARKER: u8 = 0xff;
/// Current handle encoding version byte.
pub const HANDLE_VERSION: u8 = 0;

const INPUT_PROOF_DOMAIN_SEPARATOR: &[u8] = b"zama-solana-input-proof-v1";

/// Initialization arguments for the singleton [`HostConfig`] account.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeHostConfigArgs {
    /// Host-chain id encoded into newly derived handles.
    pub chain_id: u64,
    /// Authority used by mock and signed encrypted-input bind paths.
    pub input_verifier_authority: Pubkey,
    /// Authority allowed to commit ciphertext material readiness.
    pub material_authority: Pubkey,
    /// Authority allowed to call `test_emit_*` event shims.
    pub test_authority: Pubkey,
    /// Whether the mock encrypted-input bind path is enabled.
    pub mock_input_enabled: bool,
    /// Whether test event shims are enabled.
    pub test_shims_enabled: bool,
    /// Whether persistent grants must include a deny-list witness.
    pub grant_deny_list_enabled: bool,
}

/// Singleton host configuration and authority surface.
///
/// `HostConfig` is the runtime switchboard for this PoC. Production-shaped
/// instructions reject while paused, and mock/test-only instructions require
/// both the corresponding feature gate and configured signer.
#[account]
pub struct HostConfig {
    /// Program administrator allowed to update config flags.
    pub admin: Pubkey,
    /// Host-chain id included in handle derivation.
    pub chain_id: u64,
    /// Configured authority for input verification paths.
    pub input_verifier_authority: Pubkey,
    /// Configured authority for material-commitment paths.
    pub material_authority: Pubkey,
    /// Configured signer for `test_emit_*` shims.
    pub test_authority: Pubkey,
    /// Pauses production-shaped host instructions when true.
    pub paused: bool,
    /// Enables the mock encrypted-input bind instruction.
    pub mock_input_enabled: bool,
    /// Enables test event shim instructions.
    pub test_shims_enabled: bool,
    /// Enables deny-list checks for persistent grant authorities.
    pub grant_deny_list_enabled: bool,
    /// Slot in which the config was initialized or last changed.
    pub updated_slot: u64,
    /// PDA bump for `PDA("host-config")`.
    pub bump: u8,
}

impl HostConfig {
    pub const SPACE: usize = 32 + 8 + 32 + 32 + 32 + 1 + 1 + 1 + 1 + 8 + 1;
}

/// Pubkey plus role flags stored inline or in an overflow permission PDA.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AclSubjectEntry {
    /// Subject identity. For app users this is usually the owner pubkey; for
    /// app programs this can be a program-controlled compute signer PDA.
    pub pubkey: Pubkey,
    /// Bitset of `ACL_ROLE_*` flags.
    pub role_flags: u8,
}

impl AclSubjectEntry {
    /// Builds an owner/user subject with use, grant, and public-decrypt roles.
    pub fn user(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            role_flags: ACL_ROLE_USER,
        }
    }

    /// Builds a compute subject without public-decrypt authority.
    pub fn compute(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            role_flags: ACL_ROLE_COMPUTE_SUBJECT,
        }
    }

    /// Builds a subject that may use the handle but may not grant or publish it.
    pub fn use_only(pubkey: Pubkey) -> Self {
        Self {
            pubkey,
            role_flags: ACL_ROLE_USE,
        }
    }
}

/// Canonical ACL state for one host handle.
///
/// The account address is independent from the opaque handle:
/// `PDA("acl-record", nonce_key, nonce_sequence)`. The handle is stored in the
/// account body so computed handles can be bound after transaction accounts are
/// declared.
#[account]
pub struct AclRecord {
    /// Opaque FHEVM handle controlled by this ACL record.
    pub handle: [u8; 32],
    /// Domain-separated nonce key derived from app account metadata.
    pub nonce_key: [u8; 32],
    /// App-maintained sequence for this nonce key.
    pub nonce_sequence: u64,
    /// App-level ACL domain, such as a confidential token mint.
    pub acl_domain_key: Pubkey,
    /// App-owned account whose encrypted field is represented by this handle.
    pub app_account: Pubkey,
    /// Domain-separated encrypted field label inside `app_account`.
    pub encrypted_value_label: [u8; 32],
    /// Inline subjects for the common case.
    pub subjects: [Pubkey; MAX_ACL_SUBJECTS],
    /// Role flags parallel to [`AclRecord::subjects`].
    pub subject_roles: [u8; MAX_ACL_SUBJECTS],
    /// Number of valid entries in the inline subject arrays.
    pub subject_count: u8,
    /// Number of overflow subjects represented by [`AclPermission`] PDAs.
    pub overflow_subject_count: u32,
    /// Durable public-decrypt flag checked by KMS-style verification.
    pub public_decrypt: bool,
    /// Canonical material commitment PDA sealed to this ACL record.
    pub material_commitment: Pubkey,
    /// Canonical material commitment hash sealed to this ACL record.
    pub material_commitment_hash: [u8; 32],
    /// Material key identifier sealed to this ACL record.
    pub material_key_id: [u8; 32],
    /// Slot in which this ACL record was first bound.
    pub created_slot: u64,
    /// PDA bump for this record.
    pub bump: u8,
}

impl AclRecord {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32
        + 32
        + 8
        + 32
        + 32
        + 32
        + (32 * MAX_ACL_SUBJECTS)
        + MAX_ACL_SUBJECTS
        + 1
        + 4
        + 1
        + 32
        + 32
        + 32
        + 8
        + 1;

    /// Returns the inline subject index if the subject is embedded in the ACL record.
    pub fn inline_subject_index(&self, subject: Pubkey) -> Option<usize> {
        let subject_count = (self.subject_count as usize).min(MAX_ACL_SUBJECTS);
        self.subjects[..subject_count]
            .iter()
            .position(|candidate| *candidate == subject)
    }

    /// Returns true when an inline subject has every flag in `role`.
    pub fn inline_subject_has_role(&self, subject: Pubkey, role: u8) -> bool {
        self.inline_subject_index(subject)
            .map(|index| subject_has_role(self.subject_roles[index], role))
            .unwrap_or(false)
    }
}

/// Overflow subject witness for an [`AclRecord`].
///
/// The canonical address is `PDA("acl-permission", acl_record, subject)`.
/// KMS/Gateway requests that rely on overflow membership must carry this
/// account as an explicit witness.
#[account]
pub struct AclPermission {
    /// ACL record this permission extends.
    pub acl_record: Pubkey,
    /// Subject granted by this overflow permission.
    pub subject: Pubkey,
    /// Bitset of `ACL_ROLE_*` flags granted to `subject`.
    pub role_flags: u8,
    /// PDA bump for this permission account.
    pub bump: u8,
}

impl AclPermission {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 1 + 1;
}

/// Host-owned commitment proving ciphertext material is available for a handle.
///
/// ACL birth and material readiness are deliberately separate. KMS-style
/// decryption checks must verify both the canonical ACL record and this
/// material commitment before authorizing a decryptable handle.
#[account]
pub struct HandleMaterialCommitment {
    /// ACL record whose handle has committed material.
    pub acl_record: Pubkey,
    /// Handle copied from the ACL record at commitment time.
    pub handle: [u8; 32],
    /// Release/key identifier for the ciphertext material.
    pub key_id: [u8; 32],
    /// Digest of the ciphertext material.
    pub ciphertext_digest: [u8; 32],
    /// Digest of the SNS ciphertext material.
    pub sns_ciphertext_digest: [u8; 32],
    /// Release-pinned coprocessor-set digest.
    pub coprocessor_set_digest: [u8; 32],
    /// Canonical commitment hash over the material witness fields.
    pub material_commitment_hash: [u8; 32],
    /// Slot in which the commitment was recorded.
    pub created_slot: u64,
    /// Commitment state. Native-v0 decryptability requires committed.
    pub state: u8,
    /// PDA bump for this material commitment.
    pub bump: u8,
}

impl HandleMaterialCommitment {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + (32 * 5) + 8 + 1 + 1;
}

/// Optional deny-list record used to fail persistent grants closed.
#[account]
pub struct DenySubjectRecord {
    /// Subject controlled by this deny-list record.
    pub subject: Pubkey,
    /// Whether `subject` is currently denied for grant-authority use.
    pub denied: bool,
    /// PDA bump for this deny-list account.
    pub bump: u8,
}

impl DenySubjectRecord {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 1 + 1;
}

/// PoC user-decryption delegation witness.
///
/// Gateway/KMS payloads do not yet carry these records, but the account shape is
/// present so the final witness format has a concrete Solana state target.
#[account]
pub struct UserDecryptionDelegation {
    /// User granting delegated decrypt rights.
    pub delegator: Pubkey,
    /// Delegate allowed to request user decryption.
    pub delegate: Pubkey,
    /// App context for the delegation.
    pub app_account: Pubkey,
    /// Slot after which the delegation is invalid.
    pub expiration_slot: u64,
    /// Monotonic counter incremented on every grant, regrant, and revoke.
    pub delegation_counter: u64,
    /// Slot in which this row was last updated.
    pub last_update_slot: u64,
    /// Whether the delegation has been revoked by the delegator.
    pub revoked: bool,
    /// PDA bump for this delegation account.
    pub bump: u8,
}

impl UserDecryptionDelegation {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32 + 32 + 32 + 8 + 8 + 8 + 1 + 1;
}

/// Native Solana verifier payload for binding external encrypted inputs.
///
/// The proof is not trusted by itself. `verify_input_and_bind` requires an
/// Ed25519 verifier pre-instruction from `HostConfig::input_verifier_authority`
/// over [`input_proof_message`]. The selected handle from `handles` is the only
/// handle that may be written into the output ACL record.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SolanaInputProof {
    /// Ordered handles certified by the input verifier.
    pub handles: Vec<[u8; 32]>,
    /// Selected handle index for this bind operation.
    pub handle_index: u8,
    /// User associated with the encrypted input.
    pub user: Pubkey,
    /// App account to which this input authorization is scoped.
    pub app_account: Pubkey,
    /// App ACL domain to which this input authorization is scoped.
    pub acl_domain_key: Pubkey,
    /// Opaque verifier payload, such as transcript/proof metadata.
    pub extra_data: Vec<u8>,
}

/// ACL metadata covered by a native Solana encrypted-input verifier signature.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct SolanaInputBindIntent {
    /// Nonce key for the output ACL record.
    pub output_nonce_key: [u8; 32],
    /// Nonce sequence for the output ACL record.
    pub output_nonce_sequence: u64,
    /// ACL domain key for the output ACL record.
    pub output_acl_domain_key: Pubkey,
    /// App account authorized to create the output ACL record.
    pub output_app_account: Pubkey,
    /// Encrypted value label for the output ACL record.
    pub output_encrypted_value_label: [u8; 32],
    /// Initial subjects on the output ACL record.
    pub output_subjects: Vec<AclSubjectEntry>,
    /// Initial public decrypt flag on the output ACL record.
    pub output_public_decrypt: bool,
}

/// Caller-supplied policy for adding a transient capability to a session.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransientCapabilityGrant {
    /// Subject allowed to use the handle through the session.
    pub subject: Pubkey,
    /// Top-level receiver program expected to consume the capability.
    pub receiver_program: Pubkey,
    /// App-level ACL domain to which durable output, if allowed, must remain bound.
    pub acl_domain_key: Pubkey,
    /// App account to which durable output, if allowed, must remain bound.
    pub app_account: Pubkey,
    /// Roles granted by the transient capability.
    pub role_flags: u8,
    /// Maximum number of successful uses before the capability is exhausted.
    ///
    /// The host currently requires this to be `1`.
    pub max_uses: u8,
    /// Whether a computation using this capability may bind a durable output ACL record.
    pub durable_output_allowed: bool,
    /// Whether a computation using this capability may make the durable output public-decryptable.
    pub public_decrypt_allowed: bool,
}

impl TransientCapabilityGrant {
    /// Serialized size of the account body.
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 1 + 1 + 1 + 1;
}

/// One one-shot transient handle capability.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransientCapability {
    /// Handle controlled by the capability.
    pub handle: [u8; 32],
    /// Capability policy.
    pub grant: TransientCapabilityGrant,
    /// Number of times the capability has been consumed.
    pub used_count: u8,
}

impl TransientCapability {
    /// Serialized size of the account body.
    pub const SPACE: usize = 32 + TransientCapabilityGrant::SPACE + 1;
}

/// Host-owned one-shot transient compute capability account.
///
/// This is not durable ACL state and must not be accepted for KMS/public/user
/// decrypt witnesses. It is a short-lived explicit account witness for
/// same-slot compute authorization across instructions or CPIs.
#[account]
pub struct TransientSession {
    /// Caller-chosen session nonce used in the PDA.
    pub session_nonce: [u8; 32],
    /// Authority that may append, seal, consume, and close before expiry.
    pub authority: Pubkey,
    /// Account that receives lamports when the session is closed.
    pub refund_recipient: Pubkey,
    /// Subject allowed by capabilities in this session.
    pub compute_subject: Pubkey,
    /// Slot in which the session was created.
    pub created_slot: u64,
    /// Last slot in which sealed capabilities may be consumed.
    pub expires_slot: u64,
    /// Session state. See `TRANSIENT_SESSION_STATE_*`.
    pub state: u8,
    /// Caller-selected capacity, currently required to be `1`.
    pub max_entries: u8,
    /// Capability entries.
    pub entries: Vec<TransientCapability>,
    /// PDA bump for this session.
    pub bump: u8,
}

impl TransientSession {
    /// Serialized size of the account body, excluding Anchor discriminator.
    pub const SPACE: usize = 32
        + 32
        + 32
        + 32
        + 8
        + 8
        + 1
        + 1
        + 4
        + (MAX_TRANSIENT_CAPABILITIES * TransientCapability::SPACE)
        + 1;
}

/// Binary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheBinaryOpCode {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Greater-than-or-equal comparison.
    Ge,
}

/// Ternary FHE operators currently modeled by the PoC.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FheTernaryOpCode {
    /// Selects `if_true` when `control` is true, otherwise `if_false`.
    IfThenElse,
}

/// Arguments for composed instruction-local FHE evaluation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FheEvalArgs {
    /// Caller-chosen domain separator for transient handles in this eval.
    ///
    /// Callers should use a fresh value for each logical eval session.
    pub context_id: [u8; 32],
    /// Ordered operation list. Each transient operand may only reference an
    /// output produced by an earlier index in this vector.
    pub ops: Vec<FheEvalOp>,
}

/// One binary operation inside a composed FHE eval.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FheEvalOp {
    /// Binary operator.
    pub op: FheBinaryOpCode,
    /// Left-hand operand.
    pub lhs: FheEvalOperand,
    /// Right-hand operand or scalar bytes.
    pub rhs: FheEvalOperand,
    /// FHE type byte embedded in the output handle.
    pub output_fhe_type: u8,
    /// Caller-supplied output handle verified by host derivation.
    pub result: [u8; 32],
    /// Whether this output remains instruction-local or is bound into durable ACL state.
    pub output: FheEvalOutput,
}

/// Operand source for a composed FHE eval operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalOperand {
    /// Durable input authorized by a canonical ACL record in `remaining_accounts`.
    Durable {
        /// Handle expected in the ACL record.
        handle: [u8; 32],
        /// Index into `remaining_accounts` for the ACL record.
        acl_record_index: u16,
        /// Optional index into `remaining_accounts` for overflow subject permission.
        permission_index: Option<u16>,
    },
    /// Instruction-local output produced by an earlier operation.
    Transient {
        /// Producer operation index.
        producer_index: u16,
    },
    /// Sealed one-shot session capability in `remaining_accounts`.
    TransientSession {
        /// Handle expected in the session capability.
        handle: [u8; 32],
        /// Index into `remaining_accounts` for the transient session account.
        session_index: u16,
        /// Entry index inside the transient session.
        capability_index: u16,
    },
    /// Plaintext scalar bytes. Scalar operands are only valid on the RHS.
    Scalar([u8; 32]),
}

/// Output policy for a composed FHE eval operation.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum FheEvalOutput {
    /// Output remains instruction-local and has no durable ACL record.
    Transient,
    /// Output is appended to an open transient session instead of durable ACL state.
    TransientSession {
        /// Index into `remaining_accounts` for the transient session account.
        session_index: u16,
        /// Capability policy for the produced result.
        capability: TransientCapabilityGrant,
    },
    /// Output is bound into a durable ACL record.
    Durable {
        /// Index into `remaining_accounts` for the output ACL record PDA.
        output_acl_record_index: u16,
        /// Nonce key for the output ACL record.
        output_nonce_key: [u8; 32],
        /// Nonce sequence for the output ACL record.
        output_nonce_sequence: u64,
        /// ACL domain key for the output ACL record.
        output_acl_domain_key: Pubkey,
        /// App account authorized to create the output ACL record.
        output_app_account: Pubkey,
        /// Encrypted value label for the output ACL record.
        output_encrypted_value_label: [u8; 32],
        /// Initial subjects on the output ACL record.
        output_subjects: Vec<AclSubjectEntry>,
        /// Initial public decrypt flag on the output ACL record.
        output_public_decrypt: bool,
    },
}

impl FheBinaryOpCode {
    /// Stable byte encoding used in handle derivation and events.
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Ge => 14,
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

/// Returns true when `role_flags` contains every flag in `role`.
pub fn subject_has_role(role_flags: u8, role: u8) -> bool {
    role_flags & role == role
}

/// Returns true when a role bitset is nonempty and uses only known role bits.
pub fn role_flags_are_known(role_flags: u8) -> bool {
    role_flags != 0 && role_flags & !ACL_ROLE_KNOWN == 0
}

/// Returns true when the inline ACL subject arrays are exact and KMS-decodable.
pub fn acl_record_subject_slots_are_canonical(record: &AclRecord) -> bool {
    let subject_count = record.subject_count as usize;
    if subject_count > MAX_ACL_SUBJECTS {
        return false;
    }
    let mut index = 0usize;
    while index < subject_count {
        let subject = record.subjects[index];
        let role_flags = record.subject_roles[index];
        if subject == Pubkey::default() || !role_flags_are_known(role_flags) {
            return false;
        }
        let mut previous = 0usize;
        while previous < index {
            if record.subjects[previous] == subject {
                return false;
            }
            previous += 1;
        }
        index += 1;
    }
    while index < MAX_ACL_SUBJECTS {
        if record.subjects[index] != Pubkey::default() || record.subject_roles[index] != 0 {
            return false;
        }
        index += 1;
    }
    true
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
pub fn assert_input_handle_for_chain(handle: [u8; 32], chain_id: u64) -> Result<()> {
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

pub fn assert_supported_fhe_type(fhe_type: u8) -> Result<()> {
    require!(
        is_supported_fhe_type(fhe_type),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

/// Checks that a binary operation's declared result type matches the shipped operator.
pub fn assert_supported_binary_output_type(op: FheBinaryOpCode, fhe_type: u8) -> Result<()> {
    assert_supported_fhe_type(fhe_type)?;
    let valid = match op {
        FheBinaryOpCode::Add | FheBinaryOpCode::Sub => matches!(fhe_type, 2..=6),
        FheBinaryOpCode::Ge => fhe_type == 0,
    };
    require!(valid, ZamaHostError::UnsupportedFheType);
    Ok(())
}

/// Checks binary operand metadata against the EVM executor's type discipline.
pub fn assert_binary_operand_types(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    output_fhe_type: u8,
) -> Result<()> {
    assert_supported_binary_output_type(op, output_fhe_type)?;
    let lhs_type = handle_fhe_type(lhs);
    require!(
        matches!(lhs_type, 2..=6),
        ZamaHostError::UnsupportedFheType
    );
    if matches!(op, FheBinaryOpCode::Add | FheBinaryOpCode::Sub) {
        require!(
            lhs_type == output_fhe_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    if !scalar {
        require!(
            handle_fhe_type(rhs) == lhs_type,
            ZamaHostError::BinaryOperandTypeMismatch
        );
    }
    Ok(())
}

pub fn assert_supported_rand_type(fhe_type: u8) -> Result<()> {
    require!(
        matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 8),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

pub fn assert_supported_bounded_rand_type(fhe_type: u8) -> Result<()> {
    require!(
        bounded_rand_type_bits(fhe_type).is_some(),
        ZamaHostError::UnsupportedFheType
    );
    Ok(())
}

pub fn assert_valid_bounded_rand_upper_bound(upper_bound: [u8; 32], fhe_type: u8) -> Result<()> {
    assert_supported_bounded_rand_type(fhe_type)?;
    let bit_index =
        power_of_two_bit_index(upper_bound).ok_or(ZamaHostError::InvalidRandomUpperBound)?;
    if let Some(max_bits) = bounded_rand_type_bits(fhe_type).flatten() {
        require!(
            bit_index <= max_bits,
            ZamaHostError::InvalidRandomUpperBound
        );
    }
    Ok(())
}

fn is_supported_fhe_type(fhe_type: u8) -> bool {
    matches!(fhe_type, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 8)
}

fn bounded_rand_type_bits(fhe_type: u8) -> Option<Option<u16>> {
    match fhe_type {
        2 => Some(Some(8)),
        3 => Some(Some(16)),
        4 => Some(Some(32)),
        5 => Some(Some(64)),
        6 => Some(Some(128)),
        8 => Some(None),
        _ => None,
    }
}

fn power_of_two_bit_index(value: [u8; 32]) -> Option<u16> {
    let mut bit_index = None;
    for (byte_index, byte) in value.iter().enumerate() {
        if *byte == 0 {
            continue;
        }
        if byte.count_ones() != 1 || bit_index.is_some() {
            return None;
        }
        let bit_in_byte = 7 - byte.leading_zeros() as u16;
        bit_index = Some(((31 - byte_index) as u16 * 8) + bit_in_byte);
    }
    bit_index
}

/// Canonical bytes signed by the native Solana input verifier authority.
pub fn input_proof_message(
    proof: &SolanaInputProof,
    bind_intent: &SolanaInputBindIntent,
    host_program_id: Pubkey,
    chain_id: u64,
) -> Vec<u8> {
    let mut message = Vec::with_capacity(
        INPUT_PROOF_DOMAIN_SEPARATOR.len()
            + 32
            + 8
            + 32
            + 32
            + 32
            + 32
            + 8
            + 32
            + 32
            + 32
            + 4
            + (bind_intent.output_subjects.len() * (32 + 1))
            + 1
            + 4
            + 1
            + (proof.handles.len() * 32)
            + 4
            + proof.extra_data.len(),
    );
    message.extend_from_slice(INPUT_PROOF_DOMAIN_SEPARATOR);
    message.extend_from_slice(host_program_id.as_ref());
    message.extend_from_slice(&chain_id.to_be_bytes());
    message.extend_from_slice(proof.user.as_ref());
    message.extend_from_slice(proof.app_account.as_ref());
    message.extend_from_slice(proof.acl_domain_key.as_ref());
    message.extend_from_slice(&bind_intent.output_nonce_key);
    message.extend_from_slice(&bind_intent.output_nonce_sequence.to_be_bytes());
    message.extend_from_slice(bind_intent.output_acl_domain_key.as_ref());
    message.extend_from_slice(bind_intent.output_app_account.as_ref());
    message.extend_from_slice(&bind_intent.output_encrypted_value_label);
    message.extend_from_slice(&(bind_intent.output_subjects.len() as u32).to_be_bytes());
    for subject in &bind_intent.output_subjects {
        message.extend_from_slice(subject.pubkey.as_ref());
        message.push(subject.role_flags);
    }
    message.push(u8::from(bind_intent.output_public_decrypt));
    message.extend_from_slice(&(proof.handles.len() as u32).to_be_bytes());
    message.push(proof.handle_index);
    for handle in &proof.handles {
        message.extend_from_slice(handle);
    }
    message.extend_from_slice(&(proof.extra_data.len() as u32).to_be_bytes());
    message.extend_from_slice(&proof.extra_data);
    message
}

/// Returns the canonical singleton host config address.
pub fn host_config_address() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[HOST_CONFIG_SEED], &crate::ID)
}

/// Returns the canonical ACL record address for a nonce key and sequence.
pub fn acl_record_address(nonce_key: [u8; 32], nonce_sequence: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            ACL_RECORD_SEED,
            nonce_key.as_ref(),
            &nonce_sequence.to_le_bytes(),
        ],
        &crate::ID,
    )
}

/// Returns the canonical overflow permission address for a subject.
pub fn acl_permission_address(acl_record: Pubkey, subject: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[ACL_PERMISSION_SEED, acl_record.as_ref(), subject.as_ref()],
        &crate::ID,
    )
}

/// Returns the canonical deny-list address for a subject.
pub fn deny_subject_address(subject: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DENY_SUBJECT_SEED, subject.as_ref()], &crate::ID)
}

/// Returns the canonical user-decryption delegation address.
pub fn user_decryption_delegation_address(
    delegator: Pubkey,
    delegate: Pubkey,
    app_account: Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            DELEGATION_SEED,
            delegator.as_ref(),
            delegate.as_ref(),
            app_account.as_ref(),
        ],
        &crate::ID,
    )
}

/// Returns the canonical transient session address for an authority and nonce.
pub fn transient_session_address(authority: Pubkey, session_nonce: [u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            TRANSIENT_SESSION_SEED,
            authority.as_ref(),
            session_nonce.as_ref(),
        ],
        &crate::ID,
    )
}

/// Returns the canonical material commitment address for an ACL record.
pub fn handle_material_address(acl_record: Pubkey) -> (Pubkey, u8) {
    let (host_config, _) = host_config_address();
    Pubkey::find_program_address(
        &[
            HANDLE_MATERIAL_SEED,
            host_config.as_ref(),
            acl_record.as_ref(),
        ],
        &crate::ID,
    )
}

/// Computes the commitment hash stored in [`HandleMaterialCommitment`].
pub fn handle_material_commitment_hash(
    material_commitment: Pubkey,
    acl_record: Pubkey,
    key_id: [u8; 32],
    ciphertext_digest: [u8; 32],
    sns_ciphertext_digest: [u8; 32],
    coprocessor_set_digest: [u8; 32],
) -> [u8; 32] {
    let (host_config, _) = host_config_address();
    hashv(&[
        b"zama-solana-material-commitment-v1",
        host_config.as_ref(),
        crate::ID.as_ref(),
        material_commitment.as_ref(),
        acl_record.as_ref(),
        &key_id,
        &ciphertext_digest,
        &sns_ciphertext_digest,
        &coprocessor_set_digest,
    ])
    .to_bytes()
}

/// Derives the app-controlled nonce key for one encrypted field.
///
/// The nonce key intentionally contains app metadata, not the opaque handle.
/// This lets the app predeclare the ACL account address before the host program
/// computes or binds the final handle.
pub fn acl_nonce_key(
    acl_domain_key: Pubkey,
    app_account: Pubkey,
    encrypted_value_label: [u8; 32],
) -> [u8; 32] {
    hashv(&[
        b"zama-acl-nonce-key-v1",
        acl_domain_key.as_ref(),
        app_account.as_ref(),
        &encrypted_value_label,
    ])
    .to_bytes()
}

/// Derives an unbound binary-op handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer [`computed_binary_handle_for_current_slot_with_chain_id`].
pub fn computed_binary_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
) -> Result<[u8; 32]> {
    computed_binary_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
    )
}

/// Derives an unbound binary-op handle using the current slot context and chain id.
pub fn computed_binary_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
    ))
}

/// Derives a nonce-bound binary-op output handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer
/// [`computed_bound_binary_handle_for_current_slot_with_chain_id`].
pub fn computed_bound_binary_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    computed_bound_binary_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        output_nonce_key,
        output_nonce_sequence,
    )
}

/// Derives a nonce-bound binary-op output handle using the current slot context and chain id.
pub fn computed_bound_binary_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_bound_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    ))
}

/// Derives a nonce-bound ternary-op output handle using the current slot context and chain id.
pub fn computed_bound_ternary_handle_for_current_slot_with_chain_id(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_bound_ternary_handle(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        output_nonce_key,
        output_nonce_sequence,
    ))
}

/// Derives an instruction-local eval handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer [`computed_eval_handle_for_current_slot_with_chain_id`].
pub fn computed_eval_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
) -> Result<[u8; 32]> {
    computed_eval_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        context_id,
        op_index,
    )
}

/// Derives an instruction-local eval handle using the current slot context and chain id.
pub fn computed_eval_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    context_id: [u8; 32],
    op_index: u16,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        context_id,
        op_index,
    ))
}

/// Derives a nonce-bound eval output handle using the current slot context.
///
/// This helper uses [`SOLANA_POC_CHAIN_ID`]. CPI callers that have a
/// [`HostConfig`] should prefer
/// [`computed_bound_eval_handle_for_current_slot_with_chain_id`].
pub fn computed_bound_eval_handle_for_current_slot(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    computed_bound_eval_handle_for_current_slot_with_chain_id(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        SOLANA_POC_CHAIN_ID,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    )
}

/// Derives a nonce-bound eval output handle using the current slot context and chain id.
pub fn computed_bound_eval_handle_for_current_slot_with_chain_id(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> Result<[u8; 32]> {
    let clock = Clock::get()?;
    let previous_bank_hash = previous_bank_hash(clock.slot)?;
    Ok(computed_bound_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        clock.unix_timestamp,
        context_id,
        op_index,
        output_nonce_key,
        output_nonce_sequence,
    ))
}

/// Deterministically derives an unbound binary-op handle from explicit entropy.
pub fn computed_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let scalar_byte = [u8::from(scalar)];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
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

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

/// Deterministically derives a nonce-bound binary-op output handle.
///
/// Binding the handle to `(output_nonce_key, output_nonce_sequence)` prevents
/// two durable output ACL records from intentionally storing the same computed
/// handle for the same operation.
pub fn computed_bound_binary_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_binary_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &hashv(&[
            b"FHE_bound_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
    result
}

/// Derives an unbound ternary-op handle from explicit entropy.
pub fn computed_ternary_handle(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
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

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

/// Deterministically derives a nonce-bound ternary-op output handle.
pub fn computed_bound_ternary_handle(
    op: FheTernaryOpCode,
    control: [u8; 32],
    if_true: [u8; 32],
    if_false: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_ternary_handle(
        op,
        control,
        if_true,
        if_false,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &hashv(&[
            b"FHE_bound_ternary_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
    result
}

/// Derives an instruction-local eval operation handle from explicit entropy.
pub fn computed_eval_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
) -> [u8; 32] {
    let op_byte = [op.as_u8()];
    let scalar_byte = [u8::from(scalar)];
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let op_index_bytes = op_index.to_be_bytes();
    let mut result = hashv(&[
        b"FHE_eval",
        &context_id,
        &op_index_bytes,
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

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

/// Derives a nonce-bound durable output handle for composed eval.
pub fn computed_bound_eval_handle(
    op: FheBinaryOpCode,
    lhs: [u8; 32],
    rhs: [u8; 32],
    scalar: bool,
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    context_id: [u8; 32],
    op_index: u16,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let base_result = computed_eval_handle(
        op,
        lhs,
        rhs,
        scalar,
        fhe_type,
        chain_id,
        previous_bank_hash,
        unix_timestamp,
        context_id,
        op_index,
    );
    let mut result = base_result;
    result[..21].copy_from_slice(
        &hashv(&[
            b"FHE_bound_eval_output",
            &base_result,
            &output_nonce_key,
            &sequence_bytes,
        ])
        .to_bytes()[..21],
    );
    result
}

/// Deterministically derives a trivial-encrypt handle from explicit entropy.
pub fn computed_trivial_handle(
    plaintext: [u8; 32],
    fhe_type: u8,
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[2],
        &plaintext,
        &fhe_type_bytes,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
        &output_nonce_key,
        &sequence_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

/// Derives the random seed emitted for a durable random-handle birth.
pub fn computed_rand_seed(
    chain_id: u64,
    previous_bank_hash: [u8; 32],
    unix_timestamp: i64,
    output_nonce_key: [u8; 32],
    output_nonce_sequence: u64,
) -> [u8; 16] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let timestamp_bytes = unix_timestamp.to_be_bytes();
    let sequence_bytes = output_nonce_sequence.to_be_bytes();
    let hash = hashv(&[
        RANDOM_SEED_DOMAIN_SEPARATOR,
        crate::ID.as_ref(),
        &chain_id_bytes,
        &previous_bank_hash,
        &timestamp_bytes,
        &output_nonce_key,
        &sequence_bytes,
    ])
    .to_bytes();
    hash[..16].try_into().expect("slice has length 16")
}

/// Deterministically derives a random-ciphertext handle from the emitted seed.
pub fn computed_rand_handle(seed: [u8; 16], fhe_type: u8, chain_id: u64) -> [u8; 32] {
    let chain_id_bytes = chain_id.to_be_bytes();
    let fhe_type_bytes = [fhe_type];
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[3],
        &fhe_type_bytes,
        &seed,
        crate::ID.as_ref(),
        &chain_id_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
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
    let mut result = hashv(&[
        COMPUTATION_DOMAIN_SEPARATOR,
        &[4],
        &upper_bound,
        &fhe_type_bytes,
        &seed,
        crate::ID.as_ref(),
        &chain_id_bytes,
    ])
    .to_bytes();

    result[21..32].fill(0);
    result[21] = COMPUTED_HANDLE_MARKER;
    result[22..30].copy_from_slice(&chain_id_bytes);
    result[30] = fhe_type;
    result[31] = HANDLE_VERSION;
    result
}

/// Returns the previous slot hash, or zero in the initial local-test slot.
///
/// The zero fallback is test glue for LiteSVM bootstrap behavior. Real cluster
/// execution is expected to use the slot-hash sysvar branch.
pub fn previous_bank_hash(current_slot: u64) -> Result<[u8; 32]> {
    let Some(previous_slot) = current_slot.checked_sub(1) else {
        return Ok([0; 32]);
    };
    let slot_hashes =
        PodSlotHashes::fetch().map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?;
    if let Some(hash) = slot_hashes
        .get(&previous_slot)
        .map_err(|_| error!(ZamaHostError::PreviousBankHashUnavailable))?
    {
        return Ok(hash.to_bytes());
    }

    // LiteSVM starts from an empty slot-hash history in these PoC tests.
    // Real cluster execution should take the branch above.
    Ok([0; 32])
}

/// Returns true when an ACL record grants the base use role to an inline subject.
pub fn record_allows(record: &AclRecord, subject: Pubkey) -> bool {
    record.inline_subject_has_role(subject, ACL_ROLE_USE)
}
