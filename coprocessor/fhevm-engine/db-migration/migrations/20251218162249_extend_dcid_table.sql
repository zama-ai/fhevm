ALTER TABLE dependence_chain
-- List of dependence_chain_ids that depend on this one
ADD COLUMN dependents bytea[] NOT NULL DEFAULT '{}',
-- Number of dependencies this dependence chain has
ADD COLUMN dependency_count integer NOT NULL DEFAULT 0,
-- Block at which this dependence chain was created/updated
ADD COLUMN block_height bigint,
ADD COLUMN block_hash bytea,
ADD COLUMN block_timestamp TIMESTAMPTZ;

-- Update index to consider dependency_count
DROP INDEX IF EXISTS idx_pending_dependence_chain;
CREATE INDEX idx_pending_dependence_chain
    ON dependence_chain USING BTREE (last_updated_at)
    WHERE status = 'updated' AND worker_id IS NULL AND dependency_count = 0;