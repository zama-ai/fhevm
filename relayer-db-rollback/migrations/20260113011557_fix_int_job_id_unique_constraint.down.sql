-- Down for 20260113011557_fix_int_job_id_unique_constraint.
--
-- The up dropped the table-level UNIQUE constraints on int_job_id that the
-- baseline created implicitly (`int_job_id BYTEA NOT NULL UNIQUE`), leaving the
-- partial unique indexes as the only uniqueness rule. This restores them under
-- the names Postgres generated in the baseline.
--
-- This fails, without changing anything, if two rows of the same table share an
-- int_job_id. That is legal after the up (a failed or timed-out request can be
-- resubmitted with the same content hash) and illegal before it, so there is no
-- lossless way to revert in that case: the duplicated rows have to be dealt
-- with by hand before retrying.

DO $$
DECLARE
    user_duplicates integer;
    public_duplicates integer;
BEGIN
    SELECT count(*) INTO user_duplicates
    FROM (SELECT int_job_id FROM user_decrypt_req GROUP BY int_job_id HAVING count(*) > 1) AS duplicated;

    SELECT count(*) INTO public_duplicates
    FROM (SELECT int_job_id FROM public_decrypt_req GROUP BY int_job_id HAVING count(*) > 1) AS duplicated;

    IF user_duplicates > 0 OR public_duplicates > 0 THEN
        RAISE EXCEPTION
            'Cannot restore UNIQUE (int_job_id): % int_job_id values in user_decrypt_req and % in public_decrypt_req are used by more than one row. Remove the duplicates before re-running.',
            user_duplicates, public_duplicates;
    END IF;
END
$$;

ALTER TABLE user_decrypt_req
    ADD CONSTRAINT user_decrypt_req_int_job_id_key UNIQUE (int_job_id);

ALTER TABLE public_decrypt_req
    ADD CONSTRAINT public_decrypt_req_int_job_id_key UNIQUE (int_job_id);
