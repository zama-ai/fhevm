-- Immutable association between a handle produced by an allowed TFHE
-- computation and its exact host-chain producer block. The table starts empty:
-- manifests intentionally cover only handles observed after this migration.
CREATE TABLE handle_producer_block
(
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    handle BYTEA NOT NULL CHECK (OCTET_LENGTH(handle) = 32),
    producer_block_number BIGINT NOT NULL CHECK (producer_block_number >= 0),
    producer_block_hash BYTEA NOT NULL CHECK (OCTET_LENGTH(producer_block_hash) = 32),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (host_chain_id, handle, producer_block_hash)
);

-- Manifest discovery retrieves every handle attributed to an exact producer
-- block. Including the handle makes this a covering index for that lookup.
CREATE INDEX idx_handle_producer_block_block
ON handle_producer_block (host_chain_id, producer_block_hash, handle);
