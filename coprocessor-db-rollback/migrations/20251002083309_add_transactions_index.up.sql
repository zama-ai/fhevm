-- For completed txns
CREATE INDEX idx_transactions_completed_createdat
  ON transactions (created_at)
  WHERE completed_at IS NOT NULL;

-- For incomplete txns
CREATE INDEX idx_transactions_incomplete_createdat
  ON transactions (created_at)
  WHERE completed_at IS NULL;
