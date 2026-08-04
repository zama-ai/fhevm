-- Latest complete coprocessor registry snapshot read from GatewayConfig.
-- gw-listener replaces all rows in one transaction, so readers never observe
-- a partially refreshed operator set.
CREATE TABLE IF NOT EXISTS public.gateway_config_coprocessors
(
    tx_sender_address BYTEA PRIMARY KEY
        CHECK (OCTET_LENGTH(tx_sender_address) = 20),
    signer_address BYTEA NOT NULL UNIQUE
        CHECK (OCTET_LENGTH(signer_address) = 20),
    s3_bucket_url TEXT NOT NULL CHECK (LENGTH(s3_bucket_url) > 0),

    coprocessor_threshold BIGINT NOT NULL CHECK (coprocessor_threshold > 0),
    gateway_chain_id BIGINT NOT NULL CHECK (gateway_chain_id >= 0),
    gateway_config_address BYTEA NOT NULL
        CHECK (OCTET_LENGTH(gateway_config_address) = 20),
    snapshot_block_number BIGINT NOT NULL CHECK (snapshot_block_number >= 0),
    snapshot_block_hash BYTEA NOT NULL
        CHECK (OCTET_LENGTH(snapshot_block_hash) = 32),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
