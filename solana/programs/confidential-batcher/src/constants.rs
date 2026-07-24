//! Shared constants and PDA seed bytes for the confidential-batcher program.

/// App event schema version, following the demo-vault pattern. The batcher is
/// not ingested by the coprocessor host-listener (host protocol events are
/// emitted by the ZamaHost CPIs it composes); this versions app/demo indexer
/// reads. Version 2: direction-neutral join/payout field names and the
/// `direction` field on `BatcherInitialized`.
pub const APP_EVENT_VERSION: u8 = 2;

/// Fixed-point scale of a batch's public payout rate:
/// `payout_rate = payout_received * RATE_SCALE / total_joined`.
///
/// The rate is event-facing and informational only. Claims do NOT multiply by
/// it: each claim is the exact proportional floor
/// `encrypted(joined) * payout_received / total_joined` in one MulDiv eval,
/// which strands strictly less than the rate's double rounding would (the
/// intermediate `joined * payout_received < 2^128` stays inside the
/// coprocessor's widened MulDiv, and the result is at most `payout_received`,
/// so it fits euint64). On the redeem direction the rate itself can exceed
/// u64 at extreme share prices, so it saturates instead of failing settle.
pub const RATE_SCALE: u64 = 1_000_000_000;

/// PDA seed for a batch, keyed by batcher and batch index.
pub const BATCH_SEED: &[u8] = b"batch";

/// PDA seed for the per-batch authority that owns the batch's confidential and
/// SPL token accounts, signs its FHE evals, and authorizes its token CPIs.
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
