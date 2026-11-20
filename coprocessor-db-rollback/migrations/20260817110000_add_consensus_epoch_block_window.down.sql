-- Down for 20260817110000_add_consensus_epoch_block_window.
-- Drops the table. The block windows of every epoch are lost. The column
-- 20260930120100 added and the function 20260930120000 created on top of this
-- table are already gone when this file runs.

DROP TABLE IF EXISTS consensus_epoch_block_window;
