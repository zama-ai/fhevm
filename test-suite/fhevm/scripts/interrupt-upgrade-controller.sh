#!/usr/bin/env bash
# Own one opt-in interruption from hook installation through supervised recovery.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
: "${SC_RESTORE_LOG:?restoration ledger required}"
: "${UPGRADE_FAULT_DATABASE:?database required}"
: "${UPGRADE_FAULT_VERSION:?candidate version required}"
: "${UPGRADE_FAULT_OLD_VERSION:?live version required}"
: "${BLUE_GREEN_INTERRUPT:?boundary required}"
case "$BLUE_GREEN_INTERRUPT" in dry-run-started|before-cutover-commit|after-cutover-commit) ;; *) exit 2 ;; esac
[[ "$UPGRADE_FAULT_DATABASE" =~ ^coprocessor_[0-9]+$ ]] || exit 2
[[ "$UPGRADE_FAULT_VERSION" =~ ^[a-zA-Z0-9.+_-]+$ && "$UPGRADE_FAULT_OLD_VERSION" =~ ^[a-zA-Z0-9.+_-]+$ ]] || exit 2
target=coprocessor1-gcs-upgrade-controller
sc_init
sql() { docker exec -e "PGPASSWORD=${POSTGRES_PASSWORD:-postgres}" "${POSTGRES_CONTAINER:-coprocessor-and-kms-db}" psql -U "${POSTGRES_USER:-postgres}" -d "$UPGRADE_FAULT_DATABASE" -At -v ON_ERROR_STOP=1 -c "$1"; }
owned=0
cleanup() {
  local status=$?
  hc_cleanup_signals
  hc_begin_cleanup || exit 1
  if [[ "$owned" == 1 ]]; then
    sql "DELETE FROM public.consensus_test_upgrade_fault WHERE stage='$BLUE_GREEN_INTERRUPT' AND version='$UPGRADE_FAULT_VERSION'" >/dev/null || status=1
  fi
  sc_run_restores || status=1
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
# A previous controller campaign must be reconciled before using this stack.
[[ "$(sql "SELECT to_regclass('public.consensus_test_upgrade_fault') IS NULL")" == t ]] || { echo 'upgrade fault table already exists; use a fresh isolated stack' >&2; exit 1; }
sql "CREATE TABLE public.consensus_test_upgrade_fault(stage text PRIMARY KEY,version text NOT NULL,reached boolean NOT NULL DEFAULT false,observed_at timestamptz)" >/dev/null
owned=1
sql "INSERT INTO public.consensus_test_upgrade_fault(stage,version) VALUES('$BLUE_GREEN_INTERRUPT','$UPGRADE_FAULT_VERSION')" >/dev/null
if [[ "$(sc_restart_budget "$target")" == exhausted ]]; then sc_reset_restart_budget "$target"; fi
printf 'ROLLOUT_HOLD_READY\n'
deadline=$((SECONDS + 1800))
until [[ "$(sql "SELECT reached FROM public.consensus_test_upgrade_fault WHERE stage='$BLUE_GREEN_INTERRUPT'")" == t ]]; do
  [[ "$SECONDS" -lt "$deadline" ]] || { echo 'upgrade hook was never observed' >&2; exit 1; }
  # EOF means the owning CLI died. A release before the hook is a failed setup.
  if IFS= read -r -t 1 command; then exit 1; else [[ "$?" -gt 128 ]] || exit 1; fi
done
expected="$UPGRADE_FAULT_OLD_VERSION"
[[ "$BLUE_GREEN_INTERRUPT" != after-cutover-commit ]] || expected="$UPGRADE_FAULT_VERSION"
observed="$(sql "SELECT stack_version FROM public.versioning WHERE singleton")"
[[ "$observed" == "$expected" ]] || { echo "upgrade hook has wrong durable side: expected=$expected observed=$observed" >&2; exit 1; }
sql "SELECT stage,version,observed_at FROM public.consensus_test_upgrade_fault WHERE reached" > "$(dirname "$SC_RESTORE_LOG")/boundary.txt"
before="$(sc_kill "$target")"
sc_wait_replaced "$target" "$before" 120
printf 'UPGRADE_INTERRUPT_OBSERVED stage=%s version=%s replacement=%s\n' "$BLUE_GREEN_INTERRUPT" "$observed" "$(sc_identity "$target")"
