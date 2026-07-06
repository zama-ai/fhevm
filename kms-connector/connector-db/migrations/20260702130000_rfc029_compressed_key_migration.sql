-- RFC-029 one-time compressed-key migration.
--
-- The migration keygen rides the existing keygen request/response
-- pipeline, discriminated by these columns; no parallel tables.

-- The existing key a migration keygen re-materializes. NULL for normal
-- keygen requests.
ALTER TABLE keygen_requests
    ADD COLUMN migrated_key_id BYTEA;

-- KMS Core's signing enum has four values; the digest type is part of
-- the signed KeygenVerification payload, so the labels must round-trip
-- unchanged through the connector.
ALTER TYPE key_type ADD VALUE IF NOT EXISTS 'CompressedPublic';
ALTER TYPE key_type ADD VALUE IF NOT EXISTS 'CompressedKeyset';
