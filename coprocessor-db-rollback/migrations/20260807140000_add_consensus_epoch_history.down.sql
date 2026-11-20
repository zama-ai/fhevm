-- Down for 20260807140000_add_consensus_epoch_history.
-- Drops both tables. The epoch history, including the 'legacy' row the
-- migration inserted, is lost. consensus_epoch_block_window, which references
-- consensus_epoch_history, is already gone when this file runs.

DROP INDEX IF EXISTS uq_consensus_epoch_history_proposal;
DROP TABLE IF EXISTS consensus_epoch_history;
DROP TABLE IF EXISTS blue_green_consensus_epoch;
