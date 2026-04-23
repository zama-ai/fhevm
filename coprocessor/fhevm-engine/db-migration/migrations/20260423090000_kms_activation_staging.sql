CREATE TABLE kms_key_activation_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA NOT NULL,
    key_id BYTEA NOT NULL,
    key_content_server BYTEA,
    key_content_sns_server OID,
    key_content_public BYTEA,
    key_digest_server BYTEA,
    key_digest_public BYTEA,
    storage_urls TEXT[] NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (
            status IN (
                'pending',
                'ready',
                'activated',
                'cancelled',
                'error'
            )
        ),
    retry_count INT NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, key_id)
);

CREATE TABLE kms_crs_activation_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA NOT NULL,
    crs_id BYTEA NOT NULL,
    crs_content BYTEA,
    crs_digest BYTEA NOT NULL,
    storage_urls TEXT[] NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (
            status IN (
                'pending',
                'ready',
                'activated',
                'cancelled',
                'error'
            )
        ),
    retry_count INT NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, crs_id)
);
