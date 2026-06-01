-- Responses for the ProtocolConfig events forwarded by kms-worker to KMS Core.
CREATE TABLE IF NOT EXISTS new_kms_context_responses (
    context_id BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    PRIMARY KEY (context_id)
);

CREATE TABLE IF NOT EXISTS epoch_result_responses (
    context_id BYTEA NOT NULL,
    epoch_id BYTEA NOT NULL,
    keys BYTEA NOT NULL,
    crs_list BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    PRIMARY KEY (epoch_id)
);

-- updated_at refresh triggers
CREATE OR REPLACE FUNCTION refresh_updated_at_new_kms_context_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_new_kms_context_responses_on_update
BEFORE UPDATE ON new_kms_context_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_new_kms_context_responses();

CREATE OR REPLACE FUNCTION refresh_updated_at_epoch_result_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_epoch_result_responses_on_update
BEFORE UPDATE ON epoch_result_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_epoch_result_responses();

-- Notify tx-sender that new response rows have landed.
CREATE OR REPLACE FUNCTION notify_new_kms_context_response()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY new_kms_context_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_new_kms_context_responses_insertions
AFTER INSERT ON new_kms_context_responses
FOR EACH STATEMENT
EXECUTE FUNCTION notify_new_kms_context_response();

CREATE OR REPLACE FUNCTION notify_epoch_result_response()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY epoch_result_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_epoch_result_responses_insertions
AFTER INSERT ON epoch_result_responses
FOR EACH STATEMENT
EXECUTE FUNCTION notify_epoch_result_response();

-- Mark the originating request as `completed` once its response has been stored, mirroring the
-- garbage-collection triggers wired for decryptions and key generations.
CREATE OR REPLACE FUNCTION complete_new_kms_context()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE new_kms_context SET status = 'completed' WHERE context_id = NEW.context_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_new_kms_context_on_response_insert
AFTER INSERT ON new_kms_context_responses
FOR EACH ROW
EXECUTE FUNCTION complete_new_kms_context();

CREATE OR REPLACE FUNCTION complete_new_kms_epoch()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE new_kms_epoch SET status = 'completed'
    WHERE epoch_id = NEW.epoch_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_new_kms_epoch_on_response_insert
AFTER INSERT ON epoch_result_responses
FOR EACH ROW
EXECUTE FUNCTION complete_new_kms_epoch();
