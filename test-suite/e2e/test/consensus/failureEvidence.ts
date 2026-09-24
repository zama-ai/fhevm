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

/** A truncated handshake must not silently shrink a backlog acceptance set. */
export function requireBacklogTargets(handles: string[], transactions: string[], blocks?: unknown): void {
  if (handles.length !== 12 || transactions.length !== 12 || new Set(handles).size !== 12 || new Set(transactions).size !== 12 ||
      [...handles, ...transactions].some(value => !/^0x[0-9a-f]{64}$/.test(value))) throw new Error('backlog needs twelve unique receipt-identified targets');
  if (blocks !== undefined && (!Array.isArray(blocks) || blocks.length !== 12 ||
      blocks.some((height, index) => !Number.isSafeInteger(height) || height < 1 || (index > 0 && height <= blocks[index - 1])) ||
      blocks[11] - blocks[0] < 8)) throw new Error('backlog must span more than two four-block pages');
}
