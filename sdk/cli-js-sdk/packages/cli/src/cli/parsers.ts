import { InvalidArgumentError } from "@commander-js/extra-typings";
import { isAddress, isHex, type Hex } from "viem";

import { FHE_VALUE_TYPES } from "@cli-fhevm-sdk/toolkit/types";
import type { FheValueType } from "@cli-fhevm-sdk/toolkit/types";

export const parseValueType = (value: string): FheValueType => {
  if (FHE_VALUE_TYPES.includes(value as FheValueType)) {
    return value as FheValueType;
  }
  throw new InvalidArgumentError(
    `Unsupported type "${value}". Supported: ${FHE_VALUE_TYPES.join(", ")}`,
  );
};

export const parseAddress = (value: string): Hex => {
  if (isAddress(value)) return value;
  throw new InvalidArgumentError(`Invalid address: ${value}`);
};

export const parseBytes32 = (value: string): Hex => {
  if (isHex(value) && value.length === 66) return value;
  throw new InvalidArgumentError(`Invalid bytes32 hex value: ${value}`);
};

export const parsePrivateKey = (value: string): Hex => {
  if (isHex(value) && value.length === 66) return value;
  throw new InvalidArgumentError(`Invalid private key: ${value}`);
};

export const parsePositiveInteger = (value: string): number => {
  const parsed = Number(value);
  if (Number.isSafeInteger(parsed) && parsed > 0) return parsed;
  throw new InvalidArgumentError(`Invalid positive integer: ${value}`);
};

const EUINT64_UPPER_BOUND = 1n << 64n;

export const parseTokenAmount = (value: string): bigint => {
  let parsed: bigint;
  try {
    parsed = BigInt(value);
  } catch {
    throw new InvalidArgumentError(`Invalid amount: ${value}`);
  }
  if (parsed > 0n && parsed < EUINT64_UPPER_BOUND) return parsed;
  throw new InvalidArgumentError(
    `Amount must be greater than 0 and less than 2^64: ${value}`,
  );
};

export const collectHandle = (value: string, previous: Hex[] = []): Hex[] => [
  ...previous,
  parseBytes32(value),
];

export const collectValueType = (
  value: string,
  previous: FheValueType[] = [],
): FheValueType[] => [...previous, parseValueType(value)];
