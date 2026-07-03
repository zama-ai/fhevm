-- Tables used by the confidential-bridge feature (see RFC 008).
--
-- ---------------------------------------------------------------------------
-- bridge_handle_events: `BridgeHandle(senderDapp, srcHandle, dstChainId, guid)`
-- emitted on the source chain.
--
-- UNIQUE (src_handle, dst_chain_id, block_hash): a row is the approval
-- "src_handle may be bridged to dst_chain" as observed in one source-chain
-- block. The block hash is part of the key so an orphaned observation cannot
-- mask a later canonical one at the same height.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS bridge_handle_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    src_handle BYTEA NOT NULL,
    dst_chain_id BIGINT NOT NULL CHECK (dst_chain_id >= 0),
    src_chain_id BIGINT NOT NULL CHECK (src_chain_id >= 0),
    sender_dapp BYTEA NOT NULL,
    guid BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL DEFAULT ''::BYTEA,
    transaction_id BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE bridge_handle_events
    ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE bridge_handle_events
    DROP CONSTRAINT IF EXISTS bridge_handle_events_src_handle_dst_chain_id_key;

CREATE UNIQUE INDEX IF NOT EXISTS idx_bridge_handle_events_src_dst_block_hash
    ON bridge_handle_events (src_handle, dst_chain_id, block_hash);

CREATE INDEX IF NOT EXISTS idx_bridge_handle_events_chain_block
    ON bridge_handle_events (src_chain_id, block_number);

-- ---------------------------------------------------------------------------
-- handle_bridged_events: `HandleBridged(receiverDapp, srcHandle, dstHandle, guid)`
-- emitted on the destination chain.
--
-- UNIQUE (dst_handle, block_hash): the destination handle identifies the
-- bridged ciphertext, and block_hash keeps competing branch observations
-- distinct until orphan filtering/cleanup resolves them.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS handle_bridged_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    src_handle BYTEA NOT NULL,
    dst_handle BYTEA NOT NULL,
    dst_chain_id BIGINT NOT NULL CHECK (dst_chain_id >= 0),
    receiver_dapp BYTEA NOT NULL,
    guid BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL DEFAULT ''::BYTEA,
    transaction_id BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_associated BOOLEAN NOT NULL DEFAULT FALSE
);

ALTER TABLE handle_bridged_events
    ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE handle_bridged_events
    DROP CONSTRAINT IF EXISTS handle_bridged_events_dst_handle_key;

CREATE UNIQUE INDEX IF NOT EXISTS idx_handle_bridged_events_dst_block_hash
    ON handle_bridged_events (dst_handle, block_hash);

CREATE INDEX IF NOT EXISTS idx_handle_bridged_events_src_handle_chain
    ON handle_bridged_events (src_handle, dst_chain_id);

CREATE INDEX IF NOT EXISTS idx_handle_bridged_events_chain_block
    ON handle_bridged_events (dst_chain_id, block_number);

CREATE INDEX IF NOT EXISTS idx_handle_bridged_events_pending
    ON handle_bridged_events (id) WHERE NOT is_associated;
