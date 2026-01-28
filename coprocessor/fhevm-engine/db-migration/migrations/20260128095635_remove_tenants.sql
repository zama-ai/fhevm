BEGIN;

-- tenant -> keys
ALTER TABLE tenants RENAME TO keys;

ALTER TABLE keys DROP COLUMN tenant_id;
ALTER TABLE keys DROP COLUMN tenant_api_key;

ALTER TABLE keys ADD COLUMN sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY;

ALTER TABLE keys ALTER COLUMN key_id SET NOT NULL;
ALTER TABLE keys ADD CONSTRAINT unique_key_id UNIQUE (key_id);
ALTER TABLE keys ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

ALTER TABLE keys DROP COLUMN is_admin;
ALTER TABLE keys DROP COLUMN chain_id;
ALTER TABLE keys DROP COLUMN verifying_contract_address;
ALTER TABLE keys DROP COLUMN acl_contract_address;
ALTER TABLE keys DROP COLUMN sns_sk;

-- split CRS from keys
CREATE TABLE crs (
    sequence_number BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    crs_id BYTEA NOT NULL,
    crs BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_crs_id UNIQUE (crs_id)
);

-- move CRS
INSERT INTO CRS (crs_id, crs)
    SELECT '\x'::BYTEA, public_params FROM keys;
ALTER TABLE keys DROP COLUMN public_params;

-- host chains
CREATE TABLE host_chains (
    chain_id BIGINT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

COMMIT;