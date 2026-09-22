-- Undeployed Solana requests are replaced by fresh preview state, not backfilled.
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM user_decryption_requests WHERE solana_request IS NOT NULL) THEN
        RAISE EXCEPTION 'Old Solana requests require fresh preview connector state before this migration';
    END IF;
END $$;

ALTER TABLE user_decryption_requests
    ADD COLUMN attestation_type TEXT NOT NULL DEFAULT 'legacy',
    ADD COLUMN user_pubkey BYTEA,
    ADD COLUMN allowed_keys BYTEA[],
    ADD COLUMN encrypted_stores BYTEA[],
    ADD COLUMN allowed_scopes BYTEA[],
    ADD COLUMN host_program_id BYTEA,
    ALTER COLUMN user_address DROP NOT NULL,
    DROP COLUMN solana_request;

-- Tagging is not a processing attempt; retain the existing retry and recovery timestamps.
ALTER TABLE user_decryption_requests DISABLE TRIGGER refresh_updated_at_user_decryption_requests_on_update;
UPDATE user_decryption_requests
SET attestation_type = 'eip712-unified-user-decrypt-v1'
WHERE signature IS NOT NULL;
ALTER TABLE user_decryption_requests ENABLE TRIGGER refresh_updated_at_user_decryption_requests_on_update;

-- PostgreSQL CHECK accepts NULL. Check elements and dimensions explicitly, including empty arrays.
CREATE FUNCTION bytea_array_has_width(items BYTEA[], width INTEGER) RETURNS BOOLEAN
LANGUAGE SQL IMMUTABLE PARALLEL SAFE AS $$
    SELECT items IS NOT NULL
        AND (cardinality(items) = 0 OR (array_ndims(items) = 1 AND array_lower(items, 1) = 1))
        AND NOT EXISTS (SELECT FROM unnest(items) item WHERE item IS NULL OR octet_length(item) <> width)
$$;

-- These are admission invariants: the row decoder must be able to reconstruct the signed permit.
CREATE FUNCTION solana_user_decryption_arrays_valid(handles BYTEA[], scopes BYTEA[]) RETURNS BOOLEAN
LANGUAGE SQL IMMUTABLE PARALLEL SAFE AS $$
    SELECT NOT EXISTS (
        SELECT FROM unnest(handles) handle
        WHERE substring(handle FROM 23 FOR 8) <> substring(handles[1] FROM 23 FOR 8)
    ) AND NOT EXISTS (
        SELECT FROM generate_subscripts(scopes, 1) i WHERE i > 1 AND scopes[i-1] >= scopes[i]
    )
$$;

ALTER TABLE user_decryption_requests ADD CONSTRAINT user_decryption_attestation_shape CHECK (
    (
        attestation_type IN ('legacy', 'eip712-unified-user-decrypt-v1')
        AND user_address IS NOT NULL AND octet_length(user_address) = 20
        AND user_pubkey IS NULL AND allowed_keys IS NULL AND encrypted_stores IS NULL
        AND allowed_scopes IS NULL AND host_program_id IS NULL
        -- Completed historical EVM rows may still carry the pre-handle representation.
        AND (ct_handles IS NULL OR bytea_array_has_width(ct_handles, 32))
        AND (
            (attestation_type = 'legacy' AND signature IS NULL
                AND start_timestamp IS NULL AND duration_seconds IS NULL
                AND handle_owner_addresses IS NULL AND handle_contract_addresses IS NULL
                AND allowed_contracts IS NULL)
            OR
            (attestation_type = 'eip712-unified-user-decrypt-v1' AND signature IS NOT NULL
                AND start_timestamp IS NOT NULL AND start_timestamp >= 0
                AND duration_seconds IS NOT NULL AND duration_seconds >= 0
                AND bytea_array_has_width(handle_owner_addresses, 20)
                AND bytea_array_has_width(handle_contract_addresses, 20)
                AND bytea_array_has_width(allowed_contracts, 20)
                AND cardinality(handle_owner_addresses) = cardinality(handle_contract_addresses)
                AND (ct_handles IS NULL OR cardinality(ct_handles) = cardinality(handle_owner_addresses)))
        )
    ) OR (
        attestation_type = 'solana-srfc38-user-decrypt-v1'
        AND user_address IS NULL AND sns_ct_materials IS NULL
        AND handle_owner_addresses IS NULL AND handle_contract_addresses IS NULL AND allowed_contracts IS NULL
        AND user_pubkey IS NOT NULL AND octet_length(user_pubkey) = 32
        AND host_program_id IS NOT NULL AND octet_length(host_program_id) = 32
        AND signature IS NOT NULL AND octet_length(signature) = 64
        AND octet_length(public_key) = 869
        AND octet_length(extra_data) = 65 AND get_byte(extra_data, 0) = 2
        AND start_timestamp IS NOT NULL AND start_timestamp BETWEEN 0 AND 253402300799
        AND duration_seconds IS NOT NULL AND duration_seconds BETWEEN 1 AND 31536000
        AND bytea_array_has_width(ct_handles, 32) AND cardinality(ct_handles) BETWEEN 1 AND 33
        AND bytea_array_has_width(allowed_keys, 32) AND cardinality(allowed_keys) = cardinality(ct_handles)
        AND bytea_array_has_width(encrypted_stores, 32) AND cardinality(encrypted_stores) = cardinality(ct_handles)
        AND bytea_array_has_width(allowed_scopes, 64) AND cardinality(allowed_scopes) <= 7
        AND solana_user_decryption_arrays_valid(ct_handles, allowed_scopes)
    )
);
