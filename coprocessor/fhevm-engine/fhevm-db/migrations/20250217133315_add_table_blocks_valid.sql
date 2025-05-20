CREATE TABLE IF NOT EXISTS blocks_valid (
    chain_id INT NOT NULL,
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    listener_tfhe BOOLEAN NOT NULL DEFAULT FALSE, -- all tfhe events have been propagated for this block
    listener_acl BOOLEAN NOT NULL DEFAULT FALSE, -- all acl events have been propagated for this block
    PRIMARY KEY (chain_id, block_hash)
);
