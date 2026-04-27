-- no-transaction

CREATE UNIQUE INDEX CONCURRENTLY idx_ciphertext_digest_no_tenant
ON ciphertext_digest (handle);
