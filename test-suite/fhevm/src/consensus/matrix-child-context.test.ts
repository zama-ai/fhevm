import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync, writeFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const cli = path.resolve(import.meta.dir, "../..");
const original = readFileSync(path.join(cli, "scripts/run-failure-matrix.sh"), "utf8");
const verdict = original.slice(original.indexOf('  local state_out=PASS detail=""'), original.indexOf('\n}\n\n# Establish a selected target'));

for (const [column, caseId, expectedState] of [
  ["db", "FMDB-OUTAGE", "PASS"],
  ["all", "FMDB-OUTAGE", "FAIL"],
  ["stall", "FM-SNS-STALL", "PASS"],
] as const) {
  test(`matrix spawned child preserves ${column} selection for ${caseId}`, () => {
    const directory = mkdtempSync(path.join(tmpdir(), "matrix-child-context-"));
    try {
      const denied = path.join(directory, "docker-attempt");
      writeFileSync(path.join(directory, "docker"), `#!/bin/sh\necho forbidden >> '${denied}'\nexit 97\n`, { mode: 0o700 });
      const replacement = `
# Keep real run_cell, run_timed_cell, host supervision, record CLI and finalization.
# Substitute the already-successful fault/recovery phases and all live cleanup.
docker() { echo forbidden >> '${denied}'; return 97; }
restore_case() { cancel_host_case; }
dispose_case() { :; }
finish_runner() { exit "$1"; }
run_cell_body() {
  local case_id="$1" service=fixture-worker workload=compute-chain
  local verify_status=0 auto_restart=pass no_progress=pass reaction=pass cleanup_state=ok
  local verify_reason="" cleanup_detail="" workload_ids=fixture-work
  local -a record=(cleanup=ok assert=fault=pass:fixture assert=liveness=pass:fixture assert=safety=pass:fixture)
  printf '%s\\n' "\${COLUMN-unset}" > '${directory}/child-column'
  printf '%s\\n' "$BASHPID" > '${directory}/child-pid'
${verdict}
}
CR_RUN_ID=matrix-child-context
CR_REVISION=fixture
CR_SCENARIO=three-of-three
CR_OPERATORS=3
CR_THRESHOLD=3
CR_BACKEND_CLASS=cpu
CR_HARDWARE_CLASS=fixture
COLUMN=db
COLUMN=${column}
export -n COLUMN
printf '%s\\n' "$BASHPID" > '${directory}/parent-pid'
run_cell ${caseId} ${caseId === "FMDB-OUTAGE" ? "db" : "stall"} fixture-worker pause compute-chain
exit $?
`;
      const source = original
        .replace(/^readonly SCRIPT_DIR=.*$/m, `readonly SCRIPT_DIR='${cli}/scripts'`)
        .replace(/^main\s*$/m, () => replacement);
      const fixture = path.join(directory, "runner.sh");
      writeFileSync(fixture, source);
      const result = Bun.spawnSync(["bash", fixture], {
        cwd: cli,
        env: { ...process.env, PATH: `${directory}:${process.env.PATH}`, FHEVM_STATE_DIR: directory,
          TMPDIR: directory, SC_RESTORE_LOG: path.join(directory, "restores"), CONSENSUS_RESULTS_DIR: path.join(directory, "results"),
          CONSENSUS_ARTIFACT_IDENTITIES_FILE: "", CONSENSUS_ARTIFACT_IDENTITIES: "",
          DOCKER_CONTEXT: "", DOCKER_HOST: "unix:///nonexistent/matrix-child-context.sock" },
        timeout: 15000,
      });
      expect(result.exitCode, result.stdout.toString() + result.stderr.toString()).toBe(expectedState === "PASS" ? 0 : 1);
      expect(readFileSync(path.join(directory, "child-column"), "utf8").trim()).toBe(column);
      expect(readFileSync(path.join(directory, "child-pid"), "utf8")).not.toBe(readFileSync(path.join(directory, "parent-pid"), "utf8"));
      const record = JSON.parse(readFileSync(path.join(directory, "results/matrix-child-context.jsonl"), "utf8"));
      expect(record.state).toBe(expectedState);
      expect(record.assertions.some((entry: { name: string; outcome: string }) => entry.name === "isolation" && entry.outcome === "pass")).toBe(column === "db");
      expect(existsSync(denied)).toBe(false);
    } finally { rmSync(directory, { recursive: true, force: true }); }
  });
}
