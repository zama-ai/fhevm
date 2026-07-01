-- RFC-029 finalized key-material download queue. The legacy-key cutover is
-- applied directly from finalized KMSGeneration events.

CREATE TABLE IF NOT EXISTS kms_key_material_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    key_id BYTEA NOT NULL,
    key_digest BYTEA,
    storage_urls TEXT[] NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'published')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, key_id)
);
