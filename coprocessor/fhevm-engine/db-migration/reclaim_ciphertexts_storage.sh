#!/bin/bash
# Reclaims heap storage left behind by dropping the legacy
# `ciphertexts.ciphertext128` column (migration
# 20260430120000_drop_ciphertexts_ciphertext128_column.sql).
#
# Online: pg_repack rewrites the table while concurrent reads/writes continue.
# An ACCESS EXCLUSIVE lock is taken only briefly (sub-second on most workloads)
# during the final swap, so coprocessor services can stay up.
#
# Prerequisites:
#   * Server-side `pg_repack` extension is installed in the target database.
#     Migration 20260430120100_install_pg_repack_extension.sql does this for
#     you (CREATE EXTENSION IF NOT EXISTS pg_repack), but the role used by
#     migrations must be a member of `rds_superuser` (RDS) or have SUPERUSER
#     (self-managed) for that to succeed.
#   * The `pg_repack` client binary must be on $PATH where this script runs.
#     When invoked inside the db-migration Docker image, pg_repack is shipped
#     at /usr/local/bin/pg_repack (built from the upstream PGDG
#     `postgresql-17-repack` package — see Dockerfile stage `pgrepack-builder`).
#     If running from a host instead, install the matching OS package.
#   * Free disk roughly equal to the current `ciphertexts` table size; pg_repack
#     builds a shadow copy before swapping.
#
# Required environment variables:
#   DATABASE_URL    - postgres connection string (e.g. postgres://user:pass@host:5432/coprocessor)

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ -f /prepare_database_url.sh ]]; then
  source /prepare_database_url.sh
else
  source "${script_dir}/prepare_database_url.sh"
fi

if [ -z "${DATABASE_URL:-}" ]; then
  echo "ERROR: DATABASE_URL is required"
  exit 1
fi

if ! command -v pg_repack >/dev/null 2>&1; then
  echo "ERROR: pg_repack client binary not found on PATH"
  echo "       install the OS package (e.g. postgresql-17-repack) or run from a host that has it"
  exit 1
fi

echo "Reclaiming storage on public.ciphertexts via pg_repack (online)…"
pg_repack --no-superuser-check --dbname "$DATABASE_URL" --table public.ciphertexts
echo "Reclaim complete"
