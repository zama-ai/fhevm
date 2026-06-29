-- Branch-context wave 1: ancestry metadata on observed host-chain blocks.
--
-- `parent_hash` lets the workers rebuild the canonical branch ancestry for a
-- block range without an RPC round-trip. It is nullable: rows observed before
-- this migration have no recorded parent. The host-listener repairs NULLs on
-- re-observation (COALESCE upsert in mark_block_as_valid).
ALTER TABLE host_chain_blocks_valid
ADD COLUMN IF NOT EXISTS parent_hash BYTEA NULL DEFAULT NULL;
