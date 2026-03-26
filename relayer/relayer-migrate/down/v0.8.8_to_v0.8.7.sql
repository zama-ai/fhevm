-- Down migration: v0.8.8 → v0.8.7
-- Run BEFORE downgrading relayer binary to v0.8.7
--
-- What this does:
--   1. Fails in-flight input_proof requests (users must resubmit)
--   2. Restores input_proof_req schema to v0.8.7 format
--   3. Restores user/public decrypt constraints
--   4. Cleans migration history for re-upgrade capability

BEGIN;

-- ============================================================
-- PART 1: input_proof_req - restore v0.8.7 schema
-- ============================================================

-- 1a. Fail in-flight requests (they use content-hash ids, incompatible with v0.8.7)
UPDATE input_proof_req
SET
  req_status = 'failure'::req_status,
  err_reason = 'Request interrupted by system downgrade. Please resubmit.',
  updated_at = NOW()
WHERE req_status NOT IN ('completed', 'failure', 'timed_out');

-- 1b. Drop v0.8.8 indexes
DROP INDEX IF EXISTS idx_input_proof_req_unique_int_job_id_partial;
DROP INDEX IF EXISTS idx_input_proof_req_int_job_id;

-- 1c. Drop v0.8.8 column
ALTER TABLE input_proof_req DROP COLUMN IF EXISTS int_job_id;

-- 1d. Add back v0.8.7 column (generates new UUIDs for existing rows)
-- Note: v0.8.7 used UUID v7 (time-ordered), but gen_random_uuid() gives UUID v4.
-- This is safe because:
--   - JobId::from_uuid_v7() just wraps any UUID, doesn't validate version bits
--   - int_request_id is internal-only (not exposed to users)
--   - All rows are terminal (completed/failure) after step 1a
--   - v0.8.7 startup recovery only processes non-terminal rows (none exist)
--   - v0.8.7 code just needs unique UUIDs, doesn't require v7 ordering
ALTER TABLE input_proof_req
ADD COLUMN int_request_id UUID NOT NULL DEFAULT gen_random_uuid();

-- 1e. Add unique constraint (v0.8.7 expects this)
ALTER TABLE input_proof_req
ADD CONSTRAINT input_proof_req_int_request_id_key UNIQUE (int_request_id);

-- 1f. Remove default (new inserts must provide real UUID)
ALTER TABLE input_proof_req ALTER COLUMN int_request_id DROP DEFAULT;

-- 1g. Recreate B-Tree index for v0.8.7
CREATE INDEX IF NOT EXISTS idx_input_proof_req_int_request_id
ON input_proof_req (int_request_id);

-- ============================================================
-- PART 2: user/public decrypt - restore table-level constraints
-- ============================================================

-- 2a. Re-add UNIQUE constraints that v0.8.8 dropped
-- (These are redundant with partial indexes but v0.8.7 created them)
ALTER TABLE user_decrypt_req
ADD CONSTRAINT user_decrypt_req_int_job_id_key UNIQUE (int_job_id);

ALTER TABLE public_decrypt_req
ADD CONSTRAINT public_decrypt_req_int_job_id_key UNIQUE (int_job_id);

-- ============================================================
-- PART 3: Clean migration history
-- ============================================================

-- Remove v0.8.8 migration records (allows re-upgrade later)
DELETE FROM _sqlx_migrations
WHERE version IN (20260113011557, 20260113101031, 20260116031357);

COMMIT;
