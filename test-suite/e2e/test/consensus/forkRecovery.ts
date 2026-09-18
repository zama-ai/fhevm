/** Read state only after the exact branch transaction has succeeded on-chain. */
export async function successfulForkReceipt<T extends { status: number | null; blockNumber: number }>(
  transaction: { wait(): Promise<T | null> }, label: string,
): Promise<T> {
  const receipt = await transaction.wait();
  if (!receipt || receipt.status !== 1 || !Number.isSafeInteger(receipt.blockNumber) || receipt.blockNumber < 1) {
    throw new Error(`${label} did not produce a successful mined receipt`);
  }
  return receipt;
}

export interface ForkSentinelObservation {
  replacementSeen: boolean;
  total: number;
  completed: number;
  errors: number;
}

/** A stopped listener/worker cannot satisfy recovery with old fork evidence. */
export async function waitForForkSentinel(
  read: () => Promise<ForkSentinelObservation>,
  options: { timeoutMs?: number; now?: () => number; pause?: () => Promise<void> } = {},
): Promise<void> {
  const now = options.now ?? Date.now;
  const deadline = now() + (options.timeoutMs ?? 6 * 60_000);
  const pause = options.pause ?? (() => new Promise(resolve => setTimeout(resolve, 2_000)));
  for (;;) {
    const row = await read();
    if ([row.total, row.completed, row.errors].some(value => !Number.isSafeInteger(value) || value < 0)) throw new Error('invalid recovery counts');
    if (row.errors !== 0 || row.total > 1) throw new Error('canonical sentinel computation failed or duplicated');
    if (row.replacementSeen && row.total === 1 && row.completed === 1) return;
    if (now() >= deadline) throw new Error('forked operator did not ingest the canonical replacement and complete its fresh sentinel');
    await pause();
  }
}
