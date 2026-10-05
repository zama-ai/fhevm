-- The last sealed block the Solana host listener applied, written in that block's transaction so
-- a restart resumes exactly after the recorded work (Yellowstone replays inclusively from it).
CREATE TABLE solana_listener_checkpoint (
    singleton SMALLINT PRIMARY KEY DEFAULT 1 CHECK (singleton = 1),
    slot BIGINT NOT NULL CHECK (slot >= 0),
    block_hash BYTEA NOT NULL CHECK (octet_length(block_hash) = 32),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
