import { expect, test } from "bun:test";
import { existsSync, mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
const scripts = path.resolve(import.meta.dir, "../../scripts");
for (const recoveryHangs of [false, true]) {
  test(`standalone host deadline interrupts Docker and recovery has its own bound (hang=${recoveryHangs})`, () => {
    const directory = mkdtempSync(path.join(tmpdir(), "host-command-deadline-"));
    try {
      mkdirSync(path.join(directory, "bin"));
      writeFileSync(path.join(directory, "bin/docker"), `#!/bin/bash
if [[ "$1" == pause ]]; then touch '${directory}/pause-launched'; fi
if [[ "$1" == pause || "${recoveryHangs}" == true ]]; then sleep 30; else echo restored; fi
`, { mode: 0o755 });
      const started = Date.now();
      const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
source "$SCRIPT_DIR/lib/host-command.sh"
source "$SCRIPT_DIR/lib/case-deadline.sh"
trap 'status=$?; trap - EXIT INT TERM; hc_begin_cleanup; docker start victim; restored=$?; hc_stop_timer; echo recovery=$restored; exit "$status"' EXIT
trap 'exit 143' TERM
# Integer epoch budgets can leave almost no startup time with a one-second
# allowance. Keep one full second available for the blocking mock to launch.
case_deadline_start 2
hc_arm_deadline
# This actual external mock used to block the host for all thirty seconds.
docker pause victim
exit 2
`], { env: { ...process.env, SCRIPT_DIR: scripts, PATH: `${directory}/bin:${process.env.PATH}`, HC_CLEANUP_TIMEOUT_SECONDS: "1" }, timeout: 10000 });
      expect(existsSync(path.join(directory, "pause-launched")), run.stderr.toString()).toBe(true);
      expect(run.exitCode, run.stderr.toString()).toBe(143);
      expect(run.stdout.toString()).toContain(recoveryHangs ? "recovery=124" : "restored\nrecovery=0");
      expect(Date.now() - started).toBeLessThan(5000);
    } finally { rmSync(directory, {recursive: true, force: true}); }
  }, 10000);
}

test("an explicit SQL timeout consumes remaining cleanup budget and failure publication still runs", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "cleanup-budget-"));
  try {
    const started = Date.now();
    const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${directory}'; SP_RUNTIME_DIR='${directory}/runtime'
mkdir -p "$SP_RUNTIME_DIR"
source "$SCRIPT_DIR/lib/host-command.sh"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/runner-assertions.sh"
source "$SCRIPT_DIR/lib/result-staging.sh"
CR_RUN_ID=final; CR_REVISION=abc123; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=cpu
CR_SCENARIO=none; CR_OPERATORS=0; CR_THRESHOLD=0
CONSENSUS_RESULTS_DIR='${directory}/results'; export CONSENSUS_RESULTS_DIR
rs_stage_results
cr_record_checked_pass HAR-03-READINESS-CONTRACTS assert=safety=pass:fixture cleanup=ok
HC_CLEANUP_TIMEOUT_SECONDS=1
hc_begin_cleanup
# Representative explicit SQL wrapper: the inner timeout must not reset budget.
hc_run timeout --kill-after=2s 30s sleep 30; [[ "$?" == 124 ]] || exit 2
hc_run sleep 30; [[ "$?" == 124 ]] || exit 3
rs_finalize_results 1 failed; [[ "$?" == 1 ]] || exit 4
cat '${directory}/results/final.jsonl'
hc_stop_timer
`], {timeout: 10000});
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    expect(run.stdout.toString()).toContain('"state":"FAIL"');
    expect(Date.now() - started).toBeLessThan(4000);
  } finally { rmSync(directory, {recursive: true, force: true}); }
});

test('a second TERM cannot interrupt owned restoration after EXIT cleanup starts', async () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'cleanup-second-signal-'));
  const fs = await import('node:fs');
  const child = Bun.spawn(['bash','-c', `
source '${scripts}/lib/host-command.sh'
cleanup() {
 local status=$?
 hc_cleanup_signals
 hc_begin_cleanup
 touch '${directory}/cleanup-started'
 hc_run sleep 1
 echo restored > '${directory}/restored'
 exit "$status"
}
trap cleanup EXIT
trap 'exit 143' TERM
sleep 30 & wait
`], {stdout:'pipe',stderr:'pipe'});
  try {
    await Bun.sleep(100);
    child.kill('SIGTERM');
    for(let i=0;i<100&&!fs.existsSync(path.join(directory,'cleanup-started'));i++) await Bun.sleep(10);
    expect(fs.existsSync(path.join(directory,'cleanup-started'))).toBe(true);
    child.kill('SIGTERM');
    expect(await child.exited).toBe(143);
    expect(fs.readFileSync(path.join(directory,'restored'),'utf8')).toBe('restored\n');
  } finally {child.kill();rmSync(directory,{recursive:true,force:true});}
}, 5000);
