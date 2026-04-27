-- no-transaction

CREATE UNIQUE INDEX CONCURRENTLY idx_pbs_computations_no_tenant
ON pbs_computations (handle);
