//! App-local events for the confidential batcher.
//!
//! For frontend/demo indexers. The generic coprocessor host-listener does not
//! consume these: the FHE-protocol events are emitted by the ZamaHost CPIs the
//! batcher composes. Only settled events carry amounts — everything else stays
//! encrypted.

use anchor_lang::prelude::*;

use crate::state::BatchDirection;

/// Emitted when a batcher config is created.
#[event]
pub struct BatcherInitialized {
    /// Event schema version.
    pub version: u8,
    /// Batcher config account.
    pub batcher: Pubkey,
    /// Direction of this batcher instance.
    pub direction: BatchDirection,
    /// Confidential mint users join batches with.
    pub join_confidential_mint: Pubkey,
    /// Confidential mint claims pay out in.
    pub payout_confidential_mint: Pubkey,
    /// Public vault the batcher fronts.
    pub vault: Pubkey,
    /// Minimum slots a batch must stay open before dispatch.
    pub min_batch_age_slots: u64,
}

/// Emitted when a new batch opens.
#[event]
pub struct BatchOpened {
    /// Event schema version.
    pub version: u8,
    /// Batcher config account.
    pub batcher: Pubkey,
    /// Batch account.
    pub batch: Pubkey,
    /// Zero-based batch index within the batcher.
    pub index: u64,
    /// Per-batch authority PDA owning the batch's token accounts.
    pub batch_authority: Pubkey,
    /// Slot the batch opened at.
    pub opened_slot: u64,
}

/// Emitted when a user joins a batch. The amount stays encrypted; only the
/// participation is public (by design — see CONFIDENTIAL_VAULTS.md).
#[event]
pub struct JoinedBatch {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
    /// Joining user.
    pub user: Pubkey,
    /// `EncryptedValue` lineage holding the user's accumulated joined amount.
    pub joined_encrypted_value: Pubkey,
    /// Current handle of the joined lineage after this join.
    pub joined_handle: [u8; 32],
}

/// Emitted when a user quits a pending batch: the exact recorded amount was
/// transferred back and the joined lineage reset to zero.
#[event]
pub struct QuitBatch {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
    /// Quitting user.
    pub user: Pubkey,
}

/// Emitted when a batch is dispatched: its full encrypted balance was burned
/// and the born-public burned handle awaits KMS certification.
#[event]
pub struct BatchDispatched {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
    /// Born-public handle of the burned batch total.
    pub burned_total_handle: [u8; 32],
}

/// Emitted when a batch settles: the certified total went through the vault
/// and the batch's public settle results are frozen. The amounts here are the
/// batch aggregates, intentionally public.
#[event]
pub struct BatchSettled {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
    /// KMS-certified batch join total (public by design).
    pub total_joined: u64,
    /// Payout units the vault leg produced for the batch total.
    pub payout_received: u64,
    /// Informational rate `payout_received * RATE_SCALE / total_joined`,
    /// saturating at u64::MAX. Claims use exact proportional division.
    pub payout_rate: u64,
}

/// Emitted when a zero-total batch is canceled at settle time (nothing to
/// move through the vault, no rate to freeze — the division never happens).
#[event]
pub struct BatchCanceled {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
}

/// Emitted when a user's confidential payout is claimed from a settled batch.
#[event]
pub struct PayoutClaimed {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
    /// User the payout was transferred to.
    pub user: Pubkey,
    /// `EncryptedValue` lineage holding the claimed payout amount.
    pub claim_encrypted_value: Pubkey,
}
