//! Event types for ZamaHost. There are only two kinds, and the rule for which one you get is about
//! who needs the data, not about how interesting it is:
//!
//! - **Emitted, always, through the event CPI** (`crate::event_cpi`). Two groups qualify. The admin
//!   and config lifecycle — `HostConfig*`, `*KmsContext*`, `DenySubjectUpdated`,
//!   `HcuAppTrustUpdated` — because an off-chain component that watches the protocol has to be able
//!   to see an admin change without scanning for it. And `FheExecuteRandomSeedsEvent` plus
//!   `PublicOutputsProducedEvent`, which carry the only data an indexer cannot recompute from
//!   instruction data at all (seeds derived from block entropy, and output handles). Nothing here
//!   uses `emit!`: a log can be truncated by the RPC provider a reader goes through, so it is a hint
//!   rather than a delivery, and a hint is not good enough for either group. Authorization still
//!   comes from host-owned account state and never from event bytes — reliable delivery is about
//!   observability, not trust.
//! - **Not emitted at all.** Everything else, which is most of it: per-step compute shapes (they
//!   live in `records.rs` as decoded op records), `EncryptedValue` ACL mutations (indexers rebuild
//!   MMR leaves through the shared `zama_solana_acl` crate), and user-decryption delegation. The
//!   listener reconstructs these from instruction data over Yellowstone, which is the normal path
//!   for anything reconstructible. Delegation is the clearest case: nothing off-chain consumes it at
//!   all yet (INVARIANTS #27), and the consumer being built for it — the KMS connector's
//!   `verify_delegation` — reads the record at its canonical PDA rather than watching for an event,
//!   so the event this module used to emit had no reader and no prospective one.

use anchor_lang::prelude::*;

/// One public persistent output produced by an `fhe_execute` execution.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProducedPublicOutput {
    /// Zero-based step index within the execution.
    pub step_index: u16,
    /// Host-owned persistent `EncryptedValue` account bound by the step.
    pub encrypted_value: Pubkey,
    /// Block-entropy-derived output handle written to the account.
    pub output_handle: [u8; 32],
}

/// Emitted once for the public outputs produced by an `fhe_execute` execution.
#[event]
pub struct PublicOutputsProducedEvent {
    /// Event schema version.
    pub version: u8,
    /// Produced public outputs in execution step order.
    pub outputs: Vec<ProducedPublicOutput>,
}

/// One host-derived random seed used by an `fhe_execute` step.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct FheExecuteRandomSeed {
    /// Zero-based step index within the execution.
    pub step_index: u16,
    /// Seed derived from live persistent account state.
    pub seed: [u8; 16],
}

/// Emitted once for the random steps in an `fhe_execute` execution.
#[event]
pub struct FheExecuteRandomSeedsEvent {
    /// Event schema version.
    pub version: u8,
    /// Random seeds in execution step order.
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
    /// Current max total HCU per `fhe_execute` execution (`u64::MAX` = unlimited).
    pub max_hcu_per_tx: u64,
    /// Current max critical-path HCU per `fhe_execute` execution (`u64::MAX` = unlimited).
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
