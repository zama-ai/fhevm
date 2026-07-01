-- Support for aborting key generation and CRS generation.
--
-- Two flows are involved:
--   1. The `AbortKeygen`/`AbortCrsgen` events emitted by the KMSGeneration contract are stored in
--      their own tables (below) so the kms-worker can relay `abort_key_gen`/`abort_crs_gen` to the
--      KMS Core. They are kept separate from the request tables so an abort is always persisted and
--      relayed, regardless of whether the original keygen/crsgen request row still exists.
--   2. Once the Core is aborted, polling `get_*_result` on the original request returns
--      `tonic::Code::Aborted`. The kms-worker maps that to the new `'aborted'` terminal status on
--      the original keygen/crsgen request row (see `OperationStatus::Aborted`).

-- New terminal status, distinct from `'failed'`, for requests retired by an abort.
-- Added on its own so the value is committed before any runtime code uses it.
ALTER TYPE operation_status ADD VALUE IF NOT EXISTS 'aborted';

-- `abort_keygen_requests` is keyed by `prep_keygen_id` (the ID carried by the `AbortKeygen` event),
-- which is what `abort_key_gen` expects on the KMS Core and aborts both the preprocessing and any
-- key generation that consumed it.
CREATE TABLE IF NOT EXISTS abort_keygen_requests (
    prep_keygen_id BYTEA NOT NULL,
    tx_hash BYTEA,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    already_sent BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (prep_keygen_id)
);

CREATE TABLE IF NOT EXISTS abort_crsgen_requests (
    crs_id BYTEA NOT NULL,
    tx_hash BYTEA,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    already_sent BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (crs_id)
);

--- TODO it is a bit unclear to me if all these steps below are indeed needed
CREATE INDEX IF NOT EXISTS idx_abort_keygen_requests_status_updated_at
    ON abort_keygen_requests (status, updated_at);
CREATE INDEX IF NOT EXISTS idx_abort_crsgen_requests_status_updated_at
    ON abort_crsgen_requests (status, updated_at);

CREATE OR REPLACE FUNCTION refresh_updated_at_abort_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_abort_keygen_requests_on_update
BEFORE UPDATE ON abort_keygen_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_abort_keygen_requests();

CREATE OR REPLACE FUNCTION refresh_updated_at_abort_crsgen_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_abort_crsgen_requests_on_update
BEFORE UPDATE ON abort_crsgen_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_abort_crsgen_requests();

-- Notify kms-worker that new abort request rows have landed.
CREATE OR REPLACE FUNCTION notify_abort_keygen_request()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY abort_keygen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_abort_keygen_requests_insertions
AFTER INSERT ON abort_keygen_requests
FOR EACH STATEMENT
EXECUTE FUNCTION notify_abort_keygen_request();

CREATE OR REPLACE FUNCTION notify_abort_crsgen_request()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY abort_crsgen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_abort_crsgen_requests_insertions
AFTER INSERT ON abort_crsgen_requests
FOR EACH STATEMENT
EXECUTE FUNCTION notify_abort_crsgen_request();
