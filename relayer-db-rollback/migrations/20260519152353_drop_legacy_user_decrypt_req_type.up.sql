-- Rewrite the `user_decrypt_req_type` enum in one swap:
--   - drop `legacy` (defunct column-add default from 20260204120000;
--     pre-existing rows are cleared by the 7-day expiry cron, per
--     that migration's note),
--   - add `unified` (new unified EIP-712 attestation type),
--   - keep `user_decrypt` and `delegated_user_decrypt` so already-
--     persisted v2 rows remain readable while the legacy variants
--     are deprecated.
--
-- Postgres has no `ALTER TYPE ... DROP VALUE`; the recipe is to
-- recreate the enum and swap the column type, casting through text
-- so existing values land unchanged. sqlx runs each migration in a
-- transaction by default, so a mid-step failure rolls back cleanly.

-- Safety: refuse to run if any rows still carry the `legacy` value.
-- Cast through text so the comparison works regardless of which
-- variants the current enum has (e.g. if a previous attempt got
-- partway through and the enum was already partially rewritten).
DO $$
DECLARE
    leftover_count integer;
BEGIN
    SELECT count(*) INTO leftover_count
    FROM user_decrypt_req
    WHERE req_type::text = 'legacy';

    IF leftover_count > 0 THEN
        RAISE EXCEPTION
            'Cannot drop ''legacy'' enum value: % rows in user_decrypt_req still use it. Clear them before re-running.',
            leftover_count;
    END IF;
END
$$;

-- Defensive cleanup: if an earlier run failed after the rename but
-- before the drop (would only happen with autocommit forced on),
-- remove the dangling type so the rename below succeeds.
DROP TYPE IF EXISTS user_decrypt_req_type_old;

ALTER TYPE user_decrypt_req_type RENAME TO user_decrypt_req_type_old;

CREATE TYPE user_decrypt_req_type AS ENUM (
    'user_decrypt',
    'delegated_user_decrypt',
    'unified'
);

-- Drop the column default first (Postgres won't change a column's
-- type while a default of the old type is set), then cast through
-- text so the surviving values map straight across.
ALTER TABLE user_decrypt_req
    ALTER COLUMN req_type DROP DEFAULT,
    ALTER COLUMN req_type TYPE user_decrypt_req_type
        USING req_type::text::user_decrypt_req_type;

DROP TYPE user_decrypt_req_type_old;

COMMENT ON TYPE user_decrypt_req_type IS
    '`user_decrypt` and `delegated_user_decrypt` are deprecated: kept '
    'only so already-persisted v2 rows remain readable until the '
    'legacy EIP-712 formats are removed. New writes use `unified`.';

COMMENT ON COLUMN user_decrypt_req.req_type IS
    'Attestation type of the persisted request. `user_decrypt` and '
    '`delegated_user_decrypt` are deprecated; `unified` is the current '
    'value. Column can be dropped once the legacy values are removed.';
