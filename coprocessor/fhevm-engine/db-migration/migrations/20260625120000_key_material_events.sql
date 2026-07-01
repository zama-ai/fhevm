-- RFC-029 event inboxes. The listener may see blocks before finality, so the
-- KMSGeneration event parameters are kept until host_chain_blocks_valid marks
-- the containing block finalized.

CREATE TABLE IF NOT EXISTS kms_key_material_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    key_id BYTEA NOT NULL,
    key_digest BYTEA,
    storage_urls TEXT[] NOT NULL DEFAULT '{}',
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'published', 'cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, key_id)
);

CREATE TABLE IF NOT EXISTS kms_key_material_schedule_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    key_id BYTEA NOT NULL,
    host_chain_ids BIGINT[] NOT NULL,
    host_target_blocks BIGINT[] NOT NULL,
    gateway_target_block BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'applied', 'cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    PRIMARY KEY (chain_id, block_hash, key_id)
);
