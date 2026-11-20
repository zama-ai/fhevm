-- Down for 20260928120000_blocks_valid_pending_index.
--
-- This migration belongs to v0.14.x as well, but its version is above the
-- v0.15.x range, so a rollback to 20260704130000 reverts it. Drops the index;
-- no data impact. The v0.14.x db-migration image applies this migration again
-- on its next start, which recreates the index.

DROP INDEX IF EXISTS idx_host_chain_blocks_valid_pending;
