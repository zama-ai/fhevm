import type { EncryptValue, FheValueType } from "../types";
import {
  randomAddress,
  randomUint8,
  randomUint16,
  randomUint32,
  randomUint64,
  randomUint128,
  randomUint256,
} from "./random";

const maxUint = (bits: bigint): bigint => (1n << bits) - 1n;

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
