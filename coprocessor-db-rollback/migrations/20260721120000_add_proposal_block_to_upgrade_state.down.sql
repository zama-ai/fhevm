-- Down for 20260721120000_add_proposal_block_to_upgrade_state.
-- Drops the column. The index on it (20260725120000) is already gone.

ALTER TABLE upgrade_state
    DROP COLUMN IF EXISTS proposal_block;
