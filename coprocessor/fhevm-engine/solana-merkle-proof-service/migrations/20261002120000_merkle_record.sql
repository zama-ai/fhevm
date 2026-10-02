-- The RFC 035 leaf record of Solana encrypted stores, rebuilt by the Merkle indexer from
-- confirmed blocks since the zama-host deployment. The KMS connector reads inclusion proofs
-- from it and verifies them against the on-chain store's peaks; the record itself is never
-- trusted for authorization.

-- One row per encrypted store the record follows: its MMR cursor after `last_slot`.
CREATE TABLE encrypted_stores (
    encrypted_store BYTEA PRIMARY KEY CHECK (octet_length(encrypted_store) = 32),
    leaf_count BIGINT NOT NULL CHECK (leaf_count >= 0),
    peaks BYTEA[] NOT NULL,
    last_slot BIGINT NOT NULL CHECK (last_slot >= 0)
);

-- leaf_kind: 0 = historical-access leaf (ZAMA_HIST_ACCESS_LEAF_V1, keyed by handle + allowed_key),
--            1 = public-decrypt leaf   (ZAMA_PUBLIC_DECRYPT_LEAF_V1, keyed by handle, allowed_key NULL).
CREATE TABLE leaves (
    encrypted_store BYTEA NOT NULL REFERENCES encrypted_stores (encrypted_store),
    leaf_index BIGINT NOT NULL CHECK (leaf_index >= 0),
    commitment BYTEA NOT NULL CHECK (octet_length(commitment) = 32),
    leaf_kind SMALLINT NOT NULL CHECK (leaf_kind IN (0, 1)),
    handle BYTEA NOT NULL CHECK (octet_length(handle) = 32),
    allowed_key BYTEA CHECK (allowed_key IS NULL OR octet_length(allowed_key) = 32),
    block_slot BIGINT NOT NULL CHECK (block_slot >= 0),
    transaction_index BIGINT NOT NULL CHECK (transaction_index >= 0),
    PRIMARY KEY (encrypted_store, leaf_index)
);

-- A proof serves the first leaf matching (store, kind, handle, allowed_key), so the lookup
-- index ends with the leaf's position: repeated allows of one handle cost one index probe.
CREATE INDEX leaves_semantic_idx
    ON leaves (encrypted_store, leaf_kind, handle, allowed_key, leaf_index);

-- The MMR nodes of height 1 and above, written in the transaction that appends the leaves
-- completing them. A proof takes its path from here by position; height-0 siblings are leaf
-- rows. Node (height, node_index) covers leaves [node_index << height, (node_index + 1) << height).
CREATE TABLE nodes (
    encrypted_store BYTEA NOT NULL REFERENCES encrypted_stores (encrypted_store),
    height SMALLINT NOT NULL CHECK (height BETWEEN 1 AND 63),
    node_index BIGINT NOT NULL CHECK (node_index >= 0),
    node BYTEA NOT NULL CHECK (octet_length(node) = 32),
    PRIMARY KEY (encrypted_store, height, node_index)
);

-- The last sealed block the indexer applied, written in that block's transaction so a restart
-- resumes exactly after the recorded work.
CREATE TABLE checkpoint (
    singleton SMALLINT PRIMARY KEY DEFAULT 1 CHECK (singleton = 1),
    slot BIGINT NOT NULL CHECK (slot >= 0),
    block_hash BYTEA NOT NULL CHECK (octet_length(block_hash) = 32),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
