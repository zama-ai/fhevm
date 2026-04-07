-- Re-initialize last_block_polled for key generation events because of the migration from Gateway to Ethereum.
UPDATE last_block_polled SET block_number = NULL WHERE event_type IN ('PrepKeygenRequest', 'KeygenRequest', 'CrsgenRequest');

-- Add extra_data column to key generation event tables.
ALTER TABLE prep_keygen_requests ADD COLUMN extra_data BYTEA;
UPDATE prep_keygen_requests SET extra_data = NULL;

ALTER TABLE keygen_requests ADD COLUMN extra_data BYTEA;
UPDATE keygen_requests SET extra_data = NULL;

ALTER TABLE crsgen_requests ADD COLUMN extra_data BYTEA;
UPDATE crsgen_requests SET extra_data = NULL;

-- prss_init and key_reshare_same_set tables will be remove in next release (v0.14.0).
-- We keep them for now for compatibility of the DB migration with v0.12 release.
-- Tracking issue: TODO.
