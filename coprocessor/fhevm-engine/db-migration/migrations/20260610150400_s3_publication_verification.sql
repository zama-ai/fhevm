-- Tracks the RFC 023 handle-keyed S3 object version that was verified for a
-- branch digest row. Settlement can then use DB state instead of issuing S3
-- HEAD requests in the finalization transaction.

ALTER TABLE ciphertext_digest_branch
ADD COLUMN IF NOT EXISTS s3_publication_verified_at TIMESTAMPTZ NULL;

ALTER TABLE ciphertext_digest_branch
ADD COLUMN IF NOT EXISTS s3_publication_verified_digest BYTEA NULL;

ALTER TABLE ciphertext_digest_branch
ADD COLUMN IF NOT EXISTS s3_publication_verified_producer_block_hash BYTEA NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_s3_unverified
ON ciphertext_digest_branch (host_chain_id, block_number, created_at)
WHERE ciphertext IS NULL
   OR s3_publication_verified_at IS NULL
   OR s3_publication_verified_digest IS DISTINCT FROM ciphertext
   OR s3_publication_verified_producer_block_hash IS DISTINCT FROM producer_block_hash;

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_s3_verified
ON ciphertext_digest_branch (host_chain_id, s3_publication_verified_at)
WHERE s3_publication_verified_at IS NOT NULL;
