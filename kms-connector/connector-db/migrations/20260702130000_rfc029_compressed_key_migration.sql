-- RFC-029 one-time compressed-key migration.
--
-- The migration keygen rides the existing keygen request/response
-- pipeline, discriminated by these columns; no parallel tables.

-- The existing key a migration keygen re-materializes. NULL for normal
-- keygen requests.
ALTER TABLE keygen_requests
    ADD COLUMN migrated_key_id BYTEA;

-- Whether the response answers a migration keygen; routes the
-- tx-sender to addCompressedKeyMaterials instead of keygenResponse.
ALTER TABLE keygen_responses
    ADD COLUMN is_migration BOOLEAN NOT NULL DEFAULT FALSE;
