-- Two-track cutover gating for the GCS blue-green upgrade.
--
-- consensus-detector now emits `event_unanimity_consensus` for BOTH the host
-- chain (payload chain_id = host chain id) and the Gateway (payload chain_id =
-- gw chain id, over the re-randomized input ciphertexts). Cutover must wait
-- until unanimity has been observed on BOTH tracks — either alone is
-- insufficient, since host-chain agreement says nothing about whether the
-- Gateway input ciphertexts match across operators (and vice versa).
--
--   host_chain_id           : the host chain this GCS upgrade belongs to, copied
--                             from the activation payload. Lets
--                             handle_unanimity_consensus classify each event as
--                             host-track (chain_id == host_chain_id) or
--                             Gateway-track (anything else).
--   host_consensus_reached  : set once the host-chain unanimity event (with
--                             block_height within [start_block, end_block]) has
--                             been observed.
--   gw_consensus_reached    : set once a Gateway unanimity event has been
--                             observed.
--
-- Both latches are reset to FALSE on each (re)activation of an upgrade window
-- (see upgrade-controller `handle_upgrade_activated`).
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS host_chain_id          BIGINT  NULL,
    ADD COLUMN IF NOT EXISTS host_consensus_reached BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS gw_consensus_reached   BOOLEAN NOT NULL DEFAULT FALSE;
