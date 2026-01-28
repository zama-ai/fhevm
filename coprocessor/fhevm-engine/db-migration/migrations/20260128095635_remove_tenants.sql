BEGIN;

-- Create the keys table.
ALTER TABLE tenants RENAME TO keys;

-- Enforce that keys (or what we called tenants) has zero or one row.
DO $$
BEGIN
    IF (SELECT COUNT(*) FROM keys) > 1 THEN
        RAISE EXCEPTION 'Expected zero or one row in keys table, but found %', (SELECT COUNT(*) FROM keys);
    END IF;
END $$;

ALTER TABLE keys DROP COLUMN tenant_api_key;
ALTER TABLE keys DROP COLUMN is_admin;
ALTER TABLE keys DROP COLUMN sns_sk;
ALTER TABLE keys DROP COLUMN verifying_contract_address;;

-- key_id contains the key ID from the server key metadata (that is used in ciphertext metadata).
ALTER TABLE keys ALTER COLUMN key_id SET NOT NULL;
-- The current key_id for the existing key is empty.
UPDATE keys SET key_id = ''::BYTEA;
ALTER TABLE keys ADD CONSTRAINT unique_key_id UNIQUE (key_id);

--key_id_gw contains the key ID from the GW event (that could be different from key_id).
ALTER TABLE keys ADD column key_id_gw BYTEA NOT NULL;
ALTER TABLE keys ADD CONSTRAINT unique_key_id_gw UNIQUE (key_id_gw);

ALTER TABLE keys ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Split CRS from keys.
CREATE TABLE crs (
    -- The sequence number to identify the latest CRS.
    sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    crs_id BYTEA NOT NULL,
    crs BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_crs_id UNIQUE (crs_id)
);

-- Move CRS from keys to crs.
INSERT INTO crs (crs_id, crs)
    SELECT ''::BYTEA, public_params FROM keys;
ALTER TABLE keys DROP COLUMN public_params;

-- Host chains.
CREATE TABLE host_chains (
    chain_id BIGINT PRIMARY KEY NOT NULL CHECK (chain_id > 0),
    name TEXT NOT NULL,
    acl_contract_address TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Move ACL contract address.
INSERT INTO host_chains (chain_id, name, acl_contract_address)
    SELECT chain_id, 'ethereum', acl_contract_address FROM keys;
ALTER TABLE keys DROP COLUMN acl_contract_address;


-- allowed_handles.tenant_id no longer needed.
ALTER TABLE allowed_handles DROP COLUMN tenant_id;
ALTER TABLE allowed_handles ADD PRIMARY KEY (handle, account_address);

-- input_blobs.tenant_id no longer needed.
ALTER TABLE input_blobs DROP CONSTRAINT input_blobs_pkey;
ALTER TABLE input_blobs DROP COLUMN tenant_id;
ALTER TABLE input_blobs ADD PRIMARY KEY (blob_hash);

-- ciphertext_digest.tenant_id no longer needed. Instead, put the host_chain_id there directly.
ALTER TABLE ciphertext_digest ADD COLUMN host_chain_id BIGINT DEFAULT NULL;
UPDATE ciphertext_digest SET host_chain_id = (SELECT chain_id FROM keys WHERE tenant_id = ciphertext_digest.tenant_id);
ALTER TABLE ciphertext_digest ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE ciphertext_digest ADD CONSTRAINT ciphertext_digest_host_chain_id_positive CHECK (host_chain_id > 0);
ALTER TABLE ciphertext_digest DROP COLUMN tenant_id;
ALTER TABLE ciphertext_digest ADD PRIMARY KEY (handle);
ALTER TABLE ciphertext_digest ADD COLUMN key_id BYTEA NOT NULL; -- TODO for the future is to make this an index

-- ciphertexts.tenant_id no longer needed.
ALTER TABLE ciphertexts DROP CONSTRAINT ciphertexts_pkey;
ALTER TABLE ciphertexts DROP COLUMN tenant_id;
ALTER TABLE ciphertexts ADD PRIMARY KEY (handle, ciphertext_version);

-- computations.tenant_id no longer needed.
ALTER TABLE computations DROP CONSTRAINT computations_pkey;
DROP INDEX IF EXISTS idx_computations_pk;
ALTER TABLE computations DROP COLUMN tenant_id;
ALTER TABLE computations ADD PRIMARY KEY (output_handle, transaction_id);

-- pbs_computations.tenant_id no longer needed.
ALTER TABLE pbs_computations DROP CONSTRAINT pbs_computations_pkey;
ALTER TABLE pbs_computations DROP COLUMN tenant_id;
ALTER TABLE pbs_computations ADD PRIMARY KEY (handle);

-- We can now safely drop tenant_id and chain_id from keys.
ALTER TABLE keys DROP COLUMN tenant_id;
ALTER TABLE keys DROP COLUMN chain_id;
-- The sequence_number can be used to identify the latest key.
ALTER TABLE keys ADD COLUMN sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY;

COMMIT;
