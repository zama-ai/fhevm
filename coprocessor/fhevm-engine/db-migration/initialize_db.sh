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

run_block_scope_materialization_wave1_prerequisites() {
  # Pre-upgrade step for the block-scope materialization (wave1) rollout.
  #
  # The wave1 migration 20260610130000 builds idx_host_chain_blocks_valid_parent_hash
  # on the populated, monotonically-growing host_chain_blocks_valid table. A plain
  # CREATE INDEX takes a SHARE lock that blocks block ingestion for the whole build.
  # Build it CONCURRENTLY here, while existing services keep running, so the
  # in-migration CREATE INDEX IF NOT EXISTS later no-ops.
  echo "Pre-creating block-scope materialization (wave1) ancestry index concurrently..."

  # parent_hash is metadata-only (constant NULL default) and must exist before
  # the concurrent index build. Idempotent: no-op if the column already exists.
  run_sql "ALTER TABLE host_chain_blocks_valid
           ADD COLUMN IF NOT EXISTS parent_hash BYTEA NULL DEFAULT NULL;"

  precreate_index "idx_host_chain_blocks_valid_parent_hash" \
    "CREATE INDEX CONCURRENTLY idx_host_chain_blocks_valid_parent_hash \
     ON host_chain_blocks_valid (chain_id, parent_hash);"
}

echo "-------------- Start database initilaization --------------"

echo "Creating database..."
sqlx database create || { echo "Failed to create database."; exit 1; }

# The wave1 squash (#2848) shipped an in-place edit of the already-applied
# migration 20260616120000_bridge_tables.sql; this tree restores the original
# file and carries the delta in 20260704100000 instead. A database whose FIRST
# migration run used the edited file recorded its checksum and would now fail
# `sqlx migrate run` with VersionMismatch before applying anything newer.
# Rewrite exactly that known checksum (SHA-384 of the edited file) to the
# restored file's; a strict no-op everywhere else, including fresh databases
# and databases that applied the original #2734 file.
repair_bridge_tables_migration_checksum() {
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "
    DO \$\$
    BEGIN
      IF to_regclass('_sqlx_migrations') IS NOT NULL THEN
        UPDATE _sqlx_migrations
        SET checksum = decode('36eee489f352fbd4f2c05c3a696b2aa144a7a1c314cbaf814402c49a22b8d166fbdd7e26cb8f996691517df3fde1f6e8', 'hex')
        WHERE version = 20260616120000
          AND checksum = decode('7f80a69bd35610c02950bbc253ac1c34c006217d242f17cd23f23e4fb990d94009587c4fc3fbd8b5ba042f17f0d09810', 'hex');
      END IF;
    END
    \$\$;" || { echo "Failed to repair bridge_tables migration checksum."; exit 1; }
}

echo "Running migrations..."
if [ "${RUN_MIGRATIONS_UNTIL_REMOVE_TENANTS:-}" = "true" ]; then
  # Partial migrations — the host_chains table doesn't exist yet on this path,
  # so do not attempt to seed.
  run_remove_tenants_prerequisites
elif [ "${RUN_BLOCK_SCOPE_WAVE1_PREREQUISITES:-}" = "true" ]; then
  # Pre-upgrade pass: build the wave1 ancestry index CONCURRENTLY against the
  # live DB before `helm upgrade` applies the rest of the wave1 migrations.
  # Does not run the remaining migrations and does not seed.
  run_block_scope_materialization_wave1_prerequisites
else
  repair_bridge_tables_migration_checksum
  sqlx migrate run --source "$MIGRATION_DIR" || { echo "Failed to run migrations."; exit 1; }
  seed_host_chains
fi

echo "Database initialization completed successfully."
