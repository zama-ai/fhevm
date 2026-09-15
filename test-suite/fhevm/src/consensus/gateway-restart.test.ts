import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

const source = readFileSync(new URL('../../scripts/run-degraded-consensus.sh', import.meta.url), 'utf8');
const start = source.indexOf('  # Queue SIGKILL');
const end = source.indexOf('  local recovery_observed;', start);

test('pending gateway events cannot run on their old listener during replacement', () => {
  expect(start).toBeGreaterThan(0);
  const result = Bun.spawnSync(['bash', '-c', `
set -eu
run() {
  local -a listeners=(gw0 gw1 gw2) identities_before=(old0 old1 old2)
  local listener restart_failed=0
  declare -A killed=()
  sc_state() { echo paused; }
  sc_kill() { [[ "$2" == KILL && "$3" == 1 ]] || exit 8; killed[$1]=1; echo "killed $1" >&2; }
  sc_resume() {
    [[ "\${killed[$1]:-0}" == 1 ]] || { echo 'pending event ran on old process' >&2; exit 9; }
  }
  sc_wait_replaced() { [[ "\${killed[$1]:-0}" == 1 ]]; }
${source.slice(start, end)}
  [[ "$restart_failed" == 0 && "\${#killed[@]}" == 3 ]]
}
run
`]);
  expect(result.exitCode).toBe(0);
  expect(result.stderr.toString().trim().split('\n')).toEqual(['killed gw0', 'killed gw1', 'killed gw2']);
});

test('gateway poison cleanup precedes resume, and failed cleanup cannot resume frozen listeners', () => {
  const start = source.indexOf('cleanup_on_exit() {');
  const end = source.indexOf('\ntrap cleanup_on_exit EXIT', start);
  for (const cleanupFails of [false, true]) {
    const result = Bun.spawnSync(['bash','-c', `
GW_POISON_ARMED=1
SP_FORCED_STOP=0
SP_RUNTIME_DIR=/tmp/nonexistent-gateway-fixture
hc_begin_cleanup() { :; }
rs_finalize_results() { return "$1"; }
sp_case_cleanup() { gw_restore_originals; }
sp_cancel_all() { echo cancelled; }
sp_recover_suite_state() { echo journal-restored; }
sp_dispose() { return 0; }
gw_restore_originals() { echo restored; return ${cleanupFails ? 1 : 0}; }
sc_run_restores() { echo resumed; }
${source.slice(start,end)}
trap cleanup_on_exit EXIT
exit 143
`]);
    expect(result.exitCode).toBe(cleanupFails ? 1 : 143);
    expect(result.stdout.toString().trim()).toBe(cleanupFails ? 'cancelled\njournal-restored\nrestored' : 'cancelled\njournal-restored\nrestored\nresumed');
  }
});

test('gateway event poisoning refuses environment or argument configurations that enable auto-revert', () => {
  const start = source.indexOf('gw_require_no_auto_revert() {');
  const end = source.indexOf('\ngw_wait_exact_event_warnings()', start);
  for (const [enabled, arg, expected] of [['false','',0],['true','',1],['false','--drift-auto-revert-enabled',1]] as const) {
    const inspect = JSON.stringify([{Config:{Env:[`DRIFT_AUTO_REVERT_ENABLED=${enabled}`],Cmd:arg ? [arg] : []}}]);
    const result = Bun.spawnSync(['bash','-c', `
set -o pipefail
docker() { printf '%s' '${inspect}'; }
${source.slice(start,end)}
gw_require_no_auto_revert listener
`]);
    expect(result.exitCode).toBe(expected);
  }
});
