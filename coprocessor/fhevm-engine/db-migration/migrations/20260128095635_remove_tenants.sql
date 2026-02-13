BEGIN;

-- Enforce that tenants has zero or one row.
DO $$
BEGIN
    IF (SELECT COUNT(*) FROM tenants) > 1 THEN
        RAISE EXCEPTION 'Expected zero or one row in tenants table, but found %', (SELECT COUNT(*) FROM tenants);
    END IF;
END $$;

-- ============================================================
-- New tables: keys, crs, host_chains
-- ============================================================

-- keys: replaces tenants, keeping only key material.
-- key_id contains the key ID from the server key metadata (that is used in ciphertext metadata).
-- key_id_gw contains the key ID from the GW event (that could be different from key_id).
CREATE TABLE keys (
    sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    key_id_gw BYTEA NOT NULL,
    key_id BYTEA NOT NULL,
    pks_key BYTEA NOT NULL,
    sks_key BYTEA NOT NULL,
    cks_key BYTEA,
    sns_pk OID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_key_id_gw UNIQUE (key_id_gw),
    CONSTRAINT unique_key_id UNIQUE (key_id)
);

INSERT INTO keys (key_id_gw, key_id, pks_key, sks_key, cks_key, sns_pk)
    SELECT key_id, ''::BYTEA, pks_key, sks_key, cks_key, sns_pk FROM tenants;

-- crs: split out from tenants.
CREATE TABLE crs (
    sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    crs_id BYTEA NOT NULL,
    crs BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_crs_id UNIQUE (crs_id)
);

-- Use an empty ID for the existing CRS.
INSERT INTO crs (crs_id, crs)
    SELECT ''::BYTEA, public_params FROM tenants;

-- host_chains: split out from tenants.
CREATE TABLE host_chains (
    chain_id BIGINT PRIMARY KEY NOT NULL CHECK (chain_id >= 0),
    name TEXT NOT NULL,
    acl_contract_address TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO host_chains (chain_id, name, acl_contract_address)
    SELECT chain_id, 'ethereum', acl_contract_address FROM tenants;

-- ============================================================
-- Existing tables: remove tenant_id, add host_chain_id where needed
-- ============================================================

-- verify_proofs: rename chain_id to host_chain_id.
ALTER TABLE verify_proofs RENAME COLUMN chain_id TO host_chain_id;
ALTER TABLE verify_proofs RENAME CONSTRAINT verify_proofs_chain_id_check TO verify_proofs_host_chain_id_check;

-- host_chain_blocks_valid: widen chain_id from INT to BIGINT to match the rest of the schema.
ALTER TABLE host_chain_blocks_valid ALTER COLUMN chain_id TYPE BIGINT;
ALTER TABLE host_chain_blocks_valid ADD CONSTRAINT host_chain_blocks_valid_chain_id_check CHECK (chain_id >= 0);

-- allowed_handles: drop tenant_id.
ALTER TABLE allowed_handles DROP COLUMN tenant_id;
ALTER TABLE allowed_handles ADD PRIMARY KEY (handle, account_address);

-- input_blobs: drop tenant_id.
ALTER TABLE input_blobs DROP CONSTRAINT input_blobs_pkey;
ALTER TABLE input_blobs DROP COLUMN tenant_id;
ALTER TABLE input_blobs ADD PRIMARY KEY (blob_hash);

-- ciphertext_digest: replace tenant_id with host_chain_id and key_id_gw.
ALTER TABLE ciphertext_digest ADD COLUMN host_chain_id BIGINT DEFAULT NULL;
UPDATE ciphertext_digest SET host_chain_id = (SELECT chain_id FROM tenants WHERE tenant_id = ciphertext_digest.tenant_id);
ALTER TABLE ciphertext_digest ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE ciphertext_digest ADD CONSTRAINT ciphertext_digest_host_chain_id_positive CHECK (host_chain_id >= 0);
ALTER TABLE ciphertext_digest ADD COLUMN key_id_gw BYTEA DEFAULT NULL;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM ciphertext_digest) AND NOT EXISTS (SELECT 1 FROM tenants) THEN
        RAISE EXCEPTION 'ciphertext_digest has rows but tenants is empty; cannot populate key_id_gw';
    END IF;
END $$;
UPDATE ciphertext_digest SET key_id_gw = (SELECT key_id FROM tenants LIMIT 1);
ALTER TABLE ciphertext_digest ALTER COLUMN key_id_gw SET NOT NULL;
ALTER TABLE ciphertext_digest DROP COLUMN tenant_id;
ALTER TABLE ciphertext_digest ADD PRIMARY KEY (handle);

-- ciphertexts: drop tenant_id.
ALTER TABLE ciphertexts DROP CONSTRAINT ciphertexts_pkey;
ALTER TABLE ciphertexts DROP COLUMN tenant_id;
ALTER TABLE ciphertexts ADD PRIMARY KEY (handle, ciphertext_version);

-- ciphertexts128: drop tenant_id.
ALTER TABLE ciphertexts128 DROP CONSTRAINT ciphertexts128_pkey;
ALTER TABLE ciphertexts128 DROP COLUMN tenant_id;
ALTER TABLE ciphertexts128 ADD PRIMARY KEY (handle);
DROP INDEX IF EXISTS idx_ciphertexts128_handle;

-- computations: replace tenant_id with host_chain_id.
-- TODO: host_chain_id can be part of an index, but will be done in the future where we want workers per host chain
ALTER TABLE computations ADD COLUMN host_chain_id BIGINT DEFAULT NULL;
UPDATE computations SET host_chain_id = (SELECT chain_id FROM tenants WHERE tenant_id = computations.tenant_id);
ALTER TABLE computations ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE computations ADD CONSTRAINT computations_host_chain_id_positive CHECK (host_chain_id >= 0);
ALTER TABLE computations DROP CONSTRAINT computations_pkey;
DROP INDEX IF EXISTS idx_computations_pk;
ALTER TABLE computations DROP COLUMN tenant_id;
ALTER TABLE computations ADD PRIMARY KEY (output_handle, transaction_id);

-- pbs_computations: replace tenant_id with host_chain_id.
-- TODO: host_chain_id can be part of an index, but will be done in the future where we want workers per host chain
ALTER TABLE pbs_computations ADD COLUMN host_chain_id BIGINT DEFAULT NULL;
UPDATE pbs_computations SET host_chain_id = (SELECT chain_id FROM tenants WHERE tenant_id = pbs_computations.tenant_id);
ALTER TABLE pbs_computations ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE pbs_computations ADD CONSTRAINT pbs_computations_host_chain_id_positive CHECK (host_chain_id >= 0);
ALTER TABLE pbs_computations DROP CONSTRAINT pbs_computations_pkey;
ALTER TABLE pbs_computations DROP COLUMN tenant_id;
ALTER TABLE pbs_computations ADD PRIMARY KEY (handle);

-- ============================================================
-- Drop the old tenants table.
-- ============================================================
DROP TABLE tenants;

COMMIT;
