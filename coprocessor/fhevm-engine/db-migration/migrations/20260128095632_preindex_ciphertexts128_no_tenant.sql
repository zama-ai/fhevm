-- no-transaction

CREATE UNIQUE INDEX CONCURRENTLY idx_ciphertexts128_no_tenant
ON ciphertexts128 (handle);
