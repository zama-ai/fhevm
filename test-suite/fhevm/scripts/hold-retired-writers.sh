#!/usr/bin/env bash
# Challenge real retired release binaries with prepared canonical work while
# their promoted replacements are stopped. Restoring Green remains mandatory.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
source "$SCRIPT_DIR/lib/retired-writer-evidence.sh"
: "${SC_RESTORE_LOG:?}"
: "${RETIRED_OPERATOR_COUNT:?}"
[[ "$RETIRED_OPERATOR_COUNT" =~ ^[23]$ ]] || exit 2
[[ $# == 1 && ( "$1" == sns-worker || "$1" == tfhe-worker ) ]] || exit 2
role="$1"; binary="/usr/local/bin/${role//-/_}"
sc_init
retired=()
cleanup() {
  local status=$? target
  hc_cleanup_signals
  hc_begin_cleanup || exit 1
  for target in "${retired[@]}"; do docker stop "$target" >/dev/null || status=1; done
  sc_run_restores || status=1
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
for ((operator=0;operator<RETIRED_OPERATOR_COUNT;operator++)); do
  prefix="coprocessor${operator}"
  [[ "$operator" != 0 ]] || prefix=coprocessor
  old="$prefix-$role"; green="$prefix-gcs-$role"
  image="$(docker inspect -f '{{.Image}}' "$old")"
  before="$(docker run --rm --network none "$image" "$binary" --stack-version)"
  after="$(docker exec "$green" "$binary" --stack-version)"
  [[ "$before" =~ ^0\.14\.0$ && "$after" =~ ^0\.15\.0$ ]] || { echo 'retired-release challenge requires the actual 0.14 -> 0.15 pair' >&2; exit 1; }
  printf '%s image=%s compiled=%s promoted=%s\n' "$old" "$image" "$before" "$after" >> "$(dirname "$SC_RESTORE_LOG")/retired-images.txt"
  sc_stop "$green"
  retired+=("$old")
  since="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  docker restart "$old" >/dev/null
  deadline=$((SECONDS+90))
  while true; do
    logs="$(docker logs --since "$since" "$old" 2>&1)"
    if rw_rejection_observed "$role" "$logs"; then break; fi
    ((SECONDS < deadline)) || { echo 'retired binary did not reach its real rejection path' >&2; exit 1; }
    sleep 1
  done
  printf '%s\n' "$logs" > "$(dirname "$SC_RESTORE_LOG")/$old.log"
done
printf 'ROLLOUT_HOLD_READY\n'
IFS= read -r command || exit 1
[[ "$command" == release ]] || exit 2
