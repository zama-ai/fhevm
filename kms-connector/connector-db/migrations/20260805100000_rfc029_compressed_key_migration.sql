-- Keep the durable key identifier separate from the temporary migration request identifier.
-- A NULL value identifies an ordinary key generation request.
ALTER TABLE keygen_requests
    ADD COLUMN existing_key_id BYTEA;
