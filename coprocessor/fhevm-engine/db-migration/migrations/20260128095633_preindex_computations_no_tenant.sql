-- no-transaction

CREATE UNIQUE INDEX CONCURRENTLY idx_computations_no_tenant
ON computations (output_handle, transaction_id);
