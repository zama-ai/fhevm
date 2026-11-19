-- Down for 20260701000000_add_abort_key_crs_gen.
--
-- Drops the two abort request tables with their triggers and functions, and
-- removes the `aborted` value from the `operation_status` enum.
--
-- Postgres has no `ALTER TYPE ... DROP VALUE`, so the enum is renamed, created
-- again as left by 20251208161022_implem_garbage_collection, and every
-- `status` column is moved to it in a single rewrite per table.
--
-- Data impact: the stored abort requests are lost, and a request that was
-- `aborted` becomes `failed`, the closest terminal status the previous enum
-- has. Both mean the request will not be processed again.

DROP TABLE abort_keygen_requests;
DROP TABLE abort_crsgen_requests;

DROP FUNCTION refresh_updated_at_abort_keygen_requests();
DROP FUNCTION refresh_updated_at_abort_crsgen_requests();
DROP FUNCTION notify_abort_keygen_request();
DROP FUNCTION notify_abort_crsgen_request();

ALTER TYPE operation_status RENAME TO operation_status_with_aborted;

CREATE TYPE operation_status AS ENUM (
    'pending',
    'under_process',
    'completed',
    'failed'
);

DO $$
DECLARE
    -- Every table that has a `status` column once the abort tables are gone.
    status_tables constant text[] := ARRAY[
        'public_decryption_requests',
        'user_decryption_requests',
        'prep_keygen_requests',
        'keygen_requests',
        'crsgen_requests',
        'prss_init',
        'key_reshare_same_set',
        'public_decryption_responses',
        'user_decryption_responses',
        'prep_keygen_responses',
        'keygen_responses',
        'crsgen_responses',
        'new_kms_context',
        'new_kms_epoch',
        'new_kms_context_responses',
        'epoch_result_responses'
    ];
    status_table text;
BEGIN
    FOREACH status_table IN ARRAY status_tables LOOP
        EXECUTE format('ALTER TABLE %I ALTER COLUMN status DROP DEFAULT', status_table);
        EXECUTE format(
            'ALTER TABLE %I ALTER COLUMN status TYPE operation_status
                 USING (CASE WHEN status::text = ''aborted'' THEN ''failed'' ELSE status::text END)::operation_status',
            status_table
        );
        EXECUTE format('ALTER TABLE %I ALTER COLUMN status SET DEFAULT ''pending''', status_table);
    END LOOP;
END
$$;

-- Fails here, undoing everything above, if a column the list does not know
-- still uses the previous enum.
DROP TYPE operation_status_with_aborted;
