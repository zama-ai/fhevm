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
