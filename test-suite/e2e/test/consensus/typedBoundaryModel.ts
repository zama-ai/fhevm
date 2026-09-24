/** Independent modular arithmetic, separate from Solidity's FHE calls. */
export function typedBoundaryModel(bits: 2 | 8 | 16 | 32 | 64 | 128 | 256, stage: boolean): (bigint | boolean)[] {
  if (bits === 2) return [...(stage ? [true] : []), false, true, false];
  const max = (1n << BigInt(bits)) - 1n;
  const arithmetic = bits === 256 ? [] : [0n, 1n, 2n, max - 1n, max / 2n, 1n, max - 1n, max];
  return [...(stage ? [0n, max] : []), ...arithmetic, max - 1n, true, max, bits === 8 ? max : 255n];
}
