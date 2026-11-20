-- Receipt audits must survive downtime and missing receipt hashes. Only a
-- verified finalized-chain observation retires an acknowledged publication
-- from auditing; elapsed blocks alone do not establish that evidence.
DO $$
DECLARE
    target RECORD;
BEGIN
    FOR target IN
        SELECT table_schema, table_name FROM information_schema.tables
        WHERE (table_schema = 'public' OR table_schema LIKE 'gcs-%')
          AND table_name IN ('ciphertext_digest', 'ciphertext_digest_branch')
    LOOP
        EXECUTE format(
            'ALTER TABLE %I.%I ADD COLUMN IF NOT EXISTS txn_finalized_at TIMESTAMPTZ',
            target.table_schema, target.table_name
        );
    END LOOP;
END $$;
