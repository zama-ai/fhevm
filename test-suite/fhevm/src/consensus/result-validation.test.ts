import { expect, test } from "bun:test";
import { mkdtempSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { aggregate, appendCaseResult, parseCaseResult, ResultError, type CaseResult } from "./results";
import { loadInventory } from "./inventory";
const inventory = loadInventory();
const entry = inventory.cases.find((row) => row.id === "MAT-01-BOUNDARY-FANOUT")!;
const good = (): CaseResult => ({ schemaVersion: 2, runId: "validation", caseId: entry.id, state: "PASS", revision: "source",
  executionClass: { software: "source", backend: "cpu", hardware: "cpu" }, topology: { scenario: entry.topology.scenario, operators: entry.topology.operators, threshold: entry.topology.threshold },
  startedAt: "2026-09-13T00:00:00Z", endedAt: "2026-09-13T00:01:00Z",
  assertions: entry.assertions.map((item) => ({ name: item.split(":")[0], outcome: "pass" })), cleanup: { state: "ok" },
});
const patches: Record<string, unknown>[] = [
  { executionClass: {} }, { executionClass: [] }, { executionClass: { software: "source", backend: 4, hardware: "cpu" } },
  { topology: { ...entry.topology, operators: "3" } }, { topology: { ...entry.topology, threshold: NaN } },
  { topology: { ...entry.topology, threshold: 4 } }, { topology: { ...entry.topology, scenario: " " } },
  { assertions: null }, { assertions: [null] }, { assertions: [{ name: "safety", outcome: "sure" }] },
  { assertions: [{ name: "", outcome: "pass" }] }, { cleanup: {} }, { cleanup: { state: "maybe" } },
  { workloadIds: [7] }, { workloadIds: [""] }, { processesBefore: [null] },
  { processesAfter: [{ target: "worker", identity: "id", pid: -1 }] }, { processesAfter: [{ target: "worker" }] },
  { artifactIdentities: [] }, { artifactIdentities: { image: 3 } }, { artifactIdentities: { image: "" } },
  { startedAt: "yesterday" }, { endedAt: "2026-09-12T23:59:59Z" }, { faultObservedAt: "invalid" },
  { recoveryObservedAt: "2026-09-13T00:00:01Z", faultObservedAt: "2026-09-13T00:00:02Z" },
  { state: "FAIL", detail: {} }, { state: "FAIL", detail: " " }, { runId: "../../escape" },
];
test("malformed records are rejected before aggregate accesses nested fields", () => {
  for (const patch of patches) {
    const record = { ...good(), ...patch };
    expect(() => parseCaseResult(record), JSON.stringify(patch)).toThrow(ResultError);
    const report = aggregate({ inventory, selected: [entry], results: [record as CaseResult], partial: true });
    expect(report.ok, JSON.stringify(patch)).toBe(false);
    expect(report.problems.join("\n")).not.toContain("TypeError");
  }
});
test("valid records round-trip, including full process identity and observation timestamps", () => {
  const record = { ...good(), workloadIds: ["0xhandle"], processesAfter: [{ target: "worker", identity: "pid=2 uuid=3", pid: 2 }],
    faultObservedAt: "2026-09-13T00:00:10Z", recoveryObservedAt: "2026-09-13T00:00:20Z" };
  expect(parseCaseResult(record)).toMatchObject(record);
  expect(aggregate({ inventory, selected: [entry], results: [record], partial: true }).ok).toBe(true);
});
test("append also rejects invalid cleanup or unsafe output names before creating files", () => {
  const directory = mkdtempSync(path.join(tmpdir(), "result-validation-"));
  try {
    for (const patch of [{ cleanup: {} }, { runId: "../escape" }]) expect(() => appendCaseResult({ ...good(), ...patch } as CaseResult, directory)).toThrow(ResultError);
    expect(readdirSync(directory)).toEqual([]);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
