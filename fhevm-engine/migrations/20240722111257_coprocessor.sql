
CREATE TABLE IF NOT EXISTS computations (
    tenant_id INT NOT NULL,
    output_handle BYTEA NOT NULL,
    -- can be handle or scalar, depends on is_scalar field
    -- only second dependency can ever be scalar
    dependencies BYTEA[] NOT NULL,
    fhe_operation SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP,
    is_scalar BOOLEAN NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT 'f',
    is_error BOOLEAN NOT NULL DEFAULT 'f',
    error_message TEXT,
    PRIMARY KEY (tenant_id, output_handle)
);

CREATE TABLE IF NOT EXISTS ciphertexts (
    tenant_id INT NOT NULL,
    handle BYTEA NOT NULL,
    ciphertext BYTEA NOT NULL,
    ciphertext_version SMALLINT NOT NULL,
    ciphertext_type SMALLINT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (tenant_id, handle, ciphertext_version)
);

CREATE TABLE IF NOT EXISTS tenants (
    tenant_id SERIAL PRIMARY KEY,
    tenant_api_key UUID NOT NULL DEFAULT gen_random_uuid(),
    pks_key BYTEA NOT NULL,
    sks_key BYTEA NOT NULL,
    -- for debugging, can be null
    cks_key BYTEA,
    -- admin api key is allowed to create more tenants with their keys
    is_admin BOOLEAN DEFAULT 'f'
);

CREATE INDEX IF NOT EXISTS computations_dependencies_index ON computations USING GIN (dependencies);
CREATE INDEX IF NOT EXISTS computations_completed_index ON computations (is_completed);
CREATE INDEX IF NOT EXISTS computations_errors_index ON computations (is_error);
CREATE INDEX IF NOT EXISTS tenants_by_api_key ON tenants (tenant_api_key);