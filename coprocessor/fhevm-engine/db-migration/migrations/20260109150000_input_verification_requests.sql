CREATE TABLE IF NOT EXISTS input_verification_requests (
    request_id BIGINT PRIMARY KEY NOT NULL CHECK (request_id >= 0),
    commitment BYTEA NOT NULL,
    user_address TEXT NOT NULL,
    contract_chain_id BIGINT NOT NULL CHECK (contract_chain_id >= 0),
    contract_address TEXT NOT NULL,
    user_signature BYTEA NOT NULL,
    timestamp BIGINT NOT NULL CHECK (timestamp >= 0),
    transaction_id BYTEA NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_input_verification_requests_chain_id
    ON input_verification_requests(contract_chain_id);
