-- Add migration script here
BEGIN;
-- Lock the table to ensure the backfill of schedule_order is safe
LOCK TABLE pbs_computations IN ACCESS EXCLUSIVE MODE;

ALTER TABLE pbs_computations
    ADD COLUMN IF NOT EXISTS error TEXT,
    ADD COLUMN IF NOT EXISTS schedule_order TIMESTAMPTZ DEFAULT NOW();

-- Backfill existing rows
UPDATE pbs_computations
SET schedule_order = created_at
WHERE schedule_order IS NULL;

-- enforce not-null after backfill
ALTER TABLE pbs_computations
    ALTER COLUMN schedule_order SET NOT NULL;
COMMIT;

-- Partial index for unfinished rows ordered/filtered by created_at
CREATE INDEX IF NOT EXISTS idx_pbs_comp_unfinished_created_at
ON pbs_computations (schedule_order, created_at)
WHERE is_completed = FALSE;