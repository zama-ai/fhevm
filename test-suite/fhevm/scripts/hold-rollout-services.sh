#!/usr/bin/env bash
# Keep an observed rollout outage owned until the parent releases it. EOF also
# restores services if the parent exits or is killed. The durable ledger remains
# available when recovery fails.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
: "${SC_RESTORE_LOG:?a rollout-owned restoration ledger is required}"
sc_init
cleanup() {
  local status=$?
  hc_cleanup_signals
  hc_begin_cleanup
  if ! sc_run_restores; then status=1; fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
[[ $# -gt 0 ]] || exit 2
for target in "$@"; do
  [[ "$target" =~ ^coprocessor[0-9]*-gcs-host-listener(-[a-zA-Z0-9-]+)?$ ]] || exit 2
  sc_stop "$target"
done
printf 'ROLLOUT_HOLD_READY\n'
IFS= read -r command || exit 1
[[ "$command" == release ]] || exit 2
