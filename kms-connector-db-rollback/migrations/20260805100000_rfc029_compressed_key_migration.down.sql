-- Down for 20260805100000_rfc029_compressed_key_migration.
--
-- Drops the `existing_key_id` column from the two keygen request tables.
--
-- A non-zero `existing_key_id` marks a request that migrates an existing key
-- instead of generating a fresh one. The previous version has no such notion:
-- an in-flight request of that kind would be processed as a fresh key
-- generation. This fails, without changing anything, while such a request is
-- pending or under process.
--
-- Forced rollback (`migrate.sh rollback <target> --force`): those requests are
-- marked `failed` and the revert goes ahead.

DO $$
DECLARE
    zero_key constant bytea := '\x0000000000000000000000000000000000000000000000000000000000000000';
    in_flight bigint;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('kms_connector_db_rollback.force', true), '') = 'on';

    SELECT (SELECT count(*) FROM prep_keygen_requests
            WHERE existing_key_id <> zero_key AND status IN ('pending', 'under_process'))
         + (SELECT count(*) FROM keygen_requests
            WHERE existing_key_id <> zero_key AND status IN ('pending', 'under_process'))
    INTO in_flight;

    IF in_flight > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot drop existing_key_id: % in-flight keygen requests target an existing key and would be processed as fresh key generations. Let them finish, or force the rollback to mark them failed.',
            in_flight;
    END IF;

    IF in_flight > 0 THEN
        UPDATE prep_keygen_requests SET status = 'failed'
        WHERE existing_key_id <> zero_key AND status IN ('pending', 'under_process');
        UPDATE keygen_requests SET status = 'failed'
        WHERE existing_key_id <> zero_key AND status IN ('pending', 'under_process');

        RAISE NOTICE 'Forced rollback: marked % in-flight keygen requests targeting an existing key as failed.', in_flight;
    END IF;
END
$$;

ALTER TABLE prep_keygen_requests DROP COLUMN existing_key_id;
ALTER TABLE keygen_requests DROP COLUMN existing_key_id;
