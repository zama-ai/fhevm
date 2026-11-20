ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS is_allowed BOOLEAN NOT NULL DEFAULT FALSE,
  ADD COLUMN IF NOT EXISTS schedule_order TIMESTAMP NOT NULL DEFAULT NOW(),
  ADD COLUMN IF NOT EXISTS uncomputable_counter SMALLINT NOT NULL DEFAULT 1;

-- We update is_allowed flag of all computations that are not yet
-- computed and producing an allowed handle
UPDATE computations
  SET is_allowed = TRUE
  WHERE (output_handle, tenant_id) IN (
   	SELECT handle, tenant_id FROM allowed_handles
	 WHERE is_computed = FALSE
	);

CREATE INDEX IF NOT EXISTS idx_computations_is_allowed
  ON computations USING BTREE (is_allowed)
  WHERE is_completed = false;
CREATE INDEX IF NOT EXISTS idx_computations_schedule_order
  ON computations USING BTREE (schedule_order)
  WHERE is_completed = false;
CREATE INDEX IF NOT EXISTS idx_computations_pk
  ON computations USING BTREE (tenant_id, output_handle, transaction_id);

DROP INDEX IF EXISTS idx_allowed_handles_schedule_order;
ALTER TABLE allowed_handles DROP COLUMN IF EXISTS schedule_order;
ALTER TABLE allowed_handles DROP COLUMN IF EXISTS uncomputable_counter;
ALTER TABLE allowed_handles DROP COLUMN IF EXISTS is_computed;

