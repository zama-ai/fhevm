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

precreate_blocks_valid_pending_index() {
  local has_table
  has_table=$(psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atc \
    "SELECT to_regclass('public.host_chain_blocks_valid') IS NOT NULL;")
  if [ "$has_table" != "t" ]; then
    echo "Skipping pending-blocks index pre-creation (host_chain_blocks_valid not created yet)"
    return 0
  fi

  # This index only affects performance, so an INVALID leftover from an
  # interrupted build is rebuilt rather than hard-failing every Job retry.
  local index_valid
  index_valid=$(psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atc \
    "SELECT i.indisvalid
       FROM pg_class c
       JOIN pg_index i ON i.indexrelid = c.oid
       JOIN pg_namespace n ON n.oid = c.relnamespace
      WHERE n.nspname = 'public'
        AND c.relname = 'idx_host_chain_blocks_valid_pending';")
  if [ "$index_valid" = "f" ]; then
    echo "Dropping invalid pending-blocks index left by an interrupted build"
    run_sql "DROP INDEX CONCURRENTLY IF EXISTS idx_host_chain_blocks_valid_pending;"
  fi

  echo "Pre-creating the pending-blocks index concurrently..."
  precreate_index "idx_host_chain_blocks_valid_pending" \
    "CREATE INDEX CONCURRENTLY idx_host_chain_blocks_valid_pending \
     ON host_chain_blocks_valid (chain_id, block_number) \
     WHERE block_status = 'pending';"
}

ciphertext_digest_column_state() {
  local column_name="$1"
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atc "
    SELECT CASE
      WHEN to_regclass('public.ciphertext_digest') IS NULL THEN 'no-table'
      WHEN EXISTS (
        SELECT 1
        FROM pg_attribute
        WHERE attrelid = to_regclass('public.ciphertext_digest')
          AND attname = '$column_name'
          AND NOT attisdropped
      ) THEN 'present'
      ELSE 'missing'
    END;"
}

# These indexes only affect performance, so an INVALID leftover from an
# interrupted build is rebuilt rather than hard-failing every Job retry.
drop_invalid_index() {
  local index_name="$1"
  local index_valid
  index_valid=$(psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atc "
    SELECT i.indisvalid
      FROM pg_class c
      JOIN pg_index i ON i.indexrelid = c.oid
      JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE n.nspname = 'public'
       AND c.relname = '$index_name';")
  if [ "$index_valid" = "f" ]; then
    echo "Dropping invalid index $index_name left by an interrupted build"
    run_sql "DROP INDEX CONCURRENTLY IF EXISTS $index_name;"
  fi
}

# 20260526090000 sets s3_format_version = 0 on every uploaded row with a
# full-table UPDATE (about an hour on testnet). Before that migration runs,
# add the column with DEFAULT 0 instead. A constant default given when the
# column is added applies to every existing row: Postgres stores it once in
# the catalog and does not rewrite the table. A later SET DEFAULT only affects
# new rows, so restoring DEFAULT NULL keeps existing rows at 0. Resetting the
# version of not-yet-uploaded rows to NULL then gives the state the migration
# would leave: it has nothing to update and its file stays unchanged.
instant_prefill_s3_format_version() {
  local column_state
  column_state=$(ciphertext_digest_column_state s3_format_version)
  if [ "$column_state" != "missing" ]; then
    echo "Skipping s3_format_version prefill (column $column_state)"
    return 0
  fi

  echo "Prefilling s3_format_version without a table rewrite..."
  run_sql "
    ALTER TABLE ciphertext_digest ADD COLUMN s3_format_version SMALLINT DEFAULT 0;
    ALTER TABLE ciphertext_digest ALTER COLUMN s3_format_version SET DEFAULT NULL;
    UPDATE ciphertext_digest
       SET s3_format_version = NULL
     WHERE ciphertext IS NULL
       AND ciphertext128 IS NULL;"
}

# The SNS startup backfill and 20260526090000 select digest-bearing rows with
# a NULL version. Without this index each run reads the whole table, which
# exceeds the SNS statement timeout on large databases and crash-loops the
# worker even when no row is left to update.
precreate_s3_format_version_null_index() {
  local column_state
  column_state=$(ciphertext_digest_column_state s3_format_version)
  if [ "$column_state" != "present" ]; then
    echo "Skipping s3_format_version index pre-creation (column $column_state)"
    return 0
  fi

  drop_invalid_index "idx_ciphertext_digest_s3_format_version_null"
  precreate_index "idx_ciphertext_digest_s3_format_version_null" \
    "CREATE INDEX CONCURRENTLY idx_ciphertext_digest_s3_format_version_null \
     ON ciphertext_digest (handle) \
     WHERE s3_format_version IS NULL \
       AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL);"
}

# Before every batch the S3 migration counts its pending and failed rows. The
# migration pending index also carries the 32-byte handle (1.4 GB for 21M rows
# on testnet, about 5 s per count). Keyed on the failure count alone, Postgres
# deduplicates the entries into a ~10x smaller index, and the failed count
# becomes a range on its first column.
precreate_s3_migration_pending_count_index() {
  local column_state
  column_state=$(ciphertext_digest_column_state s3_migration_failure_count)
  if [ "$column_state" != "present" ]; then
    echo "Skipping S3 migration count index pre-creation (column $column_state)"
    return 0
  fi

  drop_invalid_index "idx_ciphertext_digest_s3_migration_pending_count"
  precreate_index "idx_ciphertext_digest_s3_migration_pending_count" \
    "CREATE INDEX CONCURRENTLY idx_ciphertext_digest_s3_migration_pending_count \
     ON ciphertext_digest (s3_migration_failure_count) \
     WHERE s3_format_version = 0 \
       AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL);"
}

# Those counts are index-only scans, which skip the table only on pages VACUUM
# marked all-visible. Each migrated row leaves a dead index entry and clears
# that mark, and autovacuum by default waits for 10% of the table to be dead
# (days on testnet), so the counts slowly read the table again. While rows that
# were never attempted remain, vacuum before the SNS workers start and let
# autovacuum run at 1% dead rows; reset that setting once they are gone. Rows
# that keep failing do not count: they may never migrate, and alone they leave
# both counts cheap. VACUUM does not block reads or writes; TRUNCATE false also
# skips its brief exclusive lock for giving trailing empty pages back to the
# filesystem.
tune_ciphertext_digest_for_s3_migration() {
  local migration_pending
  migration_pending=$(psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atc "
    SELECT EXISTS (
      SELECT 1
      FROM ciphertext_digest
      WHERE s3_format_version = 0
        AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
        AND s3_migration_failure_count = 0
    );")
  local autovacuum_tuned
  autovacuum_tuned=$(psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -Atc "
    SELECT coalesce('autovacuum_vacuum_scale_factor=0.01' = ANY (reloptions), false)
    FROM pg_class
    WHERE oid = 'public.ciphertext_digest'::regclass;")

  if [ "$migration_pending" != "t" ]; then
    if [ "$autovacuum_tuned" = "t" ]; then
      echo "No S3 migration row left to attempt, resetting ciphertext_digest autovacuum threshold"
      run_sql "ALTER TABLE ciphertext_digest RESET (autovacuum_vacuum_scale_factor);"
    fi
    return 0
  fi

  if [ "$autovacuum_tuned" != "t" ]; then
    echo "Lowering ciphertext_digest autovacuum threshold during the S3 migration"
    run_sql "ALTER TABLE ciphertext_digest SET (autovacuum_vacuum_scale_factor = 0.01);"
  fi
  echo "Vacuuming ciphertext_digest before the S3 migration..."
  run_sql "VACUUM (ANALYZE, TRUNCATE false) ciphertext_digest;"
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
  precreate_blocks_valid_pending_index
  instant_prefill_s3_format_version
  precreate_s3_format_version_null_index
  sqlx migrate run --source "$MIGRATION_DIR" || { echo "Failed to run migrations."; exit 1; }
  # A fresh database only has the column once the migrations ran.
  precreate_s3_format_version_null_index
  precreate_s3_migration_pending_count_index
  tune_ciphertext_digest_for_s3_migration
  seed_host_chains
fi

echo "Database initialization completed successfully."
