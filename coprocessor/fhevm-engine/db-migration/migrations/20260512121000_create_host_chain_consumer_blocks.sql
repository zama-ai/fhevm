CREATE TABLE IF NOT EXISTS host_chain_consumer_blocks (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    duplicate_count BIGINT NOT NULL DEFAULT 0,
    stats_processed BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (chain_id, block_hash)
);

CREATE INDEX IF NOT EXISTS host_chain_consumer_blocks_unprocessed_idx
    ON host_chain_consumer_blocks (chain_id, block_number)
    WHERE stats_processed = FALSE;
