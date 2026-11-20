-- Down for 20260708120000_add_consensus_latches_to_upgrade_state.
-- Drops the three columns. The per-chain rows and the primary key on
-- (stack_role, host_chain_id) are already gone: 20260725120000 is reverted
-- before this file runs.

ALTER TABLE upgrade_state
    DROP COLUMN IF EXISTS gw_consensus_reached,
    DROP COLUMN IF EXISTS host_consensus_reached,
    DROP COLUMN IF EXISTS host_chain_id;
