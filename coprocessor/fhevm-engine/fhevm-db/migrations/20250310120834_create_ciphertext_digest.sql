-- Add migration script here

CREATE TABLE ciphertext_digest (
    tenant_id INT NOT NULL,
    handle BYTEA NOT NULL,
    ciphertext BYTEA NULL DEFAULT NULL,  -- ciphertext64 digest (nullable)
    ciphertext128 BYTEA NULL DEFAULT NULL, -- ciphertext128 digest (nullable)
    
    txn_is_sent BOOLEAN DEFAULT FALSE,
    txn_retry_count INT DEFAULT 0,
    txn_last_error TEXT DEFAULT NULL,
    txn_last_error_at TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (tenant_id, handle)
);
