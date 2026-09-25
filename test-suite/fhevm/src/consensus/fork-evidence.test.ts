import { expect, test } from "bun:test";
import { forkEvidence } from "./fork-evidence";
const row = { runId: "run", caseId: "FORK-02-DISTINCT-HANDLES", workloadIds: [`0x${"ab".repeat(32)}`],
  faultObservedAt: "2026-09-18T00:00:00.000Z", artifacts: { canonical_block: `0x${"cd".repeat(32)}` } };
const line = (value: unknown = row) => `[consensus-fault] ${JSON.stringify(value)}`;
test("fault evidence is bound to the exact case and run", () => {
  expect(forkEvidence(line(), "run", row.caseId)).toContain(`workload=${row.workloadIds[0]}`);
  expect(() => forkEvidence(line(), "another", row.caseId)).toThrow();
  expect(() => forkEvidence(line(), "run", "FORK-03-ORPHAN-ALLOW-INERT")).toThrow();
  expect(() => forkEvidence(`${line()}\n${line()}`, "run", row.caseId)).toThrow();
});
test("fault evidence needs a real timestamp, workload and block evidence", () => {
  for (const patch of [{ workloadIds: [] }, { faultObservedAt: "never" }, { artifacts: {} }]) {
    expect(() => forkEvidence(line({ ...row, ...patch }), "run", row.caseId)).toThrow();
  }
});
