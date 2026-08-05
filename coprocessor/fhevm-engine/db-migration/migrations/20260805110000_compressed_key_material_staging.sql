CREATE TABLE IF NOT EXISTS kms_compressed_key_material_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA,
    key_id BYTEA NOT NULL,
    key_digest BYTEA NOT NULL,
    storage_urls TEXT[] NOT NULL,
    key_content BYTEA,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'ready', 'applied', 'cancelled')),
    retry_count INT NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, key_id)
);

