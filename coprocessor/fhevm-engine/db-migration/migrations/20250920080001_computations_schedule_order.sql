ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS schedule_order TIMESTAMP NOT NULL DEFAULT NOW(),
  ADD COLUMN IF NOT EXISTS uncomputable_counter SMALLINT NOT NULL DEFAULT 1;

CREATE INDEX IF NOT EXISTS idx_computations_schedule_order
  ON computations USING BTREE (schedule_order)
  WHERE is_completed = false;

DROP INDEX IF EXISTS idx_allowed_handles_schedule_order;

ALTER TABLE allowed_handles DROP COLUMN IF EXISTS schedule_order;
ALTER TABLE allowed_handles DROP COLUMN IF EXISTS uncomputable_counter;
ALTER TABLE allowed_handles DROP COLUMN IF EXISTS is_computed;

