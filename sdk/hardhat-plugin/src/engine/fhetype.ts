/**
 * FHE type identifiers and their clear-text sizes.
 *
 * For INPUT handles the "FhevmType" and the on-chain "FheType" numeric ids coincide (verified against
 * the old `@fhevm/mock-utils`), so a single enum suffices here. `byteLength` is `ceil(clearBits / 8)`:
 * the size of the clear value as packed into the mock ciphertext blob preimage. It is NOT the encrypted
 * bit width (bool encrypts as 2 bits, etc.) — that width only matters for the 2048-bit packing guard,
 * which the mock does not need to reproduce byte-for-byte.
 */
export enum FheType {
  ebool = 0,
  euint8 = 2,
  euint16 = 3,
  euint32 = 4,
  euint64 = 5,
  euint128 = 6,
  eaddress = 7,
  euint256 = 8,
}

interface FheTypeInfo {
  readonly clearBits: number;
  readonly byteLength: number;
  /** Max clear value; undefined for eaddress (validated as an address instead). */
  readonly maxValue?: bigint;
}

function uintInfo(clearBits: number): FheTypeInfo {
  return { clearBits, byteLength: Math.ceil(clearBits / 8), maxValue: (1n << BigInt(clearBits)) - 1n };
}

export const FHE_TYPE_INFO: Record<FheType, FheTypeInfo> = {
  [FheType.ebool]: { clearBits: 1, byteLength: 1, maxValue: 1n },
  [FheType.euint8]: uintInfo(8),
  [FheType.euint16]: uintInfo(16),
  [FheType.euint32]: uintInfo(32),
  [FheType.euint64]: uintInfo(64),
  [FheType.euint128]: uintInfo(128),
  [FheType.eaddress]: { clearBits: 160, byteLength: 20 },
  [FheType.euint256]: uintInfo(256),
};

export function fheTypeByteLength(type: FheType): number {
  return FHE_TYPE_INFO[type].byteLength;
}
