-- Keep the existing key identifier available through both keygen phases.
-- Rows predating this migration are fresh-key requests, so they are backfilled
-- with zero before the NOT NULL constraint is enforced.
ALTER TABLE prep_keygen_requests ADD COLUMN existing_key_id BYTEA;
UPDATE prep_keygen_requests SET existing_key_id = '\x0000000000000000000000000000000000000000000000000000000000000000';
ALTER TABLE prep_keygen_requests ALTER COLUMN existing_key_id SET NOT NULL;

ALTER TABLE keygen_requests ADD COLUMN existing_key_id BYTEA;
UPDATE keygen_requests SET existing_key_id = '\x0000000000000000000000000000000000000000000000000000000000000000';
ALTER TABLE keygen_requests ALTER COLUMN existing_key_id SET NOT NULL;
