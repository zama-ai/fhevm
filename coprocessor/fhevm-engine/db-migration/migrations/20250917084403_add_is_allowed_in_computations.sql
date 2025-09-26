ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS is_allowed BOOL NOT NULL DEFAULT FALSE;

UPDATE computations SET is_allowed = true WHERE (output_handle, tenant_id) IN (
    SELECT handle, tenant_id FROM allowed_handles WHERE is_computed=false
);

CREATE INDEX IF NOT EXISTS idx_computations_is_allowed
    ON computations (is_allowed)
    WHERE is_computed=false;
