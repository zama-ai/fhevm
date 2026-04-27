-- no-transaction

CREATE UNIQUE INDEX CONCURRENTLY idx_input_blobs_no_tenant
ON input_blobs (blob_hash);
