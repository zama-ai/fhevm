-- MMR inclusion proof for historical/public confidential-balance decrypts (RFC-021 P2).
-- All three columns are nullable and the write path stores NULL on EVM rows and on Solana
-- current-ACL rows (all-zero value key + empty proof + zero slot). NULL is the "no lineage"
-- sentinel: `WHERE solana_acl_value_key IS NULL` selects current-ACL rows. The read path
-- reconstructs an all-zero value key from NULL, so an MMR-proof row always has all three set.
-- proof_slot is the lineage leaf_count the proof was built against (a u64 on-chain; stored as
-- BIGINT/i64 via a checked conversion on the write path so an overflow fails loudly).
ALTER TABLE user_decryption_requests ADD COLUMN solana_acl_value_key BYTEA;
ALTER TABLE user_decryption_requests ADD COLUMN solana_mmr_proof BYTEA;
ALTER TABLE user_decryption_requests ADD COLUMN solana_proof_slot BIGINT;
