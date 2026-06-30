-- RFC-029: the `MigrationKeygenRequested` event carries the migration config out-of-band from the
-- ordinary `KeygenRequest` (whose extra_data stays plain v2 context+epoch). We persist the mapping
-- keyed by key_id so the kms-worker can recover it when it builds the KMS KeyGenRequest for that
-- key_id and switch to a keygen-from-existing-shares (UseExisting + copy-to-original).
CREATE TABLE migration_keygen (
    key_id BYTEA PRIMARY KEY,
    existing_key_id BYTEA NOT NULL,
    copy_to_original BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
