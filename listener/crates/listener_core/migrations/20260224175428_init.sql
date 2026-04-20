-- Create enum type for block status
CREATE TYPE block_status AS ENUM ('CANONICAL', 'FINALIZED', 'UNCLE');

-- Create blocks table
CREATE TABLE IF NOT EXISTS blocks (
    id UUID PRIMARY KEY,
    chain_id BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL,
    parent_hash BYTEA NOT NULL,
    status block_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_blocks_chain_block_hash UNIQUE (chain_id, block_hash)
);

-- Indices for common query patterns (all include chain_id as first column)
CREATE INDEX IF NOT EXISTS idx_blocks_chain_created_at ON blocks(chain_id, created_at);
CREATE INDEX IF NOT EXISTS idx_blocks_chain_number_status ON blocks(chain_id, block_number, status);
CREATE INDEX IF NOT EXISTS idx_blocks_chain_number_created ON blocks(chain_id, block_number DESC, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_blocks_chain_status_number_desc ON blocks(chain_id, status, block_number DESC);

-- Enforce exactly one CANONICAL block per (chain_id, block_number).
-- This is critical for the cursor algorithm integrity:
-- get_latest_canonical_block() must return a single unambiguous tip.
-- Other statuses (UNCLE, FINALIZED) are not constrained — multiple can coexist at the same height.
CREATE UNIQUE INDEX IF NOT EXISTS idx_blocks_unique_canonical_per_number
    ON blocks(chain_id, block_number) WHERE status = 'CANONICAL';

-- Trigger function for auto-updating updated_at
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create filters table
CREATE TABLE IF NOT EXISTS filters (
    id UUID PRIMARY KEY,
    chain_id BIGINT NOT NULL,
    consumer_id VARCHAR(128) NOT NULL,
    "from" VARCHAR(42),
    "to" VARCHAR(42),
    "log_address" VARCHAR(42),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Unique index: one filter per (chain, consumer, from, to, log_address) combination.
-- COALESCE maps NULL → '' so that NULLs are treated as equal for uniqueness
-- (compatible with Postgres versions before 15 which lack NULLS NOT DISTINCT).
CREATE UNIQUE INDEX IF NOT EXISTS idx_filters_unique_chain_consumer_from_to
    ON filters(chain_id, consumer_id, COALESCE("from", ''), COALESCE("to", ''), COALESCE("log_address", ''));

-- Index for fast retrieval of all filters for a given chain_id.
CREATE INDEX IF NOT EXISTS idx_filters_chain_id ON filters(chain_id);
