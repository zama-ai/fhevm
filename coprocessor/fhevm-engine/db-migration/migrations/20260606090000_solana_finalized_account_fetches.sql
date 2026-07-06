CREATE TABLE IF NOT EXISTS solana_finalized_account_fetches (
    account_key BYTEA NOT NULL CHECK (octet_length(account_key) = 32),
    kind SMALLINT NOT NULL CHECK (kind BETWEEN 1 AND 9),
    reason TEXT NOT NULL,
    handle BYTEA NULL CHECK (handle IS NULL OR octet_length(handle) = 32),
    handle_key BYTEA NOT NULL CHECK (octet_length(handle_key) = 33),
    related_account BYTEA NULL CHECK (related_account IS NULL OR octet_length(related_account) = 32),
    subject BYTEA NULL CHECK (subject IS NULL OR octet_length(subject) = 32),
    transaction_id BYTEA NOT NULL CHECK (octet_length(transaction_id) = 32),
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    status TEXT NOT NULL DEFAULT 'pending',
    attempts INT NOT NULL DEFAULT 0,
    last_error TEXT NULL,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (account_key, kind, reason, handle_key)
);

CREATE INDEX IF NOT EXISTS idx_solana_finalized_account_fetches_pending
ON solana_finalized_account_fetches (status, block_number);

CREATE TABLE IF NOT EXISTS solana_finalized_account_witnesses (
    account_key BYTEA PRIMARY KEY CHECK (octet_length(account_key) = 32),
    owner BYTEA NOT NULL CHECK (octet_length(owner) = 32),
    lamports BIGINT NOT NULL CHECK (lamports >= 0),
    executable BOOLEAN NOT NULL,
    data BYTEA NOT NULL,
    observed_slot BIGINT NOT NULL CHECK (observed_slot >= 0),
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
