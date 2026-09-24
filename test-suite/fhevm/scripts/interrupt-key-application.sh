#!/usr/bin/env bash
# Hold the real compressed-key UPDATE after download, then kill its owners.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
: "${SC_RESTORE_LOG:?restoration ledger required}"
: "${MIGRATION_KEY_HEX:?key required}"
: "${MIGRATION_DATABASE:?database required}"
[[ "$MIGRATION_DATABASE" =~ ^coprocessor_[0-9]+$ && "$MIGRATION_KEY_HEX" =~ ^[0-9a-f]{64}$ ]] || exit 2
[[ "$#" -ge 3 ]] || exit 2
for target in "$@"; do [[ "$target" =~ ^coprocessor1-gcs-host-listener(-[a-zA-Z0-9-]+)?$ ]] || exit 2; done
sc_init
psql_cmd=(docker exec -i -e "PGPASSWORD=${POSTGRES_PASSWORD:-postgres}" "${POSTGRES_CONTAINER:-coprocessor-and-kms-db}" psql -U "${POSTGRES_USER:-postgres}" -d "$MIGRATION_DATABASE" -At -v ON_ERROR_STOP=1)
sql() { "${psql_cmd[@]}" -c "$1"; }
owned=0
gate_pid=""
gate_write=""
release_gate() {
  if [[ -n "$gate_write" ]]; then
    printf 'SELECT pg_advisory_unlock(721029,1);\n\\q\n' >&"$gate_write" || true
    exec {gate_write}>&-
    gate_write=""
    wait "$gate_pid" || return 1
    gate_pid=""
  fi
}
cleanup() {
  local status=$?
  hc_cleanup_signals
  hc_begin_cleanup || exit 1
  release_gate || status=1
  if [[ "$owned" == 1 ]]; then sql "$(bun "$SCRIPT_DIR/key-application-gate.ts" drop)" >/dev/null || status=1; fi
  sc_run_restores || status=1
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
[[ "$(sql "SELECT to_regprocedure('public.consensus_test_key_application_gate()') IS NULL")" == t ]] || { echo 'previous key gate needs recovery; use a fresh stack' >&2; exit 1; }
[[ "$(sql "SELECT count(*) FROM pg_locks WHERE locktype='advisory' AND classid=721029 AND objid=1 AND objsubid=2")" == 0 ]] || exit 1
for target in "$@"; do [[ "$(sc_state "$target")" == running ]] || exit 1; done
# The private gate session owns the advisory lock; EOF releases it even on SIGKILL.
coproc KEY_GATE { HC_COMMAND_TIMEOUT_SECONDS=2100 "${psql_cmd[@]}"; }
gate_pid="$KEY_GATE_PID"
exec {gate_write}>&"${KEY_GATE[1]}"
exec {gate_read}<&"${KEY_GATE[0]}"
printf "SELECT pg_advisory_lock(721029,1); SELECT 'KEY_GATE_READY';\n" >&"$gate_write"
ready=0
while IFS= read -r -t 30 line <&"$gate_read"; do [[ "$line" != KEY_GATE_READY ]] || { ready=1; break; }; done
[[ "$ready" == 1 ]] || exit 1
sql "$(bun "$SCRIPT_DIR/key-application-gate.ts" install "$MIGRATION_KEY_HEX")" >/dev/null
owned=1
printf 'ROLLOUT_HOLD_READY\n'
deadline=$((SECONDS + 1800))
released=0
until [[ "$(sql "$(bun "$SCRIPT_DIR/key-application-gate.ts" waiters)")" -gt 0 ]]; do
  [[ "$SECONDS" -lt "$deadline" ]] || { echo 'key application never reached the transaction gate' >&2; exit 1; }
  # KMS generation can finish before the listener starts its UPDATE. A normal
  # release requests completion of this fault, not cancellation before it fires.
  # EOF without that request still means the parent disappeared: restore now.
  if [[ "$released" == 0 ]]; then
    if IFS= read -r -t 1 command; then
      [[ "$command" == release ]] || exit 2
      released=1
    else [[ "$?" -gt 128 ]] || exit 1; fi
  else sleep 1; fi
done
[[ "$(sql "SELECT (compressed_xof_keyset IS NULL)::int FROM keys WHERE key_id=decode('$MIGRATION_KEY_HEX','hex')")" == 1 ]] || exit 1
sql "SELECT pid,mode,granted FROM pg_locks WHERE locktype='advisory' AND classid=721029 AND objid=1 AND objsubid=2" > "$(dirname "$SC_RESTORE_LOG")/key-application-boundary.txt"
# All redundant application owners belong to this recipient. Killing each avoids
# claiming interruption of a guessed listener while another owns the transaction.
for target in "$@"; do
  sc_register_restore "$target" start
  before="$(sc_kill "$target")"
  sc_wait_replaced "$target" "$before" 120
  sc_stop "$target"
done
# A backend blocked in the advisory lock may not observe its dead client's
# socket until the lock is released. All application owners are stopped now.
# Removing the trigger takes a table lock, draining those abandoned UPDATEs
# before checking that none committed its material.
release_gate
sql "$(bun "$SCRIPT_DIR/key-application-gate.ts" drop)" >/dev/null
owned=0
[[ "$(sql "$(bun "$SCRIPT_DIR/key-application-gate.ts" waiters)")" == 0 ]] || exit 1
[[ "$(sql "SELECT (compressed_xof_keyset IS NULL)::int FROM keys WHERE key_id=decode('$MIGRATION_KEY_HEX','hex')")" == 1 ]] || { echo 'interrupted transaction partially published key material' >&2; exit 1; }
sc_run_restores
printf 'KEY_APPLICATION_INTERRUPTED key=%s rollback=observed owners=replaced\n' "$MIGRATION_KEY_HEX"
