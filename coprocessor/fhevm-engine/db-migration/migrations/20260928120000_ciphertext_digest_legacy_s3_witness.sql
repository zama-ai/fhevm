-- `20260817130000` backfilled the S3 publication witness once. A pre-witness
-- SNS worker (e.g. a v0.14 blue stack still running after this migration)
-- keeps completing uploads without stamping it, and every witness consumer
-- (upgrade-controller cutover, txn-sender, tfhe-worker bridge, gw-listener
-- drift checks, SNS GC) would wait on those rows forever.
--
-- A legacy writer sets the digests and `s3_format_version` in the same UPDATE,
-- only after S3 postflight; a legacy bridge copies them in one INSERT. The
-- current SNS leaves `s3_format_version` NULL until postflight and stamps the
-- witness in that same statement. So `s3_format_version` turning non-NULL
-- with digests present and no witness means a legacy upload completed: stamp
-- it, exactly as the backfill did. Only the NULL -> non-NULL transition
-- counts, so an S3 format migration of an unwitnessed row does not stamp it.
--
-- Drop this trigger once no pre-witness SNS worker or bridge can write here.
CREATE OR REPLACE FUNCTION stamp_legacy_s3_publication_witness()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.s3_publication_verified_at IS NULL
       AND NEW.s3_format_version IS NOT NULL
       AND NEW.ciphertext IS NOT NULL
       AND NEW.ciphertext128 IS NOT NULL
       AND (TG_OP = 'INSERT' OR OLD.s3_format_version IS NULL)
    THEN
        NEW.s3_publication_verified_at := NOW();
        NEW.s3_publication_verified_digest := NEW.ciphertext;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER stamp_legacy_s3_publication_witness
BEFORE INSERT OR UPDATE OF s3_format_version ON ciphertext_digest
FOR EACH ROW
EXECUTE FUNCTION stamp_legacy_s3_publication_witness();

-- Uploads a legacy worker completed between `20260817130000` and now.
UPDATE ciphertext_digest
   SET s3_publication_verified_at = NOW(),
       s3_publication_verified_digest = ciphertext
 WHERE s3_publication_verified_at IS NULL
   AND ciphertext IS NOT NULL
   AND ciphertext128 IS NOT NULL
   AND s3_format_version IS NOT NULL;
