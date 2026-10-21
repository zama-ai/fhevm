-- The PRF endpoint keeps one row for each requested output, from admission to the signed result.
-- Both blue-green fleets share this table, so it has no GCS copy (see `COPROCESSOR_TABLES`).
CREATE TABLE IF NOT EXISTS pads
(
    prf_instance INTEGER NOT NULL CHECK (prf_instance BETWEEN 0 AND 65535),
    label BYTEA NOT NULL CHECK (OCTET_LENGTH(label) = 32),
    -- The attestation signs `padLength` as a uint16.
    pad_length INTEGER NOT NULL CHECK (pad_length BETWEEN 1 AND 65535),
    -- The KMS Connector that pays for the evaluation while the row is pending.
    triggered_by BYTEA NOT NULL CHECK (OCTET_LENGTH(triggered_by) = 20),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    claimed_at TIMESTAMPTZ NULL,
    retry_count INTEGER NOT NULL DEFAULT 0,
    is_error BOOLEAN NOT NULL DEFAULT FALSE,
    -- The pad-worker writes the output and its attestation. A row is complete when it has them.
    ciphertext128 BYTEA NULL,
    ciphertext_digest BYTEA NULL CHECK (OCTET_LENGTH(ciphertext_digest) = 32),
    sns_ciphertext_digest BYTEA NULL CHECK (OCTET_LENGTH(sns_ciphertext_digest) = 32),
    signer BYTEA NULL CHECK (OCTET_LENGTH(signer) = 20),
    signature BYTEA NULL CHECK (OCTET_LENGTH(signature) = 65),
    s3_verified_at TIMESTAMPTZ NULL,

    PRIMARY KEY (prf_instance, label, pad_length),
    CHECK (NUM_NULLS(ciphertext128, ciphertext_digest, sns_ciphertext_digest, signer, signature) IN (0, 5))
);

CREATE INDEX IF NOT EXISTS idx_pads_pending_by_connector
    ON pads (triggered_by)
    WHERE signature IS NULL AND NOT is_error;

CREATE INDEX IF NOT EXISTS idx_pads_unclaimed
    ON pads (created_at)
    WHERE claimed_at IS NULL AND signature IS NULL AND NOT is_error;

CREATE INDEX IF NOT EXISTS idx_pads_claimed
    ON pads (claimed_at)
    WHERE claimed_at IS NOT NULL AND signature IS NULL AND NOT is_error;

-- GC uses this index to find old completed and failed rows.
CREATE INDEX IF NOT EXISTS idx_pads_done
    ON pads (created_at)
    WHERE signature IS NOT NULL OR is_error;

-- Wake the pad-workers when a row goes into the queue: a new row, or a row that the reaper releases.
-- `WHEN` cannot use TG_OP, so insert and update are separate triggers.
CREATE OR REPLACE FUNCTION notify_pad_work_available()
    RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('pad_work_available', '');
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER pads_work_available_insert
    AFTER INSERT
    ON pads
    FOR EACH ROW
    EXECUTE FUNCTION notify_pad_work_available();

CREATE TRIGGER pads_work_available_update
    AFTER UPDATE
    ON pads
    FOR EACH ROW
    WHEN (OLD.claimed_at IS NOT NULL AND NEW.claimed_at IS NULL
        AND NEW.signature IS NULL AND NOT NEW.is_error)
    EXECUTE FUNCTION notify_pad_work_available();
