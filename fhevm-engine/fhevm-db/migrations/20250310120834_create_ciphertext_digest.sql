-- Add migration script here

CREATE TABLE ciphertext_digest (
    tenant_id INT NOT NULL,
    handle BYTEA NOT NULL,
    ciphertext BYTEA NULL DEFAULT NULL,  -- ciphertext64 digest (nullable)
    ciphertext128 BYTEA NULL DEFAULT NULL, -- ciphertext128 digest (nullable)
    retry_send INT DEFAULT 0,
    is_sent BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (tenant_id, handle)
);
