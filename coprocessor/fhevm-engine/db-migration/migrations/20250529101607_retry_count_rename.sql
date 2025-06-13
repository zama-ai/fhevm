ALTER TABLE ciphertext_digest
    RENAME COLUMN txn_retry_count TO txn_limited_retries_count;

ALTER TABLE ciphertext_digest
    RENAME COLUMN txn_transport_retry_count TO txn_unlimited_retries_count;

ALTER TABLE allowed_handles
    RENAME COLUMN txn_retry_count TO txn_limited_retries_count;

ALTER TABLE allowed_handles 
    RENAME COLUMN txn_transport_retry_count TO txn_unlimited_retries_count;
