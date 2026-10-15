//! Account layouts, PDA helpers and encrypted-value labels.
//!
//! Public API surface: off-chain callers deriving batcher PDAs — `runtime-tests`' `batcher_mollusk`
//! fixtures. TypeScript clients are generated from the Anchor IDL.

use anchor_lang::prelude::*;

use crate::constants::*;

/// Which way through the vault a batcher instance moves value. Direction lives
/// on the `Batcher` config — one instance per direction, mirroring the EVM's
/// two batcher deployments — so a pending deposit batch never blocks a redeem
/// batch, and every batch inherits its batcher's direction.
#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BatchDirection {
    /// Users join with confidential underlying; claims pay confidential shares.
    Deposit,
    /// Users join with confidential shares; claims pay confidential underlying.
    Redeem,
}

/// Batcher config wiring a join confidential mint, a payout confidential mint,
/// and one public vault together for one direction.
#[account]
#[derive(InitSpace)]
pub struct Batcher {
    /// Direction of this batcher instance.
    pub direction: BatchDirection,
    /// Confidential mint users join batches with. Wraps the vault's underlying
    /// mint for deposit batchers, the vault's share mint for redeem batchers.
    pub join_confidential_mint: Pubkey,
    /// Confidential mint claims pay out in. Wraps the vault's share mint for
    /// deposit batchers, the vault's underlying mint for redeem batchers.
    pub payout_confidential_mint: Pubkey,
    /// Public `demo_vault::Vault` the batcher fronts.
    pub vault: Pubkey,
    /// Key that may pause the batcher, set its policy and nominate the next admin.
    pub admin: Pubkey,
    /// Nominee who becomes `admin` by calling `accept_admin`; `Pubkey::default()` when none.
    pub pending_admin: Pubkey,
    /// While paused, joins, dispatches and settles of a nonzero total are refused.
    pub paused: bool,
    /// Minimum batch age copied into each batch when it opens.
    pub min_batch_age_secs: u64,
    /// Settle deadline copied into each batch when it opens.
    pub settle_deadline_secs: u64,
    /// Index of the next batch to open; the pending batch is the one before it.
    pub next_batch_index: u64,
}

/// Lifecycle of a batch.
#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone, Copy, PartialEq, Eq, Debug)]
pub enum BatchStatus {
    /// Open for joins and quits.
    Pending,
    /// Total burned; awaiting the KMS certificate.
    Dispatched,
    /// Payout received; claims open.
    Settled,
    /// Certified total was zero; nothing to claim.
    Canceled,
    /// Dispatch burn was cancelled; existing users may only retrieve their recorded joins.
    Refunding,
}

/// One batch: its own confidential token account (so the revealed total is
/// exactly this batch's sum), lifecycle status, and the public settle results.
#[account]
#[derive(InitSpace)]
pub struct Batch {
    /// Batcher config this batch belongs to (carries the direction).
    pub batcher: Pubkey,
    /// Zero-based index within the batcher.
    pub index: u64,
    /// Lifecycle status.
    pub status: BatchStatus,
    /// Unix time from which the batch may be dispatched: when it opened plus the batcher's
    /// minimum batch age at that time.
    pub dispatchable_at: i64,
    /// Seconds after dispatch during which only `settle` may finish the batch, frozen when it
    /// opened; from then on only `cancel_dispatch` may, and anyone can call it.
    pub settle_deadline_secs: u64,
    /// Unix time from which the dispatched batch can no longer settle: dispatch time plus
    /// `settle_deadline_secs` (zero before dispatch).
    pub settle_deadline: i64,
    /// Number of join calls routed into this batch.
    pub join_count: u64,
    /// Bump of the per-batch authority PDA.
    pub authority_bump: u8,
    /// Created-public handle of the burned batch total (set at dispatch).
    pub burned_total_handle: [u8; 32],
    /// KMS-certified batch join total (set at settle; public by design).
    pub total_joined: u64,
    /// Payout units the vault phase produced for the batch total (set at settle).
    pub payout_received: u64,
}

/// Per-(batch, user) join record. The encrypted amount itself lives in the batcher-owned
/// `EncryptedStore` account at `joined_encrypted_store`: the user may decrypt their pending
/// amount, and the batch authority computes refunds and claims from it by signature.
#[account]
#[derive(InitSpace)]
pub struct JoinRecord {
    /// Batch this record belongs to.
    pub batch: Pubkey,
    /// Joining user.
    pub user: Pubkey,
    /// Whether the user's payout was claimed.
    pub claimed: bool,
    /// PDA bump for `(batch, user)`.
    pub bump: u8,
}

/// Returns the batch PDA for a batcher and index.
pub fn batch_address(batcher: Pubkey, index: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[BATCH_SEED, batcher.as_ref(), &index.to_le_bytes()],
        &crate::ID,
    )
}

/// Returns the per-batch authority PDA that owns the batch's token accounts.
pub fn batch_authority_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_AUTHORITY_SEED, batch.as_ref()], &crate::ID)
}

/// Returns the join record PDA for a batch and user.
pub fn join_record_address(batch: Pubkey, user: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[JOIN_RECORD_SEED, batch.as_ref(), user.as_ref()],
        &crate::ID,
    )
}

/// Returns the batch's plain SPL account in the join mint's underlying.
pub fn batch_join_underlying_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_JOIN_UNDERLYING_SEED, batch.as_ref()], &crate::ID)
}

/// Returns the batch's plain SPL account in the payout mint's underlying.
pub fn batch_payout_underlying_address(batch: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[BATCH_PAYOUT_UNDERLYING_SEED, batch.as_ref()], &crate::ID)
}

pub fn join_store_id(batch: Pubkey, record: Pubkey) -> zama_fhe::StoreId {
    zama_fhe::StoreId::new(crate::ID, record, batch)
}

/// Join-store key of the user's encrypted joined amount.
#[constant]
pub const JOINED_AMOUNT_KEY: [u8; 32] = *b"joined_amount___________________";
