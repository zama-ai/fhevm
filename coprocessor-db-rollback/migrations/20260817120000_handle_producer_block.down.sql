-- Down for 20260817120000_handle_producer_block.
-- Drops the table. The producer block of each handle is lost.

DROP INDEX IF EXISTS idx_handle_producer_block_block;
DROP TABLE IF EXISTS handle_producer_block;
