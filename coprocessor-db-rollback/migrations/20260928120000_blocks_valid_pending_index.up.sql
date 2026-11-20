DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM pg_class c
        JOIN pg_index i ON i.indexrelid = c.oid
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
          AND c.relname = 'idx_host_chain_blocks_valid_pending'
          AND NOT i.indisvalid
    ) THEN
        EXECUTE 'DROP INDEX idx_host_chain_blocks_valid_pending';
    END IF;
END
$$;

CREATE INDEX IF NOT EXISTS idx_host_chain_blocks_valid_pending
ON host_chain_blocks_valid (chain_id, block_number)
WHERE block_status = 'pending';
