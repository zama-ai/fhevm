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
    /// Key that may pause the batcher, set its policy and hand the role on.
    pub admin: Pubkey,
    /// Minimum seconds a batch must stay open before dispatch.
    pub min_batch_age_secs: u64,
    /// Seconds after dispatch until anyone may cancel an unsettled batch.
    pub settle_deadline_secs: u64,
}

/// Emitted when the admin pauses the batcher.
#[event]
pub struct BatcherPaused {
    /// Batcher config account.
    pub batcher: Pubkey,
}

/// Emitted when the admin unpauses the batcher.
#[event]
pub struct BatcherUnpaused {
    /// Batcher config account.
    pub batcher: Pubkey,
}

/// Emitted when the admin sets the minimum batch age that batches opened from now on copy.
#[event]
pub struct MinBatchAgeSet {
    /// Batcher config account.
    pub batcher: Pubkey,
    /// New minimum batch age in seconds.
    pub min_batch_age_secs: u64,
}

/// Emitted when the admin sets the settle deadline that batches opened from now on copy.
#[event]
pub struct SettleDeadlineSet {
    /// Batcher config account.
    pub batcher: Pubkey,
    /// New settle deadline in seconds after dispatch.
    pub settle_deadline_secs: u64,
}

/// Emitted when the admin role moves to a new key.
#[event]
pub struct AdminSet {
    /// Batcher config account.
    pub batcher: Pubkey,
    /// The admin that signed the change.
    pub previous_admin: Pubkey,
    /// The admin from now on.
    pub new_admin: Pubkey,
}

/// Emitted when a new batch opens.
#[event]
pub struct BatchOpened {
    /// Batcher config account.
    pub batcher: Pubkey,
    /// Batch account.
    pub batch: Pubkey,
    /// Zero-based batch index within the batcher.
    pub index: u64,
    /// Per-batch authority PDA owning the batch's token accounts.
    pub batch_authority: Pubkey,
    /// Unix time the batch opened at.
    pub opened_at: i64,
    /// Minimum batch age the batch opened with.
    pub min_batch_age_secs: u64,
    /// Settle deadline the batch opened with.
    pub settle_deadline_secs: u64,
}

/// Emitted when a user joins a batch. The amount stays encrypted; only the
/// participation is public (by design — see CONFIDENTIAL_VAULTS.md).
#[event]
pub struct JoinedBatch {
    /// Batch account.
    pub batch: Pubkey,
    /// Joining user.
    pub user: Pubkey,
    /// `EncryptedStore` encrypted store holding the user's accumulated joined amount.
    pub joined_encrypted_store: Pubkey,
    /// Current handle of the joined encrypted store after this join.
    pub joined_handle: [u8; 32],
}

/// Emitted when a user quits a pending or refunding batch: the exact recorded amount was
/// transferred back, the joined encrypted store reset to zero and the join record closed.
#[event]
pub struct QuitBatch {
    /// Batch account.
    pub batch: Pubkey,
    /// Quitting user.
    pub user: Pubkey,
}

/// Emitted when a batch is dispatched: its full encrypted balance was burned
/// and the created-public burned handle awaits KMS certification.
#[event]
pub struct BatchDispatched {
    /// Batch account.
    pub batch: Pubkey,
    /// Created-public handle of the burned batch total.
    pub burned_total_handle: [u8; 32],
}

/// Emitted when a batch settles: the certified total went through the vault
/// and the batch's public settle results are frozen. The amounts here are the
/// batch aggregates, intentionally public.
#[event]
pub struct BatchSettled {
    /// Batch account.
    pub batch: Pubkey,
    /// KMS-certified batch join total (public by design).
    pub total_joined: u64,
    /// Payout units the vault phase produced for the batch total.
    pub payout_received: u64,
}

/// Emitted when a zero-total batch is canceled at settle time (nothing to
/// move through the vault).
#[event]
pub struct BatchCanceled {
    /// Batch account.
    pub batch: Pubkey,
}

/// Emitted when a dispatched batch becomes refund-only: `cancel_dispatch` restored its burn, or
/// `settle` wrapped back a deposit worth zero vault shares.
#[event]
pub struct BatchDispatchCancelled {
    /// Batch account.
    pub batch: Pubkey,
}

/// Emitted when a user's confidential payout is claimed from a settled batch.
#[event]
pub struct PayoutClaimed {
    /// Batch account.
    pub batch: Pubkey,
    /// User the payout was transferred to.
    pub user: Pubkey,
    /// `EncryptedStore` encrypted store holding the claimed payout amount.
    pub claim_store: Pubkey,
    /// Calculated entitlement, distinct from the token transfer event's paid handle.
    pub claim_handle: [u8; 32],
}
