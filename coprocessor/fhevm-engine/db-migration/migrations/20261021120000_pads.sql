-- The PRF endpoint keeps one row for each requested output, from admission to the signed result.
CREATE TABLE IF NOT EXISTS pads
(
    prf_instance INTEGER NOT NULL CHECK (prf_instance BETWEEN 0 AND 65535),
    label BYTEA NOT NULL CHECK (OCTET_LENGTH(label) = 32),
    -- `MAX_PAD_LENGTH` of prf-service, so a row added outside the service cannot ask for more.
    pad_length INTEGER NOT NULL CHECK (pad_length BETWEEN 1 AND 2048),
    -- The KMS Connector that pays for the evaluation while the row is pending.
    triggered_by BYTEA NOT NULL CHECK (OCTET_LENGTH(triggered_by) = 20),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_error BOOLEAN NOT NULL DEFAULT FALSE,
    -- The pad-worker writes the output and its attestation. A row is complete when it has them.
    ciphertext128 BYTEA NULL,
    ciphertext_digest BYTEA NULL CHECK (OCTET_LENGTH(ciphertext_digest) = 32),
    sns_ciphertext_digest BYTEA NULL CHECK (OCTET_LENGTH(sns_ciphertext_digest) = 32),
    signer BYTEA NULL CHECK (OCTET_LENGTH(signer) = 20),
    signature BYTEA NULL CHECK (OCTET_LENGTH(signature) = 65),

    PRIMARY KEY (prf_instance, label, pad_length),
    CHECK (NUM_NULLS(ciphertext128, ciphertext_digest, sns_ciphertext_digest, signer, signature) IN (0, 5))
);

CREATE INDEX IF NOT EXISTS idx_pads_pending_by_connector
    ON pads (triggered_by)
    WHERE signature IS NULL AND NOT is_error;

-- Wake the requests of a complete or failed row. Payload: `<prf>:<label hex>:<pad_length>`.
CREATE OR REPLACE FUNCTION notify_pad_ready()
    RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify(
        'pad_ready',
        NEW.prf_instance || ':' || encode(NEW.label, 'hex') || ':' || NEW.pad_length
    );
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER pads_ready
    AFTER UPDATE
    ON pads
    FOR EACH ROW
    WHEN ((OLD.signature IS NULL AND NEW.signature IS NOT NULL) OR (NOT OLD.is_error AND NEW.is_error))
    EXECUTE FUNCTION notify_pad_ready();
