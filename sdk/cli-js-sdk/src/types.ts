import type { Hex } from "viem";

export const NETWORKS = ["testnet"] as const;
export type NetworkName = (typeof NETWORKS)[number];

export const DECRYPT_TYPES = [
  "bool",
  "uint8",
  "uint128",
  "address",
  "mixed",
] as const;
export type DecryptType = (typeof DECRYPT_TYPES)[number];

export type EncryptValue = Readonly<{
  type:
    | "bool"
    | "uint8"
    | "uint16"
    | "uint32"
    | "uint64"
    | "uint128"
    | "uint256"
    | "address";
  value: boolean | number | bigint | string;
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
