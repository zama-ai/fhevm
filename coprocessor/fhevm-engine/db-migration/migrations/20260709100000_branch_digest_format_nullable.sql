-- ciphertext_digest_branch was created LIKE ciphertext_digest, inheriting the
-- legacy NOT NULL DEFAULT 10 on ciphertext128_format. The branch-table
-- writers use different semantics: enqueue_upload_task records NULL while the
-- ct128 conversion is still pending (the format is unknown at enqueue time),
-- and mark_ciphertexts_uploaded fills the real format on upload; the upload
-- waiter reads the column as nullable and treats NULL as "not yet recorded".
-- Passing that explicit NULL violated the inherited NOT NULL.
--
-- Only the constraint is dropped: DEFAULT 10 stays so writers that omit the
-- column (legacy mirror, transaction-sender fixtures) keep their behavior,
-- and the legacy ciphertext_digest table is untouched.
--
-- Catalog-only change, but the ACCESS EXCLUSIVE lock request must not convoy
-- live digest traffic behind a long-running writer; bound the wait like the
-- other DDL on these tables and let the migration retry instead.
SET LOCAL lock_timeout = '3s';

ALTER TABLE ciphertext_digest_branch
ALTER COLUMN ciphertext128_format DROP NOT NULL;
