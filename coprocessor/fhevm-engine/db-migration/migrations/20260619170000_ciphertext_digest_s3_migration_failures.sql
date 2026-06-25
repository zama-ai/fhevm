ALTER TABLE ciphertext_digest
    ADD COLUMN IF NOT EXISTS s3_migration_failure_count INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS s3_migration_last_error TEXT DEFAULT NULL,
    ADD COLUMN IF NOT EXISTS s3_migration_last_error_at TIMESTAMPTZ DEFAULT NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_s3_migration_pending
ON ciphertext_digest (s3_format_version, s3_migration_failure_count, handle)
WHERE s3_format_version = 0
  AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL);

COMMENT ON COLUMN ciphertext_digest.s3_migration_failure_count IS
    'Number of failed S3 format migration attempts. Automatic migration handles zero-failure rows first, then replays the smallest non-zero failure-count group.';

COMMENT ON COLUMN ciphertext_digest.s3_migration_last_error IS
    'Last error recorded while trying to migrate this row to the current S3 object format.';

COMMENT ON COLUMN ciphertext_digest.s3_migration_last_error_at IS
    'Timestamp of the last recorded S3 format migration error for this row.';
