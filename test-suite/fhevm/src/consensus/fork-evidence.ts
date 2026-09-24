/** Case-bound observations emitted only after that case observed its own fault. */
export function forkEvidence(output: string, runId: string, caseId: string): string[] {
  const prefix = "[consensus-fault] ";
  const receipts = output.split("\n").filter(line => line.startsWith(prefix)).map(line => JSON.parse(line.slice(prefix.length)));
  const matches = receipts.filter(row => row?.runId === runId && row?.caseId === caseId);
  if (matches.length !== 1) throw new Error(`${caseId}: expected exactly one fault observation for this run and case`);
  const row = matches[0];
  if (!Array.isArray(row.workloadIds) || row.workloadIds.length === 0 ||
      row.workloadIds.some((value: unknown) => typeof value !== "string" || !/^0x[a-f0-9]{64}$/i.test(value)) ||
      typeof row.faultObservedAt !== "string" || !/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d{3}Z$/.test(row.faultObservedAt) ||
      !Number.isFinite(Date.parse(row.faultObservedAt)) || !row.artifacts || typeof row.artifacts !== "object" || Array.isArray(row.artifacts)) {
    throw new Error(`${caseId}: invalid fault observation`);
  }
  const artifacts = Object.entries(row.artifacts).map(([name, value]) => {
    if (!/^[a-z][a-z0-9_]*$/.test(name) || typeof value !== "string" || !/^0x[a-f0-9]{64}$/i.test(value)) throw new Error("invalid fork artifact");
    return `artifact=${name}=${value}`;
  });
  if (!artifacts.length) throw new Error("fork observation lacks block evidence");
  return [...row.workloadIds.map((value: string) => `workload=${value}`), `fault_observed_at=${row.faultObservedAt}`, ...artifacts];
}
