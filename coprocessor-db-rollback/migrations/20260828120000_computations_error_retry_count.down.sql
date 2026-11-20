-- Down for 20260828120000_computations_error_retry_count.
-- Drops the column. Retry counters are lost.

ALTER TABLE computations
    DROP COLUMN IF EXISTS error_retry_count;
