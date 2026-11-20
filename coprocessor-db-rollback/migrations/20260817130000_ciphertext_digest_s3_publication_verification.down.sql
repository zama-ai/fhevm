-- Down for 20260817130000_ciphertext_digest_s3_publication_verification.
-- Drops the two columns. The verification witness of each digest is lost; the
-- rows the migration stamped keep every other column. The index on
-- s3_publication_verified_at (20260904120000) and the trigger that stamps it
-- (20261001120000) are already gone when this file runs.

ALTER TABLE ciphertext_digest
    DROP COLUMN IF EXISTS s3_publication_verified_digest,
    DROP COLUMN IF EXISTS s3_publication_verified_at;
