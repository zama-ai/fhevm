/** Required fault evidence must never turn absent observations into defaults. */
export function offlineCatchupRange(watermark: number | null, head: number, blocks: number[]): { start: number; end: number } {
  const start = requiredEvidenceCount(watermark ?? 0, 'offline ingestion watermark') + 1;
  const end = requiredEvidenceCount(head, 'current RPC head', 1);
  if (blocks.length !== 12 || start > end || blocks.some(block => block < start || block > end)) {
    throw new Error(`offline catchup [${start}, ${end}] must cover all twelve receipt blocks [${blocks.join(', ')}]`);
  }
  return { start, end };
}

export function requireCompletedConsumerCatchup(
  row: { id?: unknown; status?: unknown; finalized?: unknown; blockStart?: unknown; blockEnd?: unknown },
  previousId: unknown,
  blocks: unknown,
): number[] {
  if (typeof previousId !== 'string' || !previousId || typeof row.id !== 'string' || !row.id || row.id === previousId) {
    throw new Error('offline recovery requires a new catchup identity');
  }
  if (row.finalized !== false || row.status !== 'DONE') {
    throw new Error('offline recovery requires a completed non-finalized catchup');
  }
  if (!Array.isArray(blocks) || blocks.length !== 12) throw new Error('offline recovery requires twelve receipt blocks');
  const start = requiredEvidenceCount(row.blockStart, 'catchup start');
  const end = requiredEvidenceCount(row.blockEnd, 'catchup end');
  const receipts = blocks.map(block => requiredEvidenceCount(block, 'receipt block', 1));
  const covered = receipts.filter(block => block >= start && block <= end);
  if (covered.length === 0) throw new Error(`catchup ${row.id} [${start}, ${end}] covers none of the offline receipt blocks [${receipts.join(', ')}]`);
  return covered;
}

export function missingIngestedReceiptBlocks(
  blocks: number[],
  blockHashes: unknown,
  ingested: { blockNumber: number; blockHash: string; blockStatus: string }[],
): number[] {
  if (!Array.isArray(blockHashes) || blockHashes.length !== blocks.length ||
      blockHashes.some(hash => typeof hash !== 'string' || !/^0x[0-9a-f]{64}$/.test(hash))) {
    throw new Error('offline recovery requires the exact hash of every receipt block');
  }
  const stored = new Set(ingested
    .filter(block => block.blockStatus === 'pending' || block.blockStatus === 'finalized')
    .map(block => `${block.blockNumber}:${block.blockHash}`));
  return blocks.filter((block, index) => !stored.has(`${block}:${blockHashes[index]}`));
}

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
      blocks[11] - blocks[0] < 8)) throw new Error('backlog must span at least nine host blocks so catch-up needs several pages');
}
