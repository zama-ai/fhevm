-- Finalization marks losing blocks as orphaned synchronously, but branch-table
-- cleanup can be retried asynchronously from a durable host-listener queue.

CREATE TABLE IF NOT EXISTS branch_cleanup_jobs
(
    chain_id BIGINT NOT NULL,
    finalized_block_hash BYTEA NOT NULL,
    finalized_block_number BIGINT NOT NULL,
    orphaned_block_hashes BYTEA[] NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    locked_at TIMESTAMPTZ NULL,
    last_error TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chain_id, finalized_block_hash),
    CONSTRAINT branch_cleanup_jobs_status_check
        CHECK (status IN ('pending', 'completed'))
);

CREATE INDEX IF NOT EXISTS idx_branch_cleanup_jobs_pending
ON branch_cleanup_jobs (chain_id, finalized_block_number, updated_at)
WHERE status = 'pending';

CREATE INDEX IF NOT EXISTS idx_branch_cleanup_jobs_stale_lock
ON branch_cleanup_jobs (locked_at)
WHERE status = 'pending' AND locked_at IS NOT NULL;
