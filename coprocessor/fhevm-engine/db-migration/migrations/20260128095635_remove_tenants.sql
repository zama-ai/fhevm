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
-- Existing tables: add new columns only
-- ============================================================

-- host_chain_blocks_valid: widen chain_id from INT to BIGINT to match the rest of the schema.
ALTER TABLE host_chain_blocks_valid ALTER COLUMN chain_id TYPE BIGINT;
ALTER TABLE host_chain_blocks_valid ADD CONSTRAINT host_chain_blocks_valid_chain_id_check CHECK (chain_id >= 0);

-- Set tenant_id default to the existing tenant's ID (or 0 if empty) so new code
-- can insert without specifying tenant_id and rollback to old code sees real IDs.
DO $$
DECLARE
    tid INT;
BEGIN
    SELECT COALESCE((SELECT tenant_id FROM tenants LIMIT 1), 0) INTO tid;
    EXECUTE format('ALTER TABLE allowed_handles ALTER COLUMN tenant_id SET DEFAULT %s', tid);
    EXECUTE format('ALTER TABLE input_blobs ALTER COLUMN tenant_id SET DEFAULT %s', tid);
    EXECUTE format('ALTER TABLE ciphertext_digest ALTER COLUMN tenant_id SET DEFAULT %s', tid);
    EXECUTE format('ALTER TABLE ciphertexts ALTER COLUMN tenant_id SET DEFAULT %s', tid);
    EXECUTE format('ALTER TABLE ciphertexts128 ALTER COLUMN tenant_id SET DEFAULT %s', tid);
    EXECUTE format('ALTER TABLE computations ALTER COLUMN tenant_id SET DEFAULT %s', tid);
    EXECUTE format('ALTER TABLE pbs_computations ALTER COLUMN tenant_id SET DEFAULT %s', tid);
END $$;

-- Add unique indices for new code that queries without tenant_id.
CREATE UNIQUE INDEX idx_allowed_handles_no_tenant ON allowed_handles (handle, account_address);
CREATE UNIQUE INDEX idx_input_blobs_no_tenant ON input_blobs (blob_hash);
CREATE UNIQUE INDEX idx_ciphertext_digest_no_tenant ON ciphertext_digest (handle);
CREATE UNIQUE INDEX idx_ciphertexts_no_tenant ON ciphertexts (handle, ciphertext_version);
CREATE UNIQUE INDEX idx_ciphertexts128_no_tenant ON ciphertexts128 (handle);
CREATE UNIQUE INDEX idx_computations_no_tenant ON computations (output_handle, transaction_id);
CREATE UNIQUE INDEX idx_pbs_computations_no_tenant ON pbs_computations (handle);

-- ciphertext_digest: add host_chain_id and key_id_gw.
ALTER TABLE ciphertext_digest ADD COLUMN host_chain_id BIGINT DEFAULT NULL;
UPDATE ciphertext_digest SET host_chain_id = (SELECT chain_id FROM tenants WHERE tenant_id = ciphertext_digest.tenant_id);
ALTER TABLE ciphertext_digest ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE ciphertext_digest ADD CONSTRAINT ciphertext_digest_host_chain_id_positive CHECK (host_chain_id >= 0);
ALTER TABLE ciphertext_digest ADD COLUMN key_id_gw BYTEA DEFAULT NULL;
UPDATE ciphertext_digest SET key_id_gw = (SELECT key_id FROM tenants LIMIT 1);
ALTER TABLE ciphertext_digest ALTER COLUMN key_id_gw SET NOT NULL;

-- computations: add host_chain_id.
-- TODO: host_chain_id can be part of an index, but will be done in the future where we want workers per host chain
ALTER TABLE computations ADD COLUMN host_chain_id BIGINT DEFAULT NULL;
UPDATE computations SET host_chain_id = (SELECT chain_id FROM tenants WHERE tenant_id = computations.tenant_id);
ALTER TABLE computations ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE computations ADD CONSTRAINT computations_host_chain_id_positive CHECK (host_chain_id >= 0);

-- pbs_computations: add host_chain_id, keep tenant_id.
-- TODO: host_chain_id can be part of an index, but will be done in the future where we want workers per host chain
ALTER TABLE pbs_computations ADD COLUMN host_chain_id BIGINT DEFAULT NULL;
UPDATE pbs_computations SET host_chain_id = (SELECT chain_id FROM tenants WHERE tenant_id = pbs_computations.tenant_id);
ALTER TABLE pbs_computations ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE pbs_computations ADD CONSTRAINT pbs_computations_host_chain_id_positive CHECK (host_chain_id >= 0);

-- Set host_chain_id and key_id_gw defaults for backward compatibility with old code that does not
-- supply these columns. Uses the single host chain / key inserted above (or 0 / empty on empty DB).
DO $$
DECLARE
    hcid BIGINT;
    kid  BYTEA;
BEGIN
    SELECT COALESCE((SELECT chain_id FROM host_chains LIMIT 1), 0) INTO hcid;
    SELECT COALESCE((SELECT key_id_gw FROM keys LIMIT 1), ''::bytea) INTO kid;

    EXECUTE format('ALTER TABLE computations ALTER COLUMN host_chain_id SET DEFAULT %s', hcid);
    EXECUTE format('ALTER TABLE pbs_computations ALTER COLUMN host_chain_id SET DEFAULT %s', hcid);
    EXECUTE format('ALTER TABLE ciphertext_digest ALTER COLUMN host_chain_id SET DEFAULT %s', hcid);
    EXECUTE format('ALTER TABLE ciphertext_digest ALTER COLUMN key_id_gw SET DEFAULT %L::bytea', kid::text);
END $$;

COMMIT;