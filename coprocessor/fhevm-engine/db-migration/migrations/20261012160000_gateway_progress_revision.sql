-- Each Gateway cursor update invalidates older scans, including a rewind to a
-- previously used height. Random revisions also distinguish recreated schemas.
ALTER TABLE public.gw_listener_last_block
    ADD COLUMN progress_revision UUID NOT NULL DEFAULT gen_random_uuid();

-- An upgrade may already have created its Green tables before this migration.
DO $$
DECLARE green_schema TEXT;
BEGIN
    FOR green_schema IN
        SELECT table_schema FROM information_schema.tables
        WHERE table_name = 'gw_listener_last_block' AND table_schema LIKE 'gcs-%'
    LOOP
        EXECUTE format('ALTER TABLE %I.gw_listener_last_block ADD COLUMN IF NOT EXISTS progress_revision UUID NOT NULL DEFAULT gen_random_uuid()', green_schema);
    END LOOP;
END $$;
