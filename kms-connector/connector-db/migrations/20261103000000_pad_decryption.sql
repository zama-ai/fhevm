-- RFC 030 pad decryption tables.

--------------------------------------------------------------------------------------------------
--                                     Request table                                            --
--------------------------------------------------------------------------------------------------

CREATE TABLE pad_decryption_requests (
    decryption_id BYTEA NOT NULL,
    witness_version SMALLINT NOT NULL,
    prf_id INTEGER NOT NULL,
    chain_id BIGINT NOT NULL,
    -- ABI-encoded `PadEntry[]` (label, per-pad witness fields, pad length).
    pads BYTEA NOT NULL,
    -- RFC-016 user-signed fields and signature.
    user_address BYTEA NOT NULL,
    public_key BYTEA NOT NULL,
    allowed_contracts BYTEA[] NOT NULL,
    start_timestamp BIGINT NOT NULL,
    duration_seconds BIGINT NOT NULL,
    extra_data BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    -- Processing bookkeeping, same semantics as the other decryption request tables.
    source request_source DEFAULT 'http' NOT NULL CHECK (source = 'http'),
    status operation_status DEFAULT 'pending' NOT NULL,
    already_sent BOOLEAN DEFAULT FALSE NOT NULL,
    error_counter SMALLINT DEFAULT 0 NOT NULL,
    otlp_context BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    PRIMARY KEY (decryption_id)
);

CREATE INDEX idx_pad_decryption_requests_status_updated_at
    ON pad_decryption_requests (status, updated_at);

CREATE OR REPLACE FUNCTION refresh_updated_at_pad_decryption_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_pad_decryption_requests_on_update
    BEFORE UPDATE ON pad_decryption_requests
    FOR EACH ROW
    EXECUTE FUNCTION refresh_updated_at_pad_decryption_requests();

-- Wakes up the kms-worker once per inserting statement.
CREATE OR REPLACE FUNCTION notify_pad_decryption_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY pad_decryption_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_pad_decryption_requests_insertions
    AFTER INSERT
    ON pad_decryption_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_pad_decryption_request();

--------------------------------------------------------------------------------------------------
--                                    Response table                                            --
--------------------------------------------------------------------------------------------------

CREATE TABLE pad_decryption_responses (
    decryption_id BYTEA NOT NULL,
    encrypted_payload BYTEA,
    signature BYTEA,
    extra_data BYTEA NOT NULL,
    error_code TEXT,
    error_details TEXT,
    source request_source DEFAULT 'http' NOT NULL CHECK (source = 'http'),
    status operation_status DEFAULT 'pending' NOT NULL,
    otlp_context BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    PRIMARY KEY (decryption_id),
    CONSTRAINT pad_decryption_responses_payload_or_error CHECK (
        (error_code IS NULL AND error_details IS NULL AND encrypted_payload IS NOT NULL AND signature IS NOT NULL)
        OR
        (error_code IS NOT NULL AND encrypted_payload IS NULL AND signature IS NULL)
    )
);

CREATE INDEX idx_pad_decryption_responses_status_updated_at
    ON pad_decryption_responses (status, updated_at);

CREATE OR REPLACE FUNCTION refresh_updated_at_pad_decryption_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_pad_decryption_responses_on_update
    BEFORE UPDATE ON pad_decryption_responses
    FOR EACH ROW
    EXECUTE FUNCTION refresh_updated_at_pad_decryption_responses();

-- Wakes up the Connector endpoint connection waiting for this response.
-- Fires on UPDATE too: a retried request overrides its previous error row.
CREATE OR REPLACE FUNCTION notify_http_pad_decryption_response()
    RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('http_pad_decryption_response_available', encode(NEW.decryption_id, 'hex'));
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_http_pad_decryption_responses_upserts
    AFTER INSERT OR UPDATE
    ON pad_decryption_responses
    FOR EACH ROW
    EXECUTE FUNCTION notify_http_pad_decryption_response();
