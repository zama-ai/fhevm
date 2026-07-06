-- Durable per-block observations of FallbackGrantedPlaintext events.
--
-- Fallback grants were previously fire-and-forget: ingestion synthesized a
-- TrivialEncrypt once and discarded the event when the handle already looked
-- materialized. That dedup reads reorg-unstable state — a grant observed on a
-- fork suppressed the same grant's canonical re-inclusion, and once the fork's
-- state was cleaned up the grant was lost forever (the listener never revisits
-- a processed block). Recording every observation keyed by block hash gives
-- reorg cleanup and operators a durable source of truth, mirroring
-- handle_bridged_events.
CREATE TABLE IF NOT EXISTS fallback_granted_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    dst_chain_id BIGINT NOT NULL,
    dst_handle BYTEA NOT NULL,
    plaintext BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL DEFAULT ''::BYTEA,
    transaction_id BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (dst_handle, block_hash)
);

CREATE INDEX IF NOT EXISTS idx_fallback_granted_events_chain_block
    ON fallback_granted_events (dst_chain_id, block_number);
