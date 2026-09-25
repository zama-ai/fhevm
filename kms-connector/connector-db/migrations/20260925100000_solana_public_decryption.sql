-- A Solana public decryption names the encrypted store of each handle: a store is not derivable
-- from a handle, and the worker proves each handle's public-decrypt leaf against it. EVM rows
-- leave the column NULL, which is what tells the row reader which shape a row has.
ALTER TABLE public_decryption_requests ADD COLUMN handle_encrypted_stores BYTEA[];

ALTER TABLE public_decryption_requests ADD CONSTRAINT public_decryption_requests_encrypted_stores
    CHECK (
        handle_encrypted_stores IS NULL
        OR (
            ct_handles IS NOT NULL
            AND cardinality(handle_encrypted_stores) = cardinality(ct_handles)
        )
    );
