-- enqueue fills ciphertext/ciphertext128 immediately, so
-- idx_ciphertext_digest_ciphertext_null no longer covers the upload
-- waiter. Metrics COUNT and fetch_pending_uploads select
-- s3_publication_verified_at IS NULL.
CREATE INDEX idx_ciphertext_digest_s3_publication_pending
    ON ciphertext_digest (created_at, handle)
    WHERE s3_publication_verified_at IS NULL;
