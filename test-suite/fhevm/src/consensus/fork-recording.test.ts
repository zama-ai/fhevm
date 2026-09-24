import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import path from "node:path";

const source = readFileSync(path.resolve(import.meta.dir, "../../scripts/run-fork-consensus.sh"), "utf8");
const scripts = path.resolve(import.meta.dir, "../../scripts");
const cases = ["FORK-01-COLLIDING-HANDLE", "FORK-02-DISTINCT-HANDLES", "FORK-03-ORPHAN-ALLOW-INERT"];
const handle = (index: number) => `0x${String(index).padStart(64, "0")}`;
const observation = (caseId: string, index: number) => "[consensus-fault] " + JSON.stringify({
  runId: "fork-test", caseId, workloadIds: [handle(index)], faultObservedAt: "2026-09-17T12:00:00.000Z",
  artifacts: { canonical_block: handle(index + 10), fork_block: handle(index + 20) },
});
const caseMain = source.slice(source.indexOf("case_main() {"), source.indexOf("\nmain() {"));
for (const mode of ["success", "body-failure", "hook-failure", "missing-observation"] as const) {
  test(`fork evidence survives phase result attribution (${mode})`, () => {
    const failed = mode === "body-failure";
    const phase = [
      ...cases.slice(0, failed || mode === "missing-observation" ? 2 : 3).map((id, index) => observation(id, index + 1)),
      "[fork-consensus/F1] CASE COMPLETE", "[fork-consensus/F2] CASE COMPLETE",
      failed ? "F3 assertion failed" : "[fork-consensus/F3] CASE COMPLETE",
    ].join("\n");
    const result = Bun.spawnSync(["bash", "-c", `set -uo pipefail
sp_case_start() { :; }
cr_now() { echo 2026-09-17T11:59:00.000Z; }
log() { :; }
run_phase() { local -n output="$1"; output="$PHASE_OUTPUT"; return "$PHASE_STATUS"; }
cr_failure_reason() { echo 'F3 assertion failed'; }
cr_record_checked_pass() { printf 'PASS'; printf '\t%s' "$@"; printf '\n'; }
cr_record() { printf 'RECORD'; printf '\t%s' "$@"; printf '\n'; }
FAILURES=0; RS_FINAL_FAILURE=0; CR_RUN_ID=fork-test; SCRIPT_DIR='${scripts}'
${caseMain}
case_main
printf 'failures=%s\n' "$FAILURES"
printf 'final_failure=%s\n' "$RS_FINAL_FAILURE"
`], { env: { ...process.env, PHASE_OUTPUT: phase, PHASE_STATUS: failed || mode === "hook-failure" ? "1" : "0" } });
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    const output = result.stdout.toString();
    const passes = output.split("\n").filter((line) => line.startsWith("PASS\t"));
    expect(passes).toHaveLength(failed || mode === "missing-observation" ? 2 : 3);
    for (const [index, pass] of passes.entries()) {
      expect(pass).toContain(`\tworkload=${handle(index + 1)}`);
      expect(pass).toContain("\tfault_observed_at=2026-09-17T12:00:00.000Z");
      expect(pass).toContain(`\tartifact=canonical_block=${handle(index + 11)}`);
      expect(pass).toContain(`\tartifact=fork_block=${handle(index + 21)}`);
    }
    expect(output).toContain(`failures=${mode === "success" ? 0 : 1}`);
    expect(output).toContain(`final_failure=${mode === "hook-failure" ? 1 : 0}`);
    if (failed || mode === "missing-observation") expect(output).toContain("RECORD\tFORK-03-ORPHAN-ALLOW-INERT\tFAIL");
  });
}
