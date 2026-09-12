-- Digests are written at SNS conversion. s3_publication_verified_* is stamped
-- after S3 postflight. txn-sender (addCiphertext), the tfhe-worker bridge,
-- gw-listener drift checks, and SNS GC wait on that witness. Block-manifest
-- sealing does not: consensus is independent of ciphertext object availability.
ALTER TABLE ciphertext_digest
    ADD COLUMN s3_publication_verified_at TIMESTAMPTZ NULL,
    ADD COLUMN s3_publication_verified_digest BYTEA NULL
        CHECK (
            s3_publication_verified_digest IS NULL
            OR OCTET_LENGTH(s3_publication_verified_digest) = 32
        );

-- Before this change, both digest columns were written only after a successful
-- S3 postflight. Treat that pair as already published. Do not touch
-- s3_format_version: it was already set on those rows, and new pre-upload
-- rows get it together with the witness at postflight.
UPDATE ciphertext_digest
   SET s3_publication_verified_at = NOW(),
       s3_publication_verified_digest = ciphertext
 WHERE s3_publication_verified_at IS NULL
   AND ciphertext IS NOT NULL
   AND ciphertext128 IS NOT NULL;
