-- Use UTC timestamps to avoid timezone mismatch while computing performance
ALTER TABLE public_decryption_requests ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE user_decryption_requests ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE prep_keygen_requests ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE keygen_requests ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE crsgen_requests ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE prss_init ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE key_reshare_same_set ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';

-- Update every other tables for consistency
ALTER TABLE last_block_polled RENAME COLUMN update_at TO updated_at;
ALTER TABLE last_block_polled ALTER updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

ALTER TABLE public_decryption_responses ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE user_decryption_responses ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE prep_keygen_responses ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE keygen_responses ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';
ALTER TABLE crsgen_responses ALTER created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';

-- Drop default TIMESTAMPZ value as we want to use the KMS Connector time everywhere to avoid time
-- drift, and have more accurate latency results
ALTER TABLE public_decryption_requests ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE user_decryption_requests ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE prep_keygen_requests ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE keygen_requests ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE crsgen_requests ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE prss_init ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE key_reshare_same_set ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE last_block_polled ALTER COLUMN updated_at DROP DEFAULT;
ALTER TABLE public_decryption_responses ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE user_decryption_responses ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE prep_keygen_responses ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE keygen_responses ALTER COLUMN created_at DROP DEFAULT;
ALTER TABLE crsgen_responses ALTER COLUMN created_at DROP DEFAULT;
