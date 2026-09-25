-- The MMR nodes of height 1 and above of each encrypted state, written by the Solana host
-- listener in the transaction that appends the leaves completing them. A leaf-proof
-- read takes its path from here by position; height-0 siblings are leaf rows. Node
-- (height, node_index) covers leaves [node_index << height, (node_index + 1) << height).
CREATE TABLE solana_encrypted_state_nodes (
    encrypted_state BYTEA NOT NULL
        REFERENCES solana_encrypted_states (encrypted_state),
    height SMALLINT NOT NULL CHECK (height BETWEEN 1 AND 63),
    node_index BIGINT NOT NULL CHECK (node_index >= 0),
    node BYTEA NOT NULL CHECK (octet_length(node) = 32),
    PRIMARY KEY (encrypted_state, height, node_index)
);
