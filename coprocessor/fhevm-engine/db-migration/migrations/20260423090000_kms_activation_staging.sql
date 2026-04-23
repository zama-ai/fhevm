CREATE TABLE kms_key_activation_events (
    sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA NOT NULL,
    log_index BIGINT NOT NULL,
    key_id_gw BYTEA NOT NULL,
    key_digests JSONB NOT NULL,
    s3_bucket_urls TEXT[] NOT NULL,
    download_status TEXT NOT NULL DEFAULT 'pending'
        CHECK (
            download_status IN (
                'pending',
                'failed',
                'digest_mismatch',
                'invalid_event',
                'materialized',
                'orphaned'
            )
        ),
    retry_count INT NOT NULL DEFAULT 0,
    last_error TEXT,
    last_attempt_at TIMESTAMPTZ,
    materialized_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (chain_id, block_hash, transaction_hash, log_index)
);

CREATE INDEX idx_kms_key_activation_retryable
    ON kms_key_activation_events (chain_id, sequence_number)
    WHERE download_status IN ('pending', 'failed');

CREATE INDEX idx_kms_key_activation_block
    ON kms_key_activation_events (chain_id, block_hash);

CREATE TABLE kms_crs_activation_events (
    sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA NOT NULL,
    log_index BIGINT NOT NULL,
    crs_id BYTEA NOT NULL,
    crs_digest BYTEA NOT NULL,
    s3_bucket_urls TEXT[] NOT NULL,
    download_status TEXT NOT NULL DEFAULT 'pending'
        CHECK (
            download_status IN (
                'pending',
                'failed',
                'digest_mismatch',
                'invalid_event',
                'materialized',
                'orphaned'
            )
        ),
    retry_count INT NOT NULL DEFAULT 0,
    last_error TEXT,
    last_attempt_at TIMESTAMPTZ,
    materialized_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (chain_id, block_hash, transaction_hash, log_index)
);

CREATE INDEX idx_kms_crs_activation_retryable
    ON kms_crs_activation_events (chain_id, sequence_number)
    WHERE download_status IN ('pending', 'failed');

CREATE INDEX idx_kms_crs_activation_block
    ON kms_crs_activation_events (chain_id, block_hash);
