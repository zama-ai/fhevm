CREATE TABLE IF NOT EXISTS delegate_user_decrypt (
    key BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    delegator BYTEA NOT NULL,
    delegate BYTEA NOT NULL,
    contract_address BYTEA NOT NULL,
    delegation_counter BIGINT NOT NULL,
    old_expiration_date NUMERIC NOT NULL, -- 0 = first time delegation
    new_expiration_date NUMERIC NOT NULL, -- 0 = revoke
    host_chain_id BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash BYTEA NOT NULL, -- to check finality
    transaction_id BYTEA,
    on_gateway BOOL NOT NULL, -- if it is on gateway chain
    reorg_out BOOL NOT NULL, -- if it was reorged out
    -- error and retry handling
    gateway_nb_attempts BIGINT NOT NULL DEFAULT 0,
    gateway_last_error TEXT,
    UNIQUE(delegator, delegate, contract_address, delegation_counter, old_expiration_date, new_expiration_date, block_number, block_hash, transaction_id)
);

CREATE INDEX IF NOT EXISTS idx_delegate_user_decrypt_block_number
    ON delegate_user_decrypt (block_number); -- for delay and clean

CREATE INDEX IF NOT EXISTS idx_delegate_user_decrypt_on_gateway_reorg_out
    ON delegate_user_decrypt (on_gateway, reorg_out); -- for selecting ready delegation

CREATE INDEX IF NOT EXISTS idx_delegate_user_decrypt_block_hash
    ON delegate_user_decrypt (block_hash); -- for update
