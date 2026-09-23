-- ── Watcher filter types ────────────────────────────────────────────────────

-- Watcher type: LIVE (head-of-chain) vs FINAL (finalized-only, delivery flow added later).
CREATE TYPE filter_type AS ENUM ('LIVE', 'FINAL');

-- Existing rows and legacy writers default to LIVE.
ALTER TABLE filters
    ADD COLUMN filter_type filter_type NOT NULL DEFAULT 'LIVE';

-- Replace uniqueness so the same (chain, consumer, addresses) combination may
-- exist once per watcher type. COALESCE keeps NULL = '' semantics
-- (compatible with Postgres versions before 15 which lack NULLS NOT DISTINCT).
DROP INDEX IF EXISTS idx_filters_unique_chain_consumer_from_to;
CREATE UNIQUE INDEX idx_filters_unique_chain_consumer_from_to_type
    ON filters(chain_id, consumer_id, COALESCE("from", ''), COALESCE("to", ''), COALESCE("log_address", ''), filter_type);

-- Hot-path publish queries select one watcher type at a time; partial indexes
-- match the predicate exactly and keep each btree small. (chain_id, consumer_id)
-- also serves the per-consumer catchup lookup and the ORDER BY consumer_id.
CREATE INDEX idx_filters_chain_consumer_live
    ON filters(chain_id, consumer_id) WHERE filter_type = 'LIVE';
CREATE INDEX idx_filters_chain_consumer_final
    ON filters(chain_id, consumer_id) WHERE filter_type = 'FINAL';

-- Redundant: every remaining query carries a filter_type predicate (covered by
-- the partial indexes), and the unique index above is (chain_id, ...)-leftmost.
DROP INDEX IF EXISTS idx_filters_chain_id;

-- ── Final blocks tip table ──────────────────────────────────────────────────

-- Finalized blocks never reorg, so there is no block_status dimension and
-- exactly one final block per (chain_id, block_number).
CREATE TABLE IF NOT EXISTS final_blocks (
    id UUID PRIMARY KEY,
    chain_id BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL,
    parent_hash BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_final_blocks_chain_block_hash UNIQUE (chain_id, block_hash)
);

-- One final block per height per chain. This btree also serves every read path:
-- latest-final (ORDER BY block_number DESC LIMIT 1), MIN(block_number), and the
-- ranked scan of the retention cleanup.
CREATE UNIQUE INDEX IF NOT EXISTS idx_final_blocks_unique_chain_number
    ON final_blocks(chain_id, block_number);
