-- Down for 20261111120000_pad_worker.
-- Drops the trigger, the function, the three indexes and the five columns.
-- Claim state, attempt counters and upload witnesses of pads are lost.

DROP TRIGGER IF EXISTS pads_work_available ON pads;
DROP FUNCTION IF EXISTS notify_pad_work_available();
DROP INDEX IF EXISTS idx_pads_done_by_age;
DROP INDEX IF EXISTS idx_pads_not_uploaded_by_instance;
DROP INDEX IF EXISTS idx_pads_pending_by_instance;

ALTER TABLE pads
    DROP COLUMN IF EXISTS s3_verified_at,
    DROP COLUMN IF EXISTS error_message,
    DROP COLUMN IF EXISTS attempts,
    DROP COLUMN IF EXISTS claim_token,
    DROP COLUMN IF EXISTS claimed_at;
