-- Down for 20260421092426_unified_user_decryption.
--
-- Drops the six RFC016 columns from `user_decryption_requests`.
--
-- `signature IS NOT NULL` is what identifies an RFC016 (unified) request.
-- Without these columns such a row is indistinguishable from a legacy request.
-- That is harmless for a finished request, which is never read again, so those
-- stay. An in-flight one (`pending` or `under_process`) would be processed by
-- the previous version as a legacy request. This fails, without changing
-- anything, while such a request exists.
--
-- Forced rollback (`migrate.sh rollback <target> --force`, which sets the
-- `kms_connector_db_rollback.force` session setting): the in-flight unified
-- requests are deleted and the revert goes ahead. They are never answered by
-- this party.

DO $$
DECLARE
    in_flight bigint;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('kms_connector_db_rollback.force', true), '') = 'on';

    SELECT count(*) INTO in_flight
    FROM user_decryption_requests
    WHERE signature IS NOT NULL AND status IN ('pending', 'under_process');

    IF in_flight > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot drop the RFC016 columns: % in-flight rows in user_decryption_requests are unified requests and would be processed as legacy ones. Let them finish, or force the rollback to delete them.',
            in_flight;
    END IF;

    IF in_flight > 0 THEN
        DELETE FROM user_decryption_requests
        WHERE signature IS NOT NULL AND status IN ('pending', 'under_process');

        RAISE NOTICE 'Forced rollback: deleted % in-flight unified user decryption requests.', in_flight;
    END IF;
END
$$;

ALTER TABLE user_decryption_requests
    DROP COLUMN handle_owner_addresses,
    DROP COLUMN handle_contract_addresses,
    DROP COLUMN allowed_contracts,
    DROP COLUMN start_timestamp,
    DROP COLUMN duration_seconds,
    DROP COLUMN signature;
