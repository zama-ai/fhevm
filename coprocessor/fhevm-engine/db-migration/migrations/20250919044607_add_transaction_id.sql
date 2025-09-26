-- Add transaction_id column to pbs_computations (if not present)
ALTER TABLE pbs_computations
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;

-- Add transaction_id column to allowed_handles (if not present)
ALTER TABLE allowed_handles
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;

-- Add transaction_id column to verify_proofs (if not present)
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;

-- Add transaction_id column to ciphertext_digest (if not present)
ALTER TABLE ciphertext_digest
ADD COLUMN IF NOT EXISTS transaction_id bytea NULL;

CREATE TABLE transactions (
    id BYTEA PRIMARY KEY,
    src_id INT NOT NULL, -- 1 for L1, 2 for L2, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    block_number BIGINT NOT NULL,
    completed_at TIMESTAMPTZ DEFAULT NULL
);
