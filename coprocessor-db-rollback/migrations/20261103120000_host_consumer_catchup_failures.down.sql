-- Down for 20261103120000_host_consumer_catchup_failures.
--
-- Restores the status and block range constraints of 20261015120000 and
-- drops failure_reason. Rows with status = 'FAILED' cannot exist under the
-- restored constraints. This file refuses to run while there are any. With the
-- `coprocessor_db_rollback.force` session setting on, it deletes them.

DO $$
DECLARE
    force_enabled BOOLEAN := current_setting('coprocessor_db_rollback.force', true) = 'on';
    failed_rows BIGINT;
BEGIN
    SELECT count(*) INTO failed_rows FROM host_consumer_catchups WHERE status = 'FAILED';

    IF failed_rows > 0 THEN
        IF NOT force_enabled THEN
            RAISE EXCEPTION
                'Refusing to revert 20261103120000_host_consumer_catchup_failures: % row(s) with status = ''FAILED'' in host_consumer_catchups. Delete them first, or run the rollback with --force.',
                failed_rows;
        END IF;
        DELETE FROM host_consumer_catchups WHERE status = 'FAILED';
    END IF;
END
$$;

ALTER TABLE host_consumer_catchups
    DROP CONSTRAINT IF EXISTS host_consumer_catchups_failed_inactive,
    DROP CONSTRAINT IF EXISTS host_consumer_catchups_failure_check,
    DROP CONSTRAINT IF EXISTS host_consumer_catchups_block_end_check,
    DROP CONSTRAINT IF EXISTS host_consumer_catchups_status_check,
    DROP COLUMN IF EXISTS failure_reason,
    ADD CONSTRAINT host_consumer_catchups_status_check
        CHECK (status IN ('ONGOING', 'DONE', 'SUPPRESSED', 'SKIPPED')),
    ADD CONSTRAINT host_consumer_catchups_check
        CHECK (block_end >= block_start);
