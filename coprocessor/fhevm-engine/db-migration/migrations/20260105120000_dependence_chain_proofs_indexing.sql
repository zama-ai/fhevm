-- Improve indexing for verify_proofs table

CREATE INDEX IF NOT EXISTS idx_verify_proofs_retry_count
ON verify_proofs (retry_count);

-- Improve indexing for dependence_chain table

CREATE INDEX IF NOT EXISTS idx_dependence_chain_unlock
ON dependence_chain (last_updated_at, lock_expires_at)
WHERE dependency_count = 0;

