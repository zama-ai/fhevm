-- A pad-worker can run an older image than the stack: add only, and keep the `pad_ready` payload.

ALTER TABLE pads
    ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ NULL,
    ADD COLUMN IF NOT EXISTS attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    ADD COLUMN IF NOT EXISTS error_message TEXT NULL,
    ADD COLUMN IF NOT EXISTS s3_verified_at TIMESTAMPTZ NULL;

-- The claim: pending rows of one instance, oldest first.
CREATE INDEX IF NOT EXISTS idx_pads_pending_by_instance
    ON pads (prf_instance, created_at)
    WHERE signature IS NULL AND NOT is_error;

-- The resubmit and the age metric: complete rows without an S3 copy.
CREATE INDEX IF NOT EXISTS idx_pads_not_uploaded_by_instance
    ON pads (prf_instance, created_at)
    WHERE signature IS NOT NULL AND s3_verified_at IS NULL;

-- The GC of old outputs.
CREATE INDEX IF NOT EXISTS idx_pads_done_by_age
    ON pads (created_at)
    WHERE s3_verified_at IS NOT NULL OR is_error;

-- Wakes the pad-worker of a new row. Payload: `<prf>`.
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
