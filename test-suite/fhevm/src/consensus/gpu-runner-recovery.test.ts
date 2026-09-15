import { expect, test } from "bun:test";
import { mkdtempSync, mkdirSync, readFileSync, writeFileSync, symlinkSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
const scripts = path.resolve(import.meta.dir, "../../scripts");
for (const retryWorks of [false, true]) test(`mixed guard retries failed stop on EXIT and retains failed owners (retry=${retryWorks})`, () => {
  const dir = mkdtempSync(path.join(tmpdir(), "gpu-guard-recovery-"));
  try {
    mkdirSync(path.join(dir, "bin"));
    symlinkSync(path.join(scripts, "lib"), path.join(dir, "lib"));
    writeFileSync(path.join(dir, "state"), "stopped\n");
    writeFileSync(path.join(dir, "bin/systemctl"), '#!/bin/bash\necho active\n', {mode: 0o755});
    writeFileSync(path.join(dir, "bin/docker"), `#!/bin/bash
case "$1" in
 start) echo running > "$FIXTURE/state";;
 stop)
  echo attempt >> "$FIXTURE/stops"
  if [[ '${retryWorks}' == true && $(wc -l < "$FIXTURE/stops") -gt 1 ]]; then echo stopped > "$FIXTURE/state"; else exit 1; fi;;
 inspect) [[ $(cat "$FIXTURE/state") == stopped ]] && echo 'false 0';;
 *) exit 1;;
esac
`, {mode: 0o755});
    for (const helper of ["gpu-consensus-workers.sh", "consensus-validity.sh"]) writeFileSync(path.join(dir, helper), `#!/bin/bash
if [[ $(cat "$FIXTURE/state") == running ]]; then echo 'worker queue operator conflict'; exit 1; fi
`, {mode: 0o755});
    const source = readFileSync(path.join(scripts, "run-mixed-backend-guard.sh"), "utf8").replace(/^SCRIPT_DIR=.*$/m, `SCRIPT_DIR='${dir}'`);
    writeFileSync(path.join(dir, "runner.sh"), source);
    const run = Bun.spawnSync(["bash", path.join(dir, "runner.sh")], {env: {...process.env, FIXTURE: dir, FHEVM_STATE_DIR: dir, SC_RESTORE_LOG: path.join(dir, "owners"), PATH: `${dir}/bin:${process.env.PATH}`}, timeout: 5000});
    expect(run.exitCode, run.stderr.toString()).toBe(1);
    expect(readFileSync(path.join(dir, "stops"), "utf8").trim().split("\n").length).toBe(2);
    expect(readFileSync(path.join(dir, "state"), "utf8").trim()).toBe(retryWorks ? "stopped" : "running");
    expect(readFileSync(path.join(dir, "owners"), "utf8")).toBe(retryWorks ? "" : "coprocessor1-tfhe-worker|stop-container\n");
  } finally { rmSync(dir, {recursive: true, force: true}); }
});

test("GPU01 failed final exclusivity records cleanup failure and prevents the next lifecycle launch", () => {
  const dir = mkdtempSync(path.join(tmpdir(), "gpu-guard-verdict-"));
  try {
    const source = readFileSync(path.join(scripts, "run-gpu-lifecycle-cases.sh"), "utf8");
    const gpu01 = source.slice(source.indexOf("case_gpu01() {"), source.indexOf("# ---------------------------------------------------------------- GPU-02"));
    const main = source.slice(source.indexOf("main() {"), source.lastIndexOf("\nmain"));
    const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'; ENV_DIR='${dir}'; GPU_BASELINE='${dir}/baseline'; SC_RESTORE_LOG='${dir}/owners'
CASE=all; OPERATOR=1; FAILURES=0
cr_skip_wrong_scenario() { return 1; }; require_session() { return 0; }
cr_now() { echo now; }; log() { :; }; operator_count() { echo 3; }
cr_init() { :; }; rs_stage_results() { :; }; sc_snapshot_running() { :; }; docker() { :; }
run_with_markers() { CASE_OUT=failed; MISSING_MARKER=missing; status=1; }
hc_run() { return 1; }
cr_record() { echo "$*"; }
case_gpu03() { echo UNSAFE_NEXT_LAUNCH; }
${gpu01}
${main}
main; status=$?
[[ "$status" == 1 && "$GPU_LIFECYCLE_CLEANUP_FAILED" == 1 ]]
`], {timeout: 5000});
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    expect(run.stdout.toString()).toContain("GPU-01-MIXED-BACKEND-GUARD FAIL");
    expect(run.stdout.toString()).toContain("cleanup=failed");
    expect(run.stdout.toString()).not.toContain("UNSAFE_NEXT_LAUNCH");
  } finally { rmSync(dir, {recursive: true, force: true}); }
});
