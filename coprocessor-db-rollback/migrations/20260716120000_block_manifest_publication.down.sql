-- Down for 20260716120000_block_manifest_publication.
-- Drops the three tables and their indexes. Archived manifests, block states
-- and range commitments are lost. The tables that reference block_manifest
-- (20260716120100) are already gone when this file runs.

DROP INDEX IF EXISTS idx_block_manifest_state_open_discovery;
DROP INDEX IF EXISTS idx_block_manifest_state_pending_manifest;
DROP INDEX IF EXISTS idx_block_manifest_state_discovery_frontier;
DROP INDEX IF EXISTS idx_block_manifest_state_parent;
DROP TABLE IF EXISTS block_manifest;
DROP TABLE IF EXISTS block_range_commitment;
DROP TABLE IF EXISTS block_manifest_state;
