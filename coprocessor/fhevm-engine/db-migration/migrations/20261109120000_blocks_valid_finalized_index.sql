-- no-transaction
-- Index for the latest-finalized-block lookups that run under the host history
-- lock: `finalized_tip`, the finalization candidates' MAX(block_number), and
-- `missing_finalized_link`'s nearest finalized neighbor. Without it they walk
-- idx_host_chain_blocks_valid_block_number past every pending row above the
-- finalized frontier: 131 ms at 289k pending BSC rows, growing with the gap,
-- while ingestion, finalization and manifest sealing wait for the lock. With it
-- each lookup is a single probe.
--
-- Built CONCURRENTLY so ingestion keeps writing, hence the `no-transaction`
-- directive above (sqlx-cli >= 0.8) and a single statement: Postgres runs a
-- multi-statement query string as one transaction. IF NOT EXISTS keeps the
-- migration idempotent, e.g. where the index was created by hand. An
-- interrupted concurrent build leaves an INVALID index that this then skips:
-- drop it (DROP INDEX CONCURRENTLY) and rerun.
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_host_chain_blocks_valid_finalized
ON host_chain_blocks_valid (chain_id, block_number)
WHERE block_status = 'finalized';
