ALTER TABLE computations
ADD COLUMN IF NOT EXISTS transaction_id BYTEA;
