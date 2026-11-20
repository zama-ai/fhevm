-- Multi-chain Blue/Green.
--
-- A proposal is represented directly by one upgrade_state row per host-chain
-- evaluation window. Proposal-level FSM fields are intentionally repeated on
-- those rows; state transitions update the complete proposal atomically.
--
-- Rows created before host_chain_id existed predate multi-chain and can't name a
-- real chain. Drop them rather than stamp a phantom id that would wedge consensus
-- (never times out) and block every future proposal; any live upgrade re-seeds.
DELETE FROM upgrade_state WHERE host_chain_id IS NULL;

ALTER TABLE upgrade_state
    ALTER COLUMN host_chain_id SET NOT NULL;

ALTER TABLE upgrade_state
    DROP CONSTRAINT IF EXISTS upgrade_state_pkey;

ALTER TABLE upgrade_state
    ADD PRIMARY KEY (stack_role, host_chain_id);

CREATE INDEX IF NOT EXISTS upgrade_state_attempt_idx
    ON upgrade_state (stack_role, proposal_id, proposal_block);
