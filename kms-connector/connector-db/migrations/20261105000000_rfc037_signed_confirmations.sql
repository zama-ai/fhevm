-- The KMS Core now signs EIP-712 confirmations for context creation, epoch activation, and
-- context and epoch destruction. The kms-worker stores them so the tx-sender can submit them to
-- the `ProtocolConfig` contract on Ethereum.

-- Rows created before this migration carry no signature and can no longer be submitted.
ALTER TABLE new_kms_context_responses
    ADD COLUMN signature BYTEA NOT NULL DEFAULT '\x',
    ADD COLUMN extra_data BYTEA NOT NULL DEFAULT '\x';
ALTER TABLE new_kms_context_responses
    ALTER COLUMN signature DROP DEFAULT,
    ALTER COLUMN extra_data DROP DEFAULT;
UPDATE new_kms_context_responses SET status = 'failed' WHERE status IN ('pending', 'under_process');

ALTER TABLE epoch_result_responses
    ADD COLUMN signature BYTEA NOT NULL DEFAULT '\x',
    ADD COLUMN extra_data BYTEA NOT NULL DEFAULT '\x';
ALTER TABLE epoch_result_responses
    ALTER COLUMN signature DROP DEFAULT,
    ALTER COLUMN extra_data DROP DEFAULT;
UPDATE epoch_result_responses SET status = 'failed' WHERE status IN ('pending', 'under_process');

CREATE TABLE IF NOT EXISTS kms_context_destruction_responses (
    context_id BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    extra_data BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    PRIMARY KEY (context_id)
);

CREATE TABLE IF NOT EXISTS kms_epoch_destruction_responses (
    epoch_id BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    extra_data BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    PRIMARY KEY (epoch_id)
);

-- updated_at refresh triggers
CREATE OR REPLACE FUNCTION refresh_updated_at_kms_context_destruction_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_kms_context_destruction_responses_on_update
BEFORE UPDATE ON kms_context_destruction_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_kms_context_destruction_responses();

CREATE OR REPLACE FUNCTION refresh_updated_at_kms_epoch_destruction_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_kms_epoch_destruction_responses_on_update
BEFORE UPDATE ON kms_epoch_destruction_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_kms_epoch_destruction_responses();

-- Notify tx-sender that new response rows have landed.
CREATE OR REPLACE FUNCTION notify_kms_context_destruction_response()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY kms_context_destruction_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_kms_context_destruction_responses_insertions
AFTER INSERT ON kms_context_destruction_responses
FOR EACH STATEMENT
EXECUTE FUNCTION notify_kms_context_destruction_response();

CREATE OR REPLACE FUNCTION notify_kms_epoch_destruction_response()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY kms_epoch_destruction_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_kms_epoch_destruction_responses_insertions
AFTER INSERT ON kms_epoch_destruction_responses
FOR EACH STATEMENT
EXECUTE FUNCTION notify_kms_epoch_destruction_response();

-- Mark the originating request as `completed` once its response has been stored.
CREATE OR REPLACE FUNCTION complete_kms_context_destroyed()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE kms_context_destroyed SET status = 'completed' WHERE context_id = NEW.context_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_kms_context_destroyed_on_response_insert
AFTER INSERT ON kms_context_destruction_responses
FOR EACH ROW
EXECUTE FUNCTION complete_kms_context_destroyed();

CREATE OR REPLACE FUNCTION complete_kms_epoch_destroyed()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE kms_epoch_destroyed SET status = 'completed' WHERE epoch_id = NEW.epoch_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_kms_epoch_destroyed_on_response_insert
AFTER INSERT ON kms_epoch_destruction_responses
FOR EACH ROW
EXECUTE FUNCTION complete_kms_epoch_destroyed();
