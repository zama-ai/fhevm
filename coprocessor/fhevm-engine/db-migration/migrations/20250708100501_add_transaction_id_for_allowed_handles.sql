ALTER TABLE allowed_handles
ADD COLUMN IF NOT EXISTS transaction_id BYTEA;
