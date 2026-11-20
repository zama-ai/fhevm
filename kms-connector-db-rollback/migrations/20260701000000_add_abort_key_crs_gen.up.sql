-- Support for aborting key generation and CRS generation.

-- New terminal status, distinct from `'failed'`, for requests retired by an abort.
ALTER TYPE operation_status ADD VALUE IF NOT EXISTS 'aborted';

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

-- Refresh the `updated_at` timestamp of abort requests on update.
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
