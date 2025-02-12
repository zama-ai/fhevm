CREATE TABLE IF NOT EXISTS verify_proofs (
    zk_proof_id BIGINT PRIMARY KEY NOT NULL CHECK (zk_proof_id >= 0),
    chain_id INTEGER NOT NULL CHECK(chain_id >= 0),
    contract_address TEXT NOT NULL,
    user_address TEXT NOT NULL,
    handles BYTEA NOT NULL,
    retry_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_verify_proofs_retry_count ON verify_proofs(retry_count);
