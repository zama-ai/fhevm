import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import path from "node:path";

const source = readFileSync(path.resolve(import.meta.dir, "../../scripts/run-fork-consensus.sh"), "utf8");
const caseMain = source.slice(source.indexOf("case_main() {"), source.indexOf("\nmain() {"));
for (const failed of [false, true]) {
  test(`fork evidence survives phase result attribution (failed=${failed})`, () => {
    const phase = [
      "branches diverged: handle 0x1234 canonical 0xabcd fork 0xef01 at 2026-09-17T12:00:00.000Z",
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
FAILURES=0
${caseMain}
case_main
printf 'failures=%s\n' "$FAILURES"
`], { env: { ...process.env, PHASE_OUTPUT: phase, PHASE_STATUS: failed ? "1" : "0" } });
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    const output = result.stdout.toString();
    const passes = output.split("\n").filter((line) => line.startsWith("PASS\t"));
    expect(passes).toHaveLength(failed ? 2 : 3);
    for (const pass of passes) {
      expect(pass).toContain("\tworkload=0x1234");
      expect(pass).toContain("\tfault_observed_at=2026-09-17T12:00:00.000Z");
      expect(pass).toContain("\tartifact=canonical_block=0xabcd");
      expect(pass).toContain("\tartifact=fork_block=0xef01");
    }
    expect(output).toContain(`failures=${failed ? 1 : 0}`);
    if (failed) expect(output).toContain("RECORD\tFORK-03-ORPHAN-ALLOW-INERT\tFAIL");
  });
}
