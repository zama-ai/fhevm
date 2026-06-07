-- Model-2 (EVM-parity) Solana user-decryption response storage.
--
-- The Solana user-decrypt REQUEST is a Solana-native, ed25519 signMessage-signed envelope
-- (seam a) — ingested via solana_native_decryption_requests_v0 and authorized by the existing
-- solana_flow admission (RPC-verified ACL + signMessage). The RESPONSE, however, mirrors EVM
-- exactly: the kms-core returns the standard signcrypted shares (+ secp256k1 response signature)
-- bound to the request via compute_link_solana, and the user de-signcrypts it off-chain. So the
-- response is NOT the Solana-native ed25519 envelope of solana_native_decryption_responses_v0;
-- it is the standard response body, relayed to the user, keyed by the native request hash.
CREATE TABLE IF NOT EXISTS solana_user_decrypt_responses_v0 (
    request_hash BYTEA NOT NULL,
    -- bincode-serialized kms_grpc UserDecryptionResponse (the standard EVM-parity response).
    raw_response_body BYTEA NOT NULL,
    status operation_status DEFAULT 'pending' NOT NULL,
    error_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    PRIMARY KEY (request_hash),
    CHECK (octet_length(request_hash) = 32)
);

CREATE INDEX IF NOT EXISTS idx_solana_user_decrypt_responses_v0_status
    ON solana_user_decrypt_responses_v0 (status);
