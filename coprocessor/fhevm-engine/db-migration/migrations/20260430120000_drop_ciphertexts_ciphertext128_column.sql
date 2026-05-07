-- Drop the legacy `ciphertext128` column from the `ciphertexts` table.
--
-- This column has been superseded by:
--   * the standalone `ciphertexts128` table (see migration 20260106150619), and
--   * the `ciphertext128` column on `ciphertext_digest`.
-- No application code reads or writes `ciphertexts.ciphertext128` anymore.
--
-- Drop the dependent indexes explicitly first (rather than using CASCADE) so the
-- migration is self-documenting and impossible to extend silently.
--
-- NOTE on storage reclamation:
--   ALTER TABLE ... DROP COLUMN is metadata-only and does NOT free the column
--   bytes still present in existing heap rows. Index storage IS freed here
--   (DROP INDEX is immediate), but to reclaim heap storage an operator must
--   rewrite the table post-deploy. Use the online path:
--
--     ./reclaim_ciphertexts_storage.sh   # drives pg_repack on `ciphertexts`
--
--   The `pg_repack` extension is created by migration
--   20260430120100_install_pg_repack_extension.sql. See that file's header for
--   the privilege requirements.

DROP INDEX IF EXISTS idx_ciphertexts_tenant_handle;
DROP INDEX IF EXISTS idx_ciphertexts_created_at;
DROP INDEX IF EXISTS idx_ciphertexts_ciphertext128_null;

ALTER TABLE ciphertexts DROP COLUMN IF EXISTS ciphertext128;
