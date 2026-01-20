ALTER TABLE IF EXISTS host_chain_blocks_valid
ADD COLUMN IF NOT EXISTS block_status TEXT NOT NULL DEFAULT 'unknown' CHECK (block_status IN ('pending', 'unknown', 'finalized', 'orphaned'));

ALTER TABLE IF EXISTS host_chain_blocks_valid
ALTER COLUMN block_status DROP DEFAULT;
