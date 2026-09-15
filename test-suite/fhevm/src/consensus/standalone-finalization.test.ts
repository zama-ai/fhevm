import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
const scripts = path.resolve(import.meta.dir, "../../scripts");
const fixtures = [
  ["run-fork-consensus.sh", "cleanup_on_exit", "sp_recover_suite_state"],
  ["run-materialization-consensus.sh", "cleanup_suite", "sp_recover_suite_state"],
] as const;
for (const [file, cleanup, failure] of fixtures) {
  for (const fails of [false, true]) test(`${file} publishes only after final recovery (fails=${fails})`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "standalone-finalize-"));
    try {
      const source = readFileSync(path.join(scripts, file), "utf8");
      const body = source.slice(source.indexOf(`${cleanup}() {`), source.indexOf(`trap ${cleanup} EXIT`));
      const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${dir}'; SP_RUNTIME_DIR='${dir}/runtime'
mkdir -p "$SP_RUNTIME_DIR"
source "$SCRIPT_DIR/lib/host-command.sh"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/runner-assertions.sh"
source "$SCRIPT_DIR/lib/result-staging.sh"
CR_RUN_ID=final; CR_REVISION=abc123; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=cpu
CR_SCENARIO=none; CR_OPERATORS=0; CR_THRESHOLD=0
CONSENSUS_RESULTS_DIR='${dir}/published'; export CONSENSUS_RESULTS_DIR
rs_stage_results || exit 2
cr_record_checked_pass HAR-03-READINESS-CONTRACTS assert=safety=pass:fixture cleanup=ok || exit 3
[[ ! -f '${dir}/published/final.jsonl' ]] || exit 4
SUITE_PID=''; SP_FORCED_STOP=0; BASELINE=''; BASELINE_OWNED=0; SUITE_LOG=''
sp_cancel_all() { return 0; }; sp_recover_suite_state() { return 0; }
sc_run_restores() { return 0; }; sc_restore_running() { return 0; }
sp_case_cleanup() { return 0; }; sp_dispose() { return 0; }
${fails ? `${failure}() { return 1; }` : ""}
${body}
trap ${cleanup} EXIT
exit 0
`], {timeout: 10000});
      expect(run.exitCode, run.stderr.toString()).toBe(fails ? 1 : 0);
      const records = readFileSync(path.join(dir, "published/final.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line));
      expect(records.length).toBe(1);
      expect(records[0].state).toBe(fails ? "FAIL" : "PASS");
      expect(records[0].cleanup.state).toBe(fails ? "failed" : "ok");
    } finally { rmSync(dir, {recursive: true, force: true}); }
  });
}

for (const finalGateFails of [false, true]) test(`ordinary first-case failure preserves later PASS unless final gate fails (${finalGateFails})`, () => {
  const dir = mkdtempSync(path.join(tmpdir(), "mixed-finalize-"));
  try {
    const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${dir}'; SP_RUNTIME_DIR='${dir}/runtime'
mkdir -p "$SP_RUNTIME_DIR"
source "$SCRIPT_DIR/lib/case-result.sh"
source "$SCRIPT_DIR/lib/runner-assertions.sh"
source "$SCRIPT_DIR/lib/result-staging.sh"
CR_RUN_ID=mixed; CR_REVISION=abc123; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=cpu
CR_SCENARIO=none; CR_OPERATORS=0; CR_THRESHOLD=0
CONSENSUS_RESULTS_DIR='${dir}/published'; export CONSENSUS_RESULTS_DIR
rs_stage_results
cr_record HAR-03-READINESS-CONTRACTS FAIL cleanup=ok detail='first case failed'
cr_record_checked_pass HAR-02-INVENTORY-AGGREGATE assert=safety=pass:fixture cleanup=ok
FAILURES=1; RS_FINAL_FAILURE=${finalGateFails ? 1 : 0}
rs_finalize_results 1 ok; [[ "$?" == 1 ]]
`], {timeout: 10000});
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    const records = readFileSync(path.join(dir, "published/mixed.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line));
    expect(records.map((record) => record.state)).toEqual(["FAIL", finalGateFails ? "FAIL" : "PASS"]);
  } finally { rmSync(dir, {recursive: true, force: true}); }
});
