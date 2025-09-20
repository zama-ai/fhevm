ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS is_allowed BOOLEAN NOT NULL DEFAULT FALSE;

-- We update tranction_id of all complete computations
UPDATE computations
  SET is_allowed = TRUE
  WHERE (output_handle, tenant_id) IN (
   	SELECT handle, tenant_id FROM allowed_handles
	 WHERE is_computed = FALSE
	);
