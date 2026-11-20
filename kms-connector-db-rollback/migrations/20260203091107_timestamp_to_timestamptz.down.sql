-- Down for 20260203091107_timestamp_to_timestamptz.
--
-- Turns the `created_at` columns (and `last_block_polled.updated_at`) back into
-- TIMESTAMP, read as UTC as the up did in the other direction, restores their
-- `NOW()` default and renames `last_block_polled.updated_at` back to
-- `update_at`. No data is lost.

ALTER TABLE public_decryption_requests ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE user_decryption_requests ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE prep_keygen_requests ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE keygen_requests ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE crsgen_requests ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE prss_init ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE key_reshare_same_set ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';

ALTER TABLE last_block_polled ALTER updated_at TYPE TIMESTAMP USING updated_at AT TIME ZONE 'UTC';
ALTER TABLE last_block_polled RENAME COLUMN updated_at TO update_at;

ALTER TABLE public_decryption_responses ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE user_decryption_responses ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE prep_keygen_responses ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE keygen_responses ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';
ALTER TABLE crsgen_responses ALTER created_at TYPE TIMESTAMP USING created_at AT TIME ZONE 'UTC';

ALTER TABLE public_decryption_requests ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE user_decryption_requests ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE prep_keygen_requests ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE keygen_requests ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE crsgen_requests ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE prss_init ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE key_reshare_same_set ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE last_block_polled ALTER COLUMN update_at SET DEFAULT NOW();
ALTER TABLE public_decryption_responses ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE user_decryption_responses ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE prep_keygen_responses ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE keygen_responses ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE crsgen_responses ALTER COLUMN created_at SET DEFAULT NOW();
