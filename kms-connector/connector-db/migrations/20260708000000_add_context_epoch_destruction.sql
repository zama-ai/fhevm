-- Split the `kms_context` validation cache into context-level and epoch-level tables.
--
-- Otherwise, it was not possible to distinguish a destroyed context from a context with
-- not-cached-yet (context, epoch) pair.
CREATE TABLE IF NOT EXISTS kms_epoch (
    id BYTEA NOT NULL,
    -- NULL for invalidation written from `KmsEpochDestroyed`, which only carries the epoch ID.
    context_id BYTEA,
    is_valid BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (id)
);

-- These tables are only a cache of on-chain `ProtocolConfig` state: instead of backfilling
-- `kms_epoch` and deduplicating `kms_context` from the old pair schema, just empty the cache.
-- The kms-worker repopulates it lazily via its on-chain fallback.
TRUNCATE kms_context;

ALTER TABLE kms_context
    DROP CONSTRAINT kms_context_pkey,
    ADD PRIMARY KEY (id);
ALTER TABLE kms_context DROP COLUMN epoch_id;
-- No default: validity must always be stated explicitly
ALTER TABLE kms_context ALTER COLUMN is_valid DROP DEFAULT;

-- Store `KmsContextDestroyed` and `KmsEpochDestroyed` events emitted by the `ProtocolConfig`
-- contract on Ethereum, so the kms-worker can forward the destruction to the KMS Core
-- (`DestroyMpcContext` / `DestroyMpcEpoch`). These RPCs have no result-polling endpoint, so no
-- response table is needed: the send-side ack is the only signal.
CREATE TABLE IF NOT EXISTS kms_context_destroyed (
    context_id BYTEA NOT NULL,
    tx_hash BYTEA,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    already_sent BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (context_id)
);

CREATE TABLE IF NOT EXISTS kms_epoch_destroyed (
    epoch_id BYTEA NOT NULL,
    tx_hash BYTEA,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    status operation_status NOT NULL DEFAULT 'pending',
    already_sent BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (epoch_id)
);

CREATE OR REPLACE FUNCTION refresh_updated_at_kms_context_destroyed()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_kms_context_destroyed_on_update
BEFORE UPDATE ON kms_context_destroyed
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_kms_context_destroyed();

CREATE OR REPLACE FUNCTION refresh_updated_at_kms_epoch_destroyed()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_kms_epoch_destroyed_on_update
BEFORE UPDATE ON kms_epoch_destroyed
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_kms_epoch_destroyed();

-- Notify kms-worker that new request rows have landed.
CREATE OR REPLACE FUNCTION notify_kms_context_destroyed()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY kms_context_destroyed_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_kms_context_destroyed_insertions
AFTER INSERT ON kms_context_destroyed
FOR EACH STATEMENT
EXECUTE FUNCTION notify_kms_context_destroyed();

CREATE OR REPLACE FUNCTION notify_kms_epoch_destroyed()
RETURNS TRIGGER AS $$
BEGIN
    NOTIFY kms_epoch_destroyed_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_from_kms_epoch_destroyed_insertions
AFTER INSERT ON kms_epoch_destroyed
FOR EACH STATEMENT
EXECUTE FUNCTION notify_kms_epoch_destroyed();
