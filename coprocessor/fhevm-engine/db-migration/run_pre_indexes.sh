#!/bin/bash
set -euo pipefail

if [ -z "${DATABASE_URL:-}" ]; then
  echo "DATABASE_URL must be set"
  exit 1
fi

CONCURRENTLY="${PRE_INDEX_CONCURRENTLY:-false}"
MAX_JOBS="${PRE_INDEX_JOBS:-7}"

declare -a INDEXES=(
  "idx_allowed_handles_no_tenant|allowed_handles|handle, account_address"
  "idx_input_blobs_no_tenant|input_blobs|blob_hash"
  "idx_ciphertext_digest_no_tenant|ciphertext_digest|handle"
  "idx_ciphertexts_no_tenant|ciphertexts|handle, ciphertext_version"
  "idx_ciphertexts128_no_tenant|ciphertexts128|handle"
  "idx_computations_no_tenant|computations|output_handle, transaction_id"
  "idx_pbs_computations_no_tenant|pbs_computations|handle"
)

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

running=0
failed=0

run_index() {
  local name="$1"
  local table="$2"
  local columns="$3"
  local concurrently=""
  local started
  local status

  if [ "$CONCURRENTLY" = "true" ]; then
    concurrently="CONCURRENTLY "
  fi

  status="$(psql "$DATABASE_URL" -AtX -v ON_ERROR_STOP=1 -v index_name="$name" <<'SQL'
SELECT CASE
  WHEN NOT EXISTS (
    SELECT 1
    FROM pg_class c
    JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE n.nspname = 'public'
      AND c.relname = :'index_name'
  ) THEN 'missing'
  WHEN EXISTS (
    SELECT 1
    FROM pg_class c
    JOIN pg_index i ON i.indexrelid = c.oid
    JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE n.nspname = 'public'
      AND c.relname = :'index_name'
      AND i.indisvalid
  ) THEN 'valid'
  ELSE 'invalid'
END;
SQL
)"

  if [ "$status" = "valid" ]; then
    echo "[$name] already valid; skipping"
    return 0
  fi

  if [ "$status" = "invalid" ]; then
    echo "[$name] invalid leftover found; dropping"
    psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "DROP INDEX ${concurrently}IF EXISTS ${name};"
  fi

  started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "[$name] creating on $table ($columns) at $started"
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -c "CREATE UNIQUE INDEX ${concurrently}${name} ON ${table} (${columns});"
  echo "[$name] complete at $(date -u +%Y-%m-%dT%H:%M:%SZ)"
}

echo "-------------- Start pre-index creation --------------"
echo "PRE_INDEX_CONCURRENTLY=$CONCURRENTLY"
echo "PRE_INDEX_JOBS=$MAX_JOBS"
date -u

for spec in "${INDEXES[@]}"; do
  IFS='|' read -r name table columns <<<"$spec"

  (
    run_index "$name" "$table" "$columns"
  ) >"$tmpdir/$name.log" 2>&1 &

  running=$((running + 1))
  if [ "$running" -ge "$MAX_JOBS" ]; then
    if ! wait -n; then
      failed=1
    fi
    running=$((running - 1))
  fi
done

while [ "$running" -gt 0 ]; do
  if ! wait -n; then
    failed=1
  fi
  running=$((running - 1))
done

for log in "$tmpdir"/*.log; do
  cat "$log"
done

date -u
if [ "$failed" -ne 0 ]; then
  echo "Pre-index creation failed"
  exit 1
fi

echo "Pre-index creation completed successfully."
