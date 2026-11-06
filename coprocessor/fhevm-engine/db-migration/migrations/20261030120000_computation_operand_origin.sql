-- Preserve the executor transaction-local origin needed to derive operand
-- boundary masks when one transaction's logs arrive in separate batches.
-- Indexes are scoped to their observed block hash, since a sibling fork can
-- move the same transaction without changing its computation handles.
-- NULL pairs identify rows predating this migration; their origin cannot be
-- reconstructed safely from the legacy computation row alone.
ALTER TABLE public.computations
    ADD COLUMN IF NOT EXISTS log_index_block_hash BYTEA,
    ADD COLUMN IF NOT EXISTS log_index BIGINT,
    ADD COLUMN IF NOT EXISTS is_executor_minted BOOLEAN;

ALTER TABLE public.computations
    DROP CONSTRAINT IF EXISTS computations_operand_origin_metadata;
ALTER TABLE public.computations
    ADD CONSTRAINT computations_operand_origin_metadata
    CHECK (
        (log_index IS NULL AND is_executor_minted IS NULL)
        OR (
            log_index IS NOT NULL
            AND log_index >= 0
            AND is_executor_minted IS NOT NULL
        )
    ) NOT VALID;

ALTER TABLE public.computations_branch
    ADD COLUMN IF NOT EXISTS log_index_block_hash BYTEA,
    ADD COLUMN IF NOT EXISTS log_index BIGINT,
    ADD COLUMN IF NOT EXISTS is_executor_minted BOOLEAN;

ALTER TABLE public.computations_branch
    DROP CONSTRAINT IF EXISTS computations_branch_operand_origin_metadata;
ALTER TABLE public.computations_branch
    ADD CONSTRAINT computations_branch_operand_origin_metadata
    CHECK (
        (log_index IS NULL AND is_executor_minted IS NULL)
        OR (
            log_index IS NOT NULL
            AND log_index >= 0
            AND is_executor_minted IS NOT NULL
        )
    ) NOT VALID;

-- Existing Green schemas are not recreated on every deployment. Keep their
-- active computation tables aligned with the new writer as well as public.
DO $$
DECLARE
    green_schema TEXT;
BEGIN
    FOR green_schema IN
        SELECT nspname FROM pg_namespace WHERE nspname LIKE 'gcs-%'
    LOOP
        IF to_regclass(format('%I.computations', green_schema)) IS NOT NULL THEN
            EXECUTE format(
                'ALTER TABLE %I.computations ADD COLUMN IF NOT EXISTS log_index_block_hash BYTEA, ADD COLUMN IF NOT EXISTS log_index BIGINT, ADD COLUMN IF NOT EXISTS is_executor_minted BOOLEAN',
                green_schema
            );
            EXECUTE format(
                'ALTER TABLE %I.computations DROP CONSTRAINT IF EXISTS computations_operand_origin_metadata',
                green_schema
            );
            EXECUTE format(
                'ALTER TABLE %I.computations ADD CONSTRAINT computations_operand_origin_metadata CHECK ((log_index IS NULL AND is_executor_minted IS NULL) OR (log_index IS NOT NULL AND log_index >= 0 AND is_executor_minted IS NOT NULL)) NOT VALID',
                green_schema
            );
        END IF;
        IF to_regclass(format('%I.computations_branch', green_schema)) IS NOT NULL THEN
            EXECUTE format(
                'ALTER TABLE %I.computations_branch ADD COLUMN IF NOT EXISTS log_index_block_hash BYTEA, ADD COLUMN IF NOT EXISTS log_index BIGINT, ADD COLUMN IF NOT EXISTS is_executor_minted BOOLEAN',
                green_schema
            );
            EXECUTE format(
                'ALTER TABLE %I.computations_branch DROP CONSTRAINT IF EXISTS computations_branch_operand_origin_metadata',
                green_schema
            );
            EXECUTE format(
                'ALTER TABLE %I.computations_branch ADD CONSTRAINT computations_branch_operand_origin_metadata CHECK ((log_index IS NULL AND is_executor_minted IS NULL) OR (log_index IS NOT NULL AND log_index >= 0 AND is_executor_minted IS NOT NULL)) NOT VALID',
                green_schema
            );
        END IF;
    END LOOP;
END $$;
