/**
 * Combinadic unranking: maps a rank to the k-combination of `[0, n)` with
 * that index in colexicographic order. Lets a persisted cursor enumerate
 * C(n, k) unique handle sets without materializing them — the relayer dedups
 * public-decrypt on the handle list, so every request needs a fresh set.
 */

export const binomial = (n: number, k: number): bigint => {
  if (k < 0 || k > n) return 0n;
  let result = 1n;
  const kk = Math.min(k, n - k);
  for (let i = 1; i <= kk; i += 1) {
    result = (result * BigInt(n - kk + i)) / BigInt(i);
  }
  return result;
};

export const minimumCombinationPoolSize = (needed: bigint, k: number): number => {
  if (needed <= 0n) return 0;
  if (k <= 0) throw new Error(`Combination size k must be positive, got ${k.toString()}.`);
  let low = k;
  let high = k;
  while (binomial(high, k) < needed) high *= 2;
  while (low < high) {
    const mid = Math.floor((low + high) / 2);
    if (binomial(mid, k) >= needed) high = mid;
    else low = mid + 1;
  }
  return low;
};

/**
 * Returns the combination of `k` distinct indices in `[0, n)` at `rank`
 * (colex order), ascending. Throws when the rank is out of range.
 */
export const unrankCombination = (rank: bigint, n: number, k: number): number[] => {
  if (rank < 0n || rank >= binomial(n, k)) {
    throw new Error(
      `Combination rank ${rank.toString()} out of range for C(${n.toString()}, ${k.toString()}).`,
    );
  }
  const indices: number[] = [];
  let remaining = rank;
  let element = k;
  // Colex: pick the largest index c with C(c, element) <= remaining.
  for (; element >= 1; element -= 1) {
    let c = element - 1;
    while (binomial(c + 1, element) <= remaining) c += 1;
    indices.push(c);
    remaining -= binomial(c, element);
  }
  return indices.reverse();
};
