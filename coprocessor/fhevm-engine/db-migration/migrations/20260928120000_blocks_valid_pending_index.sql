CREATE INDEX IF NOT EXISTS idx_host_chain_blocks_valid_pending
ON host_chain_blocks_valid (chain_id, block_number)
WHERE block_status = 'pending';
