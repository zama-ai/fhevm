#!/bin/bash
set -euo pipefail

INSTANCE_INDEX="${1:-1}"
TIMEOUT_SECONDS="${DRIFT_INJECT_TIMEOUT_SECONDS:-180}"
POLL_INTERVAL_SECONDS="${DRIFT_INJECT_POLL_INTERVAL_SECONDS:-2}"
POSTGRES_CONTAINER="${POSTGRES_CONTAINER:-coprocessor-and-kms-db}"
POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-postgres}"

if ! [[ "$INSTANCE_INDEX" =~ ^[0-9]+$ ]]; then
  echo "instance index must be a non-negative integer" >&2
  exit 1
fi

db_name="coprocessor"
if [ "$INSTANCE_INDEX" -gt 0 ]; then
  db_name="coprocessor_${INSTANCE_INDEX}"
fi

psql_query() {
  docker exec -e PGPASSWORD="$POSTGRES_PASSWORD" "$POSTGRES_CONTAINER" \
    psql -U "$POSTGRES_USER" -d "$db_name" -t -A -c "$1"
}

baseline_handles="$(psql_query "SELECT encode(handle, 'hex') FROM ciphertext_digest;" || true)"
deadline=$((SECONDS + TIMEOUT_SECONDS))

while [ "$SECONDS" -lt "$deadline" ]; do
  ready_handles="$(psql_query "SELECT encode(handle, 'hex') FROM ciphertext_digest WHERE txn_is_sent = false AND ciphertext IS NOT NULL AND ciphertext128 IS NOT NULL ORDER BY created_at DESC;")"
  while IFS= read -r handle_hex; do
    [ -z "$handle_hex" ] && continue
    if printf '%s\n' "$baseline_handles" | grep -Fxq "$handle_hex"; then
      continue
    fi

    psql_query "UPDATE ciphertext_digest SET ciphertext = set_byte(ciphertext, 0, get_byte(ciphertext, 0) # 1) WHERE handle = decode('${handle_hex}', 'hex') AND txn_is_sent = false;"
    echo "$handle_hex"
    exit 0
  done <<< "$ready_handles"

  sleep "$POLL_INTERVAL_SECONDS"
done

echo "timed out waiting for a new ready ciphertext_digest row in ${db_name}" >&2
exit 1
