-- Down for 20261001120000_ciphertext_digest_legacy_s3_witness.
-- Drops the trigger and the function. The rows the migration stamped keep
-- their witness until 20260817130000 drops the columns.

DROP TRIGGER IF EXISTS stamp_legacy_s3_publication_witness ON ciphertext_digest;
DROP FUNCTION IF EXISTS stamp_legacy_s3_publication_witness();
