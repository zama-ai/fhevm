ALTER TABLE ciphertext_digest
ADD COLUMN IF NOT EXISTS s3_format_version SMALLINT DEFAULT NULL;

UPDATE ciphertext_digest
SET s3_format_version = 0
WHERE s3_format_version IS NULL
  AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL);

COMMENT ON COLUMN ciphertext_digest.s3_format_version IS
    'S3 ciphertext object format version: NULL = not uploaded, 0 = initial format, 1 = handle and signed digest metadata';
