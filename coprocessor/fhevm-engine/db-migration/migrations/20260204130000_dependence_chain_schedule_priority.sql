ALTER TABLE dependence_chain
  ADD COLUMN IF NOT EXISTS schedule_priority SMALLINT NOT NULL DEFAULT 0;

-- Keep lock acquisition ordering index aligned with:
-- ORDER BY schedule_priority ASC, last_updated_at ASC
DROP INDEX IF EXISTS idx_pending_dependence_chain;
CREATE INDEX idx_pending_dependence_chain
    ON dependence_chain (schedule_priority, last_updated_at, dependence_chain_id)
    WHERE status = 'updated' AND worker_id IS NULL AND dependency_count = 0;
