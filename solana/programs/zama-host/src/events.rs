//! Anchor events emitted by ZamaHost.
//!
//! Events are indexing hints for listeners. Authorization still comes from
//! host-owned account state, so KMS-style consumers must verify the referenced
//! ACL records and witnesses instead of trusting event bytes alone.

use anchor_lang::prelude::*;

use crate::state::{FheBinaryOpCode, FheTernaryOpCode};

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
