-- RFC 023 stores one current S3 object per (handle, coprocessor_context_id).
-- Branch coexistence remains in the DB branch tables; this queue records
-- handles whose S3 object must be republished from the surviving canonical
-- branch row after stale or orphaned branch uploads.

CREATE TABLE IF NOT EXISTS s3_canonical_repair_queue
(
    host_chain_id BIGINT NOT NULL,
    handle BYTEA NOT NULL,
    target_producer_block_hash BYTEA NOT NULL,
    target_block_hash BYTEA NOT NULL,
    target_block_number BIGINT NULL,
    reason TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    locked_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (host_chain_id, handle)
);

CREATE INDEX IF NOT EXISTS idx_s3_canonical_repair_queue_target
ON s3_canonical_repair_queue (
    host_chain_id,
    target_block_number,
    target_producer_block_hash,
    target_block_hash
);

CREATE INDEX IF NOT EXISTS idx_s3_canonical_repair_queue_unlocked
ON s3_canonical_repair_queue (updated_at)
WHERE locked_at IS NULL;
