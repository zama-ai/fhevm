ALTER TABLE ciphertext_digest
ADD COLUMN IF NOT EXISTS txn_hash BYTEA NULL DEFAULT NULL,
ADD COLUMN IF NOT EXISTS txn_block_number BIGINT NULL DEFAULT NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_txn_block_number ON ciphertext_digest(txn_block_number);