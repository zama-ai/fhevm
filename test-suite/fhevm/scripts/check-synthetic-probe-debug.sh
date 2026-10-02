#!/usr/bin/env bash
# Debug-only: proves the dry-run probe never reached a publishing path after
# cutover. For each coprocessor database, prints the probe's manifest-only copy
# and counts its rows in the live tables, then looks for the probe handle in
# every coprocessor container log and asks the Gateway whether its material was
# ever added. Never fails the job.
set -uo pipefail

db_container="${POSTGRES_CONTAINER:-coprocessor-and-kms-db}"
db_user="${POSTGRES_USER:-postgres}"
gateway_rpc="${GATEWAY_RPC:-http://localhost:8546}"
psql_in() { docker exec "$db_container" psql -U "$db_user" -d "$1" -v ON_ERROR_STOP=0 -P pager=off "${@:2}"; }

handles=""
for db in $(docker exec "$db_container" psql -U "$db_user" -At -c \
    "SELECT datname FROM pg_database WHERE datname LIKE 'coprocessor%' ORDER BY datname" 2>/dev/null); do
  echo "::group::synthetic probe in $db"
  psql_in "$db" -c "SELECT host_chain_id, encode(handle, 'hex') AS handle,
                           encode(key_id_gw, 'hex') AS key_id_gw, encode(ciphertext, 'hex') AS ct64,
                           encode(ciphertext128, 'hex') AS ct128, ciphertext128_format, is_error, created_at
                      FROM public.synthetic_handle_digest"
  psql_in "$db" -c "SELECT p.host_chain_id, p.producer_block_number, encode(p.handle, 'hex') AS handle, p.synthetic
                      FROM public.handle_producer_block p WHERE p.synthetic"
  for table in ciphertext_digest ciphertexts ciphertexts128 allowed_handles pbs_computations computations; do
    column=handle; [ "$table" = computations ] && column=output_handle
    psql_in "$db" -At -c "SELECT 'public.$table rows for the probe: ' || count(*)
                            FROM public.$table WHERE $column IN (SELECT handle FROM public.synthetic_handle_digest)"
  done
  echo "::endgroup::"
  handles="$handles $(psql_in "$db" -At -c "SELECT encode(handle, 'hex') FROM public.synthetic_handle_digest")"
done

commits=$(docker inspect coprocessor-gcs-transaction-sender coprocessor-transaction-sender \
            --format '{{range .Args}}{{println .}}{{end}}' 2>/dev/null \
          | sed -n 's/^--ciphertext-commits-address=//p' | head -1)
for handle in $(echo "$handles" | tr ' ' '\n' | grep -E '^[0-9a-f]{64}$' | sort -u); do
  echo "::group::probe 0x$handle in logs and on the Gateway"
  for container in $(docker ps -a --format '{{.Names}}' | grep -E '^coprocessor[0-9]*(-gcs)?-' | sort); do
    echo "$container: $(docker logs "$container" 2>&1 | grep -ci "$handle") line(s)"
    case "$container" in
      *transaction-sender|*sns-worker) docker logs --timestamps "$container" 2>&1 | grep -i "$handle" | head -20 ;;
    esac
  done
  if [ -n "$commits" ]; then
    echo "CiphertextCommits $commits isCiphertextMaterialAdded(0x$handle) = $(cast call "$commits" \
      'isCiphertextMaterialAdded(bytes32)(bool)' "0x$handle" --rpc-url "$gateway_rpc" 2>&1)"
  else
    echo "CiphertextCommits address not found in the transaction-sender arguments"
  fi
  echo "::endgroup::"
done
