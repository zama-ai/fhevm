-- Table used by the auto drift recovery mechanism.
-- When the gw-listener detects a ciphertext drift, it writes a row here.
-- All coprocessor services poll this table and coordinate a revert + restart/re-exec.
-- Past rows serve as an audit trail and prevent repeated auto-reverts
-- for the same host chain.

CREATE TABLE IF NOT EXISTS drift_revert_signal (
    id BIGSERIAL PRIMARY KEY,
    host_chain_id BIGINT NOT NULL,
    -- The host chain block where drift was observed (the "offending" block).
    offending_host_block_number BIGINT NOT NULL,
    -- The status of the signal itself - whether it is pending, etc.
    status TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);