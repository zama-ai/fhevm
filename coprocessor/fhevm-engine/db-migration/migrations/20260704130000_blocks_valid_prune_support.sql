-- Support indexes for host_chain_blocks_valid pruning.
--
-- The pruner deletes old finalized rows that no branch/bridge/fallback state
-- references. Its per-candidate reference probes hit these block_hash lookups
-- on the event tables, which previously had no usable index for them (their
-- unique keys carry block_hash in a trailing position).
CREATE INDEX IF NOT EXISTS idx_bridge_handle_events_block_hash
    ON bridge_handle_events (block_hash);
CREATE INDEX IF NOT EXISTS idx_handle_bridged_events_block_hash
    ON handle_bridged_events (block_hash);
CREATE INDEX IF NOT EXISTS idx_fallback_granted_events_block_hash
    ON fallback_granted_events (block_hash);
-- The pruner's candidate scan walks finalized rows below the retention
-- horizon in block order; that is already served by
-- idx_host_chain_blocks_valid_block_number (20260513120000).
