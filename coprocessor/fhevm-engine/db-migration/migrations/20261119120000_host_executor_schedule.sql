-- Preserve the positions, implementations and resolved versions of upgrades.
-- A repeated delivery can omit an upgrade or arrive while its RPC is unavailable.
SET LOCAL lock_timeout = '5s';
ALTER TABLE public.host_chain_blocks_valid
    ADD COLUMN IF NOT EXISTS tfhe_executor_schedule JSONB;

DO $$
DECLARE
    green_schema TEXT;
BEGIN
    FOR green_schema IN
        SELECT nspname FROM pg_namespace WHERE nspname LIKE 'gcs-%'
    LOOP
        IF to_regclass(format('%I.host_chain_blocks_valid', green_schema)) IS NOT NULL THEN
            EXECUTE format(
                'ALTER TABLE %I.host_chain_blocks_valid ADD COLUMN IF NOT EXISTS tfhe_executor_schedule JSONB',
                green_schema
            );
        END IF;
    END LOOP;
END $$;
