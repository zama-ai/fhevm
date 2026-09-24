#!/usr/bin/env bash
# Hold the actual selected GET response or corrupt its delivered body. The
# original MinIO object is never mutated. Parent stdin owns all temporary routes.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
: "${SC_RESTORE_LOG:?}"
: "${MIGRATION_KEY_HEX:?}"
: "${MIGRATION_DATABASE:?}"
: "${MIGRATION_DOWNLOAD_MODE:?}"
[[ "$MIGRATION_KEY_HEX" =~ ^[a-f0-9]{64}$ && "$MIGRATION_DATABASE" == coprocessor_1 && $# -ge 3 ]] || exit 2
case "$MIGRATION_DOWNLOAD_MODE" in interrupt|wrong-digest|malformed|wrong-key) ;; *) exit 2 ;; esac
sc_init
root="$(dirname "$SC_RESTORE_LOG")"
TEST_CONTAINER="${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}"
remote="/tmp/migration-download-${MIGRATION_KEY_HEX}"
proxy_pid=""
owned=()
# Written atomically: the proxy re-reads this file on every response and its
# 250ms watch, so a truncate-then-write window would hand it an empty file.
control() { docker exec "$TEST_CONTAINER" node -e 'const fs=require("fs");fs.writeFileSync(process.argv[1]+".new",JSON.stringify({mode:process.argv[2],until:Date.now()+3600000}));fs.renameSync(process.argv[1]+".new",process.argv[1])' "$remote/control" "$1"; }
binary() { case "$1" in *-poller) echo /usr/local/bin/host_listener_poller ;; *-consumer) echo /usr/local/bin/host_listener_consumer ;; *) echo /usr/local/bin/host_listener ;; esac; }
# docker cp creates the control as root; /tmp's sticky bit prevents the
# normal worker uid from removing it. Only this fixed-path helper runs as root.
private_control() { docker exec -u 0 "$1" "$(binary "$1")" --key-download-control "$2"; }
sql() { docker exec -e "PGPASSWORD=${POSTGRES_PASSWORD:-postgres}" coprocessor-and-kms-db psql -U postgres -d "$MIGRATION_DATABASE" -At -v ON_ERROR_STOP=1 -c "$1"; }
cleanup() {
  local status=$? target
  hc_cleanup_signals
  hc_begin_cleanup || exit 1
  # Clear the route before shutting down its endpoint; restart stopped roles to
  # make the fixed-path control accessible on distroless images.
  for target in "${owned[@]}"; do
    sc_start "$target" || { status=1; continue; }
    private_control "$target" clear >&2 || status=1
  done
  sc_run_restores || status=1
  if [[ -n "$proxy_pid" ]]; then
    control shutdown || status=1
    # The proxy exits within its 250ms watch of the shutdown control. If the
    # test container was recreated underneath it, the exec client can hang
    # until its hour-long cap; bound the wait, then cancel the whole process
    # group so the timeout and docker client die with the job, not just the
    # job's shell.
    local waited=0
    while kill -0 "$proxy_pid" 2>/dev/null && (( waited < 30 )); do sleep 1; waited=$((waited + 1)); done
    if kill -0 "$proxy_pid" 2>/dev/null; then kill -- "-$proxy_pid" 2>/dev/null || true; status=1; fi
    wait "$proxy_pid" 2>/dev/null || status=1
  fi
  docker exec "$TEST_CONTAINER" rm -rf "$remote" >/dev/null 2>&1 || true
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
for target in "$@"; do
  [[ "$target" =~ ^coprocessor1-gcs-host-listener(-[a-zA-Z0-9-]+)?$ && "$(sc_state "$target")" == running ]] || exit 1
  private_control "$target" empty >/dev/null
done
[[ "$(sql "SELECT count(*) FROM keys WHERE key_id=decode('$MIGRATION_KEY_HEX','hex') AND compressed_xof_keyset IS NULL")" == 1 ]] || exit 1
docker exec "$TEST_CONTAINER" rm -rf "$remote"
docker exec "$TEST_CONTAINER" mkdir "$remote"
docker cp "$SCRIPT_DIR/lib/key-download-proxy.cjs" "$TEST_CONTAINER:$remote/proxy.cjs"
wrong_args=()
if [[ "$MIGRATION_DOWNLOAD_MODE" == wrong-key ]]; then
  : "${RFC029_WRONG_KEY_FILE:?generate an independent compressed key fixture}"
  [[ -s "$RFC029_WRONG_KEY_FILE" ]] || exit 1
  (cd "$REPO_ROOT/coprocessor/fhevm-engine"; SQLX_OFFLINE=true cargo run --release -p host-listener --features test-failpoints --bin migration_test_key -- --validate "$RFC029_WRONG_KEY_FILE") >&2
  docker cp "$RFC029_WRONG_KEY_FILE" "$TEST_CONTAINER:$remote/wrong-key"
  wrong_args=("$remote/wrong-key")
fi
control "$MIGRATION_DOWNLOAD_MODE"
# This client owns the proxy for generation, fault observation, and recovery.
# The ordinary 120-second host-command cap expires before those phases finish.
# It runs as its own process group, bypassing the `docker` wrapper function:
# `$!` of a wrapped call is only the subshell, and killing that orphans the
# timeout/docker pair, which then lives on until the hour-long cap.
setsid -w timeout --kill-after=2s 3600s docker exec "$TEST_CONTAINER" node "$remote/proxy.cjs" http://minio:9000 "$MIGRATION_KEY_HEX" "$remote/control" "$remote/ready" "$remote/evidence" "${wrong_args[@]}" >&2 &
proxy_pid=$!
# setsid does not fork for a non-leader child, so the job's pid becomes the
# group id; wait for that to settle before trusting group cancellation.
for ((attempt=0;attempt<50;attempt++)); do
  [[ "$(ps -o pgid= -p "$proxy_pid" 2>/dev/null | tr -d ' ')" == "$proxy_pid" ]] && break
  kill -0 "$proxy_pid" 2>/dev/null || { echo 'proxy client exited before starting' >&2; exit 1; }
  sleep 0.1
done
[[ "$(ps -o pgid= -p "$proxy_pid" 2>/dev/null | tr -d ' ')" == "$proxy_pid" ]] || { echo 'proxy client did not become its own process group' >&2; exit 1; }
ready=""
for ((attempt=0;attempt<30;attempt++)); do
  ready="$(docker exec "$TEST_CONTAINER" cat "$remote/ready" 2>/dev/null)" || ready=""
  [[ -z "$ready" ]] || break
  kill -0 "$proxy_pid" || exit 1
  sleep 1
done
port="$(jq -r .port <<<"$ready")"
[[ "$port" =~ ^[1-9][0-9]+$ ]] || exit 1
network="$(docker inspect -f '{{.HostConfig.NetworkMode}}' "$TEST_CONTAINER")"
[[ "$network" == container:* ]] || exit 1
host="$(docker inspect -f '{{.Name}}' "${network#container:}")"; host="${host#/}"
jq -n --arg key "$MIGRATION_KEY_HEX" --arg endpoint "http://$host:$port" --argjson until "$(($(date +%s)+3600))" '{key:$key,endpoint:$endpoint,until:$until}' > "$root/route.json"
chmod 644 "$root/route.json"
for target in "$@"; do
  owned+=("$target")
  docker cp "$root/route.json" "$target:/tmp/fhevm-test-key-download"
done
printf 'ROLLOUT_HOLD_READY\n'
released=0
deadline=$((SECONDS+1800))
while true; do
  observed="$(docker exec "$TEST_CONTAINER" cat "$remote/evidence" 2>/dev/null)" || observed=""
  if jq -e -s --arg mode "$MIGRATION_DOWNLOAD_MODE" --arg key "$MIGRATION_KEY_HEX" 'any(.[]; .mode==$mode and .key==$key)' <<<"$observed" >/dev/null 2>&1; then break; fi
  ((SECONDS < deadline)) || { echo 'actual key download fault was never observed' >&2; exit 1; }
  if [[ "$released" == 0 ]]; then
    if IFS= read -r -t 1 command; then [[ "$command" == release ]] || exit 2; released=1; else [[ "$?" -gt 128 ]] || exit 1; fi
  else sleep 1; fi
done
[[ "$(sql "SELECT count(*) FROM keys WHERE key_id=decode('$MIGRATION_KEY_HEX','hex') AND compressed_xof_keyset IS NULL")" == 1 ]] || { echo 'faulted bytes were activated' >&2; exit 1; }
if [[ "$MIGRATION_DOWNLOAD_MODE" == interrupt ]]; then
  for target in "$@"; do
    sc_register_restore "$target" start
    before="$(sc_kill "$target")"
    sc_wait_replaced "$target" "$before" 120
    sc_stop "$target"
  done
  [[ "$(sql "SELECT count(*) FROM keys WHERE key_id=decode('$MIGRATION_KEY_HEX','hex') AND compressed_xof_keyset IS NULL")" == 1 ]] || exit 1
else
  deadline=$((SECONDS+300))
  until [[ "$(sql "SELECT count(*) FROM kms_key_activation_events WHERE existing_key_id=decode('$MIGRATION_KEY_HEX','hex') AND status='pending' AND last_error LIKE '%Invalid Key digest%' AND key_content_compressed_xof_keyset IS NULL")" -gt 0 ]]; do
    ((SECONDS < deadline)) || { echo 'real digest validation never rejected the delivered bytes' >&2; exit 1; }
    sleep 1
  done
fi
docker cp "$TEST_CONTAINER:$remote/evidence" "$root/download-evidence.jsonl"
control healthy
# Restore/restart every interrupted owner, but keep the route live: ordinary
# retries must fetch original bytes through this same observed HTTP endpoint.
sc_run_restores
deadline=$((SECONDS+900))
while true; do
  observed="$(docker exec "$TEST_CONTAINER" cat "$remote/evidence")"
  if jq -e -s 'any(.[]; .mode=="healthy")' <<<"$observed" >/dev/null && [[ "$(sql "SELECT count(*) FROM keys WHERE key_id=decode('$MIGRATION_KEY_HEX','hex') AND compressed_xof_keyset IS NOT NULL")" == 1 ]]; then break; fi
  ((SECONDS < deadline)) || { echo 'original migration did not recover through the observed download route' >&2; exit 1; }
  sleep 1
done
docker cp "$TEST_CONTAINER:$remote/evidence" "$root/download-evidence.jsonl"
printf 'KEY_DOWNLOAD_RECOVERED key=%s mode=%s\n' "$MIGRATION_KEY_HEX" "$MIGRATION_DOWNLOAD_MODE"
