-- no-transaction

CREATE UNIQUE INDEX CONCURRENTLY idx_allowed_handles_no_tenant
ON allowed_handles (handle, account_address);
