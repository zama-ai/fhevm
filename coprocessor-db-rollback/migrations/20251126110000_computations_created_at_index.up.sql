CREATE INDEX IF NOT EXISTS idx_computations_created_at
  ON computations USING BTREE (created_at)
  WHERE is_completed = false;
