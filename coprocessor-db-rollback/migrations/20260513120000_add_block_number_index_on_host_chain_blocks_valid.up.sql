-- Add (chain_id, block_number) index on host_chain_blocks_valid.
-- Mirrors the pattern from 20260319120000_add_block_number_for_state_revert.sql
-- which added analogous indexes to sibling tables (computations,
-- pbs_computations, allowed_handles) but missed this one.
-- Used by update_block_as_finalized.

CREATE INDEX IF NOT EXISTS idx_host_chain_blocks_valid_block_number
ON host_chain_blocks_valid (chain_id, block_number);
