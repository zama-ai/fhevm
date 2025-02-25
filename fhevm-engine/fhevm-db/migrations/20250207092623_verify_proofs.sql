CREATE TABLE IF NOT EXISTS verify_proofs (
    zk_proof_id BIGINT PRIMARY KEY NOT NULL CHECK (zk_proof_id >= 0),
    chain_id INTEGER NOT NULL CHECK(chain_id >= 0),
    contract_address TEXT NOT NULL,
    user_address TEXT NOT NULL,
    input BYTEA,
    handles BYTEA,
    retry_count INTEGER NOT NULL DEFAULT 0,
    verified BOOLEAN DEFAULT NULL,
    last_error TEXT,
    verified_at TIMESTAMPTZ,
    last_retry_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_verify_proofs_verified_retry ON verify_proofs(verified, retry_count, zk_proof_id);
