-- Completes the sibling-pattern coverage from migration
-- 20260319120000_add_block_number_for_state_revert.sql, which added
-- (host_chain_id, block_number) indexes to `computations`,
-- `pbs_computations`, and `allowed_handles` but missed
-- `host_chain_blocks_valid`. This table is filtered by
-- (chain_id, block_number) in `update_block_as_finalized`
-- (host-listener/src/database/tfhe_event_propagate.rs); the index
-- supports that access pattern.
--
-- Built transactionally (no CONCURRENTLY): the table is small enough
-- on current deployments (~1.36M rows on testnet, ~230k on dev) that
-- the brief ACCESS EXCLUSIVE lock during build is sub-second on dev
-- and well under 10s on testnet — within the listener's WS subscription
-- timeout budget. If/when this table approaches mainnet scale
-- (multi-million rows), prefer pre-creating with CONCURRENTLY via the
-- `precreate_index` helper in `initialize_db.sh` before applying.

CREATE INDEX IF NOT EXISTS idx_host_chain_blocks_valid_block_number
ON host_chain_blocks_valid (chain_id, block_number);
