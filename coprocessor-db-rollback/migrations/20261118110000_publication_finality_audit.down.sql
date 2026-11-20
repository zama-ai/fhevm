-- Down for 20261118110000_publication_finality_audit.
-- Drops txn_finalized_at from ciphertext_digest and ciphertext_digest_branch in
-- public and in the gcs-* schemas of the Blue/Green stacks. Finality timestamps
-- are lost.

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
            'ALTER TABLE %I.%I DROP COLUMN IF EXISTS txn_finalized_at',
            target.table_schema, target.table_name
        );
    END LOOP;
END $$;
