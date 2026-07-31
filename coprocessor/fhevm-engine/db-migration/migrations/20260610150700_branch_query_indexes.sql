-- Additive indexes for wave-2 branch scheduling, telemetry and cleanup.
-- On large production databases, pre-create the same indexes CONCURRENTLY
-- before running transactional migrations to avoid long write stalls.

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_transaction_id
ON ciphertext_digest_branch (transaction_id);

CREATE INDEX IF NOT EXISTS idx_allowed_handles_branch_transaction_id
ON allowed_handles_branch (transaction_id);

CREATE INDEX IF NOT EXISTS idx_pbs_computations_branch_transaction_id
ON pbs_computations_branch (transaction_id);

CREATE INDEX IF NOT EXISTS idx_s3_canonical_repair_queue_stale_lock
ON s3_canonical_repair_queue (locked_at)
WHERE locked_at IS NOT NULL;
