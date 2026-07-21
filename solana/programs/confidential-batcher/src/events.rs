//! App-local events for the confidential batcher.
//!
//! For frontend/demo indexers. The generic coprocessor host-listener does not
//! consume these: the FHE-protocol events are emitted by the ZamaHost CPIs the
//! batcher composes.

use anchor_lang::prelude::*;

/// Emitted when a batcher config is created.
#[event]
pub struct BatcherInitialized {
    /// Event schema version.
    pub version: u8,
    /// Batcher config account.
    pub batcher: Pubkey,
    /// Confidential mint users deposit through the batcher.
    pub deposit_confidential_mint: Pubkey,
    /// Confidential mint wrapping the vault's share mint.
    pub shares_confidential_mint: Pubkey,
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
    /// Joining user (deposit owner).
    pub user: Pubkey,
    /// `EncryptedValue` lineage holding the user's accumulated batch deposit.
    pub deposit_encrypted_value: Pubkey,
    /// Current handle of the deposit lineage after this join.
    pub deposit_handle: [u8; 32],
}

/// Emitted when a user quits a pending batch: the exact recorded deposit was
/// transferred back and the deposit lineage reset to zero.
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

/// Emitted when a batch settles: the certified total went into the vault and
/// the batch's public share rate is frozen.
#[event]
pub struct BatchSettled {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
    /// KMS-certified batch total (public by design).
    pub total_deposited: u64,
    /// Vault shares received for the batch total.
    pub shares_received: u64,
    /// Frozen share rate: `shares_received * RATE_SCALE / total_deposited`.
    pub share_rate: u64,
}

/// Emitted when a zero-total batch is canceled at settle time (nothing to
/// deposit, no rate to freeze — the division never happens).
#[event]
pub struct BatchCanceled {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
}

/// Emitted when a user's confidential shares are claimed from a settled batch.
#[event]
pub struct SharesClaimed {
    /// Event schema version.
    pub version: u8,
    /// Batch account.
    pub batch: Pubkey,
    /// User the shares were transferred to.
    pub user: Pubkey,
    /// `EncryptedValue` lineage holding the claimed share amount.
    pub claim_encrypted_value: Pubkey,
}
