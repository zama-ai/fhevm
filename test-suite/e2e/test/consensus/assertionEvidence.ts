/** Receipts name assertion groups that actually finished in this exact run. */
export type AssertionKind = 'precondition' | 'fault' | 'bytes' | 'digest' | 'provenance' | 'liveness' | 'quorum' | 'safety' | 'correctness' | 'scope' | 'sensitivity' | 'evidence';
export function emitAssertions(caseId: string, names: readonly AssertionKind[], detail: string): void {
  const runId = process.env.CONSENSUS_RUN_ID;
  if (!runId || !caseId || names.length === 0 || !detail.trim()) throw new Error('Assertion receipts require run, case, group and executed-check detail');
  for (const name of new Set(names)) console.info(`[consensus-assertion] ${JSON.stringify({ runId, caseId, name, outcome: 'pass', detail })}`);
}

export async function assertionGroup<T>(caseId: string, names: readonly AssertionKind[], detail: string, check: () => Promise<T>): Promise<T> {
  const result = await check();
  emitAssertions(caseId, names, detail);
  return result;
}
