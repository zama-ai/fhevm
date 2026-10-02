import { expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { readCaseResults } from "./results";

const cli = path.resolve(import.meta.dir, "../..");
const metadata = `CR_RUN_ID=finalization-test; CR_REVISION=abc123; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=cpu
CR_SCENARIO=three-of-three; CR_OPERATORS=3; CR_THRESHOLD=3`;
function fixture(runner: string, replacement: string) {
  const dir = mkdtempSync(path.join(tmpdir(), "result-finalization-"));
  const script = path.join(dir, "runner.sh");
  mkdirSync(path.join(dir, "bin"));
  // These are orchestration unit tests. Unexpected infrastructure access must
  // fail visibly, including from exported functions in the matrix child.
  writeFileSync(path.join(dir, "bin/docker"), '#!/bin/sh\nprintf "%s\\n" "$*" >> "$FHEVM_STATE_DIR/unexpected-docker"\nexit 97\n', { mode: 0o755 });
  writeFileSync(script, readFileSync(path.join(cli, "scripts", runner), "utf8")
    .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cli}/scripts'`)
    .replace(/^main\s*$/m, () => replacement));
  const result = Bun.spawnSync(["bash", script], {
    cwd: cli, timeout: 10000,
    env: { ...process.env, PATH: `${dir}/bin:${process.env.PATH}`, DOCKER_CONTEXT: "", DOCKER_HOST: `unix://${dir}/absent-docker.sock`, DB_CONTAINER: `unit-test-${path.basename(dir)}`, FHEVM_STATE_DIR: dir, CONSENSUS_RESULTS_DIR: path.join(dir, "published"), SC_RESTORE_LOG: path.join(dir, "restores") },
  });
  return { dir, result, records: () => existsSync(path.join(dir, "published")) ? readCaseResults(path.join(dir, "published")) : [] };
}

for (const mode of ["empty", "wrong-run", "delegated-alias-missing", "refused"]) {
  test(`matrix requires its expected staged result before accepting child success: ${mode}`, () => {
    const matrix = readFileSync(path.join(cli, "scripts/run-failure-matrix.sh"), "utf8");
    const tail = matrix.slice(matrix.indexOf('  record+=("assert=recovered-armed-workload='), matrix.indexOf('\nmain() {'));
    const testCase = mode === "delegated-alias-missing" ? "FM-TFHE-CRASH" : "FM-TFHE-STALL";
    const body = mode === "empty" ? ":" : mode === "wrong-run"
      ? "CR_RUN_ID=unrelated-run cr_record_checked_pass FM-TFHE-STALL assert=fault=pass:fixture assert=safety=pass:fixture assert=liveness=pass:fixture assert=bytes=pass:fixture assert=scope=pass:fixture cleanup=ok"
      : mode === "delegated-alias-missing" ? "cr_record_checked_pass CR-01-INTERRUPT-BEFORE-COMMIT assert=precondition=pass:fixture assert=quorum=pass:fixture assert=fault=pass:fixture assert=liveness=pass:fixture assert=bytes=pass:fixture assert=safety=pass:fixture cleanup=ok"
      : `cr_record_checked_pass() { echo 'injected recording refusal' >&2; return 1; }
run_cell_body() { local case_id="$1" service=fixture state_out=PASS detail='' no_progress=pass; local -a record=()
${tail}
run_cell_body "$1"`;
    const { dir, result, records } = fixture("run-failure-matrix.sh", `
${metadata}
bun() { if [[ "$2" == show ]]; then echo 'timeout: 30s'; else command bun "$@"; fi; }
sp_recover_suite_state() { :; }; sc_run_restores() { :; }; sc_restore_running() { :; }
cc_disable_failpoints() { echo disable >> "$FHEVM_STATE_DIR/control-cleanup"; }
cc_drop_audit() { echo drop >> "$FHEVM_STATE_DIR/control-cleanup"; }
run_timed_cell() { ${body}
}
run_cell ${testCase} stall worker pause compute-chain
exit $?
`);
    try {
      expect(result.exitCode, result.stderr.toString()).toBe(1);
      const expected = records().filter((record) => record.caseId === testCase && record.runId === "finalization-test");
      expect(expected).toHaveLength(1);
      expect(expected[0]!.state).toBe("FAIL");
      expect(records().some((record) => record.state === "PASS")).toBe(false);
      expect(existsSync(path.join(dir, "unexpected-docker"))).toBe(false);
      if (mode === "delegated-alias-missing") {
        expect(readFileSync(path.join(dir, "control-cleanup"), "utf8").trim().split("\n")).toEqual(["disable", "drop"]);
      }
      if (mode === "refused") expect(result.stdout.toString()).not.toContain("[FM-TFHE-STALL/fixture] PASS");
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}

for (const failure of ["none", "disable", "heal", "audit"]) {
  test(`standalone crash verdict is published only after final ${failure} cleanup outcome`, () => {
    const crash = readFileSync(path.join(cli, "scripts/run-crash-retry-consensus.sh"), "utf8");
    const init = crash.slice(crash.indexOf('  CRASH_PUBLISH_RESULTS='), crash.indexOf('  command -v docker', crash.indexOf('main() {')));
    const { dir, result, records } = fixture("run-crash-retry-consensus.sh", `
${metadata}
${init}
sp_cancel_all() { touch "$SP_RUNTIME_DIR/cancelling"; }; sp_recover_suite_state() { :; }
cc_disable_failpoints() { [[ '${failure}' != disable ]]; }
sc_run_restores() { [[ '${failure}' != heal ]]; }
cc_drop_audit() { [[ ! -f "$CRASH_PUBLISH_RESULTS/$CR_RUN_ID.jsonl" ]] || exit 9; [[ '${failure}' != audit ]]; }
SP_FORCED_STOP=0; CRASH_CONTROLS_ARMED=1
cr_record_checked_pass "$CASE_ID" assert=precondition=pass:fixture assert=quorum=pass:fixture assert=fault=pass:fixture assert=liveness=pass:fixture assert=bytes=pass:fixture assert=safety=pass:fixture cleanup=ok || exit 8
cr_record_checked_pass FM-TFHE-CRASH assert=fault=pass:fixture assert=liveness=pass:fixture assert=bytes=pass:fixture assert=safety=pass:fixture cleanup=ok || exit 8
[[ ! -f "$CRASH_PUBLISH_RESULTS/$CR_RUN_ID.jsonl" ]] || exit 9
exit 0
`);
    try {
      expect(result.exitCode, result.stderr.toString()).toBe(failure === "none" ? 0 : 1);
      expect(existsSync(path.join(dir, "unexpected-docker"))).toBe(false);
      expect(records()).toHaveLength(2);
      expect(records().map((record) => [record.state, record.cleanup.state])).toEqual(
        Array.from({ length: 2 }, () => failure === "none" ? ["PASS", "ok"] : ["FAIL", "failed"]),
      );
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}
