-- Add missing partial indexes on ciphertexts and ciphertext_digest tables

-- Partial index for ciphertexts table when searching without ciphertext_version
CREATE INDEX IF NOT EXISTS idx_ciphertexts_tenant_handle
ON ciphertexts (tenant_id, handle)
WHERE ciphertext128 IS NOT NULL;
-- Partial index for ciphertexts table when filtering by created_at
CREATE INDEX IF NOT EXISTS idx_ciphertexts_created_at
ON ciphertexts (created_at)
WHERE ciphertext128 IS NOT NULL;

-- Partial indexes for searching for NULL values for ciphertext and ciphertext128
CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_ciphertext_null
ON ciphertext_digest (ciphertext)
WHERE ciphertext IS NULL;
CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_ciphertext128_null
ON ciphertext_digest (ciphertext128)
WHERE ciphertext128 IS NULL;
CREATE INDEX IF NOT EXISTS idx_ciphertexts_ciphertext_null
ON ciphertexts (ciphertext)
WHERE ciphertext IS NULL;
CREATE INDEX IF NOT EXISTS idx_ciphertexts_ciphertext128_null
ON ciphertexts (ciphertext128)
WHERE ciphertext128 IS NULL;
