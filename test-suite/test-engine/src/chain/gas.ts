import type { ChainGas } from '../profiles/types.js';

/**
 * Gas limit for a transaction sent without an explicit `gasLimit`: the estimate plus
 * `bufferPercent`, capped at `limitCap`. Same rule as `withGasBuffer` in the e2e suite.
 */
export function bufferedGasLimit(estimate: bigint, gas: ChainGas): bigint {
  const buffered = (estimate * (100n + BigInt(gas.bufferPercent))) / 100n;
  const cap = BigInt(gas.limitCap);
  return buffered < cap ? buffered : cap;
}
