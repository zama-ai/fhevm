//! Anchor events emitted by ZamaHost.
//!
//! Events are indexing hints for listeners. Authorization still comes from
//! host-owned account state, so KMS-style consumers must verify the referenced
//! ACL records and witnesses instead of trusting event bytes alone.

use anchor_lang::prelude::*;

use crate::state::{FheBinaryOpCode, FheTernaryOpCode, FheUnaryOpCode};

/// Emitted when the singleton host config is initialized.
#[event]
pub struct HostConfigInitializedEvent {
    /// Event schema version.
    pub version: u8,
    /// Host config PDA.
    pub config: Pubkey,
    /// Configured admin signer.
    pub admin: Pubkey,
    /// Host-chain id used by handle derivation.
    pub chain_id: u64,
    /// Configured input verifier authority.
    pub input_verifier_authority: Pubkey,
    /// Configured material commitment authority.
    pub material_authority: Pubkey,
    /// Configured test-shim authority.
    pub test_authority: Pubkey,
}

/// Emitted when host config flags change.
#[event]
pub struct HostConfigUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Host config PDA.
    pub config: Pubkey,
    /// Admin signer that performed the update.
    pub admin: Pubkey,
    /// Current pause state.
    pub paused: bool,
    /// Current mock input gate.
    pub mock_input_enabled: bool,
    /// Current test-shim gate.
    pub test_shims_enabled: bool,
    /// Current grant deny-list gate.
    pub grant_deny_list_enabled: bool,
    /// Current max total HCU per `fhe_eval` plan (`0` = unlimited).
    pub max_hcu_per_tx: u64,
    /// Current max critical-path HCU per `fhe_eval` plan (`0` = unlimited).
    pub max_hcu_depth_per_tx: u64,
    /// Current per-app HCU block cap (`u64::MAX` = unrestricted, `0` = ban untrusted apps).
    pub hcu_block_cap_per_app: u64,
    /// Slot in which this update was applied.
    pub updated_slot: u64,
}

/// Emitted when a KMS context is defined (mirrors `ProtocolConfig.NewKmsContext`).
#[event]
pub struct NewKmsContextEvent {
    /// Event schema version.
    pub version: u8,
    /// The new context id.
    pub kms_context_id: u64,
    /// KMS node signer EVM addresses authorized in this context.
    pub signers: Vec<[u8; 20]>,
    /// Public-decrypt signature threshold.
    pub public_decryption_threshold: u8,
    /// User-decrypt signature threshold.
    pub user_decryption_threshold: u8,
}

/// Emitted when a KMS context is destroyed (mirrors `ProtocolConfig.KmsContextDestroyed`).
#[event]
pub struct KmsContextDestroyedEvent {
    /// Event schema version.
    pub version: u8,
    /// The destroyed context id.
    pub kms_context_id: u64,
}

/// Emitted when ciphertext material is committed for a host handle.
#[event]
pub struct HandleMaterialCommittedEvent {
    /// Event schema version.
    pub version: u8,
    /// Canonical material commitment PDA.
    pub material_commitment: Pubkey,
    /// Canonical ACL record PDA.
    pub acl_record: Pubkey,
    /// Handle whose material is committed.
    pub handle: [u8; 32],
    /// Release/key identifier for the ciphertext material.
    pub key_id: [u8; 32],
    /// Digest of the ciphertext material.
    pub ciphertext_digest: [u8; 32],
    /// Digest of the SNS ciphertext material.
    pub sns_ciphertext_digest: [u8; 32],
    /// Release-pinned coprocessor-set digest.
    pub coprocessor_set_digest: [u8; 32],
    /// Canonical commitment hash over material witness fields.
    pub material_commitment_hash: [u8; 32],
    /// Slot in which the commitment was recorded.
    pub created_slot: u64,
}

/// Emitted when committed material is sealed onto an ACL record.
#[event]
pub struct HandleMaterialSealedEvent {
    /// Event schema version.
    pub version: u8,
    /// Canonical material commitment PDA.
    pub material_commitment: Pubkey,
    /// Canonical ACL record PDA.
    pub acl_record: Pubkey,
    /// Handle whose material is sealed.
    pub handle: [u8; 32],
    /// Release/key identifier for the ciphertext material.
    pub key_id: [u8; 32],
    /// Canonical commitment hash over material witness fields.
    pub material_commitment_hash: [u8; 32],
    /// Slot in which this seal was applied.
    pub updated_slot: u64,
}

