-- Make SNS conversion failures terminal and bound retries for canonical S3
-- repairs. Quarantined repairs remain settlement blockers until an operator
-- repairs and explicitly requeues them.

ALTER TABLE pbs_computations_branch
ADD COLUMN IF NOT EXISTS is_error BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS error_message TEXT NULL,
ADD COLUMN IF NOT EXISTS error_at TIMESTAMPTZ NULL;

ALTER TABLE pbs_computations_branch
DROP CONSTRAINT IF EXISTS pbs_computations_branch_error_state_check;

ALTER TABLE pbs_computations_branch
ADD CONSTRAINT pbs_computations_branch_error_state_check
CHECK (
    (
        is_error = TRUE
        AND is_completed = FALSE
        AND error_message IS NOT NULL
        AND error_at IS NOT NULL
    )
    OR
    (
        is_error = FALSE
        AND error_message IS NULL
        AND error_at IS NULL
    )
) NOT VALID;

DROP INDEX IF EXISTS idx_pbs_computations_branch_pending_created_at;

CREATE INDEX idx_pbs_computations_branch_pending_created_at
ON pbs_computations_branch (created_at, handle)
WHERE is_completed = FALSE AND is_error = FALSE;

CREATE INDEX IF NOT EXISTS idx_pbs_computations_branch_errors
ON pbs_computations_branch (host_chain_id, block_number, error_at)
WHERE is_error = TRUE;

ALTER TABLE s3_canonical_repair_queue
ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'pending',
ADD COLUMN IF NOT EXISTS last_error TEXT NULL,
ADD COLUMN IF NOT EXISTS last_error_at TIMESTAMPTZ NULL;

ALTER TABLE s3_canonical_repair_queue
DROP CONSTRAINT IF EXISTS s3_canonical_repair_queue_status_check;

ALTER TABLE s3_canonical_repair_queue
ADD CONSTRAINT s3_canonical_repair_queue_status_check
CHECK (status IN ('pending', 'quarantined'));

DROP INDEX IF EXISTS idx_s3_canonical_repair_queue_unlocked;

CREATE INDEX idx_s3_canonical_repair_queue_unlocked
ON s3_canonical_repair_queue (updated_at)
WHERE status = 'pending' AND locked_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_s3_canonical_repair_queue_quarantined
ON s3_canonical_repair_queue (host_chain_id, target_block_number, updated_at)
WHERE status = 'quarantined';
