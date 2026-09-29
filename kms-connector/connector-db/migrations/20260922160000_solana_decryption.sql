-- Solana decryption requests share the EVM tables.
--
-- User decryption: Solana shares `user_decryption_requests` with EVM. A Solana row names its
-- requester in `user_address` and each handle's owner in `handle_owner_addresses`, as an EVM row
-- does, with 32-byte Ed25519 keys instead of 20-byte addresses. It adds the three fields EVM has
-- no counterpart for: the handles' encrypted stores, the permit's allowed scopes and the host
-- program id the permit is for.
--
-- `attestation_type` is generated from the row, so EVM writers that predate it (gw-listener and
-- endpoint during a rolling upgrade) still produce correctly typed rows. The CHECK makes a row
-- that mixes both shapes unwritable. It also checks the widths of the three Solana key columns
-- (user_address, verifying_program_id, signature); every other width and the permit rules are
-- enforced by the typed decoders before insertion and again when the row is read.

CREATE TYPE attestation_type AS ENUM ('legacy', 'eip712', 'solana');

ALTER TABLE user_decryption_requests
    ADD COLUMN handle_encrypted_stores BYTEA[],
    ADD COLUMN allowed_scopes BYTEA[],
    ADD COLUMN verifying_program_id BYTEA;

-- Solana is tested first: a Solana row carries a signature too, so the signature test alone would
-- type it as eip712.
ALTER TABLE user_decryption_requests
    ADD COLUMN attestation_type attestation_type NOT NULL GENERATED ALWAYS AS (
        CASE
            WHEN verifying_program_id IS NOT NULL THEN 'solana'::attestation_type
            WHEN signature IS NOT NULL THEN 'eip712'::attestation_type
            ELSE 'legacy'::attestation_type
        END
    ) STORED;

-- A CHECK passes on NULL. user_address is NOT NULL and a non-NULL verifying_program_id is what
-- tags a row solana, so signature is the one width test that needs an explicit IS NOT NULL.
ALTER TABLE user_decryption_requests ADD CONSTRAINT user_decryption_requests_attestation_columns
    CHECK (
        CASE attestation_type
            WHEN 'solana' THEN
                octet_length(user_address) = 32 AND octet_length(verifying_program_id) = 32
                AND signature IS NOT NULL AND octet_length(signature) = 64
                AND sns_ct_materials IS NULL AND handle_contract_addresses IS NULL
                AND allowed_contracts IS NULL
                AND start_timestamp IS NOT NULL AND duration_seconds IS NOT NULL
                AND allowed_scopes IS NOT NULL AND ct_handles IS NOT NULL
                AND handle_owner_addresses IS NOT NULL AND handle_encrypted_stores IS NOT NULL
                AND cardinality(handle_owner_addresses) = cardinality(ct_handles)
                AND cardinality(handle_encrypted_stores) = cardinality(ct_handles)
            ELSE
                handle_encrypted_stores IS NULL AND allowed_scopes IS NULL
        END
    );

-- Public decryption: a Solana row names the encrypted store of each handle. A store is not
-- derivable from a handle, and the worker proves each handle's public-decrypt leaf against it. EVM
-- rows leave the column NULL, which is what tells the row reader which shape a row has.
ALTER TABLE public_decryption_requests ADD COLUMN handle_encrypted_stores BYTEA[];

ALTER TABLE public_decryption_requests ADD CONSTRAINT public_decryption_requests_encrypted_stores
    CHECK (
        handle_encrypted_stores IS NULL
        OR (
            ct_handles IS NOT NULL
            AND cardinality(handle_encrypted_stores) = cardinality(ct_handles)
        )
    );
