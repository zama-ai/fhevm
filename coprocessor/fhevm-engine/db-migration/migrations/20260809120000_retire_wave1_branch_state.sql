-- Retire the wave-1 branch-context (block-scoped materialisation) state.
--
-- Wave 2 (block-scoped execution and value canonicalisation) has been
-- abandoned; the branch tables were its staging area and have no remaining
-- readers. Reorg handling returns to pre-#2848 semantics: blocks are marked
-- orphaned in host_chain_blocks_valid and orphaned work/ACL rows are left in
-- place. Handles are fork-scoped by construction (RFC 010/014: the parent
-- hash and timestamp are in every handle preimage), so orphaned state is
-- unreferenced on the canonical branch and benign.
--
-- NOTE: pre-retirement listener binaries wait on this schema at startup
-- (`wait_for_branch_schema` probes computations_branch, ciphertexts_branch
-- and coprocessor_settlement). Apply this migration only once no such binary
-- can be (re)started against this database; rolling back the binary after
-- this migration requires restoring the tables first.

-- Legacy-table mirror triggers (digest/allow dual-write plumbing).
DROP TRIGGER IF EXISTS mirror_allowed_handles_branchless_trigger ON allowed_handles;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_trigger ON ciphertext_digest;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_ins ON ciphertext_digest;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_upd ON ciphertext_digest;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_branchless_del ON ciphertext_digest;
DROP TRIGGER IF EXISTS mirror_ciphertext_digest_pbs_context_trigger ON pbs_computations_branch;
DROP FUNCTION IF EXISTS mirror_allowed_handles_branchless();
DROP FUNCTION IF EXISTS mirror_ciphertext_digest_branchless();
DROP FUNCTION IF EXISTS mirror_ciphertext_digest_for_pbs_context();
DROP FUNCTION IF EXISTS upsert_ciphertext_digest_branch_from_legacy(ciphertext_digest, BYTEA, BIGINT, BYTEA);

-- Branch-scoped shadow tables (indexes and their own triggers drop with them).
DROP TABLE IF EXISTS computations_branch;
DROP TABLE IF EXISTS pbs_computations_branch;
DROP TABLE IF EXISTS allowed_handles_branch;
DROP TABLE IF EXISTS ciphertext_digest_branch;
DROP TABLE IF EXISTS ciphertexts_branch;
DROP TABLE IF EXISTS ciphertexts128_branch;

-- Wave-2 settlement guard scaffolding (never read by any runtime query).
DROP TABLE IF EXISTS coprocessor_settlement;
