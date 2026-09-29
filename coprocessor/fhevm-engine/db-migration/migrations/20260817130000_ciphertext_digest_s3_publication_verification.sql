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
-- S3 postflight. `20260526090000` already set `s3_format_version = 0` on every
-- row that had a digest (`NULL` means not uploaded). New enqueue writes digests
-- before upload and leaves `s3_format_version` NULL until postflight. Do not
-- treat in-flight pre-upload rows as already published.
UPDATE ciphertext_digest
   SET s3_publication_verified_at = NOW(),
       s3_publication_verified_digest = ciphertext
 WHERE s3_publication_verified_at IS NULL
   AND ciphertext IS NOT NULL
   AND ciphertext128 IS NOT NULL
   AND s3_format_version IS NOT NULL;
