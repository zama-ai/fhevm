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
-- These may be pre-created by /run_pre_indexes.sh. Refuse invalid leftovers instead
-- of letting IF NOT EXISTS silently skip a broken index.
DO $$
DECLARE
    idx_name TEXT;
BEGIN
    FOREACH idx_name IN ARRAY ARRAY[
        'idx_allowed_handles_no_tenant',
        'idx_input_blobs_no_tenant',
        'idx_ciphertext_digest_no_tenant',
        'idx_ciphertexts_no_tenant',
        'idx_ciphertexts128_no_tenant',
        'idx_computations_no_tenant',
        'idx_pbs_computations_no_tenant'
    ] LOOP
        IF EXISTS (
            SELECT 1
            FROM pg_class c
            JOIN pg_index i ON i.indexrelid = c.oid
            JOIN pg_namespace n ON n.oid = c.relnamespace
            WHERE n.nspname = 'public'
              AND c.relname = idx_name
              AND NOT i.indisvalid
        ) THEN
            RAISE EXCEPTION 'Index % exists but is invalid; drop/recreate it before running this migration', idx_name;
        END IF;
    END LOOP;
END $$;

CREATE UNIQUE INDEX IF NOT EXISTS idx_allowed_handles_no_tenant ON allowed_handles (handle, account_address);
CREATE UNIQUE INDEX IF NOT EXISTS idx_input_blobs_no_tenant ON input_blobs (blob_hash);
CREATE UNIQUE INDEX IF NOT EXISTS idx_ciphertext_digest_no_tenant ON ciphertext_digest (handle);
CREATE UNIQUE INDEX IF NOT EXISTS idx_ciphertexts_no_tenant ON ciphertexts (handle, ciphertext_version);
CREATE UNIQUE INDEX IF NOT EXISTS idx_ciphertexts128_no_tenant ON ciphertexts128 (handle);
CREATE UNIQUE INDEX IF NOT EXISTS idx_computations_no_tenant ON computations (output_handle, transaction_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_pbs_computations_no_tenant ON pbs_computations (handle);

-- Add host_chain_id/key_id_gw with constant defaults. PostgreSQL 11+ stores
-- constant defaults in metadata for existing rows, avoiding full-table rewrites.
DO $$
DECLARE
    hcid BIGINT;
    kid  BYTEA;
BEGIN
    SELECT COALESCE((SELECT chain_id FROM host_chains LIMIT 1), 0) INTO hcid;
    SELECT COALESCE((SELECT key_id_gw FROM keys LIMIT 1), ''::bytea) INTO kid;

    EXECUTE format('ALTER TABLE computations ADD COLUMN host_chain_id BIGINT NOT NULL DEFAULT %s', hcid);
    EXECUTE format('ALTER TABLE pbs_computations ADD COLUMN host_chain_id BIGINT NOT NULL DEFAULT %s', hcid);
    EXECUTE format('ALTER TABLE ciphertext_digest ADD COLUMN host_chain_id BIGINT NOT NULL DEFAULT %s', hcid);
    EXECUTE format('ALTER TABLE ciphertext_digest ADD COLUMN key_id_gw BYTEA NOT NULL DEFAULT %L::bytea', kid::text);
END $$;

ALTER TABLE ciphertext_digest
    ADD CONSTRAINT ciphertext_digest_host_chain_id_positive CHECK (host_chain_id >= 0) NOT VALID;
ALTER TABLE computations
    ADD CONSTRAINT computations_host_chain_id_positive CHECK (host_chain_id >= 0) NOT VALID;
ALTER TABLE pbs_computations
    ADD CONSTRAINT pbs_computations_host_chain_id_positive CHECK (host_chain_id >= 0) NOT VALID;
