//! Shared constants and PDA seed bytes for the confidential-batcher program.

use anchor_lang::prelude::constant;

/// Upper bound on `Batcher::min_batch_age_secs`.
pub const MAX_MIN_BATCH_AGE_SECS: u64 = 7 * 24 * 60 * 60;

/// Upper bound on `Batcher::settle_deadline_secs`.
pub const MAX_SETTLE_DEADLINE_SECS: u64 = 30 * 24 * 60 * 60;

/// PDA seed for a batch, keyed by batcher and batch index.
#[constant]
pub const BATCH_SEED: &[u8] = b"batch";

/// PDA seed for the per-batch authority that owns the batch's confidential and
/// SPL token accounts, signs its fhe_execute batches, and authorizes its token CPIs.
pub const BATCH_AUTHORITY_SEED: &[u8] = b"batch-authority";

/// PDA seed for a user's per-batch join record.
pub const JOIN_RECORD_SEED: &[u8] = b"join-record";

/// PDA seed for the batch's plain SPL account in the JOIN mint's underlying
/// (vault underlying for deposit batchers, vault shares for redeem batchers),
/// holding the redeemed batch total between settle's redeem and vault phases.
pub const BATCH_JOIN_UNDERLYING_SEED: &[u8] = b"batch-join-underlying";

/// PDA seed for the batch's plain SPL account in the PAYOUT mint's underlying
/// (vault shares for deposit batchers, vault underlying for redeem batchers),
/// holding the vault phase's output between settle's vault and wrap phases.
pub const BATCH_PAYOUT_UNDERLYING_SEED: &[u8] = b"batch-payout-underlying";
