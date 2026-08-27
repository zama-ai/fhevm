-- Records the transaction hash of the synthetic FHE work the GCS (green) host-listener injects
-- into its dry-run window, one per host chain.
--
-- Why it has to be stored rather than recomputed: the hash is keccak over
-- (proposal_id, target_version, chain_id, block_number, block_hash). Cutover can read the first
-- four back out of this table, but `block_hash` is not persisted anywhere it can reach
-- unambiguously -- `host_chain_blocks_valid` holds one row per fork at a height, so a lookup
-- there is only unique after finalization. The injector is the only component that knows the
-- value, so it writes it down.
--
-- Used at cutover to delete the synthetic computations, their ciphertexts and their dependence
-- chains before `gcs.*` merges into `public`. Without it, synthetic handles become live
-- production rows and the newly-live green transaction-sender tries to publish ciphertext
-- digests for handles that exist on no chain.
--
-- Nullable: BCS rows never carry one, and a GCS row has none until its window reaches
-- `start_block + 1`. Cleared on rollback, because a re-attempt lands on a different block hash
-- and therefore derives a different transaction hash.
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS synthetic_txn_hash BYTEA NULL;
