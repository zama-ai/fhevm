-- Down for 20251104142103_add_last_block_polled.
--
-- Drops the `last_block_polled` table and the `event_type` enum. The polling
-- positions are lost; the previous version does not persist them.

DROP TABLE last_block_polled;
DROP TYPE event_type;
