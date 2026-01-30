-- Pending tasks index for pbs_computations table
-- This index improves the performance of queries that fetch pending tasks
-- based on their creation time.
CREATE INDEX idx_pending_tasks
    ON pbs_computations USING btree (created_at)
    WHERE is_completed = false;