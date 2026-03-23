#!/bin/bash
# run-coprocessor-db-state-revert-e2e.sh
#
# E2E test for the coprocessor state revert mechanism.
#
# Flow:
#   1. Run E2E tests to populate the coprocessor DB with computations.
#   2. Record the number of completed computations.
#   3. Revert the coprocessor state to a midpoint block.
#   4. Wait for all computations to be redone (same count, no errors).
#   5. Run E2E tests again to verify new work also succeeds after revert.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REVERT_IMAGE="ghcr.io/zama-ai/fhevm/coprocessor/db-migration:${COPROCESSOR_DB_MIGRATION_VERSION}"

POSTGRES_CONTAINER="${POSTGRES_CONTAINER:-coprocessor-and-kms-db}"
POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-postgres}"
POSTGRES_DB="${POSTGRES_DB:-coprocessor}"
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
TESTS_TO_RUN="${TESTS_TO_RUN:-test add 42 to uint64 input and decrypt}"
CHAIN_ID="${CHAIN_ID:-12345}"
REVERT_POLL_TIMEOUT_SECONDS="${REVERT_POLL_TIMEOUT_SECONDS:-300}"
REVERT_POLL_INTERVAL_SECONDS="${REVERT_POLL_INTERVAL_SECONDS:-2}"

# Coprocessor containers to stop/start around the revert.
COPROCESSOR_CONTAINERS=(
  coprocessor-host-listener
  coprocessor-host-listener-poller
  coprocessor-gw-listener
  coprocessor-tfhe-worker
  coprocessor-sns-worker
  coprocessor-transaction-sender
  coprocessor-zkproof-worker
)

stop_coprocessor() {
  echo "  Stopping coprocessor services..."
  for c in "${COPROCESSOR_CONTAINERS[@]}"; do
    docker stop "$c" 2>/dev/null || true
  done
}

start_coprocessor() {
  echo "  Starting coprocessor services..."
  for c in "${COPROCESSOR_CONTAINERS[@]}"; do
    docker start "$c" 2>/dev/null || true
  done
}

# Ensure services are restarted even if the script fails.
trap start_coprocessor EXIT

psql_query() {
  docker exec -e PGPASSWORD="$POSTGRES_PASSWORD" "$POSTGRES_CONTAINER" \
    psql -U "$POSTGRES_USER" -d "$POSTGRES_DB" -t -A -c "$1"
}

run_e2e_tests() {
  docker exec "$TEST_CONTAINER" \
    ./run-tests.sh -n staging -g "$TESTS_TO_RUN"
}

echo "=== Step 1: Run E2E tests to populate coprocessor DB ==="
run_e2e_tests

echo "=== Step 2: Stop coprocessor and record completed computation count ==="
stop_coprocessor

COMP_COMPLETED=$(psql_query "SELECT COUNT(*) FROM computations WHERE is_completed = true AND host_chain_id = $CHAIN_ID")
COMP_TOTAL=$(psql_query "SELECT COUNT(*) FROM computations WHERE host_chain_id = $CHAIN_ID")
ACL_TOTAL=$(psql_query "SELECT COUNT(*) FROM allowed_handles WHERE host_chain_id = $CHAIN_ID")
PBS_TOTAL=$(psql_query "SELECT COUNT(*) FROM pbs_computations WHERE host_chain_id = $CHAIN_ID")
DIGEST_TOTAL=$(psql_query "SELECT COUNT(*) FROM ciphertext_digest WHERE host_chain_id = $CHAIN_ID")
CT_TOTAL=$(psql_query "SELECT COUNT(*) FROM ciphertexts WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = $CHAIN_ID)")
CT128_TOTAL=$(psql_query "SELECT COUNT(*) FROM ciphertexts128 WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = $CHAIN_ID)")
echo "  computations: $COMP_COMPLETED completed / $COMP_TOTAL total"
echo "  allowed_handles: $ACL_TOTAL"
echo "  pbs_computations: $PBS_TOTAL"
echo "  ciphertext_digest: $DIGEST_TOTAL"
echo "  ciphertexts: $CT_TOTAL"
echo "  ciphertexts128: $CT128_TOTAL"

if [ "$COMP_COMPLETED" -eq 0 ]; then
  echo "ERROR: No completed computations found — nothing to revert"
  exit 1
