-- Host chain the in-flight upgrade belongs to.
--
-- The GCS tfhe-/sns-worker block-gated fallback into the live
-- `public.ciphertexts` must scope its `public.computations` guard to this
-- upgrade's host chain, otherwise a same-block computation on a *different*
-- chain could spuriously suppress a legitimate pre-snapshot row. The
-- upgrade-controller populates it from the `event_upgrade_activated` payload's
-- `chain_id` alongside `start_block`/`gw_start_block`.
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS host_chain_id BIGINT NULL CHECK (host_chain_id >= 0);
