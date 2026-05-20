import type { Hex } from "viem";

import { randomAddress, randomUint8, randomUint16, randomUint32, randomUint64, randomUint128, randomUint256 } from "./random";
import type { DecryptType, EncryptValue } from "./types";

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

export const createFreshDecryptValues = (decryptType: DecryptType): readonly EncryptValue[] => {
  switch (decryptType) {
    case "bool":
      return [{ type: "bool", value: randomUint8() % 2 === 0 }];
    case "uint8":
      return [{ type: "uint8", value: randomUint8() }];
    case "uint128":
      return [{ type: "uint128", value: randomUint128() }];
    case "address":
      return [{ type: "address", value: randomAddress() }];
    case "mixed":
      return [
        { type: "bool", value: randomUint8() % 2 === 0 },
        { type: "uint8", value: randomUint8() },
        { type: "uint128", value: randomUint128() },
        { type: "address", value: randomAddress() },
      ];
  }
};

export const serializeValue = (value: EncryptValue): Readonly<{ type: string; value: string }> => ({
  type: value.type,
  value: typeof value.value === "bigint" ? value.value.toString() : String(value.value),
});

export const normalizeHexArray = (values: readonly string[]): readonly Hex[] => values.map((value) => value as Hex);
