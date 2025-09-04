ALTER TABLE allowed_handles
ADD COLUMN IF NOT EXISTS txn_hash BYTEA NULL DEFAULT NULL,
ADD COLUMN IF NOT EXISTS txn_block_number BIGINT NULL DEFAULT NULL;

CREATE INDEX IF NOT EXISTS idx_allowed_handles_txn_block_number ON allowed_handles(txn_block_number);