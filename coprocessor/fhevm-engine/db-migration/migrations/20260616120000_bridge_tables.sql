-- Tables used by the confidential-bridge feature (see RFC 008).
--
-- ---------------------------------------------------------------------------
-- bridge_handle_events: `BridgeHandle(senderDapp, srcHandle, dstChainId, guid)`
-- emitted on the source chain.
--
-- UNIQUE (src_handle, dst_chain_id): a row is the approval "src_handle may be
-- bridged to dst_chain". Re-bridging the same handle to the same chain is a
-- no-op as we only need to know an approval exists.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS bridge_handle_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    src_handle BYTEA NOT NULL,
    dst_chain_id BIGINT NOT NULL CHECK (dst_chain_id >= 0),
    src_chain_id BIGINT NOT NULL CHECK (src_chain_id >= 0),
    sender_dapp BYTEA NOT NULL,
    guid BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_id BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (src_handle, dst_chain_id)
);

CREATE INDEX IF NOT EXISTS idx_bridge_handle_events_chain_block
    ON bridge_handle_events (src_chain_id, block_number);

-- ---------------------------------------------------------------------------
-- handle_bridged_events: `HandleBridged(receiverDapp, srcHandle, dstHandle, guid)`
-- emitted on the destination chain.
--
-- We use dst_handle as UNIQUE as src_handle is used in its calculation anyway.
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS handle_bridged_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    src_handle BYTEA NOT NULL,
    dst_handle BYTEA NOT NULL,
    dst_chain_id BIGINT NOT NULL CHECK (dst_chain_id >= 0),
    receiver_dapp BYTEA NOT NULL,
    guid BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    transaction_id BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_associated BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE (dst_handle)
);

CREATE INDEX IF NOT EXISTS idx_handle_bridged_events_src_handle_chain
    ON handle_bridged_events (src_handle, dst_chain_id);

CREATE INDEX IF NOT EXISTS idx_handle_bridged_events_chain_block
    ON handle_bridged_events (dst_chain_id, block_number);

CREATE INDEX IF NOT EXISTS idx_handle_bridged_events_pending
    ON handle_bridged_events (id) WHERE NOT is_associated;
