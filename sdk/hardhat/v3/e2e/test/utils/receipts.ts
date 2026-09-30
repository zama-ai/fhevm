// `tx.wait()` is typed `| null` for the zero-confirmation case, which these tests never ask for.
// Generic on purpose: naming ethers' receipt type would make `ethers` a direct dependency of the suite.
export function requireReceipt<T>(receipt: T | null): T {
  if (receipt === null) {
    throw new Error('Expected a transaction receipt');
  }
  return receipt;
}
