import { InvalidArgumentError } from "@commander-js/extra-typings";
import { isAddress, isHex, type Hex } from "viem";

import { FHE_VALUE_TYPES, NETWORKS } from "../types";
import type { FheValueType, NetworkName } from "../types";

export const parseNetwork = (value: string): NetworkName => {
  if (NETWORKS.includes(value as NetworkName)) return value as NetworkName;
  throw new InvalidArgumentError(
    `Unsupported network "${value}". Supported: ${NETWORKS.join(", ")}`,
  );
};

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

export const collectHandle = (value: string, previous: Hex[] = []): Hex[] => [
  ...previous,
  parseBytes32(value),
];
