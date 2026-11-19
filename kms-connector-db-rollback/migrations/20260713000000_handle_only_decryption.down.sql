-- Down for 20260713000000_handle_only_decryption.
--
-- Drops the `ct_handles` column and makes `sns_ct_materials` NOT NULL again on
-- the two decryption request tables.
--
-- The connector that subscribes to the handle-only events leaves
-- `sns_ct_materials` NULL, which the restored constraint does not allow.
--   * A finished request is never read again: its material is set to an empty
--     list so the row, and the history it belongs to, is kept.
--   * An in-flight one (`pending` or `under_process`) cannot be processed by
--     the previous version, which needs the material. This fails, without
--     changing anything, while such a request exists.
--
-- Forced rollback (`migrate.sh rollback <target> --force`): the in-flight
-- handle-only requests are deleted and the revert goes ahead. They are never
-- answered by this party.

DO $$
DECLARE
    in_flight bigint;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('kms_connector_db_rollback.force', true), '') = 'on';

    SELECT (SELECT count(*) FROM public_decryption_requests
            WHERE sns_ct_materials IS NULL AND status IN ('pending', 'under_process'))
         + (SELECT count(*) FROM user_decryption_requests
            WHERE sns_ct_materials IS NULL AND status IN ('pending', 'under_process'))
    INTO in_flight;

    IF in_flight > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot make sns_ct_materials mandatory again: % in-flight decryption requests were stored with handles only and cannot be processed without the material. Let them finish, or force the rollback to delete them.',
            in_flight;
    END IF;

    IF in_flight > 0 THEN
        DELETE FROM public_decryption_requests
        WHERE sns_ct_materials IS NULL AND status IN ('pending', 'under_process');
        DELETE FROM user_decryption_requests
        WHERE sns_ct_materials IS NULL AND status IN ('pending', 'under_process');

        RAISE NOTICE 'Forced rollback: deleted % in-flight decryption requests stored with handles only.', in_flight;
    END IF;
END
$$;

-- The refresh triggers are disabled for this statement so that the finished
-- requests keep their `updated_at`.
ALTER TABLE public_decryption_requests DISABLE TRIGGER refresh_updated_at_public_decryption_requests_on_update;
UPDATE public_decryption_requests SET sns_ct_materials = '{}' WHERE sns_ct_materials IS NULL;
ALTER TABLE public_decryption_requests ENABLE TRIGGER refresh_updated_at_public_decryption_requests_on_update;

ALTER TABLE user_decryption_requests DISABLE TRIGGER refresh_updated_at_user_decryption_requests_on_update;
UPDATE user_decryption_requests SET sns_ct_materials = '{}' WHERE sns_ct_materials IS NULL;
ALTER TABLE user_decryption_requests ENABLE TRIGGER refresh_updated_at_user_decryption_requests_on_update;

ALTER TABLE public_decryption_requests DROP COLUMN ct_handles;
ALTER TABLE public_decryption_requests ALTER COLUMN sns_ct_materials SET NOT NULL;

ALTER TABLE user_decryption_requests DROP COLUMN ct_handles;
ALTER TABLE user_decryption_requests ALTER COLUMN sns_ct_materials SET NOT NULL;
