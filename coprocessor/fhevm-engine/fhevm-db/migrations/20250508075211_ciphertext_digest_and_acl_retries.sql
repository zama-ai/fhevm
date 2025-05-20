ALTER TABLE ciphertext_digest
    ALTER COLUMN txn_is_sent SET NOT NULL,
    ALTER COLUMN txn_retry_count SET NOT NULL,
    ADD COLUMN txn_transport_retry_count INT DEFAULT 0 NOT NULL;

ALTER TABLE allowed_handles
    ALTER COLUMN txn_is_sent SET NOT NULL,
    ALTER COLUMN txn_retry_count SET NOT NULL,
    ADD COLUMN txn_transport_retry_count INT DEFAULT 0 NOT NULL;
