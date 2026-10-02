-- The leaf record moved to the Solana Merkle proof service, which rebuilds it in its own
-- database from the zama-host deployment slot. The listener keeps only its checkpoint.
DROP TABLE solana_encrypted_state_nodes;
DROP TABLE solana_encrypted_state_leaves;
DROP TABLE solana_encrypted_states;
