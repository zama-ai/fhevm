ALTER TABLE IF EXISTS host_chain_blocks_valid
ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE INDEX IF NOT EXISTS host_chain_blocks_valid_pending_chain_block_idx
    ON host_chain_blocks_valid (chain_id, block_number DESC)
    WHERE block_status = 'pending';
