-- Down for 20251109145104_create_tables.
--
-- This is the baseline schema. Reverting it would drop every relayer table and
-- all of its data, which is never what a rollback is for. The file exists so
-- that every migration has a down (the tool refuses to run otherwise) and it
-- always fails, loudly, without changing anything.
--
-- migrate.sh already rejects a rollback target of 0, so this statement is only
-- reached if someone bypasses the script and calls sqlx directly.

DO $$
BEGIN
    RAISE EXCEPTION
        'Refusing to revert the baseline migration 20251109145104_create_tables: it would drop the whole relayer schema and its data.';
END
$$;
