ALTER TABLE computations
ADD COLUMN IF NOT EXISTS dependence_chain_id BYTEA;
