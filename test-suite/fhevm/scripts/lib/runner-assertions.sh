# shellcheck shell=bash
# Host callers supply outcomes only after their own checks. E2E outcomes come
# from executed assertion-group receipts collected by sp_exec, bound to run/case.
# The record CLI independently rejects any absent required assertion kind.
cr_record_checked_pass() {
  local case_id="$1" evidence=""; shift
  local -a outcomes=()
  if [[ -n "${SP_RUNTIME_DIR:-}" ]]; then
    if ! evidence="$(bun "$SCRIPT_DIR/read-assertion-evidence.ts" "$SP_RUNTIME_DIR/assertion-evidence.jsonl" "$CR_RUN_ID" "$case_id")"; then
      CR_RECORD_FAILURES=$((${CR_RECORD_FAILURES:-0} + 1))
      echo "case-result: refused $case_id PASS: assertion evidence unreadable" >&2
      return 1
    fi
    [[ -z "$evidence" ]] || mapfile -t outcomes <<<"$evidence"
  fi
  cr_record "$case_id" PASS "$@" "${outcomes[@]}"
}
