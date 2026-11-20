-- Branch-context wave 1: ancestry metadata on observed host-chain blocks.
--
-- `parent_hash` lets the workers rebuild the canonical branch ancestry for a
-- block range without an RPC round-trip. It is nullable: rows observed before
-- this migration have no recorded parent. The host-listener repairs NULLs on
-- re-observation.
ALTER TABLE host_chain_blocks_valid
ADD COLUMN IF NOT EXISTS parent_hash BYTEA NULL DEFAULT NULL;

-- Finalization walks orphan descendants by parent hash, and ACL producer
-- resolution walks recent ancestry by parent hash. Wave 1 is the ancestry
-- warm-up phase, so the lookup index belongs here rather than waiting for wave 2.
CREATE INDEX IF NOT EXISTS idx_host_chain_blocks_valid_parent_hash
ON host_chain_blocks_valid (chain_id, parent_hash);
