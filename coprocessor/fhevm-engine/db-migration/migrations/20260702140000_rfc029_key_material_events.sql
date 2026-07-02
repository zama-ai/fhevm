-- RFC-029: finalized ingestion queues for the two migration events,
-- mirroring the kms_key_activation_events state machine
-- (pending -> ready -> applied, or cancelled/error).
--
-- Compressed material bytes are STAGED here and only copied into
-- keys.compressed_xof_keyset once the cutover schedule for the key is
-- ingested: for a legacy key row, populated compressed bytes in `keys`
-- would flip the default (COALESCE) read path on local ingestion
-- timing, which is exactly the consensus hazard RFC-029 forbids.

CREATE TABLE compressed_key_material_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA,
    key_id BYTEA NOT NULL,
    key_digest_server BYTEA,
    storage_urls TEXT[] NOT NULL,
    key_content_compressed_xof_keyset BYTEA,
    status TEXT NOT NULL DEFAULT 'pending',
    retry_count INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, key_id)
);

CREATE TABLE compressed_key_cutover_events (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_hash BYTEA,
    key_id BYTEA NOT NULL,
    gateway_cutover_block BIGINT NOT NULL,
    -- JSON [{"chain_id": .., "cutover_block": ..}, ..] as emitted on-chain
    host_cutovers TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ,
    UNIQUE (chain_id, block_hash, key_id)
);
