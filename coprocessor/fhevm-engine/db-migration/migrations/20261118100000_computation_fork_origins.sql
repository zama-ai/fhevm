-- A reusable (transaction, handle) can be observed on several forks. Retain
-- the earliest executor log index for each block hash instead of replacing
-- the previous fork's evidence. The original columns remain the legacy
-- fallback; ingestion backfills their known observation before replacing it.
-- A constant empty default avoids scanning or rewriting retained history.
DO $$
DECLARE
    target RECORD;
BEGIN
    FOR target IN
        SELECT table_schema, table_name FROM information_schema.tables
        WHERE (table_schema = 'public' OR table_schema LIKE 'gcs-%')
          AND table_name IN ('computations', 'computations_branch')
    LOOP
        EXECUTE format(
            'ALTER TABLE %I.%I ADD COLUMN IF NOT EXISTS executor_log_indexes JSONB NOT NULL DEFAULT ''{}''::jsonb',
            target.table_schema, target.table_name
        );
    END LOOP;
END $$;
