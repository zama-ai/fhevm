//! Event types for ZamaHost, in three groups with different transports:
//!
//! - Admin and config lifecycle events (`HostConfig*`, `*KmsContext*`,
//!   `DenySubjectUpdated`, `HcuAppTrustUpdated`,
//!   `UserDecryptionDelegationUpdated`) are emitted through `emit!` as
//!   indexing hints. Authorization always comes from host-owned account
//!   state, never from event bytes.
//! - `FheExecuteRandomSeedsEvent` and `PublicOutputsProducedEvent` are emitted
//!   through the event CPI. They are load-bearing for off-chain consumers:
//!   they carry the only data an indexer cannot recompute from instruction
//!   data alone (block-entropy-derived seeds and output handles).
//! - Per-step compute shapes are never emitted on-chain; they live in
//!   `records.rs` as plain decoded op records the listener reconstructs
//!   from instruction data (see the listener's `solana_reconstruct`).
//!
//! `EncryptedValue` ACL mutations emit nothing at all — indexers reconstruct
//! MMR leaves from instruction data via the shared `zama_solana_acl` crate
//! (see `instructions/encrypted_value.rs`).

use anchor_lang::prelude::*;

/// One public persistent output produced by an `fhe_execute` batch.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProducedPublicOutput {
    /// Zero-based step index within the batch.
    pub step_index: u16,
    /// Host-owned persistent `EncryptedValue` account bound by the step.
    pub encrypted_value: Pubkey,
    /// Block-entropy-derived output handle written to the account.
    pub output_handle: [u8; 32],
}

/// Emitted once for the public outputs produced by an `fhe_execute` batch.
#[event]
pub struct PublicOutputsProducedEvent {
    /// Event schema version.
    pub version: u8,
    /// Produced public outputs in batch step order.
    pub outputs: Vec<ProducedPublicOutput>,
}

/// One host-derived random seed used by an `fhe_execute` step.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FheExecuteRandomSeed {
    /// Zero-based step index within the batch.
    pub step_index: u16,
    /// Seed derived from live persistent account state.
    pub seed: [u8; 16],
}

/// Emitted once for the random steps in an `fhe_execute` batch.
#[event]
pub struct FheExecuteRandomSeedsEvent {
    /// Event schema version.
    pub version: u8,
    /// Random seeds in batch step order.
    pub seeds: Vec<FheExecuteRandomSeed>,
}

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
    /// Current grant deny-list gate.
    pub grant_deny_list_enabled: bool,
    /// Current max total HCU per `fhe_execute` batch (`u64::MAX` = unlimited).
    pub max_hcu_per_tx: u64,
    /// Current max critical-path HCU per `fhe_execute` batch (`u64::MAX` = unlimited).
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
    pub account: Pubkey,
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
