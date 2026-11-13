-- Down for 20260116031357_fix_input_proof_index_include_completed.
--
-- The up rebuilt the partial unique index on input_proof_req.int_job_id so that
-- completed rows take part in the uniqueness rule and the all-zero placeholder
-- id does not. This restores the definition created by
-- 20260113101031_replace_int_request_id_with_int_job_id.
--
-- This fails, without changing anything, if two non-terminal rows share an
-- int_job_id - only possible for the all-zero placeholder, which the up
-- deliberately excluded from the rule.

DROP INDEX idx_input_proof_req_unique_int_job_id_partial;

CREATE UNIQUE INDEX idx_input_proof_req_unique_int_job_id_partial
    ON input_proof_req (int_job_id)
    WHERE req_status NOT IN ('completed', 'failure', 'timed_out');
