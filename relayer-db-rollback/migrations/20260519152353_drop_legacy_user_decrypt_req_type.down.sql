-- Down for 20260519152353_drop_legacy_user_decrypt_req_type.
--
-- The up rewrote the user_decrypt_req_type enum: it removed `legacy`, added
-- `unified`, dropped the column default and added two comments. This restores
-- the enum, the default and the absence of comments as left by
-- 20260204120000_add_user_decrypt_req_type.
--
-- The previous enum has no `unified` value, so a row carrying it cannot be
-- represented after the revert. Guessing a replacement would silently change
-- what a stored request means, so this fails, without changing anything, while
-- such rows exist. `unified` is the value the up introduced for new requests,
-- so expect this to trigger on a database that served traffic after it. Those
-- rows have to go before retrying: delete them, or let the relayer's expiry
-- worker clear them (if it is enabled) before the relayer is stopped.
--
-- Forced rollback (`migrate.sh rollback <target> --force`, which sets the
-- `relayer_db_rollback.force` session setting): the `unified` requests are
-- deleted, together with the shares collected for them, and the revert goes
-- ahead. Those requests are gone for good. Meant for test environments.

DO $$
DECLARE
    unified_count integer;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('relayer_db_rollback.force', true), '') = 'on';

    SELECT count(*) INTO unified_count
    FROM user_decrypt_req
    WHERE req_type::text = 'unified';

    IF unified_count > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot restore the previous user_decrypt_req_type enum: % rows in user_decrypt_req use ''unified'', which it does not have. Clear them before re-running, or force the rollback to delete them.',
            unified_count;
    END IF;

    IF unified_count > 0 THEN
        DELETE FROM user_decrypt_share
        WHERE gw_reference_id IN (
            SELECT gw_reference_id
            FROM user_decrypt_req
            WHERE req_type::text = 'unified' AND gw_reference_id IS NOT NULL
        );

        DELETE FROM user_decrypt_req
        WHERE req_type::text = 'unified';

        RAISE NOTICE 'Forced rollback: deleted % user_decrypt_req rows using ''unified''.', unified_count;
    END IF;
END
$$;

-- Postgres has no `ALTER TYPE ... DROP VALUE`: recreate the enum and swap the
-- column type, casting through text, exactly as the up did.
ALTER TYPE user_decrypt_req_type RENAME TO user_decrypt_req_type_new;

CREATE TYPE user_decrypt_req_type AS ENUM (
    'legacy',
    'user_decrypt',
    'delegated_user_decrypt'
);

ALTER TABLE user_decrypt_req
    ALTER COLUMN req_type TYPE user_decrypt_req_type
        USING req_type::text::user_decrypt_req_type;

ALTER TABLE user_decrypt_req
    ALTER COLUMN req_type SET DEFAULT 'legacy';

-- Dropping the swapped-out type also drops the comment the up put on it; the
-- recreated type starts without one.
DROP TYPE user_decrypt_req_type_new;

COMMENT ON COLUMN user_decrypt_req.req_type IS NULL;
