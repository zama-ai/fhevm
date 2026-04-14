/**
 * Executes promise factories with control over batching behavior.
 * @param factories - Array of functions that create promises (not promises themselves)
 * @param parallel - If true, executes all concurrently. If false, executes one at a time (default `false`).
 *
 * @example
 * ```typescript
 *  const rpcCalls = [
 *    () => contract.balanceOf(address1),
 *    () => contract.balanceOf(address2),
 *    () => contract.totalSupply(),
 *. ];
 *
 *  // Sequential: one RPC call at a time
 *  const resultsSeq = await executeWithBatching(rpcCalls, false);
 *
 *  // Concurrent: all fire together (lets ethers batch them)
 *  const resultsConcurrent = await executeWithBatching(rpcCalls, true);
 * ```
 */
export async function executeWithBatching<T>(factories: Array<() => Promise<T>>, parallel?: boolean): Promise<T[]> {
  if (parallel === true) {
    return Promise.all(factories.map((f) => f()));
  }

  const results: T[] = [];
  for (const factory of factories) {
    results.push(await factory());
  }
  return results;
}