fi

echo "=== Step 3: Revert to midpoint block ==="

MAX_BLOCK=$(psql_query "SELECT COALESCE(MAX(block_number), 0) FROM transactions WHERE chain_id = $CHAIN_ID")
REVERT_TO=$((MAX_BLOCK / 2))

if [ "$REVERT_TO" -le 0 ]; then
  echo "ERROR: Not enough blocks to revert (max_block=$MAX_BLOCK)"
  exit 1
fi

echo "  Max block: $MAX_BLOCK"
echo "  Reverting to: $REVERT_TO"

# Run the revert using the db-migration image.
DB_NETWORK=$(docker inspect "$POSTGRES_CONTAINER" --format '{{range $k, $v := .NetworkSettings.Networks}}{{$k}}{{end}}' | head -1)

echo "  Running revert via $REVERT_IMAGE on network $DB_NETWORK..."
docker run --rm \
  --network "$DB_NETWORK" \
  -e DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_CONTAINER}:5432/${POSTGRES_DB}" \
  -e CHAIN_ID="$CHAIN_ID" \
  -e TO_BLOCK_NUMBER="$REVERT_TO" \
  "$REVERT_IMAGE" \
  "/revert_coprocessor_db_state.sh"

COMP_AFTER=$(psql_query "SELECT COUNT(*) FROM computations WHERE host_chain_id = $CHAIN_ID")
ACL_AFTER=$(psql_query "SELECT COUNT(*) FROM allowed_handles WHERE host_chain_id = $CHAIN_ID")
PBS_AFTER=$(psql_query "SELECT COUNT(*) FROM pbs_computations WHERE host_chain_id = $CHAIN_ID")
DIGEST_AFTER=$(psql_query "SELECT COUNT(*) FROM ciphertext_digest WHERE host_chain_id = $CHAIN_ID")
CT_AFTER=$(psql_query "SELECT COUNT(*) FROM ciphertexts WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = $CHAIN_ID)")
CT128_AFTER=$(psql_query "SELECT COUNT(*) FROM ciphertexts128 WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = $CHAIN_ID)")
echo "  After revert:"
echo "    computations: $COMP_AFTER (was $COMP_TOTAL)"
echo "    allowed_handles: $ACL_AFTER (was $ACL_TOTAL)"
echo "    pbs_computations: $PBS_AFTER (was $PBS_TOTAL)"
echo "    ciphertext_digest: $DIGEST_AFTER (was $DIGEST_TOTAL)"
echo "    ciphertexts: $CT_AFTER (was $CT_TOTAL)"
echo "    ciphertexts128: $CT128_AFTER (was $CT128_TOTAL)"

REVERT_OK=true
[ "$COMP_AFTER" -ge "$COMP_TOTAL" ] && echo "ERROR: Revert did not delete any computations" && REVERT_OK=false
[ "$ACL_AFTER" -ge "$ACL_TOTAL" ] && [ "$ACL_TOTAL" -gt 0 ] && echo "ERROR: Revert did not delete any allowed_handles" && REVERT_OK=false
[ "$PBS_AFTER" -ge "$PBS_TOTAL" ] && [ "$PBS_TOTAL" -gt 0 ] && echo "ERROR: Revert did not delete any pbs_computations" && REVERT_OK=false
[ "$DIGEST_AFTER" -ge "$DIGEST_TOTAL" ] && [ "$DIGEST_TOTAL" -gt 0 ] && echo "ERROR: Revert did not delete any ciphertext_digest" && REVERT_OK=false
[ "$CT_AFTER" -ge "$CT_TOTAL" ] && [ "$CT_TOTAL" -gt 0 ] && echo "ERROR: Revert did not delete any ciphertexts" && REVERT_OK=false
[ "$CT128_AFTER" -ge "$CT128_TOTAL" ] && [ "$CT128_TOTAL" -gt 0 ] && echo "ERROR: Revert did not delete any ciphertexts128" && REVERT_OK=false
if [ "$REVERT_OK" = false ]; then
  exit 1
fi

