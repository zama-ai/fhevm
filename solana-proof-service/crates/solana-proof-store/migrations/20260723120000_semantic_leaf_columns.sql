-- Semantic leaf resolution columns (fhevm-internal #1721).
--
-- The proof service resolves (encrypted_value_account, handle[, subject], kind) -> leaf_index for the
-- access-proof / public-proof endpoints instead of taking a mechanical leaf_index from the
-- caller. Ingest populates these for every appended leaf (see StagedLeaf); they are nullable
-- only so this additive migration applies cleanly to an already-populated leaf table. Leaves
-- written by this build always carry them, and the resolution queries only ever match rows
-- that do.
--
-- leaf_kind: 0 = historical-access leaf (ZAMA_HIST_ACCESS_LEAF_V1, keyed by handle + subject),
--            1 = public-decrypt leaf   (ZAMA_PUBLIC_DECRYPT_LEAF_V1, keyed by handle, subject NULL).
--
-- OPERATIONAL RULE (stale-store trap): a store populated BEFORE this migration must be rebuilt
-- from genesis (drop + re-ingest from slot 0). Pre-migration leaf rows carry NULL semantics but
-- still count in solana_proof_encrypted_value_accounts.leaf_count, so at parity with chain a semantic query for a
-- leaf that genuinely exists on chain would resolve to nothing and serve a terminal 404
-- (leaf_not_found) instead of a proof. Ingest by this build always populates the columns; startup
-- validation (has_pre_semantic_leaf_rows) fails closed if any NULL-semantic row is found in a
-- nonempty store. The columns are nullable only so this ALTER applies without a table rewrite.

ALTER TABLE solana_proof_leaves
    ADD COLUMN leaf_kind SMALLINT
        CHECK (leaf_kind IS NULL OR leaf_kind IN (0, 1)),
    ADD COLUMN handle BYTEA
        CHECK (handle IS NULL OR octet_length(handle) = 32),
    ADD COLUMN subject BYTEA
        CHECK (subject IS NULL OR octet_length(subject) = 32);

-- One indexed lookup per semantic proof request: access leaves match on
-- (encrypted_value_account, leaf_kind, handle, subject); public-decrypt leaves match on
-- (encrypted_value_account, leaf_kind, handle) with subject NULL.
CREATE INDEX solana_proof_leaves_semantic_idx
    ON solana_proof_leaves (encrypted_value_account, leaf_kind, handle, subject);
