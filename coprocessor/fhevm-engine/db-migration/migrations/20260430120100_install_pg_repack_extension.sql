-- Install the `pg_repack` extension so the legacy `ciphertexts.ciphertext128`
-- column drop (migration 20260430120000) can be reclaimed online, without an
-- ACCESS EXCLUSIVE lock on the table.
--
-- Privileges:
--   * AWS RDS / Aurora: the migration role must be a member of `rds_superuser`.
--     The extension binary is pre-installed on RDS engines that support
--     pg_repack (Postgres 17 does); only the per-database `CREATE EXTENSION`
--     call is needed. See:
--     https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.PostgreSQL.CommonDBATasks.pg_repack.html
--   * Self-managed Postgres: the migration role must have `SUPERUSER`, AND the
--     OS-level pg_repack package (typically `postgresql-17-repack` on
--     Debian/Ubuntu) must be present on the database host. Otherwise this
--     statement fails with `could not open extension control file`.
--
-- The actual repack is driven from the operator-facing
-- `reclaim_ciphertexts_storage.sh` (which invokes the `pg_repack` client
-- binary), not from this migration.

CREATE EXTENSION IF NOT EXISTS pg_repack;
