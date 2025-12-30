--- Improve indexing for SNS worker selection queries

CREATE INDEX IF NOT EXISTS idx_pbs_computations_pending_created_at
ON pbs_computations (created_at, handle)
WHERE is_completed = FALSE;

CREATE INDEX IF NOT EXISTS idx_ciphertexts_handle_not_null
ON ciphertexts (handle)
WHERE ciphertext IS NOT NULL;