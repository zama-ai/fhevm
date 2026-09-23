# shellcheck shell=bash
# A runner's successful evidence is provisional until its EXIT recovery ends.
rs_stage_results() {
  RS_PUBLISH_RESULTS="$(dirname "$(cr_results_file)")" || return 1
  RS_STAGED_RESULTS="$(mktemp -d "$SP_RUNTIME_DIR/results.XXXXXX")" || return 1
  export CONSENSUS_RESULTS_DIR="$RS_STAGED_RESULTS"
}
rs_finalize_results() {
  local status="$1" cleanup="$2" state=keep detail=""
  [[ -n "${RS_STAGED_RESULTS:-}" ]] || return "$status"
  local represented=0
  if [[ "$status" != 0 && "$cleanup" == ok && "${RS_FINAL_FAILURE:-0}" == 0 ]] &&
     timeout --kill-after=2s 20s bun "$SCRIPT_DIR/finalize-case-results.ts" --has-failure "$RS_STAGED_RESULTS" "$CR_RUN_ID"; then
    represented=1
  fi
  if [[ ( "$status" != 0 && "$represented" != 1 ) || "$cleanup" == failed || "${CR_RECORD_FAILURES:-0}" != 0 || "${RS_FINAL_FAILURE:-0}" == 1 ]]; then
    state=FAIL
    detail="runner exited $status; final recovery was $cleanup"
    status=1
  fi
  if ! timeout --kill-after=2s 20s bun "$SCRIPT_DIR/finalize-case-results.ts" "$RS_STAGED_RESULTS" "$RS_PUBLISH_RESULTS" "$state" "$cleanup" "$detail"; then
    touch "$SP_RUNTIME_DIR/cleanup-failed"
    echo "case-result: final publication failed; staged evidence retained at $RS_STAGED_RESULTS" >&2
    return 1
  fi
  return "$status"
}
