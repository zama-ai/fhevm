
CREATE TABLE allowed_handles (
    tenant_id INT NOT NULL,
    handle BYTEA NOT NULL,
    account_address TEXT DEFAULT NULL,
     
    txn_is_sent BOOLEAN DEFAULT FALSE,
    txn_retry_count INT DEFAULT 0,
    txn_last_error TEXT DEFAULT NULL,
    txn_last_error_at TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (tenant_id, handle, account_address)
);
