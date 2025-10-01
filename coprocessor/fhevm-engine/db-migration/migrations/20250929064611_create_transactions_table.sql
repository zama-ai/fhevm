-- Add transaction_id column to pbs_computations (if not present)
ALTER TABLE pbs_computations
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_pbs_computations_transactions ON pbs_computations USING HASH (transaction_id);

-- Add transaction_id column to allowed_handles (if not present)
ALTER TABLE allowed_handles
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_allowed_handles_transactions ON allowed_handles USING HASH (transaction_id);

-- Add transaction_id column to verify_proofs (if not present)
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_verify_proofs_transactions ON verify_proofs USING HASH (transaction_id);

-- Add transaction_id column to ciphertext_digest (if not present)
ALTER TABLE ciphertext_digest
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_transactions ON ciphertext_digest USING HASH (transaction_id);

CREATE TABLE transactions (
    id BYTEA PRIMARY KEY,
    chain_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    block_number BIGINT NOT NULL,
    completed_at TIMESTAMPTZ DEFAULT NULL
);