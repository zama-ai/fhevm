CREATE TABLE IF NOT EXISTS public_decryption_responses (
    decryption_id BYTEA NOT NULL,
    decrypted_result BYTEA NOT NULL,
    signatures BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (decryption_id)
);

CREATE TABLE IF NOT EXISTS user_decryption_responses (
    decryption_id BYTEA NOT NULL,
    user_decrypted_shares BYTEA NOT NULL,
    signatures BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (decryption_id)
);
