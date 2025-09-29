-- Add transaction_id column to pbs_computations (if not present)
ALTER TABLE pbs_computations
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_transactions_id ON pbs_computations (transaction_id);

-- Add transaction_id column to allowed_handles (if not present)
ALTER TABLE allowed_handles
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_transactions_id ON allowed_handles (transaction_id);

-- Add transaction_id column to verify_proofs (if not present)
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_transactions_id ON verify_proofs (transaction_id);

-- Add transaction_id column to ciphertext_digest (if not present)
ALTER TABLE ciphertext_digest
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;
CREATE INDEX IF NOT EXISTS idx_transactions_id ON ciphertext_digest (transaction_id);

CREATE TABLE transactions (
    id BYTEA PRIMARY KEY,
    chain_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    block_number BIGINT NOT NULL,
    completed_at TIMESTAMPTZ DEFAULT NULL
); 