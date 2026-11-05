-- Persist the executor version known at each observed host block. This lets a
-- restarted listener continue from a canonical parent without guessing from
-- the current proxy implementation; block_hash keeps competing forks distinct.
ALTER TABLE public.host_chain_blocks_valid
    ADD COLUMN IF NOT EXISTS tfhe_executor_address BYTEA,
    ADD COLUMN IF NOT EXISTS tfhe_executor_version TEXT;

ALTER TABLE public.host_chain_blocks_valid
    ADD CONSTRAINT host_chain_blocks_valid_executor_address_length
        CHECK (tfhe_executor_address IS NULL OR octet_length(tfhe_executor_address) = 20),
    ADD CONSTRAINT host_chain_blocks_valid_executor_version_format
        CHECK (
            tfhe_executor_version IS NULL
            OR tfhe_executor_version ~ '^FHEVMExecutor v[0-9]+[.][0-9]+[.][0-9]+$'
        );

-- CREATE TABLE IF NOT EXISTS ... LIKE public does not evolve an existing
-- Green schema. A listener restarted during an upgrade reads these columns
-- immediately, so migrate all already-created Green tables as well.
DO $$
DECLARE
    green_schema TEXT;
BEGIN
    FOR green_schema IN
        SELECT nspname FROM pg_namespace WHERE nspname LIKE 'gcs-%'
    LOOP
        IF to_regclass(format('%I.host_chain_blocks_valid', green_schema)) IS NOT NULL THEN
            EXECUTE format(
                'ALTER TABLE %I.host_chain_blocks_valid ADD COLUMN IF NOT EXISTS tfhe_executor_address BYTEA, ADD COLUMN IF NOT EXISTS tfhe_executor_version TEXT',
                green_schema
            );
            EXECUTE format(
                'ALTER TABLE %I.host_chain_blocks_valid DROP CONSTRAINT IF EXISTS host_chain_blocks_valid_executor_address_length, DROP CONSTRAINT IF EXISTS host_chain_blocks_valid_executor_version_format',
                green_schema
            );
            EXECUTE format(
                'ALTER TABLE %I.host_chain_blocks_valid ADD CONSTRAINT host_chain_blocks_valid_executor_address_length CHECK (tfhe_executor_address IS NULL OR octet_length(tfhe_executor_address) = 20), ADD CONSTRAINT host_chain_blocks_valid_executor_version_format CHECK (tfhe_executor_version IS NULL OR tfhe_executor_version ~ ''^FHEVMExecutor v[0-9]+[.][0-9]+[.][0-9]+$'')',
                green_schema
            );
        END IF;
    END LOOP;
END $$;
