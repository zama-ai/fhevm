-- Service-owned Solana proof store (fhevm-internal #1682 / RFC-024).
-- Typed columns only: no event JSON, MMR node table, rollback journal, or leases.

CREATE TABLE solana_proof_progress (
    singleton SMALLINT PRIMARY KEY DEFAULT 1 CHECK (singleton = 1),
    history_complete BOOLEAN NOT NULL DEFAULT FALSE,
    history_start_slot BIGINT,
    history_start_block_hash BYTEA
        CHECK (
            history_start_block_hash IS NULL
            OR octet_length(history_start_block_hash) = 32
        ),
    checkpoint_slot BIGINT,
    checkpoint_block_hash BYTEA
        CHECK (
            checkpoint_block_hash IS NULL
            OR octet_length(checkpoint_block_hash) = 32
        ),
    integrity_halted BOOLEAN NOT NULL DEFAULT FALSE,
    integrity_halt_reason TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO solana_proof_progress (singleton) VALUES (1);

CREATE TABLE solana_proof_blocks (
    slot BIGINT PRIMARY KEY,
    block_hash BYTEA NOT NULL CHECK (octet_length(block_hash) = 32),
    parent_slot BIGINT NOT NULL,
    parent_hash BYTEA NOT NULL CHECK (octet_length(parent_hash) = 32),
    block_time BIGINT,
    block_height BIGINT,
    executed_transaction_count BIGINT NOT NULL
);

CREATE TABLE solana_proof_transactions (
    block_slot BIGINT NOT NULL REFERENCES solana_proof_blocks (slot),
    transaction_index BIGINT NOT NULL,
    signature BYTEA NOT NULL CHECK (octet_length(signature) = 64),
    succeeded BOOLEAN NOT NULL,
    is_vote BOOLEAN NOT NULL,
    PRIMARY KEY (block_slot, transaction_index),
    CONSTRAINT solana_proof_transactions_signature_key UNIQUE (signature)
);

CREATE TABLE solana_proof_encrypted_value_accounts (
    encrypted_value_account BYTEA PRIMARY KEY CHECK (octet_length(encrypted_value_account) = 32),
    current_handle BYTEA
        CHECK (current_handle IS NULL OR octet_length(current_handle) = 32),
    subjects BYTEA[] NOT NULL DEFAULT '{}',
    leaf_count BIGINT NOT NULL DEFAULT 0,
    peaks BYTEA[] NOT NULL DEFAULT '{}',
    last_slot BIGINT NOT NULL
);

CREATE TABLE solana_proof_leaves (
    encrypted_value_account BYTEA NOT NULL REFERENCES solana_proof_encrypted_value_accounts (encrypted_value_account),
    leaf_index BIGINT NOT NULL,
    commitment BYTEA NOT NULL CHECK (octet_length(commitment) = 32),
    block_slot BIGINT NOT NULL,
    transaction_index BIGINT NOT NULL,
    PRIMARY KEY (encrypted_value_account, leaf_index)
);

CREATE INDEX solana_proof_leaves_block_slot_idx
    ON solana_proof_leaves (block_slot, transaction_index);
