-- `s3_uploaded_at` is NULL until the row's PUT to S3 succeeds; the upload
-- sweep retries any NULL row until consensus passes. Used by GCS rows only
-- (BCS state hashes are not uploaded to S3); when `gcs.state_hash` is created
-- via `CREATE TABLE gcs.state_hash (LIKE public.state_hash INCLUDING ALL)`,
-- this column and its partial index propagate to the GCS copy.
ALTER TABLE state_hash ADD COLUMN IF NOT EXISTS s3_uploaded_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_state_hash_pending_upload
    ON state_hash (chain_id, block_number)
    WHERE s3_uploaded_at IS NULL;