/// Emitted when an ACL record is born and bound to a handle.
#[event]
pub struct AclRecordBoundEvent {
    /// Event schema version.
    pub version: u8,
    /// Canonical ACL record PDA.
    pub acl_record: Pubkey,
    /// Opaque handle stored in the ACL record.
    pub handle: [u8; 32],
    /// Nonce key used in the ACL record PDA.
    pub nonce_key: [u8; 32],
    /// Nonce sequence used in the ACL record PDA.
    pub nonce_sequence: u64,
    /// App-level ACL domain.
    pub acl_domain_key: Pubkey,
    /// App account whose encrypted field is represented.
    pub app_account: Pubkey,
    /// App-level encrypted field label.
    pub encrypted_value_label: [u8; 32],
    /// Number of inline subjects stored at birth.
    pub subject_count: u8,
    /// Initial public-decrypt flag.
    pub public_decrypt: bool,
    /// Slot in which the ACL record was first bound.
    pub created_slot: u64,
}

/// Emitted when a subject is granted on an ACL record.
#[event]
pub struct AclSubjectAllowedEvent {
    /// Event schema version.
    pub version: u8,
    /// Canonical ACL record PDA.
    pub acl_record: Pubkey,
    /// Handle stored in the ACL record.
    pub handle: [u8; 32],
    /// Authority that held `ACL_ROLE_GRANT`.
    pub authority_subject: Pubkey,
    /// Granted subject pubkey bytes.
    pub subject: [u8; 32],
    /// Role flags granted to the subject.
    pub role_flags: u8,
    /// Overflow permission PDA, or the default pubkey for inline subjects.
    pub overflow_permission_record: Pubkey,
    /// Inline subject index, or `u8::MAX` when the grant uses an overflow PDA.
    pub inline_index: u8,
    /// Slot in which this update was applied.
    pub updated_slot: u64,
}

/// Emitted when public decrypt is enabled for a handle.
#[event]
pub struct PublicDecryptAllowedEvent {
    /// Event schema version.
    pub version: u8,
    /// Canonical ACL record PDA.
    pub acl_record: Pubkey,
    /// Publicly decryptable handle.
    pub handle: [u8; 32],
    /// Authority that held `ACL_ROLE_PUBLIC_DECRYPT`.
    pub authority: [u8; 32],
    /// Slot in which this update was applied.
    pub updated_slot: u64,
}

/// Emitted when a subject deny-list record is updated.
#[event]
pub struct DenySubjectUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Canonical deny-list record PDA.
    pub deny_subject_record: Pubkey,
    /// Subject controlled by the deny-list record.
    pub subject: Pubkey,
    /// Whether the subject is denied for grant-authority use.
    pub denied: bool,
    /// Slot in which this update was applied.
    pub updated_slot: u64,
}

/// Emitted when an app's HCU block-cap trust registry entry is updated.
#[event]
pub struct HcuAppTrustUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// Canonical trust-registry record PDA.
    pub hcu_trusted_app_record: Pubkey,
    /// The app authority governed by the record.
    pub app: Pubkey,
    /// Whether the app bypasses the per-app block cap.
    pub trusted: bool,
    /// Slot in which this update was applied.
    pub updated_slot: u64,
}

/// Emitted when user-decryption delegation state changes.
#[event]
pub struct UserDecryptionDelegationUpdatedEvent {
    /// Event schema version.
    pub version: u8,
    /// User granting delegated decrypt rights.
    pub delegator: Pubkey,
    /// Delegate allowed to request user decryption.
    pub delegate: Pubkey,
    /// App context for the delegation.
    pub app_account: Pubkey,
    /// Monotonic counter after this update.
    pub delegation_counter: u64,
    /// Expiration slot before this update.
    pub old_expiration_slot: u64,
    /// Expiration slot after this update.
    pub new_expiration_slot: u64,
    /// Slot in which this update was applied.
    pub last_update_slot: u64,
    /// Whether the delegation is revoked after this update.
    pub revoked: bool,
}

