-- these index were unused during auction simulation on devnet
DROP INDEX IF EXISTS computations_dependencies_index;
DROP INDEX IF EXISTS idx_ciphertext_digest_txn_block_number;
DROP INDEX IF EXISTS idx_allowed_handles_txn_block_number;
DROP INDEX IF EXISTS idx_allowed_handles_handle;
DROP INDEX IF EXISTS idx_ciphertexts_tenant_handle;
