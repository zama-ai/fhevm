import { isAddress } from "viem";

import type { FheClearValue, FheValueType } from "../types";

const UINT_BITS = {
  uint8: 8n,
  uint16: 16n,
  uint32: 32n,
  uint64: 64n,
  uint128: 128n,
  uint256: 256n,
} as const satisfies Record<Exclude<FheValueType, "bool" | "address">, bigint>;

type UintValueType = keyof typeof UINT_BITS;

const maxUint = (bits: bigint): bigint => (1n << bits) - 1n;

const parseUint = (valueType: UintValueType, value: string): bigint => {
  if (value.length === 0 || value.trim() !== value) {
    throw new Error(`Invalid ${valueType} value: ${value}`);
  }

  let parsed: bigint;
  try {
    parsed = BigInt(value);
  } catch {
    throw new Error(`Invalid ${valueType} value: ${value}`);
  }

  if (parsed < 0n || parsed > maxUint(UINT_BITS[valueType])) {
    throw new Error(`Invalid ${valueType} value: ${value}`);
  }

  return parsed;
};

export const parseClearValue = (
  valueType: FheValueType,
  value: string,
): FheClearValue => {
  switch (valueType) {
    case "bool": {
      if (value === "true") return true;
      if (value === "false") return false;
      throw new Error('Bool values must be "true" or "false".');
    }
    case "address": {
      if (!isAddress(value)) throw new Error(`Invalid address: ${value}`);
      return value;
    }
    case "uint8":
    case "uint16":
    case "uint32": {
      return Number(parseUint(valueType, value));
    }
    case "uint64":
    case "uint128":
    case "uint256": {
      return parseUint(valueType, value);
    }
  }
};
