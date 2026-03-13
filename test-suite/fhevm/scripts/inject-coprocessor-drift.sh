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

install_trigger() {
  docker exec -i -e PGPASSWORD="$POSTGRES_PASSWORD" "$POSTGRES_CONTAINER" \
    psql -U "$POSTGRES_USER" -d "$db_name" >/dev/null <<'SQL'
CREATE TABLE IF NOT EXISTS drift_injection_state (
  id BOOLEAN PRIMARY KEY DEFAULT TRUE,
  enabled BOOLEAN NOT NULL,
  consumed BOOLEAN NOT NULL DEFAULT FALSE,
  injected_handle BYTEA
);

INSERT INTO drift_injection_state (id, enabled, consumed, injected_handle)
VALUES (TRUE, TRUE, FALSE, NULL)
ON CONFLICT (id) DO UPDATE
SET enabled = EXCLUDED.enabled,
    consumed = EXCLUDED.consumed,
    injected_handle = EXCLUDED.injected_handle;

CREATE OR REPLACE FUNCTION inject_ciphertext_drift_once()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
  should_inject BOOLEAN;
BEGIN
  SELECT enabled AND NOT consumed
  INTO should_inject
  FROM drift_injection_state
  WHERE id = TRUE;

  IF NOT COALESCE(should_inject, FALSE) THEN
    RETURN NEW;
  END IF;

  IF NEW.txn_is_sent = FALSE
     AND NEW.ciphertext IS NOT NULL
     AND NEW.ciphertext128 IS NOT NULL
     AND (OLD.ciphertext IS NULL OR OLD.ciphertext128 IS NULL) THEN
    NEW.ciphertext := set_byte(NEW.ciphertext, 0, get_byte(NEW.ciphertext, 0) # 1);

    UPDATE drift_injection_state
    SET consumed = TRUE,
        injected_handle = NEW.handle
    WHERE id = TRUE;
  END IF;

  RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;

CREATE TRIGGER ciphertext_drift_injector
BEFORE UPDATE ON ciphertext_digest
FOR EACH ROW
EXECUTE FUNCTION inject_ciphertext_drift_once();
SQL
}

cleanup_trigger() {
  docker exec -i -e PGPASSWORD="$POSTGRES_PASSWORD" "$POSTGRES_CONTAINER" \
    psql -U "$POSTGRES_USER" -d "$db_name" >/dev/null <<'SQL'
DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;
DROP FUNCTION IF EXISTS inject_ciphertext_drift_once();
DROP TABLE IF EXISTS drift_injection_state;
SQL
}

trap cleanup_trigger EXIT

install_trigger

deadline=$((SECONDS + TIMEOUT_SECONDS))
while [ "$SECONDS" -lt "$deadline" ]; do
  consumed="$(psql_query "SELECT consumed::int FROM drift_injection_state WHERE id = TRUE;")"
  if [ "$consumed" = "1" ]; then
    handle_hex="$(psql_query "SELECT encode(injected_handle, 'hex') FROM drift_injection_state WHERE id = TRUE;")"
    if [ -n "$handle_hex" ]; then
      echo "$handle_hex"
      exit 0
    fi
  fi
  sleep "$POLL_INTERVAL_SECONDS"
done

echo "timed out waiting for drift injection trigger to fire in ${db_name}" >&2
exit 1
