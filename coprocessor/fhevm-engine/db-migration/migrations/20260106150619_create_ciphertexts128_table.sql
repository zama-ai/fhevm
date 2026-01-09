CREATE TABLE ciphertexts128 (
    tenant_id   INTEGER   NOT NULL,
    handle      BYTEA     NOT NULL,
    ciphertext  BYTEA     NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (tenant_id, handle)
);

CREATE INDEX idx_ciphertexts128_handle
ON ciphertexts128 (handle);
