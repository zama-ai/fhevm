-- Latch: the GCS gw-listener has aligned its persisted scan watermark to this
-- proposal's gw_start_block, so the dry-run window start is guaranteed to be
-- scanned (issue #2115). Set by the listener in the same transaction as the
-- watermark rewind (or on first sight of the window when no rewind is needed);
-- it prevents a second rewind for the same proposal - e.g. after a mid-window
-- listener restart - from discarding scan progress. A missing
-- gcs.gw_listener_last_block row (fresh gcs schema) still rewinds regardless of
-- the latch, because without a watermark the latch vouches for nothing.
-- Reset by rollback_dry_run alongside the other latches; a new proposal's rows
-- start at the default.
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS gw_window_rewound BOOLEAN NOT NULL DEFAULT FALSE;
