/**
 * Reference semantics for the shift/rotate operators, mirroring tfhe-rs.
 *
 * tfhe-rs >= 1.7.0 returns 0 on an overshift (amount >= bit width) instead of
 * truncating the amount. Flip the flag together with the engine version bump.
 */
export const OVERSHIFT_RETURNS_ZERO = false;

const mask = (bits: bigint): bigint => (1n << bits) - 1n;

export function expectedShl(value: bigint, amount: bigint, bits: bigint): bigint {
  if (amount >= bits && OVERSHIFT_RETURNS_ZERO) return 0n;
  // legacy semantics truncate an oversized amount rather than saturating
  return (value << (amount % bits)) & mask(bits);
}

export function expectedShr(value: bigint, amount: bigint, bits: bigint): bigint {
  if (amount >= bits && OVERSHIFT_RETURNS_ZERO) return 0n;
  // legacy semantics truncate an oversized amount rather than saturating
  return value >> (amount % bits);
}

export function expectedRotl(value: bigint, amount: bigint, bits: bigint): bigint {
  const n = amount % bits;
  return ((value << n) | (value >> (bits - n))) & mask(bits);
}

export function expectedRotr(value: bigint, amount: bigint, bits: bigint): bigint {
  const n = amount % bits;
  return ((value >> n) | (value << (bits - n))) & mask(bits);
}
