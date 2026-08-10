-- Keep the existing key identifier available through both keygen phases.
-- Existing rows remain NULL. New fresh-key requests store zero.
ALTER TABLE prep_keygen_requests
    ADD COLUMN existing_key_id BYTEA;

ALTER TABLE keygen_requests
    ADD COLUMN existing_key_id BYTEA;
