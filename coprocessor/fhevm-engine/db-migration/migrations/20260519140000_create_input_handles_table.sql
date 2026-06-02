-- Track the handles of input ciphertexts produced by the zkproof-worker.
-- These are not computation outputs and do not flow through the scheduler,
-- so they live in a dedicated, minimal table instead of synthesising fake
-- rows in `computations`. `block_number` carries the host-chain block where
-- the originating VerifyProofRequest event was emitted (recorded by
-- gw-listener on `verify_proofs.block_number`).

CREATE TABLE IF NOT EXISTS input_handles (
    handle BYTEA PRIMARY KEY,
    block_number BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_input_handles_block_number
ON input_handles (block_number);
