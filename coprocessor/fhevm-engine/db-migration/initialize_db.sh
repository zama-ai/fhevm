#!/bin/bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ -f /prepare_database_url.sh ]]; then
  source /prepare_database_url.sh
else
  source "${script_dir}/prepare_database_url.sh"
fi

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
  MIGRATION_DIR="${script_dir}/migrations"
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

insert_host_chain_row() {
  local chain_id="$1" name="$2" acl="$3"
  echo "  INSERT host_chains chain_id=$chain_id name=$name acl=$acl"
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c \
    "INSERT INTO host_chains (chain_id, name, acl_contract_address) \
     VALUES ('$chain_id', '$name', '$acl') \
     ON CONFLICT (chain_id) DO NOTHING;"
}

seed_host_chains() {
  # Idempotent seeding of the `host_chains` table.
  #
  # Source of truth: numbered env vars rendered by the helm chart from
  # .Values.chains:
  #   HOST_CHAINS_COUNT=2
  #   HOST_CHAIN_0_ID=11155111   HOST_CHAIN_0_NAME=sepolia      HOST_CHAIN_0_ACL=<addr>
  #   HOST_CHAIN_1_ID=80002      HOST_CHAIN_1_NAME=polygonAmoy  HOST_CHAIN_1_ACL=<addr>
  # Each *_ACL env var is injected by the chart via the same
  # `valueFrom: configMapKeyRef` the listeners use, so kubelet resolves the
  # literal address at pod-start from the shared contract-address ConfigMap.
  #
  # No jq dependency: keeps the runtime image (Chainguard postgres) minimal.
  local count="${HOST_CHAINS_COUNT:-0}"
  if [[ "$count" -eq 0 ]]; then
    echo "HOST_CHAINS_COUNT is 0 or unset; skipping host_chains seeding."
    return 0
  fi

  echo "Seeding host_chains: $count entries"
  local i
  for ((i = 0; i < count; i++)); do
    local id_var="HOST_CHAIN_${i}_ID"
    local name_var="HOST_CHAIN_${i}_NAME"
    local acl_var="HOST_CHAIN_${i}_ACL"
    local chain_id="${!id_var:-}"
    local name="${!name_var:-}"
    local acl="${!acl_var:-}"
    if [[ -z "$chain_id" || -z "$name" || -z "$acl" ]]; then
      echo "Error: host_chains entry $i is missing one or more of $id_var, $name_var, $acl_var."
      echo "  Check that the chart wired chainId, name, and a literal/valueFrom ACL for this chain."
      exit 1
    fi
    insert_host_chain_row "$chain_id" "$name" "$acl"
  done

  echo "host_chains seeding completed."
}

run_remove_tenants_prerequisites() {
  echo "Running online migrations before remove_tenants..."
  sqlx migrate run --source "$MIGRATION_DIR" --target-version $REMOVE_TENANTS_PREVIOUS_VERSION || { echo "Failed to run migrations."; exit 1; }

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
if [ "${RUN_MIGRATIONS_UNTIL_REMOVE_TENANTS:-}" = "true" ]; then
  # Partial migrations — the host_chains table doesn't exist yet on this path,
  # so do not attempt to seed.
  run_remove_tenants_prerequisites
else
  sqlx migrate run --source "$MIGRATION_DIR" || { echo "Failed to run migrations."; exit 1; }
  seed_host_chains
fi

echo "Database initialization completed successfully."
