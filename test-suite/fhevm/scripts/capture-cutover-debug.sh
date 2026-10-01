#!/usr/bin/env bash
# Debug-only: full container logs and the database state around the latest
# upgrade window, for diagnosing a computation left uncomputed at cutover.
# Never fails the job: every command is best effort.
set -uo pipefail

out="${1:?output directory}"
db_container="${POSTGRES_CONTAINER:-coprocessor-and-kms-db}"
db_user="${POSTGRES_USER:-postgres}"
mkdir -p "$out/logs" "$out/db"

for container in $(docker ps -a --format '{{.Names}}' | grep -E '^coprocessor[0-9]*(-gcs)?-' | sort); do
  docker logs --timestamps "$container" > "$out/logs/$container.log" 2>&1 || true
done
docker ps -a --format '{{.Names}}\t{{.Status}}\t{{.Image}}' > "$out/containers.tsv" 2>&1 || true

psql_in() { docker exec "$db_container" psql -U "$db_user" -d "$1" -v ON_ERROR_STOP=0 -P pager=off -c "$2"; }

# Window of the latest proposal, per chain: from start_block to a few blocks
# past the upload boundary cutover recorded (or start_block + 30 before it).
window_cte="WITH epoch AS (
    SELECT consensus_epoch FROM public.consensus_epoch_history
     WHERE proposal_id IS NOT NULL ORDER BY allocated_at DESC LIMIT 1),
  w AS (
    SELECT bw.host_chain_id, bw.start_block,
           COALESCE(bw.upload_start_block, bw.start_block + 30) + 5 AS last_block
      FROM public.consensus_epoch_block_window bw JOIN epoch USING (consensus_epoch))"

for db in $(docker exec "$db_container" psql -U "$db_user" -At -c \
    "SELECT datname FROM pg_database WHERE datname LIKE 'coprocessor%' ORDER BY datname" 2>/dev/null); do
  file="$out/db/$db.txt"
  {
    echo "### epochs, windows, upgrade state"
    psql_in "$db" "SELECT * FROM public.consensus_epoch_history ORDER BY allocated_at"
    psql_in "$db" "SELECT * FROM public.consensus_epoch_block_window ORDER BY consensus_epoch, host_chain_id"
    psql_in "$db" "SELECT * FROM public.upgrade_state"
    psql_in "$db" "SELECT * FROM public.versioning"
    psql_in "$db" "SELECT nspname FROM pg_namespace WHERE nspname NOT LIKE 'pg_%' AND nspname <> 'information_schema'"
    # Green's dry-run tables are renamed to "pre-cutover" at cutover; public holds the merge.
    for schema in public '"pre-cutover"'; do
      echo "### schema $schema: computations in the window"
      psql_in "$db" "SET search_path = $schema, public; $window_cte
        SELECT c.block_number, encode(c.output_handle, 'hex') AS output_handle,
               encode(c.transaction_id, 'hex') AS transaction_id,
               encode(c.dependence_chain_id, 'hex') AS dependence_chain_id,
               c.is_completed, c.is_error, c.is_allowed, c.schedule_order, c.error_message
          FROM computations c JOIN w ON w.host_chain_id = c.host_chain_id
         WHERE c.block_number BETWEEN w.start_block AND w.last_block
         ORDER BY c.block_number, c.schedule_order"
      echo "### schema $schema: dependence chains of those computations"
      psql_in "$db" "SET search_path = $schema, public; $window_cte
        SELECT encode(d.dependence_chain_id, 'hex') AS dependence_chain_id, d.status, d.worker_id,
               d.lock_acquired_at, d.lock_expires_at, d.last_updated_at
          FROM dependence_chain d
         WHERE d.dependence_chain_id IN (
           SELECT c.dependence_chain_id FROM computations c JOIN w ON w.host_chain_id = c.host_chain_id
            WHERE c.block_number BETWEEN w.start_block AND w.last_block)"
      echo "### schema $schema: stored ciphertexts and digests of those outputs"
      psql_in "$db" "SET search_path = $schema, public; $window_cte
        SELECT encode(ct.handle, 'hex') AS handle, ct.ciphertext_version, octet_length(ct.ciphertext) AS bytes
          FROM ciphertexts ct
         WHERE ct.handle IN (
           SELECT c.output_handle FROM computations c JOIN w ON w.host_chain_id = c.host_chain_id
            WHERE c.block_number BETWEEN w.start_block AND w.last_block)"
      psql_in "$db" "SET search_path = $schema, public; $window_cte
        SELECT encode(d.handle, 'hex') AS handle, encode(d.ciphertext, 'hex') AS ct64,
               encode(d.ciphertext128, 'hex') AS ct128, d.ciphertext128_format, d.txn_is_sent
          FROM ciphertext_digest d
         WHERE d.handle IN (
           SELECT c.output_handle FROM computations c JOIN w ON w.host_chain_id = c.host_chain_id
            WHERE c.block_number BETWEEN w.start_block AND w.last_block)"
    done
    echo "### manifest state of the window blocks"
    psql_in "$db" "$window_cte
      SELECT m.consensus_epoch, m.host_chain_id, m.block_number, encode(m.block_hash, 'hex') AS block_hash,
             m.block_handle_count, m.manifest_published, m.publication_error_count, m.updated_at
        FROM public.block_manifest_state m JOIN w ON w.host_chain_id = m.host_chain_id
       WHERE m.block_number BETWEEN w.start_block AND w.last_block
       ORDER BY m.consensus_epoch, m.block_number"
    echo "### drifted handles"
    psql_in "$db" "SELECT id, consensus_epoch, block_number, encode(handle, 'hex') AS handle, reason,
                          detection_kind, is_contained, healed_at, superseded_at, heal_attempts, detected_at
                     FROM public.drifted_handle ORDER BY id"
  } > "$file" 2>&1
done
echo "cutover debug written to $out"