echo "=== Step 4: Start coprocessor, wait for data to be redone ==="
start_coprocessor
ELAPSED=0
while [ "$ELAPSED" -lt "$REVERT_POLL_TIMEOUT_SECONDS" ]; do
  CUR_COMP_TOTAL=$(psql_query "SELECT COUNT(*) FROM computations WHERE host_chain_id = $CHAIN_ID")
  CUR_COMP_DONE=$(psql_query "SELECT COUNT(*) FROM computations WHERE is_completed = true AND host_chain_id = $CHAIN_ID")
  CUR_ERR=$(psql_query "SELECT COUNT(*) FROM computations WHERE is_error = true AND host_chain_id = $CHAIN_ID")
  CUR_ACL=$(psql_query "SELECT COUNT(*) FROM allowed_handles WHERE host_chain_id = $CHAIN_ID")
  CUR_PBS=$(psql_query "SELECT COUNT(*) FROM pbs_computations WHERE host_chain_id = $CHAIN_ID")
  CUR_DIGEST=$(psql_query "SELECT COUNT(*) FROM ciphertext_digest WHERE host_chain_id = $CHAIN_ID")
  CUR_CT=$(psql_query "SELECT COUNT(*) FROM ciphertexts WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = $CHAIN_ID)")
  CUR_CT128=$(psql_query "SELECT COUNT(*) FROM ciphertexts128 WHERE handle IN (SELECT output_handle FROM computations WHERE host_chain_id = $CHAIN_ID)")

  echo "  [${ELAPSED}s] comp=$CUR_COMP_DONE/$COMP_COMPLETED total=$CUR_COMP_TOTAL/$COMP_TOTAL acl=$CUR_ACL/$ACL_TOTAL pbs=$CUR_PBS/$PBS_TOTAL digest=$CUR_DIGEST/$DIGEST_TOTAL ct=$CUR_CT/$CT_TOTAL ct128=$CUR_CT128/$CT128_TOTAL err=$CUR_ERR"

  if [ "$CUR_ERR" -gt 0 ]; then
    echo "ERROR: Found $CUR_ERR errored computations after revert"
    exit 1
  fi

  if [ "$CUR_COMP_DONE" -ge "$COMP_COMPLETED" ] && \
     [ "$CUR_COMP_TOTAL" -ge "$COMP_TOTAL" ] && \
     [ "$CUR_ACL" -ge "$ACL_TOTAL" ] && \
     [ "$CUR_PBS" -ge "$PBS_TOTAL" ] && \
     [ "$CUR_DIGEST" -ge "$DIGEST_TOTAL" ] && \
     [ "$CUR_CT" -ge "$CT_TOTAL" ] && \
     [ "$CUR_CT128" -ge "$CT128_TOTAL" ]; then
    echo "All data recovered successfully"
    break
  fi

  sleep "$REVERT_POLL_INTERVAL_SECONDS"
  ELAPSED=$((ELAPSED + REVERT_POLL_INTERVAL_SECONDS))
done

if [ "$ELAPSED" -ge "$REVERT_POLL_TIMEOUT_SECONDS" ]; then
  echo "ERROR: Timeout waiting for data to recover. Lagging tables:"
  [ "$CUR_COMP_DONE" -lt "$COMP_COMPLETED" ] && echo "  computations (completed): $CUR_COMP_DONE < $COMP_COMPLETED"
  [ "$CUR_COMP_TOTAL" -lt "$COMP_TOTAL" ] && echo "  computations (total): $CUR_COMP_TOTAL < $COMP_TOTAL"
  [ "$CUR_ACL" -lt "$ACL_TOTAL" ] && echo "  allowed_handles: $CUR_ACL < $ACL_TOTAL"
  [ "$CUR_PBS" -lt "$PBS_TOTAL" ] && echo "  pbs_computations: $CUR_PBS < $PBS_TOTAL"
  [ "$CUR_DIGEST" -lt "$DIGEST_TOTAL" ] && echo "  ciphertext_digest: $CUR_DIGEST < $DIGEST_TOTAL"
  [ "$CUR_CT" -lt "$CT_TOTAL" ] && echo "  ciphertexts: $CUR_CT < $CT_TOTAL"
  [ "$CUR_CT128" -lt "$CT128_TOTAL" ] && echo "  ciphertexts128: $CUR_CT128 < $CT128_TOTAL"
  exit 1
fi

echo "=== Step 5: Run E2E tests again to verify new work succeeds ==="
run_e2e_tests

echo "=== Coprocessor revert E2E test passed ==="