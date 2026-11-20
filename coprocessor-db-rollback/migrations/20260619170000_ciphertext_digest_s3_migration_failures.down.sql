-- Down for 20260619170000_ciphertext_digest_s3_migration_failures.
--
-- This migration is part of the baseline: every migration at or below
-- 20260704130000 (the last one shared with the v0.14.x releases below the
-- v0.15.x range) has no down. Reverting it would remove schema the v0.14.x
-- coprocessor needs. The file exists so that every migration has a down (the
-- tool refuses to run otherwise) and it always fails, loudly, without changing
-- anything.
--
-- migrate.sh already rejects a target below 20260704130000, so this statement
-- is only reached if someone bypasses the script and calls sqlx directly.

DO $$
BEGIN
    RAISE EXCEPTION
        'Refusing to revert the baseline migration 20260619170000_ciphertext_digest_s3_migration_failures: migrations at or below 20260704130000 have no down.';
END
$$;
