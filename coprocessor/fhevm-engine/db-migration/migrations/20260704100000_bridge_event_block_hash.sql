-- Reorg-aware keys for the confidential-bridge event tables.
--
-- These changes originally shipped as an in-place edit of the already-applied
-- 20260616120000_bridge_tables.sql migration, which sqlx rejects with a
-- checksum (VersionMismatch) error on every database that ran the original,
-- aborting the whole migration run. This follow-up migration carries the same
-- delta; 20260616120000 is restored to its originally-applied content.
--
-- bridge_handle_events: UNIQUE (src_handle, dst_chain_id, block_hash) — a row
-- is the approval "src_handle may be bridged to dst_chain" as observed in one
-- source-chain block. The block hash is part of the key so an orphaned
-- observation cannot mask a later canonical one at the same height.
--
-- handle_bridged_events: UNIQUE (dst_handle, block_hash) — the destination
-- handle identifies the bridged ciphertext, and block_hash keeps competing
-- branch observations distinct until orphan filtering/cleanup resolves them.
--
-- Pre-existing rows (written before block hashes were recorded) get the ''
-- sentinel, which the readers treat as "observed without branch context".

ALTER TABLE bridge_handle_events
    ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE bridge_handle_events
    DROP CONSTRAINT IF EXISTS bridge_handle_events_src_handle_dst_chain_id_key;

CREATE UNIQUE INDEX IF NOT EXISTS idx_bridge_handle_events_src_dst_block_hash
    ON bridge_handle_events (src_handle, dst_chain_id, block_hash);

ALTER TABLE handle_bridged_events
    ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE handle_bridged_events
    DROP CONSTRAINT IF EXISTS handle_bridged_events_dst_handle_key;

CREATE UNIQUE INDEX IF NOT EXISTS idx_handle_bridged_events_dst_block_hash
    ON handle_bridged_events (dst_handle, block_hash);
