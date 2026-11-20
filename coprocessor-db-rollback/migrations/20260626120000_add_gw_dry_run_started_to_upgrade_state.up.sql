-- Gateway-side dry-run gate for the GCS zkproof-worker.
--
-- The host-chain `start_block` gate (state = 'DryRunStarted') releases the
-- tfhe-/sns-workers once BCS has settled the host-chain snapshot window. The
-- zkproof-worker, however, must switch to the new re-randomization strategy at
-- the same *Gateway* block (`gw_start_block`) across all operators — otherwise
-- the re-randomized input ciphertexts differ and the cross-operator
-- stateCommitment never reaches unanimity.
--
-- `gw_dry_run_started` is the durable marker the upgrade-controller sets once
-- the GCS gw-listener has reached `gw_start_block` and the pre-start rows have
-- been pruned from `gcs.verify_proofs`. The zkproof-worker's activation watcher
-- polls it (and wakes on `event_gw_dry_run_started`) to release the worker.
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS gw_dry_run_started BOOLEAN NOT NULL DEFAULT FALSE;
