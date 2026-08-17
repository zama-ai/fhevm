-- Computed digests are persisted as soon as SNS conversion completes. Keep an
-- explicit witness for the later S3 upload and postflight verification so
-- consumers never confuse digest availability with object availability.
ALTER TABLE ciphertext_digest
    ADD COLUMN s3_publication_verified_at TIMESTAMPTZ NULL,
    ADD COLUMN s3_publication_verified_digest BYTEA NULL
        CHECK (
            s3_publication_verified_digest IS NULL
            OR OCTET_LENGTH(s3_publication_verified_digest) = 32
        );

CREATE INDEX idx_ciphertext_digest_s3_publication_verified
    ON ciphertext_digest (host_chain_id, s3_publication_verified_at)
    WHERE s3_publication_verified_at IS NOT NULL;
