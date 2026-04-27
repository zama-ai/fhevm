#!/bin/bash
set -e

# Default to using absolute paths needed for Docker containers
# Using arg --no-absolute-paths is needed for local DB initialization
USE_ABSOLUTE_PATHS=true

for arg in "$@"; do
  if [[ "$arg" == "--no-absolute-paths" ]]; then
    USE_ABSOLUTE_PATHS=false
  fi
done

if [ "$USE_ABSOLUTE_PATHS" = true ]; then
  MIGRATION_DIR="/migrations"
else
  MIGRATION_DIR="./migrations"
fi

REMOVE_TENANTS_PREVIOUS_VERSION=20260120102002

run_sql() {
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "$1"
}

precreate_index() {
  local index_name="$1"
  local create_sql="$2"

  local index_state
  index_state=$(psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atc "
    SELECT CASE
      WHEN EXISTS (
        SELECT 1
        FROM pg_class c
        JOIN pg_index i ON i.indexrelid = c.oid
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
          AND c.relname = '$index_name'
          AND i.indisvalid
      ) THEN 'valid'
      WHEN EXISTS (
        SELECT 1
        FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
          AND c.relname = '$index_name'
      ) THEN 'invalid'
      ELSE 'missing'
    END")

  case "$index_state" in
    valid)
      echo "Skipping valid pre-created index $index_name"
      ;;
    missing)
      echo "Creating pre-upgrade index $index_name"
      run_sql "$create_sql"
      ;;
    *)
      echo "Index $index_name exists but is invalid; drop it before rerunning pre-upgrade migrations."
      exit 1
      ;;
  esac
}

run_remove_tenants_prerequisites() {
  echo "Running online migrations before remove_tenants..."
  sqlx migrate run --source $MIGRATION_DIR --target-version $REMOVE_TENANTS_PREVIOUS_VERSION || { echo "Failed to run migrations."; exit 1; }

  # These indexes are required by remove_tenants-era tenant-free queries. Build
  # them concurrently while 0.11 services are still running; the blocking
  # remove_tenants migration is run later during maintenance.
  precreate_index "idx_allowed_handles_no_tenant" \
    "CREATE UNIQUE INDEX CONCURRENTLY idx_allowed_handles_no_tenant ON allowed_handles (handle, account_address);"
  precreate_index "idx_input_blobs_no_tenant" \
    "CREATE UNIQUE INDEX CONCURRENTLY idx_input_blobs_no_tenant ON input_blobs (blob_hash);"
  precreate_index "idx_ciphertext_digest_no_tenant" \
    "CREATE UNIQUE INDEX CONCURRENTLY idx_ciphertext_digest_no_tenant ON ciphertext_digest (handle);"
  precreate_index "idx_ciphertexts_no_tenant" \
    "CREATE UNIQUE INDEX CONCURRENTLY idx_ciphertexts_no_tenant ON ciphertexts (handle, ciphertext_version);"
  precreate_index "idx_ciphertexts128_no_tenant" \
    "CREATE UNIQUE INDEX CONCURRENTLY idx_ciphertexts128_no_tenant ON ciphertexts128 (handle);"
  precreate_index "idx_computations_no_tenant" \
    "CREATE UNIQUE INDEX CONCURRENTLY idx_computations_no_tenant ON computations (output_handle, transaction_id);"
  precreate_index "idx_pbs_computations_no_tenant" \
    "CREATE UNIQUE INDEX CONCURRENTLY idx_pbs_computations_no_tenant ON pbs_computations (handle);"
}

echo "-------------- Start database initilaization --------------"

echo "Creating database..."
sqlx database create || { echo "Failed to create database."; exit 1; }

echo "Running migrations..."
if [ "$RUN_MIGRATIONS_UNTIL_REMOVE_TENANTS" = "true" ]; then
  run_remove_tenants_prerequisites
else
  sqlx migrate run --source $MIGRATION_DIR || { echo "Failed to run migrations."; exit 1; }
fi

echo "Database initialization completed successfully."
