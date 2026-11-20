-- Down for 20260930120000_drift_superseded.
-- Drops the column and the function. Superseded markers are lost.

ALTER TABLE drifted_handle
    DROP COLUMN IF EXISTS superseded_at;

DROP FUNCTION IF EXISTS public.drift_superseded(TEXT, BIGINT, BIGINT);
