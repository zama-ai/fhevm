-- The WebSocket listener may commit realtime blocks ahead of finalized catchup.
-- Keep its contiguous recovery frontier separate from the HTTP poller's cursor.
CREATE TABLE host_listener_catchup_state (
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    scope_hash BYTEA NOT NULL,
    next_block BIGINT NOT NULL CHECK (next_block >= 0),
    revision BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (chain_id, scope_hash)
);
