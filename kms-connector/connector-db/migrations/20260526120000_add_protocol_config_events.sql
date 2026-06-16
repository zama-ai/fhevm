-- Move the polling cursor from per-event-type to per-chain.
--
-- Rationale: with `eth_getLogs`, gw-listener polls *all* event types on a chain
-- with a single multi-address filter, so the cursor is naturally chain-scoped.
-- Keeping a row per event type means the bookkeeping has to artificially keep
-- multiple rows in lockstep on every poll.
--
-- For rollback safety, this migration introduces a new table and leaves the
-- legacy `last_block_polled` table untouched. A follow-up migration will drop
-- the old table once the new path has soaked in production.
CREATE TABLE IF NOT EXISTS last_block_polled_by_chain (
    chain_name TEXT NOT NULL,
    block_number BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chain_name)
);

-- Seed each chain with the MIN cursor from its event types in the legacy table,
-- so the upgrade doesn't lose progress.
INSERT INTO last_block_polled_by_chain(chain_name, block_number) VALUES
    (
        'ethereum',
        (SELECT MIN(block_number) FROM last_block_polled WHERE event_type IN (
            'PrepKeygenRequest',
            'KeygenRequest',
            'CrsgenRequest'
        ))
    ),
    (
        'gateway',
        (SELECT MIN(block_number) FROM last_block_polled WHERE event_type IN (
            'PublicDecryptionRequest',
            'UserDecryptionRequest'
        ))
    )
ON CONFLICT DO NOTHING;

-- Persist the active epoch alongside the active context fetched at startup
-- via KMSVerifier::getActiveKmsContextAndEpoch().
ALTER TABLE kms_context ADD COLUMN IF NOT EXISTS epoch_id BYTEA;
-- Use the default epoch ID (little endian representation) for all existing contexts
UPDATE kms_context SET epoch_id = '\x0100000000000000000000000000000000000000000000000000000000000008'::bytea WHERE epoch_id IS NULL;
ALTER TABLE kms_context ALTER COLUMN epoch_id SET NOT NULL;
ALTER TABLE kms_context
    DROP CONSTRAINT kms_context_pkey,
    ADD PRIMARY KEY (id, epoch_id);

-- Nested ABI types (KmsNodeParams[], PreviousKeyInfo[], PreviousCrsInfo[],
-- PcrValues[], KmsThresholds) are stored as ABI-encoded BYTEA to keep the
-- schema flat. Decoding is the consumer's responsibility.

CREATE TABLE IF NOT EXISTS new_kms_context (
    context_id BYTEA NOT NULL,
    previous_context_id BYTEA NOT NULL,
    kms_node_params BYTEA NOT NULL,
    thresholds BYTEA NOT NULL,
    software_version TEXT NOT NULL,
    pcr_values BYTEA NOT NULL,
    tx_hash BYTEA,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    already_sent BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (context_id)
);

CREATE TABLE IF NOT EXISTS new_kms_epoch (
    context_id BYTEA NOT NULL,
    previous_context_id BYTEA NOT NULL,
    epoch_id BYTEA NOT NULL,
    previous_epoch_id BYTEA NOT NULL,
    keys BYTEA NOT NULL,
    crs_list BYTEA NOT NULL,
    tx_hash BYTEA,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    already_sent BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (epoch_id)
);

CREATE OR REPLACE FUNCTION refresh_updated_at_new_kms_context()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_new_kms_context_on_update
BEFORE UPDATE ON new_kms_context
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_new_kms_context();

CREATE OR REPLACE FUNCTION refresh_updated_at_new_kms_epoch()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_new_kms_epoch_on_update
BEFORE UPDATE ON new_kms_epoch
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_new_kms_epoch();

-- Notify kms-worker that new request rows have landed.
CREATE OR REPLACE FUNCTION notify_new_kms_context()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY new_kms_context_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_new_kms_context_insertions
AFTER INSERT ON new_kms_context
FOR EACH STATEMENT
EXECUTE FUNCTION notify_new_kms_context();

CREATE OR REPLACE FUNCTION notify_new_kms_epoch()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY new_kms_epoch_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_new_kms_epoch_insertions
AFTER INSERT ON new_kms_epoch
FOR EACH STATEMENT
EXECUTE FUNCTION notify_new_kms_epoch();
