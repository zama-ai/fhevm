--- Improve indexing for SNS worker selection queries

CREATE INDEX IF NOT EXISTS idx_pbs_computations_pending_created_at
ON pbs_computations (created_at, handle)
WHERE is_completed = FALSE;

CREATE INDEX IF NOT EXISTS idx_ciphertexts_handle_not_null
ON ciphertexts (handle)
WHERE ciphertext IS NOT NULL;

--- Improve indexing for Tx-sender selection queries

CREATE INDEX IF NOT EXISTS idx_allowed_txn_is_sent
ON allowed_handles (txn_is_sent);

CREATE INDEX IF NOT EXISTS idx_allowed_txn_retries
ON allowed_handles (txn_limited_retries_count)
WHERE txn_is_sent = false;
