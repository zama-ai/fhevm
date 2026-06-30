-- RFC-029: staging for the v1 (migrated) key-material PUBLISH path.
--
-- Mirrors kms_key_activation_events, but for `KeyMaterialAdded` rather than
-- `ActivateKey`. The publish path is deliberately SEPARATE from the activation
-- state machine so today's v0 behaviour is byte-for-byte untouched: a ready row
-- here only ever writes `keys.migrated_xof_keyset` (the v1 column) on the
-- already-active key row -- it never moves activeKeyId and never touches the
-- v0 columns (compressed_xof_keyset / sks_key / pks_key).
--
-- Lifecycle: pending -> (download migrated CompressedXofKeySet from S3,
-- verify digest) -> ready -> (on a finalized block, UPDATE keys.migrated_xof_keyset)
-- -> published.
CREATE TABLE IF NOT EXISTS kms_key_material_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    key_id BYTEA NOT NULL,
    material_version SMALLINT NOT NULL,
    key_digest BYTEA,
    storage_urls TEXT[] NOT NULL DEFAULT '{}',
    key_content_migrated_xof BYTEA,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (
            status IN (
                'pending',
                'ready',
                'published',
                'cancelled',
                'error'
            )
        ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, key_id, material_version)
);

-- RFC-029: staging for the cutover SCHEDULE (`KeyMaterialMigrationScheduled`).
-- Like the activation/material staging above, the schedule is written into the
-- live cutover tables (material_version_*_schedule) only once its scheduling
-- block is FINALIZED, so an orphaned governance tx can't flip workers to v1.
-- Lifecycle: pending -> (finalized block: write schedule tables + NOTIFY) ->
-- applied; orphaned block -> cancelled.
CREATE TABLE IF NOT EXISTS kms_key_material_schedule_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    host_chain_ids BIGINT[] NOT NULL,
    host_target_blocks BIGINT[] NOT NULL,
    gateway_target_block BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'applied', 'cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    PRIMARY KEY (chain_id, block_hash)
);
