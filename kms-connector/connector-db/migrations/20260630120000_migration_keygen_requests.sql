--------------------------------------------------------------------------------------------------
--   RFC-029: first-class migration keygen requests (keygen-from-existing, no side table)         --
--                                                                                                --
-- A migration keygen is its own request type: the host contract emits MigrationKeygenRequest in   --
-- place of KeygenRequest, carrying the existing key the worker needs to build a UseExisting +      --
-- compressed keygen. The kms-worker branches on the event type, so a                               --
-- migration can never silently run as a normal keygen. The KMS still answers with an ordinary      --
-- keygen response (keyed by key_id, stored in keygen_responses), which marks this request done.    --
--------------------------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS migration_keygen_requests (
    prep_keygen_id BYTEA NOT NULL,
    key_id BYTEA NOT NULL,
    existing_key_id BYTEA NOT NULL,
    extra_data BYTEA,
    tx_hash BYTEA,
    already_sent BOOLEAN NOT NULL DEFAULT FALSE,
    status operation_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL,
    otlp_context BYTEA NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (key_id)
);

-- Notify the worker when a new migration keygen request is inserted (mirrors keygen_requests).
CREATE OR REPLACE FUNCTION notify_migration_keygen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY migration_keygen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_migration_keygen_requests_insertions
    AFTER INSERT
    ON migration_keygen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_migration_keygen_request();

-- Keep updated_at fresh on every status change (mirrors the keygen_requests garbage-collection field).
CREATE OR REPLACE FUNCTION refresh_updated_at_migration_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_migration_keygen_requests_on_update
BEFORE UPDATE ON migration_keygen_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_migration_keygen_requests();

-- A migration keygen is answered by an ordinary keygen response (keyed by key_id), so mark the
-- migration request completed when that response lands (mirrors complete_keygen_now).
CREATE OR REPLACE FUNCTION complete_migration_keygen_now()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE migration_keygen_requests SET status = 'completed' WHERE key_id = NEW.key_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_migration_keygen_requests_on_response_insert
AFTER INSERT ON keygen_responses
FOR EACH ROW
EXECUTE FUNCTION complete_migration_keygen_now();

-- Seed the block cursor for the new event type (the enum value was added in the prior migration, so
-- it is now usable in this separate transaction). The gw-listener UPDATEs this row as it polls.
-- `updated_at` is NOT NULL with no default (its default was dropped in 20260203091107), so set it.
INSERT INTO last_block_polled(event_type, block_number, updated_at) VALUES
    ('MigrationKeygenRequest', NULL, NOW())
ON CONFLICT DO NOTHING;
