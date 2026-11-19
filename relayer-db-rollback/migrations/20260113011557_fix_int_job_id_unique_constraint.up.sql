-- Migration: Fix int_job_id UNIQUE constraint to allow retrying failed requests
--
-- Problem: The table-level UNIQUE constraint on int_job_id prevents inserting new rows
-- when a previous request with the same content hash has failed or timed out.
--
-- Solution: Drop the table-level UNIQUE constraints while keeping the partial unique indexes.
-- The partial indexes only enforce uniqueness for non-terminal requests (not 'failure' or 'timed_out'),
-- which allows retrying failed requests with a new ext_job_id while preventing duplicates for active requests.

-- Drop the table-level UNIQUE constraint on int_job_id for user_decrypt_req
ALTER TABLE user_decrypt_req DROP CONSTRAINT IF EXISTS user_decrypt_req_int_job_id_key;

-- Drop the table-level UNIQUE constraint on int_job_id for public_decrypt_req
ALTER TABLE public_decrypt_req DROP CONSTRAINT IF EXISTS public_decrypt_req_int_job_id_key;

-- The partial unique indexes remain and enforce uniqueness for non-terminal requests:
-- - idx_user_decrypt_req_unique_int_job_id_partial
-- - idx_public_decrypt_req_unique_int_job_id_partial
