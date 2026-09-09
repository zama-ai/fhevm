-- Preserve the checksum of the already-applied RFC035 migration. State history
-- uses the same leaf representation, but no longer stores a single current slot.
-- Existing leaf commitments retain their original account addresses: this schema
-- migration does not turn an old account's authorization into State authorization.
ALTER TABLE solana_encrypted_value_accounts RENAME TO solana_encrypted_states;
ALTER TABLE solana_encrypted_states RENAME COLUMN encrypted_value_account TO encrypted_state;
ALTER TABLE solana_encrypted_states
    DROP COLUMN program,
    DROP COLUMN encrypted_value_account_authority,
    DROP COLUMN scope,
    DROP COLUMN label,
    DROP COLUMN current_handle,
    ADD CHECK (leaf_count >= 0),
    ADD CHECK (last_slot >= 0);

ALTER TABLE solana_encrypted_value_leaves RENAME TO solana_encrypted_state_leaves;
ALTER TABLE solana_encrypted_state_leaves RENAME COLUMN encrypted_value_account TO encrypted_state;
ALTER TABLE solana_encrypted_state_leaves
    ADD CHECK (leaf_index >= 0),
    ADD CHECK (block_slot >= 0),
    ADD CHECK (transaction_index >= 0);
ALTER INDEX solana_encrypted_value_leaves_semantic_idx
    RENAME TO solana_encrypted_state_leaves_semantic_idx;
ALTER TABLE solana_listener_checkpoint ADD CHECK (slot >= 0);
