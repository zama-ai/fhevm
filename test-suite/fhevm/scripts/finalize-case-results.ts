import { appendCaseResult, parseCaseResult, readCaseResults, type CaseResult } from "../src/consensus/results";
import { existsSync } from "node:fs";

/** A successful subprocess is insufficient without its own terminal evidence. */
export const requireSuccessfulCase = (records: CaseResult[], caseId: string, runId: string): void => {
  const matching = records.filter((record) => record.caseId === caseId && record.runId === runId);
  if (matching.length === 0) throw new Error(`missing staged verdict for ${caseId} in run ${runId}`);
  if (matching.some((record) => !["PASS", "NOT_APPLICABLE"].includes(record.state) || record.cleanup.state === "failed")) {
    throw new Error(`unsuccessful staged verdict for ${caseId} in run ${runId}`);
  }
};

/** Publish child evidence only after its parent's deadline and cleanup verdict. */
export const finalizeCaseResults = (
  records: CaseResult[],
  outcome?: { state: "FAIL" | "INVALID"; cleanup: "ok" | "failed"; detail: string },
): CaseResult[] => {
  const final = new Map<string, CaseResult>();
  for (const record of records) {
    const key = `${record.runId}:${record.caseId}`;
    const previous = final.get(key);
    if (previous && previous.state !== record.state && !outcome) {
      throw new Error(`conflicting child verdicts for ${record.caseId}`);
    }
    // A parent cleanup/deadline failure invalidates every sibling claim from
    // this execution, including CR01/REG03 emitted by the delegated TFHE case.
    final.set(key, outcome ? parseCaseResult({
      ...record,
      state: record.state === "FAIL" || record.state === "INVALID" ? record.state : outcome.state,
      cleanup: { state: outcome.cleanup, detail: outcome.detail },
      detail: record.detail ? `${record.detail}; ${outcome.detail}` : outcome.detail,
      endedAt: new Date().toISOString(),
    }) : record);
  }
  return [...final.values()];
};

if (import.meta.main) {
  if (process.argv[2] === "--has-failure") {
    const [, source, runId, ...extra] = process.argv.slice(2);
    if (!source || !runId || extra.length) throw new Error("usage: --has-failure <staged-results> <run-id>");
    const records = existsSync(source) ? readCaseResults(source) : [];
    process.exit(records.some((record) => record.runId === runId && ["FAIL", "INVALID", "NOT_RUN"].includes(record.state)) ? 0 : 1);
  }
  if (process.argv[2] === "--require-case") {
    const [, source, caseId, runId, ...extra] = process.argv.slice(2);
    if (!source || !caseId || !runId || extra.length) throw new Error("usage: --require-case <staged-results> <case-id> <run-id>");
    requireSuccessfulCase(existsSync(source) ? readCaseResults(source) : [], caseId, runId);
    process.exit(0);
  }
  const [source, destination, state = "keep", cleanup, detail, ...extra] = process.argv.slice(2);
  if (!source || !destination || extra.length || !["keep", "FAIL", "INVALID"].includes(state)) {
    throw new Error("usage: finalize-case-results.ts <staged-results> <results-dir> [keep|FAIL|INVALID] [ok|failed] [detail]");
  }
  if (state !== "keep" && ((cleanup !== "ok" && cleanup !== "failed") || !detail)) throw new Error("missing parent recovery verdict");
  const records = existsSync(source) ? readCaseResults(source) : [];
  const outcome = state === "keep" ? undefined : { state: state as "FAIL" | "INVALID", cleanup: cleanup as "ok" | "failed", detail };
  for (const record of finalizeCaseResults(records, outcome)) {
    appendCaseResult(record, destination);
    console.log(`${record.runId}:${record.caseId}`);
  }
}
