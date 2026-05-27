CREATE TABLE IF NOT EXISTS solana_native_decryption_replay_v0 (
    host_chain_id BYTEA NOT NULL CHECK (octet_length(host_chain_id) = 8),
    solana_cluster_id BYTEA NOT NULL CHECK (octet_length(solana_cluster_id) = 32),
    kms_context_id BYTEA NOT NULL CHECK (octet_length(kms_context_id) = 32),
    request_signer_pubkey BYTEA NOT NULL CHECK (octet_length(request_signer_pubkey) = 32),
    nonce BYTEA NOT NULL CHECK (octet_length(nonce) = 32),
    request_hash BYTEA NOT NULL CHECK (octet_length(request_hash) = 32),
    created_at TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (
        host_chain_id,
        solana_cluster_id,
        kms_context_id,
        request_signer_pubkey,
        nonce
    )
);
