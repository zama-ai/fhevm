ALTER TABLE allowed_handles
  ADD COLUMN IF NOT EXISTS allowed_at TIMESTAMP NOT NULL DEFAULT NOW(),
  ADD COLUMN IF NOT EXISTS is_computed BOOLEAN NOT NULL DEFAULT FALSE;

-- We update the handles already in the DB where we know computation is complete
UPDATE allowed_handles
  SET is_computed = TRUE
  WHERE txn_is_sent = TRUE;

CREATE INDEX IF NOT EXISTS idx_allowed_handles_is_computed
  ON allowed_handles (is_computed);
CREATE INDEX IF NOT EXISTS idx_allowed_handles_allowed_at
  ON allowed_handles USING BTREE (allowed_at)
  WHERE is_computed = FALSE;
CREATE INDEX IF NOT EXISTS idx_allowed_handles_handle
  ON allowed_handles (handle);

