CREATE INDEX IF NOT EXISTS idx_computations_is_allowed
  ON computations USING BTREE (is_allowed)
  WHERE is_completed = false;
