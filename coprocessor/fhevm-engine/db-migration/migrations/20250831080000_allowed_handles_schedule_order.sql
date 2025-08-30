ALTER TABLE allowed_handles
  ADD COLUMN IF NOT EXISTS schedule_order TIMESTAMP NOT NULL DEFAULT NOW(),
  ADD COLUMN IF NOT EXISTS uncomputable_counter SMALLINT NOT NULL DEFAULT 1;

CREATE INDEX IF NOT EXISTS idx_allowed_handles_schedule_order
  ON allowed_handles USING BTREE (schedule_order)
  WHERE is_computed = false;

DROP INDEX IF EXISTS idx_computations_schedule_order;

ALTER TABLE computations DROP COLUMN IF EXISTS schedule_order;
ALTER TABLE computations DROP COLUMN IF EXISTS uncomputable_counter;

