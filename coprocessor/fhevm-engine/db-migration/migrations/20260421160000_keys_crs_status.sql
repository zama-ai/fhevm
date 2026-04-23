-- Add pending/active/orphaned lifecycle to keys and crs, tied to host-chain
-- block finality. host-listener inserts rows as 'pending' at live ingest and
-- promotes them to 'active' (or 'orphaned') when the source block reaches
-- finality. Existing rows are implicitly 'active' for backward compatibility.

ALTER TABLE keys DROP CONSTRAINT unique_key_id_gw;
ALTER TABLE keys DROP CONSTRAINT unique_key_id;

ALTER TABLE keys
    ADD COLUMN status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('pending', 'active', 'orphaned')),
    ADD COLUMN chain_id BIGINT,
    ADD COLUMN block_hash BYTEA;

-- Only one active row per key_id_gw. key_id remains a placeholder for
-- host-listener materialized rows until the real server-key metadata is
-- extracted, so empty key_id values do not participate in the uniqueness
-- invariant. pending and orphaned rows do not participate either, so reorgs
-- can coexist until the finalizing block flips exactly one of them to
-- 'active'.
CREATE UNIQUE INDEX uniq_keys_active_id_gw
    ON keys (key_id_gw) WHERE status = 'active';
CREATE UNIQUE INDEX uniq_keys_active_id
    ON keys (key_id) WHERE status = 'active' AND key_id <> ''::bytea;

-- Guarantees idempotency of live ingest / catchup replays: the same
-- (chain_id, block_hash, key_id_gw) cannot be inserted twice.
CREATE UNIQUE INDEX uniq_keys_chain_block_id_gw
    ON keys (chain_id, block_hash, key_id_gw);

-- Speeds up the finalize-time status transition.
CREATE INDEX idx_keys_pending_by_block
    ON keys (chain_id, block_hash) WHERE status = 'pending';

ALTER TABLE crs DROP CONSTRAINT unique_crs_id;

ALTER TABLE crs
    ADD COLUMN status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('pending', 'active', 'orphaned')),
    ADD COLUMN chain_id BIGINT,
    ADD COLUMN block_hash BYTEA;

CREATE UNIQUE INDEX uniq_crs_active_id
    ON crs (crs_id) WHERE status = 'active';

CREATE UNIQUE INDEX uniq_crs_chain_block_id
    ON crs (chain_id, block_hash, crs_id);

CREATE INDEX idx_crs_pending_by_block
    ON crs (chain_id, block_hash) WHERE status = 'pending';
