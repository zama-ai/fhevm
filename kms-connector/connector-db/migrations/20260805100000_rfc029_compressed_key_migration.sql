-- Keep the existing key identifier available through both keygen phases.
-- A NULL value identifies an ordinary fresh-key request.
ALTER TABLE prep_keygen_requests
    ADD COLUMN existing_key_id BYTEA;

ALTER TABLE keygen_requests
    ADD COLUMN existing_key_id BYTEA;
