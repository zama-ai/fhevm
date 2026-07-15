-- Bound async branch cleanup retries. Quarantined jobs are dead-lettered for
-- operator repair and must not freeze the settlement frontier forever.

ALTER TABLE branch_cleanup_jobs
DROP CONSTRAINT IF EXISTS branch_cleanup_jobs_status_check;

ALTER TABLE branch_cleanup_jobs
ADD CONSTRAINT branch_cleanup_jobs_status_check
    CHECK (status IN ('pending', 'completed', 'quarantined'));

CREATE INDEX IF NOT EXISTS idx_branch_cleanup_jobs_quarantined
ON branch_cleanup_jobs (chain_id, finalized_block_number, updated_at)
WHERE status = 'quarantined';
