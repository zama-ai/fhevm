-- no-transaction

CREATE UNIQUE INDEX CONCURRENTLY idx_ciphertexts_no_tenant
ON ciphertexts (handle, ciphertext_version);
