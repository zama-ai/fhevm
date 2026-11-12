-- A lane can run an older image than the stack: add only, and keep the `pad_ready` payload.

ALTER TABLE pads
    ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ NULL,
    ADD COLUMN IF NOT EXISTS attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    ADD COLUMN IF NOT EXISTS error_message TEXT NULL,
    ADD COLUMN IF NOT EXISTS s3_verified_at TIMESTAMPTZ NULL;

-- The claim: pending rows of one instance, oldest first.
CREATE INDEX IF NOT EXISTS idx_pads_pending_by_instance
    ON pads (prf_instance, created_at)
    WHERE signature IS NULL AND NOT is_error;

-- Wakes the lane of a new row. Payload: `<prf>`.
CREATE OR REPLACE FUNCTION notify_pad_work_available()
    RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('pad_work_available', NEW.prf_instance::text);
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS pads_work_available ON pads;
CREATE TRIGGER pads_work_available
    AFTER INSERT
    ON pads
    FOR EACH ROW
    EXECUTE FUNCTION notify_pad_work_available();

-- One row for each PRF instance that has a pad-worker lane.
CREATE TABLE IF NOT EXISTS prf_lanes
(
    prf_instance INTEGER PRIMARY KEY CHECK (prf_instance BETWEEN 0 AND 65535),
    -- Set once the lane has its key, then on each pass of the claim loop.
    heartbeat_at TIMESTAMPTZ NULL
);
