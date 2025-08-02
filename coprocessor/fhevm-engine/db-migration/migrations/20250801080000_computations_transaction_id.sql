ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS transaction_id BYTEA,
  ADD COLUMN IF NOT EXISTS dependence_chain_id BYTEA;

CREATE INDEX IF NOT EXISTS idx_computations_transaction_id
  ON computations (transaction_id);

CREATE INDEX IF NOT EXISTS idx_computations_schedule_order
  ON computations USING BTREE (schedule_order)
  WHERE is_completed = false AND is_error=false;

CREATE INDEX IF NOT EXISTS idx_computations_dependence_chain
  ON computations (dependence_chain_id)
  WHERE is_completed = false AND is_error=false;
