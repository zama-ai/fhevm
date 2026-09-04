#!/usr/bin/env bash
# Run-validity gates that need Docker, and therefore cannot live in the e2e
# container -- it deliberately has no Docker socket.
#
# The rest of the gates (key/CRS material, the deferred-transactions gauge,
# chain liveness) run inside the suites, in
# `test-suite/e2e/test/consensus/validity.ts`. This file carries the one the
# coverage register names that only the host can see: whether any tfhe-worker
# lost dependence-chain locks during the run.
#
#   consensus-validity.sh locks [--since <docker-since>] [--report-only]
#
# Exit status is 0 when no worker reports lock loss, 1 when one does (unless
# --report-only), and 2 on usage error. A one-line summary always goes to
# stdout, because a gate that passes silently teaches a reader nothing about
# what was checked.
#
# Why this is a validity gate rather than a test. "Not all locks extended" means
# another worker stole a dependence chain whose lease had lapsed and recomputed
# it. RFC-020 says that must be byte-identical, so it is not automatically a
# consensus failure -- but it does mean the run was not the clean single-owner
# case it appears to be, and a measurement that silently included stolen work is
# not the measurement anyone thinks they are reading. The message used to fire
# falsely, when a lease that lapsed while still held was stolen back by its own
# owner into a second lock-set entry; `dependence_chain.rs` excludes that case
# now, so an occurrence today is real.
set -uo pipefail

readonly MESSAGE="Not all locks extended"
SINCE=""
REPORT_ONLY=0

usage() { echo "usage: consensus-validity.sh locks [--since <docker-since>] [--report-only]" >&2; exit 2; }

worker_containers() {
  docker ps -a --format '{{.Names}}' \
    | grep -E '^coprocessor[0-9]*-tfhe-worker$' \
    | sort
}

gate_locks() {
  local name count total=0 detail=""
  while IFS= read -r name; do
    [[ -n "$name" ]] || continue
    if [[ -n "$SINCE" ]]; then
      count="$(docker logs --since "$SINCE" "$name" 2>&1 | grep -c "$MESSAGE")"
    else
      count="$(docker logs "$name" 2>&1 | grep -c "$MESSAGE")"
    fi
    count="${count:-0}"
    if [[ "$count" -gt 0 ]]; then
      total=$((total + count))
      detail="$detail $name(x$count)"
    fi
  done < <(worker_containers)

  if [[ "$total" -eq 0 ]]; then
    echo "validity: no tfhe-worker reported lock loss"
    return 0
  fi
  echo "validity: LOCK LOSS reported:$detail"
  echo "  Another worker stole a dependence chain whose lease lapsed and recomputed it. Bytes must"
  echo "  still match (RFC-020), but this run is not the clean single-owner case it looks like."
  [[ "$REPORT_ONLY" -eq 1 ]] && return 0
  return 1
}

[[ $# -ge 1 ]] || usage
GATE="$1"; shift
while [[ $# -gt 0 ]]; do
  case "$1" in
    --since) SINCE="${2:?--since needs a value}"; shift 2 ;;
    --report-only) REPORT_ONLY=1; shift ;;
    *) usage ;;
  esac
done

case "$GATE" in
  locks) gate_locks ;;
  *) usage ;;
esac
