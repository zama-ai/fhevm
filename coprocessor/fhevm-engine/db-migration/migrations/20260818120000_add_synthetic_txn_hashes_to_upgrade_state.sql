-- Records the transaction hashes of the synthetic FHE work the GCS (green) host-listener injects
-- into its dry-run window, one row per host chain.
--
-- Why the hashes must be stored rather than recomputed: each is keccak over
-- (proposal_id, target_version, chain_id, block_number, block_hash). Cutover can read the first
-- four back out of this table, but `block_hash` is not persisted anywhere it can reach
-- unambiguously -- `host_chain_blocks_valid` holds one row per fork at a height, so a lookup
-- there is only unique after finalization. The injector is the only component that knows it.
--
-- Why a concatenated BYTEA and not one value: injection happens at `start_block + 1` with no
-- wait for finality, so a reorg makes the listener inject again on the replacement block. The
-- replacement has a different block hash, hence a different transaction hash and a separate set
-- of synthetic rows. A single-value column would forget the first set and let it merge into
-- `public` as live data.
--
-- Layout is N 32-byte hashes back to back, unpacked with
-- `generate_series(1, octet_length(...), 32)`, exactly as `verify_proofs.handles` already is.
--
-- The CHECK enforces alignment in the database, so a malformed value can never become a delete
-- predicate. That matters because every delete keyed on these hashes matches
-- `computations.transaction_id`, which is `BYTEA NOT NULL DEFAULT '\x00'::BYTEA` with legacy rows
-- backfilled to `'\x01'::BYTEA` -- a short or misaligned hash would match real production rows.
--
-- Empty (not NULL) when there is nothing to clean: BCS rows never carry any, and a GCS row has
-- none until its window reaches `start_block + 1`. Reset to empty on rollback and at cutover,
-- because a later attempt derives different hashes.
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS synthetic_txn_hashes BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE upgrade_state
    DROP CONSTRAINT IF EXISTS upgrade_state_synthetic_txn_hashes_aligned;

ALTER TABLE upgrade_state
    ADD CONSTRAINT upgrade_state_synthetic_txn_hashes_aligned
    CHECK (octet_length(synthetic_txn_hashes) % 32 = 0);
