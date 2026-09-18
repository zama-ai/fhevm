/** Required fault evidence must never turn absent observations into defaults. */
export function requiredEvidenceCount(value: unknown, label: string, minimum = 0): number {
  if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < minimum) {
    throw new Error(`${label} must be a recorded integer >= ${minimum}`);
  }
  return value;
}

export function requireIngestionTarget(detail: Record<string, unknown> | undefined): { blockNumber: number; chainId: string } {
  const blockNumber = requiredEvidenceCount(detail?.blockNumber, 'ingestion workload block', 1);
  const chainId = detail?.chainId;
  if (typeof chainId !== 'string' || !/^[1-9][0-9]*$/.test(chainId)) throw new Error('ingestion workload chain ID is missing or invalid');
  return { blockNumber, chainId };
}

export function requireDriftBaseline(detail: Record<string, unknown> | undefined, victim: number, operators: number): number[] {
  const baseline = requiredEvidenceCount(detail?.signalsBefore, 'victim drift baseline');
  const all = detail?.signalsBeforeByOperator;
  if (!Array.isArray(all) || all.length !== operators || !Number.isInteger(victim) || victim < 0 || victim >= operators) {
    throw new Error('drift baseline must cover every selected operator');
  }
  const counts = all.map((value, index) => requiredEvidenceCount(value, `operator ${index} drift baseline`));
  if (counts[victim] !== baseline) throw new Error('victim drift baseline disagrees with fleet baseline');
  return counts;
}
