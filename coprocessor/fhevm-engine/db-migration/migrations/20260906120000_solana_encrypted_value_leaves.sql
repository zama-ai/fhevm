-- The RFC 035 leaf record of Solana encrypted value accounts, kept by the Solana host
-- listener next to the compute rows it derives from the same confirmed blocks, in the
-- same transaction. The KMS connector reads inclusion proofs from it and verifies them
-- against the on-chain account's peaks; the record itself is never trusted for
-- authorization.

CREATE TABLE solana_encrypted_value_accounts (
    encrypted_value_account BYTEA PRIMARY KEY
        CHECK (octet_length(encrypted_value_account) = 32),
    program BYTEA NOT NULL CHECK (octet_length(program) = 32),
    encrypted_value_account_authority BYTEA NOT NULL
        CHECK (octet_length(encrypted_value_account_authority) = 32),
    scope BYTEA NOT NULL CHECK (octet_length(scope) = 32),
    label BYTEA NOT NULL CHECK (octet_length(label) = 32),
    current_handle BYTEA NOT NULL CHECK (octet_length(current_handle) = 32),
    leaf_count BIGINT NOT NULL,
    peaks BYTEA[] NOT NULL,
    -- False when the account was first seen through an update: leaves sealed before
    -- this record started are unknown, so none of its leaves are stored and no proof
    -- can be served for it.
    history_complete BOOLEAN NOT NULL,
    last_slot BIGINT NOT NULL
);

-- leaf_kind: 0 = historical-access leaf (ZAMA_HIST_ACCESS_LEAF_V1, keyed by handle + allowed_key),
--            1 = public-decrypt leaf   (ZAMA_PUBLIC_DECRYPT_LEAF_V1, keyed by handle, allowed_key NULL).
CREATE TABLE solana_encrypted_value_leaves (
    encrypted_value_account BYTEA NOT NULL
        REFERENCES solana_encrypted_value_accounts (encrypted_value_account),
    leaf_index BIGINT NOT NULL,
    commitment BYTEA NOT NULL CHECK (octet_length(commitment) = 32),
    leaf_kind SMALLINT NOT NULL CHECK (leaf_kind IN (0, 1)),
    handle BYTEA NOT NULL CHECK (octet_length(handle) = 32),
    allowed_key BYTEA CHECK (allowed_key IS NULL OR octet_length(allowed_key) = 32),
    block_slot BIGINT NOT NULL,
    transaction_index BIGINT NOT NULL,
    PRIMARY KEY (encrypted_value_account, leaf_index)
);

-- One indexed lookup per proof request: (account, kind, handle, allowed_key).
CREATE INDEX solana_encrypted_value_leaves_semantic_idx
    ON solana_encrypted_value_leaves (encrypted_value_account, leaf_kind, handle, allowed_key);

-- The last sealed block the listener applied, written in that block's transaction so a
-- restart resumes exactly after the recorded work (Yellowstone replays inclusively from it).
CREATE TABLE solana_listener_checkpoint (
    singleton SMALLINT PRIMARY KEY DEFAULT 1 CHECK (singleton = 1),
    slot BIGINT NOT NULL,
    block_hash BYTEA NOT NULL CHECK (octet_length(block_hash) = 32),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