/// Emitted for a binary FHE operation accepted by the host.
#[event]
pub struct FheBinaryOpEvent {
    /// Event schema version.
    pub version: u8,
    /// Binary operator.
    pub op: FheBinaryOpCode,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Left-hand operand handle.
    pub lhs: [u8; 32],
    /// Right-hand operand handle or scalar bytes.
    pub rhs: [u8; 32],
    /// Whether `rhs` is plaintext scalar bytes.
    pub scalar: bool,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Emitted for a ternary FHE operation accepted by the host.
#[event]
pub struct FheTernaryOpEvent {
    /// Event schema version.
    pub version: u8,
    /// Ternary operator.
    pub op: FheTernaryOpCode,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Encrypted control handle.
    pub control: [u8; 32],
    /// Handle selected when `control` is true.
    pub if_true: [u8; 32],
    /// Handle selected when `control` is false.
    pub if_false: [u8; 32],
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Emitted for a trivial encryption accepted by the host.
#[event]
pub struct TrivialEncryptEvent {
    /// Event schema version.
    pub version: u8,
    /// Subject associated with the created handle.
    pub subject: [u8; 32],
    /// Plaintext encoded into the handle.
    pub plaintext: [u8; 32],
    /// FHE type byte embedded in the handle.
    pub fhe_type: u8,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Test-shim event for random ciphertext creation.
#[event]
pub struct FheRandEvent {
    /// Event schema version.
    pub version: u8,
    /// Subject associated with the random handle.
    pub subject: [u8; 32],
    /// Random seed carried to worker tests.
    pub seed: [u8; 16],
    /// FHE type byte.
    pub fhe_type: u8,
    /// Output handle.
    pub result: [u8; 32],
}

/// Emitted for bounded random ciphertext creation accepted by the host.
#[event]
pub struct FheRandBoundedEvent {
    /// Event schema version.
    pub version: u8,
    /// Subject associated with the random handle.
    pub subject: [u8; 32],
    /// Exclusive upper bound encoded as a 256-bit big-endian integer.
    pub upper_bound: [u8; 32],
    /// Random seed carried to worker tests.
    pub seed: [u8; 16],
    /// FHE type byte.
    pub fhe_type: u8,
    /// Output handle.
    pub result: [u8; 32],
}

/// Emitted for a unary FHE operation accepted by the host.
#[event]
pub struct FheUnaryOpEvent {
    /// Event schema version.
    pub version: u8,
    /// Unary operator.
    pub op: FheUnaryOpCode,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Operand handle.
    pub operand: [u8; 32],
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Legacy allowance event consumed by the current host listener.
///
/// New consumers should prefer [`AclSubjectAllowedEvent`] for richer context and
/// still verify account state before authorizing decrypts.
#[event]
pub struct AclAllowedEvent {
    /// Event schema version.
    pub version: u8,
    /// Allowed handle.
    pub handle: [u8; 32],
    /// Allowed subject pubkey bytes.
    pub subject: [u8; 32],
}

/// Emitted for an FHE sum operation accepted by the host.
#[event]
pub struct FheSumEvent {
    /// Event schema version.
    pub version: u8,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Input operand handles.
    pub operands: Vec<[u8; 32]>,
    /// FHE type of all operands and the output.
    pub fhe_type: u8,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}

/// Emitted for an FHE is-in test accepted by the host.
#[event]
pub struct FheIsInEvent {
    /// Event schema version.
    pub version: u8,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// Value handle being tested.
    pub value: [u8; 32],
    /// Set of handles to test against.
    pub set: Vec<[u8; 32]>,
    /// FHE type of value and set elements.
    pub fhe_type: u8,
    /// Output handle (always ebool) verified by the host formula.
    pub result: [u8; 32],
}

/// Emitted for an FHE multiply-divide operation accepted by the host.
#[event]
pub struct FheMulDivEvent {
    /// Event schema version.
    pub version: u8,
    /// Compute subject that passed ACL checks.
    pub subject: [u8; 32],
    /// First factor handle.
    pub factor1: [u8; 32],
    /// Second factor handle or scalar bytes.
    pub factor2: [u8; 32],
    /// Divisor plaintext scalar bytes.
    pub divisor: [u8; 32],
    /// Whether `factor2` is plaintext scalar bytes.
    pub scalar: bool,
    /// Output handle verified by the host formula.
    pub result: [u8; 32],
}
