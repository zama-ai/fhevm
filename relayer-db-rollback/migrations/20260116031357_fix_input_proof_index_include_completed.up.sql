-- Migration: Fix input_proof partial unique index to include completed requests
--
-- The index should only exclude 'failure' and 'timed_out' states (not 'completed')
-- to match user_decrypt and public_decrypt patterns. This allows:
-- 1. ON CONFLICT to detect completed duplicates and return cached response
-- 2. Retries after failure/timeout (new row can be inserted)
-- 3. No duplicate active requests (unique constraint enforced)
--
-- Note: Old rows from before content-hash migration have zero-byte int_job_id.
-- These are excluded from the unique constraint to avoid conflicts.

BEGIN;

-- Drop the incorrect index that excludes 'completed'
DROP INDEX IF EXISTS idx_input_proof_req_unique_int_job_id_partial;

-- Create corrected index that only excludes terminal failure states
-- Also exclude zero-byte int_job_id (legacy rows from before content-hash migration)
CREATE UNIQUE INDEX idx_input_proof_req_unique_int_job_id_partial
ON input_proof_req (int_job_id)
WHERE req_status NOT IN ('failure', 'timed_out')
  AND int_job_id != '\x0000000000000000000000000000000000000000000000000000000000000000';

COMMIT;
