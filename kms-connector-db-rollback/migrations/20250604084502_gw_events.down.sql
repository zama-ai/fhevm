-- Down for 20250604084502_gw_events.
--
-- This is the baseline schema. Reverting it would drop every connector table
-- and all of its data, which is never what a rollback is for. The file exists
-- so that every migration has a down (the tool refuses to run otherwise) and
-- it always fails, loudly, without changing anything.
--
-- migrate.sh already rejects a rollback target of 0, so this statement is only
-- reached if someone bypasses the script and calls sqlx directly.

DO $$
BEGIN
    RAISE EXCEPTION
        'Refusing to revert the baseline migration 20250604084502_gw_events: it would drop the whole KMS Connector schema and its data.';
END
$$;
