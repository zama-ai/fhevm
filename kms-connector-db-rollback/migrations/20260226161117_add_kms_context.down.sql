-- Down for 20260226161117_add_kms_context.
--
-- Drops the `kms_context` table. It caches which KMS contexts are valid; the
-- previous version does not use it.

DROP TABLE kms_context;
