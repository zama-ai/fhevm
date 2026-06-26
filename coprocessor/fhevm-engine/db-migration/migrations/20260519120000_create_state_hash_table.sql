CREATE TABLE IF NOT EXISTS state_hash (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    state_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chain_id, block_number)
);
