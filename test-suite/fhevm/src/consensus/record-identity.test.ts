import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

test("record preserves complete process identities and assertion details containing equals", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "consensus-record-"));
  try {
    const before = "container=coprocessor1-tfhe-worker pid=100 started=before restarts=0";
    const after = "container=coprocessor1-tfhe-worker pid=200 started=after restarts=1";
    const result = Bun.spawnSync([process.execPath, "scripts/consensus-inventory.ts", "record",
      "--case", "CR-01-INTERRUPT-BEFORE-COMMIT", "--run", "record-test", "--state", "PASS",
      "--revision", "test-revision", "--scenario", "three-of-three", "--operators", "3", "--threshold", "3",
      "--workload", "identified-work", "--fault-observed-at", "2026-09-13T00:00:05Z", "--cleanup", "ok",
      "--assert", "replacement=pass:old=100 new=200",
      "--assert", "precondition=pass", "--assert", "quorum=pass", "--assert", "fault=pass", "--assert", "liveness=pass", "--assert", "bytes=pass", "--assert", "safety=pass", "--process-before", `worker=${before}`,
      "--process-after", `worker=${after}`, "--results-dir", directory,
    ], {cwd: path.resolve(import.meta.dir, "../..")});
    expect(result.exitCode, result.stderr.toString()).toBe(0);
    const record = JSON.parse(readFileSync(path.join(directory, "record-test.jsonl"), "utf8"));
    expect(record.processesBefore).toEqual([{target: "worker", identity: before}]);
    expect(record.processesAfter).toEqual([{target: "worker", identity: after}]);
    expect(record.assertions[0].detail).toBe("old=100 new=200");
  } finally { rmSync(directory, {recursive: true, force: true}); }
});
