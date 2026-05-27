import { isAddress, type Hex } from "viem";

import {
  randomAddress,
  randomUint8,
  randomUint16,
  randomUint32,
  randomUint64,
  randomUint128,
  randomUint256,
} from "./random";
import type { FheClearValue, FheValueType, EncryptValue } from "./types";

export const createInputProofValues = (): readonly EncryptValue[] => [
  { type: "bool", value: randomUint8() % 2 === 0 },
  { type: "uint8", value: randomUint8() },
  { type: "uint16", value: randomUint16() },
  { type: "uint32", value: randomUint32() },
  { type: "uint64", value: randomUint64() },
  { type: "uint128", value: randomUint128() },
  { type: "uint256", value: randomUint256() },
  { type: "address", value: randomAddress() },
];

export const createRandomValue = (valueType: FheValueType): EncryptValue => {
  switch (valueType) {
    case "bool":
      return { type: "bool", value: randomUint8() % 2 === 0 };
    case "uint8":
      return { type: "uint8", value: randomUint8() };
    case "uint16":
      return { type: "uint16", value: randomUint16() };
    case "uint32":
      return { type: "uint32", value: randomUint32() };
    case "uint64":
      return { type: "uint64", value: randomUint64() };
    case "uint128":
      return { type: "uint128", value: randomUint128() };
    case "uint256":
      return { type: "uint256", value: randomUint256() };
    case "address":
      return { type: "address", value: randomAddress() };
  }
};

export const createFreshDecryptValues = (
  valueType: FheValueType,
): readonly EncryptValue[] => [createRandomValue(valueType)];

const maxUint = (bits: bigint): bigint => (1n << bits) - 1n;

export const createInitValue = (valueType: FheValueType): EncryptValue => {
  switch (valueType) {
    case "bool":
      return { type: "bool", value: true };
    case "uint8":
      return { type: "uint8", value: 255 };
    case "uint16":
      return { type: "uint16", value: 65_535 };
    case "uint32":
      return { type: "uint32", value: 4_294_967_295 };
    case "uint64":
      return { type: "uint64", value: maxUint(64n) };
    case "uint128":
      return { type: "uint128", value: maxUint(128n) };
    case "uint256":
      return { type: "uint256", value: maxUint(256n) };
    case "address":
      return {
        type: "address",
        value: "0xffffffffffffffffffffffffffffffffffffffff",
      };
  }
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
      const parsed = Number(value);
      if (!Number.isInteger(parsed) || parsed < 0) {
        throw new Error(`Invalid ${valueType} value: ${value}`);
      }
      return parsed;
    }
    case "uint64":
    case "uint128":
    case "uint256": {
      const parsed = BigInt(value);
      if (parsed < 0n) throw new Error(`Invalid ${valueType} value: ${value}`);
      return parsed;
    }
  }
};

export const serializeValue = (
  value: EncryptValue,
): Readonly<{ type: string; value: string }> => ({
  type: value.type,
  value:
    typeof value.value === "bigint"
      ? value.value.toString()
      : String(value.value),
});

export const normalizeHexArray = (values: readonly string[]): readonly Hex[] =>
  values.map((value) => value as Hex);
