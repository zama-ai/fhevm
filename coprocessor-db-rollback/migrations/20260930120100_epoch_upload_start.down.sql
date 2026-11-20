-- Down for 20260930120100_epoch_upload_start.
-- Drops the column. The upload start block of each epoch window is lost.

ALTER TABLE consensus_epoch_block_window
    DROP COLUMN IF EXISTS upload_start_block;
