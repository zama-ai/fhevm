import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
const scripts = path.resolve(import.meta.dir, "../../scripts");

test("database assertion diagnostics persist as useful redacted FAIL while raw record still rejects credentials", () => {
  const dir = mkdtempSync(path.join(tmpdir(), "redacted-diagnostic-"));
  try {
    const run = Bun.spawnSync(["bash", "-c", `set -uo pipefail
SCRIPT_DIR='${scripts}'; REPO_ROOT='${dir}'
source "$SCRIPT_DIR/lib/case-result.sh"
CR_RUN_ID=diagnostic; CR_REVISION=abc123; CR_BACKEND_CLASS=cpu; CR_HARDWARE_CLASS=cpu
CR_SCENARIO=none; CR_OPERATORS=0; CR_THRESHOLD=0
CONSENSUS_RESULTS_DIR='${dir}/records'; export CONSENSUS_RESULTS_DIR
reason="$(cr_failure_reason 'AssertionError: postgresql://test:dummy@localhost/coproc is missing fixture output')"
cr_record HAR-01-FAULT-CONTRACTS FAIL cleanup=ok detail="$reason" || exit 2
# Direct detail/cleanup/assert diagnostics are sanitized at the record boundary too.
cr_record HAR-02-INVENTORY-AGGREGATE FAIL cleanup=failed \\
 detail='postgres://test:dummy@localhost/coproc could not be queried' \\
 cleanup_detail='postgresql://test:dummy@localhost/coproc restore failed' \\
 assert='safety=fail:postgresql://test:dummy@localhost/coproc missing' || exit 3
long="$(printf 'x%.0s' {1..240})"
cr_failure_reason "AssertionError: postgresql://test:$long@localhost/coproc is missing fixture output"
bun "$CR_INVENTORY_CLI" record --run direct --case HAR-01-FAULT-CONTRACTS --state FAIL \\
 --revision abc123 --backend-class cpu --hardware-class cpu --scenario none --cleanup ok \\
 --detail 'postgresql://test:dummy@localhost/coproc assertion failed' --quiet >/dev/null 2>&1
[[ "$?" != 0 ]]
`], {timeout: 10000});
    expect(run.exitCode, run.stderr.toString()).toBe(0);
    const content = readFileSync(path.join(dir, "records/diagnostic.jsonl"), "utf8");
    const records = content.trim().split("\n").map((line) => JSON.parse(line));
    expect(records.map((record) => record.state)).toEqual(["FAIL", "FAIL"]);
    expect(records[0].detail).toBe("AssertionError: postgresql://[redacted]@localhost/coproc is missing fixture output");
    expect(content).not.toContain("dummy");
    expect(content).not.toContain("test:");
    expect(run.stdout.toString()).toContain("postgresql://[redacted]@localhost/coproc is missing fixture output");
    expect(run.stdout.toString()).not.toContain("xxxxxxxx");
  } finally { rmSync(dir, {recursive: true, force: true}); }
});
