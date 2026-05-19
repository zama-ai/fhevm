-- Drop the obsolete `legacy` value from the `user_decrypt_req_type` enum.
--
-- `legacy` was the default seeded into pre-existing rows when the
-- `req_type` column was first added (see
-- `20260204120000_add_user_decrypt_req_type.sql`). The original migration
-- noted those rows would be cleared by the 7-day expiry cron, after which
-- the value could be removed. Postgres has no `ALTER TYPE ... DROP VALUE`,
-- so we recreate the enum without it and swap the column over.
--
-- Refuses to run if any rows still carry `req_type = 'legacy'`. If that
-- happens in a non-prod environment, delete the stale rows explicitly
-- before rerunning.

DO $$
DECLARE
    leftover_count integer;
BEGIN
    SELECT count(*) INTO leftover_count
    FROM user_decrypt_req
    WHERE req_type = 'legacy';

    IF leftover_count > 0 THEN
        RAISE EXCEPTION
            'Cannot drop ''legacy'' enum value: % rows in user_decrypt_req still use it',
            leftover_count;
    END IF;
END
$$;

ALTER TYPE user_decrypt_req_type RENAME TO user_decrypt_req_type_old;

CREATE TYPE user_decrypt_req_type AS ENUM (
    'user_decrypt',
    'delegated_user_decrypt',
    'unified'
);

ALTER TABLE user_decrypt_req
    ALTER COLUMN req_type DROP DEFAULT,
    ALTER COLUMN req_type TYPE user_decrypt_req_type
        USING req_type::text::user_decrypt_req_type;

DROP TYPE user_decrypt_req_type_old;

COMMENT ON TYPE user_decrypt_req_type IS
  '`user_decrypt` and `delegated_user_decrypt` are deprecated: kept only '
  'so already-persisted v2 rows remain readable until the legacy EIP-712 '
  'formats are removed. New writes use `unified`.';

COMMENT ON COLUMN user_decrypt_req.req_type IS
  'Attestation type of the persisted request. `user_decrypt` and '
  '`delegated_user_decrypt` are deprecated; `unified` is the current '
  'value. Column can be dropped once the legacy values are removed.';
