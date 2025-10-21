CREATE TABLE IF NOT EXISTS delegate_user_decrypt (
    delegator BYTEA NOT NULL,
    delegate BYTEA NOT NULL,
    contract_address BYTEA NOT NULL,
    delegation_counter BIGINT NOT NULL,
    old_expiry_date BIGINT NOT NULL, -- 0 = first time delegation
    expiry_date BIGINT NOT NULL, -- 0 = revoke
    host_chain_id BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL, -- to check finality
    transaction_id BYTEA,
    UNIQUE(delegator, delegate, contract_address, delegation_counter, old_expiry_date, expiry_date, block_number, block_hash, transaction_id)
);