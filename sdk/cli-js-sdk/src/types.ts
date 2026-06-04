import type { Hex } from "viem";

export const NETWORKS = ["testnet", "devnet", "devnet-amoy"] as const;
export type NetworkName = (typeof NETWORKS)[number];

export const FHE_VALUE_TYPES = [
  "bool",
  "uint8",
  "uint16",
  "uint32",
  "uint64",
  "uint128",
  "uint256",
  "address",
] as const;
export type FheValueType = (typeof FHE_VALUE_TYPES)[number];

export const FHE_TYPE_IDS = {
  bool: 0,
  uint8: 2,
  uint16: 3,
  uint32: 4,
  uint64: 5,
  uint128: 6,
  address: 7,
  uint256: 8,
} as const satisfies Record<FheValueType, number>;

export type FheClearValue = boolean | number | bigint | string;

export type EncryptValue = Readonly<{
  type: FheValueType;
  value: FheClearValue;
}>;

export type InputProofResult = Readonly<{
  contractAddress: Hex;
  userAddress: Hex;
  values: readonly EncryptValue[];
  encryptedValues: readonly Hex[];
  inputProof: Hex;
}>;

export type PublicDecryptResult = Readonly<{
  encryptedValues: readonly Hex[];
  clearValues: readonly Readonly<{ type: string; value: string }>[];
  abiEncodedCleartexts: Hex;
  decryptionProof: Hex;
}>;

export type DecryptedValue = Readonly<{
  type: string;
  value: string;
}>;

export type DecryptionPermitSummary = Readonly<{
  isDelegated: boolean;
  signerAddress: Hex;
  encryptedDataOwnerAddress: Hex;
  transportPublicKey: string;
  signature: Hex;
  contractAddresses: readonly string[];
  startTimestamp: number;
  durationDays: number;
}>;

export type UserDecryptResult = Readonly<{
  contractAddress: Hex;
  ownerAddress: Hex;
  signerAddress: Hex;
  isDelegated: boolean;
  encryptedValues: readonly Hex[];
  clearValues: readonly DecryptedValue[];
  permit: DecryptionPermitSummary;
}>;

export type FheTestHandle = Readonly<{
  type: FheValueType;
  fheTypeId: number;
  account: Hex;
  handle: Hex;
  clearText?: string;
}>;
