--------------------------------------------------------------------------------------------------
--                 Native-v0 Solana decryption request/response durable storage                 --
--------------------------------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS solana_native_decryption_requests_v0 (
    request_hash BYTEA NOT NULL,
    host_chain_id BYTEA NOT NULL,
    solana_cluster_id BYTEA NOT NULL,
    kms_context_id BYTEA NOT NULL,
    request_mode SMALLINT NOT NULL,
    response_context BYTEA NOT NULL,
    request_bytes BYTEA NOT NULL,
    observed_slot BYTEA,
    observed_commitment_level SMALLINT,
    account_witnesses_hash BYTEA,
    already_sent BOOLEAN DEFAULT FALSE NOT NULL,
    error_counter SMALLINT DEFAULT 0 NOT NULL,
    status operation_status DEFAULT 'pending' NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    otlp_context BYTEA NOT NULL,
    PRIMARY KEY (request_hash),
    CHECK (octet_length(request_hash) = 32),
    CHECK (octet_length(host_chain_id) = 8),
    CHECK (octet_length(solana_cluster_id) = 32),
    CHECK (octet_length(kms_context_id) = 32),
    CHECK (request_mode >= 0 AND request_mode <= 255),
    CHECK (observed_slot IS NULL OR octet_length(observed_slot) = 8),
    CHECK (observed_commitment_level IS NULL OR (observed_commitment_level >= 0 AND observed_commitment_level <= 255)),
    CHECK (account_witnesses_hash IS NULL OR octet_length(account_witnesses_hash) = 32)
);

CREATE INDEX IF NOT EXISTS idx_solana_native_decryption_requests_v0_status_updated_at
    ON solana_native_decryption_requests_v0 (status, updated_at);

CREATE TABLE IF NOT EXISTS solana_native_decryption_responses_v0 (
    request_hash BYTEA NOT NULL,
    host_chain_id BYTEA NOT NULL,
    solana_cluster_id BYTEA NOT NULL,
    kms_context_id BYTEA NOT NULL,
    request_mode SMALLINT NOT NULL,
    response_kind SMALLINT NOT NULL,
    response_context BYTEA NOT NULL,
    response_hash BYTEA NOT NULL,
    response_payload BYTEA NOT NULL,
    raw_response_body BYTEA NOT NULL,
    certificate BYTEA NOT NULL,
    status operation_status DEFAULT 'pending' NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    otlp_context BYTEA NOT NULL,
    PRIMARY KEY (request_hash),
    FOREIGN KEY (request_hash)
        REFERENCES solana_native_decryption_requests_v0(request_hash),
    CHECK (octet_length(request_hash) = 32),
    CHECK (octet_length(host_chain_id) = 8),
    CHECK (octet_length(solana_cluster_id) = 32),
    CHECK (octet_length(kms_context_id) = 32),
    CHECK (request_mode >= 0 AND request_mode <= 255),
    CHECK (response_kind >= 0 AND response_kind <= 255),
    CHECK (octet_length(response_hash) = 32),
    CHECK (octet_length(response_payload) = 311),
    CHECK (substring(response_payload FROM 1 FOR 1) = decode('00', 'hex')),
    CHECK (octet_length(certificate) >= 69),
    CHECK ((octet_length(certificate) - 69) % 96 = 0),
    CHECK (
        CASE
            WHEN octet_length(certificate) >= 69 THEN
                octet_length(certificate) =
                    69 + 96 * (get_byte(certificate, 67) + get_byte(certificate, 68) * 256)
            ELSE FALSE
        END
    ),
    CHECK (substring(certificate FROM 1 FOR 1) = decode('00', 'hex'))
);

CREATE INDEX IF NOT EXISTS idx_solana_native_decryption_responses_v0_status_updated_at
    ON solana_native_decryption_responses_v0 (status, updated_at);

--------------------------------------------------------------------------------------------------
--                            Autofill `updated_at`/`status` fields                             --
--------------------------------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION refresh_updated_at_solana_native_decryption_requests_v0()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_solana_native_decryption_requests_v0_on_update
BEFORE UPDATE ON solana_native_decryption_requests_v0
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_solana_native_decryption_requests_v0();

CREATE OR REPLACE FUNCTION refresh_updated_at_solana_native_decryption_responses_v0()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_solana_native_decryption_responses_v0_on_update
BEFORE UPDATE ON solana_native_decryption_responses_v0
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_solana_native_decryption_responses_v0();

CREATE OR REPLACE FUNCTION complete_solana_native_decryption_request_v0()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE solana_native_decryption_requests_v0
    SET status = 'completed'
    WHERE request_hash = NEW.request_hash;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_solana_native_decryption_requests_v0_on_response_insert
AFTER INSERT ON solana_native_decryption_responses_v0
FOR EACH ROW
EXECUTE FUNCTION complete_solana_native_decryption_request_v0();

--------------------------------------------------------------------------------------------------
--                                            Notifications                                      --
--------------------------------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION notify_solana_native_decryption_request_v0()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY solana_native_decryption_request_v0_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_solana_native_decryption_requests_v0_insertions
AFTER INSERT ON solana_native_decryption_requests_v0
FOR EACH STATEMENT
EXECUTE FUNCTION notify_solana_native_decryption_request_v0();

CREATE OR REPLACE FUNCTION notify_solana_native_decryption_response_v0()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY solana_native_decryption_response_v0_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_solana_native_decryption_responses_v0_insertions
AFTER INSERT ON solana_native_decryption_responses_v0
FOR EACH STATEMENT
EXECUTE FUNCTION notify_solana_native_decryption_response_v0();
