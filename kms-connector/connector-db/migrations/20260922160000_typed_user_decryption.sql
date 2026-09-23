-- Solana user decryption shares `user_decryption_requests` with EVM. A Solana row has its own
-- typed columns and no EVM address columns.
--
-- `attestation_type` is generated from the row, so writers that predate it (EVM gw-listener and
-- endpoint during a rolling upgrade) still produce correctly typed rows. The CHECK makes a row
-- that mixes both shapes unwritable. Field widths and permit rules are enforced by the typed
-- decoders before insertion and again when the row is read.

CREATE TYPE attestation_type AS ENUM ('legacy', 'eip712', 'solana');

-- Rows in the opaque `solana_request` format predate deployment and would read as legacy here.
DO $$ BEGIN
    IF EXISTS (SELECT FROM user_decryption_requests WHERE solana_request IS NOT NULL) THEN
        RAISE EXCEPTION 'Opaque Solana requests need fresh connector state before this migration';
    END IF;
END $$;

ALTER TABLE user_decryption_requests
    DROP COLUMN solana_request,
    ALTER COLUMN user_address DROP NOT NULL,
    ADD COLUMN user_pubkey BYTEA,
    ADD COLUMN handle_allowed_keys BYTEA[],
    ADD COLUMN handle_encrypted_stores BYTEA[],
    ADD COLUMN allowed_scopes BYTEA[],
    ADD COLUMN verifying_program_id BYTEA;

ALTER TABLE user_decryption_requests
    ADD COLUMN attestation_type attestation_type NOT NULL GENERATED ALWAYS AS (
        CASE
            WHEN user_pubkey IS NOT NULL THEN 'solana'::attestation_type
            WHEN signature IS NOT NULL THEN 'eip712'::attestation_type
            ELSE 'legacy'::attestation_type
        END
    ) STORED;

ALTER TABLE user_decryption_requests ADD CONSTRAINT user_decryption_requests_attestation_columns
    CHECK (
        CASE attestation_type
            WHEN 'solana' THEN
                user_address IS NULL AND sns_ct_materials IS NULL AND handle_owner_addresses IS NULL
                AND handle_contract_addresses IS NULL AND allowed_contracts IS NULL
                AND signature IS NOT NULL AND start_timestamp IS NOT NULL
                AND duration_seconds IS NOT NULL AND allowed_scopes IS NOT NULL
                AND verifying_program_id IS NOT NULL AND ct_handles IS NOT NULL
                AND handle_allowed_keys IS NOT NULL AND handle_encrypted_stores IS NOT NULL
                AND cardinality(handle_allowed_keys) = cardinality(ct_handles)
                AND cardinality(handle_encrypted_stores) = cardinality(ct_handles)
            ELSE
                user_address IS NOT NULL AND handle_allowed_keys IS NULL
                AND handle_encrypted_stores IS NULL AND allowed_scopes IS NULL
                AND verifying_program_id IS NULL
        END
    );
