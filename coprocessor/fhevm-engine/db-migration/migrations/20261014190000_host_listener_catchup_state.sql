-- A realtime high watermark cannot identify holes in historical catchup.
-- Advance this cursor only after every block in a fetched page has committed.
CREATE TABLE host_listener_catchup_state (
    chain_id BIGINT NOT NULL,
    listener_id TEXT NOT NULL,
    next_block_number BIGINT NOT NULL CHECK (next_block_number >= 0),
    generation BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (chain_id, listener_id)
);
