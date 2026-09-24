#!/usr/bin/env bash
# Own omission, divergence or interrupted publication of one host track.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC2034 # Used by sourced service-control helpers.
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/lib/service-control.sh"
: "${SC_RESTORE_LOG:?restoration ledger required}"
: "${UPGRADE_FAULT_VERSION:?candidate version required}"
: "${UPGRADE_FAULT_CHAIN:?selected host chain required}"
: "${UPGRADE_OTHER_CHAIN:?other host chain required}"
[[ "$UPGRADE_FAULT_VERSION" =~ ^[a-zA-Z0-9.+_-]+$ ]] || exit 2
[[ "$UPGRADE_FAULT_CHAIN" =~ ^[0-9]+$ && "$UPGRADE_OTHER_CHAIN" =~ ^[0-9]+$ && "$UPGRADE_FAULT_CHAIN" != "$UPGRADE_OTHER_CHAIN" ]] || exit 2
sc_init
MODE="${HOST_REPORT_MODE:-withhold}"
case "$MODE" in withhold|diverge) proposal=1 ;; interrupt) proposal=2 ;; *) exit 2 ;; esac
report_mode=withhold
[[ "$MODE" != diverge ]] || report_mode=diverge
sql() { docker exec "${POSTGRES_CONTAINER:-coprocessor-and-kms-db}" psql -U postgres -d "$1" -At -v ON_ERROR_STOP=1 -c "$2"; }
owned=0
cleanup() {
  local status=$?
  hc_cleanup_signals
  hc_begin_cleanup || exit 1
  if [[ "$owned" == 1 ]]; then
    if [[ "$MODE" == diverge ]]; then
      # Stop the uploader before taking the journal snapshot. Failed-proposal
      # reset removed GCS rows, so recovery uses the independent private journal.
      local journal
      journal="$(dirname "$SC_RESTORE_LOG")/host-report-recovery.json"
      if sc_stop coprocessor1-gcs-consensus-detector &&
         sql coprocessor_1 "UPDATE public.consensus_test_host_report_fault SET expires_at=clock_timestamp()-interval '1 second'" >/dev/null &&
         sql coprocessor_1 'SELECT reports FROM public.consensus_test_host_report_fault' > "$journal" &&
         hc_run bun "$SCRIPT_DIR/restore-host-reports.ts" "$journal"; then
        sql coprocessor_1 'DROP TABLE public.consensus_test_host_report_fault' >/dev/null || status=1
      else
        echo "host report recovery incomplete; retain $journal and the owned control table" >&2
        status=1
      fi
    else
      sql coprocessor_1 'DROP TABLE public.consensus_test_host_report_fault' >/dev/null || status=1
    fi
  fi
  sc_run_restores || status=1
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
[[ "$(sql coprocessor_1 "SELECT to_regclass('public.consensus_test_host_report_fault') IS NULL")" == t ]] || { echo 'prior host report control needs recovery' >&2; exit 1; }
sql coprocessor_1 'CREATE TABLE public.consensus_test_host_report_fault(chain_id bigint PRIMARY KEY, version text NOT NULL, proposal_id bytea NOT NULL, expires_at timestamptz NOT NULL, observed_at timestamptz, observed_block bigint, mode text NOT NULL, reports jsonb NOT NULL DEFAULT '"'"'{}'"'"')' >/dev/null
owned=1
sql coprocessor_1 "INSERT INTO public.consensus_test_host_report_fault(chain_id,version,proposal_id,expires_at,observed_at,observed_block,mode) VALUES($UPGRADE_FAULT_CHAIN,'$UPGRADE_FAULT_VERSION',decode(lpad('$proposal',64,'0'),'hex'),clock_timestamp()+interval '20 minutes',NULL,NULL,'$report_mode')" >/dev/null
printf 'ROLLOUT_HOLD_READY\n'
deadline=$((SECONDS + 900))
observed=0
while true; do
  ((SECONDS < deadline)) || { echo 'host report control exceeded its owned deadline' >&2; exit 1; }
  if [[ "$observed" == 0 ]]; then
    block="$(sql coprocessor_1 'SELECT observed_block FROM public.consensus_test_host_report_fault WHERE observed_at IS NOT NULL AND expires_at>clock_timestamp()')"
    if [[ "$block" =~ ^[0-9]+$ ]]; then
      complete=1
      candidate="$(sql coprocessor_1 "SELECT state_hash FROM \"gcs-$UPGRADE_FAULT_VERSION\".state_hash WHERE chain_id=$UPGRADE_FAULT_CHAIN AND block_number=$block")"
      [[ "$candidate" =~ ^[0-9a-f]{64}$ ]] || complete=0
      other_block="$(sql coprocessor "SELECT block_number FROM \"gcs-$UPGRADE_FAULT_VERSION\".state_hash WHERE chain_id=$UPGRADE_OTHER_CHAIN AND s3_uploaded_at IS NOT NULL ORDER BY block_number LIMIT 1")"
      other_hash=""
      if [[ "$other_block" =~ ^[0-9]+$ ]]; then
        other_hash="$(sql coprocessor "SELECT state_hash FROM \"gcs-$UPGRADE_FAULT_VERSION\".state_hash WHERE chain_id=$UPGRADE_OTHER_CHAIN AND block_number=$other_block")"
      else complete=0; fi
      for operator in 0 1 2; do
        database=coprocessor
        [[ "$operator" == 0 ]] || database="coprocessor_$operator"
        # The computed hash agrees even when the test publication diverges.
        # Withholding alone leaves the victim unpublished; peers and the other
        # host track must continue uploading in every arm.
        expected=t; [[ "$operator" != 1 || "$MODE" == diverge ]] || expected=f
        state="$(sql "$database" "SELECT s3_uploaded_at IS NOT NULL FROM \"gcs-$UPGRADE_FAULT_VERSION\".state_hash WHERE chain_id=$UPGRADE_FAULT_CHAIN AND block_number=$block")"
        [[ "$state" == "$expected" ]] || complete=0
        hash="$(sql "$database" "SELECT state_hash FROM \"gcs-$UPGRADE_FAULT_VERSION\".state_hash WHERE chain_id=$UPGRADE_FAULT_CHAIN AND block_number=$block")"
        [[ "$hash" == "$candidate" ]] || complete=0
        if [[ "$other_block" =~ ^[0-9]+$ ]]; then
          other="$(sql "$database" "SELECT state_hash FROM \"gcs-$UPGRADE_FAULT_VERSION\".state_hash WHERE chain_id=$UPGRADE_OTHER_CHAIN AND block_number=$other_block AND s3_uploaded_at IS NOT NULL")"
          [[ -n "$other_hash" && "$other" == "$other_hash" ]] || complete=0
        fi
      done
      if [[ "$complete" == 1 && "$MODE" == diverge ]]; then
        # The control acknowledgement precedes upload. Independently fetch the
        # exact public bytes consumed by all detectors before accepting it.
        hc_run docker exec "${TEST_CONTAINER:-fhevm-test-suite-e2e-debug}" node test/consensus/hostReportObservation.cjs \
          "$UPGRADE_FAULT_CHAIN" "$block" "$candidate" > "$(dirname "$SC_RESTORE_LOG")/divergent-host-report.json" || complete=0
      fi
      if [[ "$complete" == 1 ]]; then
        sql coprocessor_1 'SELECT row_to_json(f) FROM public.consensus_test_host_report_fault f' > "$(dirname "$SC_RESTORE_LOG")/withheld-host-report.json"
        printf 'HOST_REPORT_OBSERVED mode=%s chain=%s block=%s operator=1 peers=0,2 other_chain=%s\n' "$MODE" "$UPGRADE_FAULT_CHAIN" "$block" "$UPGRADE_OTHER_CHAIN"
        observed=1
        if [[ "$MODE" == interrupt ]]; then
          target=coprocessor1-gcs-consensus-detector
          if [[ "$(sc_restart_budget "$target")" == exhausted ]]; then sc_reset_restart_budget "$target"; fi
          before="$(sc_kill "$target")"
          sc_wait_replaced "$target" "$before" 120
          # Keep publication withheld through replacement; the new invocation
          # must recover the durable queue after the owned control is removed.
          sql coprocessor_1 'DROP TABLE public.consensus_test_host_report_fault' >/dev/null
          owned=0
          printf 'DETECTOR_INTERRUPT_OBSERVED chain=%s block=%s old=%s replacement=%s\n' "$UPGRADE_FAULT_CHAIN" "$block" "$before" "$(sc_identity "$target")"
          exit 0
        fi
      fi
    fi
  fi
  if IFS= read -r -t 1 command; then
    [[ "$command" == release && "$observed" == 1 ]] || exit 1
    exit 0
  else
    [[ "$?" -gt 128 ]] || exit 1
  fi
done
