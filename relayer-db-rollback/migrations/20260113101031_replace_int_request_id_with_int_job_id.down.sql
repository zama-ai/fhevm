-- Down for 20260113101031_replace_int_request_id_with_int_job_id.
--
-- The up replaced input_proof_req.int_request_id (UUID) with int_job_id (BYTEA
-- content hash). The original UUIDs were dropped and cannot be recovered, so
-- this restores the schema, not the data:
--
--   * in-flight input proof requests are failed, because they are keyed by a
--     content hash the previous relayer version cannot resume from - clients
--     have to resubmit (the up did the same in the other direction);
--   * the int_job_id content hashes are dropped. If the up is applied again
--     later, existing rows get its all-zero placeholder;
--   * every row gets a fresh random int_request_id. The column is internal and
--     only has to be unique; after the step above every row is terminal, so the
--     previous version never reads it back for recovery.
--
-- Requires gen_random_uuid(), built in since PostgreSQL 13.

-- 1. Fail in-flight requests.
UPDATE input_proof_req
SET
    req_status = 'failure'::req_status,
    err_reason = 'Request interrupted by system downgrade. Please resubmit.',
    updated_at = NOW()
WHERE req_status NOT IN ('completed', 'failure', 'timed_out');

-- 2. Drop what the up created.
DROP INDEX idx_input_proof_req_int_job_id;
DROP INDEX idx_input_proof_req_unique_int_job_id_partial;
ALTER TABLE input_proof_req DROP COLUMN int_job_id;

-- 3. Restore int_request_id as the baseline declared it:
--    `int_request_id UUID NOT NULL UNIQUE`, plus its B-Tree index.
ALTER TABLE input_proof_req
    ADD COLUMN int_request_id UUID NOT NULL DEFAULT gen_random_uuid();

ALTER TABLE input_proof_req ALTER COLUMN int_request_id DROP DEFAULT;

ALTER TABLE input_proof_req
    ADD CONSTRAINT input_proof_req_int_request_id_key UNIQUE (int_request_id);

CREATE INDEX idx_input_proof_req_int_request_id
    ON input_proof_req (int_request_id);
