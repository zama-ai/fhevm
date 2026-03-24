-- Migration: Replace int_request_id with content-hash-based int_job_id
-- Fails in-flight requests during upgrade; users must resubmit

BEGIN;

-- 1. Fail all in-flight requests (graceful degradation)
UPDATE input_proof_req
SET
  req_status = 'failure'::req_status,
  err_reason = 'Request interrupted by system upgrade. Please resubmit.',
  updated_at = NOW()
WHERE req_status NOT IN ('completed', 'failure', 'timed_out');

-- 2. Drop old UUID-based column
ALTER TABLE input_proof_req DROP COLUMN IF EXISTS int_request_id;

-- 3. Add new content-hash column (zero bytes for existing terminal rows)
ALTER TABLE input_proof_req
ADD COLUMN IF NOT EXISTS int_job_id BYTEA NOT NULL
DEFAULT '\x0000000000000000000000000000000000000000000000000000000000000000';

-- 4. Remove default (new inserts must provide real content hash)
ALTER TABLE input_proof_req ALTER COLUMN int_job_id DROP DEFAULT;

-- 5. Add partial unique index (deduplication for active requests only)
CREATE UNIQUE INDEX IF NOT EXISTS idx_input_proof_req_unique_int_job_id_partial
ON input_proof_req (int_job_id)
WHERE req_status NOT IN ('completed', 'failure', 'timed_out');

-- 6. Add hash index for fast lookups
CREATE INDEX IF NOT EXISTS idx_input_proof_req_int_job_id
ON input_proof_req USING HASH (int_job_id);

COMMIT;
