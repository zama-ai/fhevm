-- Down for 20260407130028_key_generation_ethereum_migration.
--
-- Drops `extra_data` from the three key generation request tables and restores
-- `prep_keygen_requests.epoch_id` as a mandatory column.
--
-- The up moved the epoch into `extra_data` and dropped the column, so the
-- epoch of an existing prep keygen request cannot be put back without decoding
-- that blob. This fails, without changing anything, while
-- `prep_keygen_requests` has rows. The connector keeps finished requests, so
-- expect it on any database that ran a key generation.
--
-- Forced rollback (`migrate.sh rollback <target> --force`): those rows are
-- deleted and the revert goes ahead. Meant for test environments.
--
-- Polling cursor: the up reset the key generation cursors of
-- `last_block_polled` to NULL because those events moved from the Gateway to
-- Ethereum. The Gateway positions it erased are not recorded anywhere and the
-- current ones are Ethereum block numbers, so the three cursors are reset to
-- NULL again. The previous version derives its starting block from the cursors
-- that are left and from its configuration; check both before starting it.

DO $$
DECLARE
    prep_keygen bigint;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('kms_connector_db_rollback.force', true), '') = 'on';

    SELECT count(*) INTO prep_keygen FROM prep_keygen_requests;

    IF prep_keygen > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot restore prep_keygen_requests.epoch_id: % rows have their epoch only inside extra_data. Force the rollback to delete them.',
            prep_keygen;
    END IF;

    IF prep_keygen > 0 THEN
        DELETE FROM prep_keygen_requests;

        RAISE NOTICE 'Forced rollback: deleted % prep_keygen_requests rows.', prep_keygen;
    END IF;
END
$$;

ALTER TABLE prep_keygen_requests ADD COLUMN epoch_id BYTEA NOT NULL;

ALTER TABLE prep_keygen_requests DROP COLUMN extra_data;
ALTER TABLE keygen_requests DROP COLUMN extra_data;
ALTER TABLE crsgen_requests DROP COLUMN extra_data;

UPDATE last_block_polled SET block_number = NULL
WHERE event_type IN ('PrepKeygenRequest', 'KeygenRequest', 'CrsgenRequest');
