import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const scripts = path.resolve(import.meta.dir, "../../scripts");
const source = readFileSync(path.join(scripts, "run-materialization-consensus.sh"), "utf8");
const discovery = source.slice(source.indexOf("observed_scheduling_classes() {"), source.indexOf('\nsource "${SCRIPT_DIR}/lib/suite-process.sh"'));
const cpuStart = source.indexOf("  else\n    local revision\n");
const cpuBranch = source.slice(cpuStart + "  else\n".length, source.indexOf("\n  fi", cpuStart));
const baseStart = source.indexOf("  local -a base_record=(");
const baseRecord = source.slice(baseStart, source.indexOf("\n  )", baseStart) + "\n  )".length);
const expected = [0, 1, 2].map(index => `${index}=window:${16 + index},chains:default,threads:default,streams:default,adaptive:false,batch:default`).join(";");

for (const state of ["PASS", "FAIL"] as const) {
  test(`CPU materialization ${state} records the same observed scheduling classes passed to E2E`, () => {
    const dir = mkdtempSync(path.join(tmpdir(), "materialization-cpu-class-"));
    try {
      const result = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${dir}'
source "$SCRIPT_DIR/lib/case-result.sh"
operator_indexes() { printf '0\\n1\\n2\\n'; }
container_for() { printf 'operator%s' "$1"; }
docker() {
  [[ "$1" == inspect ]] || return 90
  if [[ "$2" != --format ]]; then return 0; fi
  if [[ "$3" == *Config.Cmd* ]]; then
    echo inspect >> '${dir}/observations'
    printf '%s\\n' "--work-items-batch-size=$((16 + \${4#operator}))"
  else printf '%s\\n' FHEVM_DCID_ADAPTIVE_BATCH_EXECUTION=false; fi
}
sr_revision() { echo fixture; }
die() { echo "$*" >&2; exit 91; }
${discovery}
run_cpu_branch() {
  local -a class_env=()
${cpuBranch}
  printf '%s\\n' "\${class_env[@]}" > '${dir}/docker-env'
  bash -c 'printf "%s" "$CONSENSUS_SCHEDULING_CLASSES"' > '${dir}/exported'
  local started=2026-09-14T00:00:00Z
${baseRecord}
  CR_RUN_ID=cpu-class; CR_REVISION=fixture; CR_SCENARIO=three-of-three
  CR_OPERATORS=3; CR_THRESHOLD=3; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=fixture
  cr_record MAT-01-BOUNDARY-FANOUT ${state} "\${base_record[@]}" detail='fixture outcome' \\
    assert=bytes=pass:fixture assert=digest=pass:fixture assert=provenance=pass:fixture \\
    assert=liveness=pass:fixture assert=quorum=pass:fixture
}
run_cpu_branch
`], { env: { ...process.env, CONSENSUS_SCHEDULING_CLASSES: "stale-inherited-value", CONSENSUS_RESULTS_DIR: path.join(dir, "results"), CONSENSUS_ARTIFACT_IDENTITIES_FILE: "", CONSENSUS_ARTIFACT_IDENTITIES: "" }, timeout: 10000 });
      expect(result.exitCode, result.stderr.toString()).toBe(0);
      expect(readFileSync(path.join(dir, "docker-env"), "utf8")).toContain(`CONSENSUS_SCHEDULING_CLASSES=${expected}\n`);
      expect(readFileSync(path.join(dir, "exported"), "utf8")).toBe(expected);
      expect(readFileSync(path.join(dir, "observations"), "utf8").trim().split("\n")).toHaveLength(3);
      const record = JSON.parse(readFileSync(path.join(dir, "results/cpu-class.jsonl"), "utf8"));
      expect(record.state).toBe(state);
      expect(record.schedulingClasses).toBe(expected);
    } finally { rmSync(dir, { recursive: true, force: true }); }
  });
}

test("CPU materialization refuses failed scheduling discovery before launching or recording", () => {
  const result = Bun.spawnSync(["bash", "-c", `set -uo pipefail
REPO_ROOT=/unused
sr_revision() { echo fixture; }
observed_scheduling_classes() { echo partial; return 1; }
die() { echo "$*" >&2; exit 91; }
run_cpu_branch() {
  local -a class_env=()
${cpuBranch}
  echo unsafe-continuation
}
run_cpu_branch
`], { timeout: 5000 });
  expect(result.exitCode).toBe(91);
  expect(result.stderr.toString()).toContain("cannot establish CPU scheduling classes");
  expect(result.stdout.toString()).not.toContain("unsafe-continuation");
});
