//! Shared constants and PDA seed bytes for the confidential-batcher program.

/// App event schema version, following the demo-vault pattern. The batcher is
/// not ingested by the coprocessor host-listener (host protocol events are
/// emitted by the ZamaHost CPIs it composes); this versions app/demo indexer
/// reads.
pub const APP_EVENT_VERSION: u8 = 1;

/// Fixed-point scale of a batch's public share rate:
/// `share_rate = shares_received * RATE_SCALE / total_deposited`, and each
/// claim is `encrypted(deposit) * share_rate / RATE_SCALE` in one MulDiv eval.
///
/// Domain bound: `share_rate <= RATE_SCALE * shares / total`, and the demo
/// vault's share price never drops below ~1 (no loss path, donations only
/// raise it), so `shares <= total` and the rate fits u64 with room to spare;
/// `freeze_share_rate` still checks the u64 fit and fails closed. The MulDiv
/// intermediate is `deposit * rate <= total * rate <= shares * RATE_SCALE
/// < 2^64 * 10^9 < 2^128`, inside the coprocessor's widened MulDiv, and the
/// result is at most `shares_received`, so it fits euint64.
pub const RATE_SCALE: u64 = 1_000_000_000;

/// PDA seed for a batch, keyed by batcher and batch index.
pub const BATCH_SEED: &[u8] = b"batch";

/// PDA seed for the per-batch authority that owns the batch's confidential and
/// SPL token accounts, signs its FHE evals, and authorizes its token CPIs.
pub const BATCH_AUTHORITY_SEED: &[u8] = b"batch-authority";

/// PDA seed for a user's per-batch deposit record.
pub const DEPOSIT_RECORD_SEED: &[u8] = b"deposit-record";

/// PDA seed for the batch's plain SPL account holding redeemed underlying
/// tokens between settle's redeem and vault-deposit legs.
pub const BATCH_UNDERLYING_SEED: &[u8] = b"batch-underlying";

/// PDA seed for the batch's plain SPL account holding vault shares between
/// settle's vault-deposit and wrap legs.
pub const BATCH_SHARE_TOKENS_SEED: &[u8] = b"batch-share-tokens";
