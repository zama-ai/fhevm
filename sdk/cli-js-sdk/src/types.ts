import type { Hex } from "viem";

export const NETWORKS = ["testnet", "devnet", "devnet-amoy"] as const;
export type NetworkName = (typeof NETWORKS)[number];

/**
 * FHETest value kinds supported by the CLI and by the contract helper mappings.
 *
 * Keep this list aligned with FHETest.sol setter/getter coverage and
 * {@link FHE_TYPE_IDS}; completion choices and parser validation derive from it.
 */
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

/**
 * FHETest's numeric type identifiers, also embedded in the last byte of handles.
 */
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

/** Clear input value tagged with the FHETest/FHEVM encrypted type to create. */
export type EncryptValue = Readonly<{
  type: FheValueType;
  value: FheClearValue;
}>;

/** Output from creating encrypted inputs and a verified input proof. */
export type InputProofResult = Readonly<{
  contractAddress: Hex;
  userAddress: Hex;
  values: readonly EncryptValue[];
  encryptedValues: readonly Hex[];
  inputProof: Hex;
}>;

/**
 * Public decryption output returned by the relayer, including proof material
 * that can be passed to on-chain signature verification.
 */
export type PublicDecryptResult = Readonly<{
  encryptedValues: readonly Hex[];
  clearValues: readonly Readonly<{ type: string; value: string }>[];
  abiEncodedCleartexts: Hex;
  decryptionProof: Hex;
}>;

/** Decrypted value normalized for JSON output; bigint-like values are strings. */
export type DecryptedValue = Readonly<{
  type: string;
  value: string;
}>;

/**
 * Stable JSON summary of the SDK permit used for user decryption.
 *
 * The transport private key is intentionally omitted.
 */
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

/** Result for self or delegated user decryption of one or more handles. */
export type UserDecryptResult = Readonly<{
  contractAddress: Hex;
  ownerAddress: Hex;
  signerAddress: Hex;
  isDelegated: boolean;
  encryptedValues: readonly Hex[];
  clearValues: readonly DecryptedValue[];
  permit: DecryptionPermitSummary;
}>;

/**
 * FHETest handle metadata read from the contract for an account/type pair.
 *
 * `clearText` is present when FHETest keeps an inspectable cleartext mirror for
 * the handle. It is not the relayer decryption result.
 */
export type FheTestHandle = Readonly<{
  type: FheValueType;
  fheTypeId: number;
  account: Hex;
  handle: Hex;
  clearText?: string;
}>;
