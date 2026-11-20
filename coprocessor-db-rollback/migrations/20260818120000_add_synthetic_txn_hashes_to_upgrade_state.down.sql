-- Down for 20260818120000_add_synthetic_txn_hashes_to_upgrade_state.
-- Drops the constraint and the column. Recorded synthetic transaction hashes
-- are lost.

ALTER TABLE upgrade_state
    DROP CONSTRAINT IF EXISTS upgrade_state_synthetic_txn_hashes_aligned;

ALTER TABLE upgrade_state
    DROP COLUMN IF EXISTS synthetic_txn_hashes;
