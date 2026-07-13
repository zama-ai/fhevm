import type { Hex } from "viem";

export const NETWORKS = ["testnet", "devnet", "devnet-amoy", "mainnet"] as const;
export type NetworkName = (typeof NETWORKS)[number];
export const DEFAULT_NETWORK: NetworkName = "testnet";

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

export const FHE_TEST_OPERATIONS = [
  "xor-bool",
  "add-uint8",
  "add-uint16",
  "add-uint32",
  "add-uint64",
  "add-uint128",
  "xor-uint256",
  "eq-address",
] as const;
export type FheTestOperation = (typeof FHE_TEST_OPERATIONS)[number];

export const FHE_TEST_OPERATION_CONFIG = {
  "xor-bool": { functionName: "xorEbool", type: "bool" },
  "add-uint8": { functionName: "addEuint8", type: "uint8" },
  "add-uint16": { functionName: "addEuint16", type: "uint16" },
  "add-uint32": { functionName: "addEuint32", type: "uint32" },
  "add-uint64": { functionName: "addEuint64", type: "uint64" },
  "add-uint128": { functionName: "addEuint128", type: "uint128" },
  "xor-uint256": { functionName: "xorEuint256", type: "uint256" },
  "eq-address": { functionName: "eqEaddress", type: "address" },
} as const satisfies Record<
  FheTestOperation,
  Readonly<{ functionName: string; type: FheValueType }>
>;

/** Returns the value type required by a supported FHETest operation. */
export const getFheTestOperationType = (
  operation: FheTestOperation,
): FheValueType => FHE_TEST_OPERATION_CONFIG[operation].type;

/** Returns the FHETest contract function name for a supported operation. */
export const getFheTestOperationFunctionName = (
  operation: FheTestOperation,
): string => FHE_TEST_OPERATION_CONFIG[operation].functionName;

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
  version: 1 | 2;
  isDelegated: boolean;
  signerAddress: Hex;
  encryptedDataOwnerAddress: Hex;
  transportPublicKey: string;
  signature: Hex;
  contractAddresses: readonly string[];
  startTimestamp: number;
  /** Signed permit lifetime in the canonical SDK unit. */
  durationSeconds: number;
}>;

/**
 * Sensitive validation artifact for replaying/verifying relayer user-decrypt
 * GET responses after the original request completed.
 *
 * The transport private key is required to decrypt signcrypted KMS shares. Do
 * not print this object to normal stdout or commit it to source control.
 */
export type UserDecryptValidationArtifact = Readonly<{
  schemaVersion: 2;
  flow: "user-decrypt" | "delegated-user-decrypt";
  network: NetworkName;
  relayer?: Readonly<{
    requestId?: string;
    jobId?: string;
  }>;
  contractAddress: Hex;
  ownerAddress: Hex;
  signerAddress: Hex;
  isDelegated: boolean;
  encryptedValues: readonly Hex[];
  handleContractPairs: readonly Readonly<{
    handle: Hex;
    contractAddress: Hex;
  }>[];
  transportKeyPair: Readonly<{
    publicKey: string;
    privateKey: string;
  }>;
  serializedPermit: unknown;
  permit: DecryptionPermitSummary;
  expectedClearValues?: readonly DecryptedValue[];
}>;

/** Result for self or delegated user decryption of one or more handles. */
export type UserDecryptResult = Readonly<{
  contractAddress: Hex;
  ownerAddress: Hex;
  signerAddress: Hex;
  isDelegated: boolean;
  relayer?: Readonly<{
    requestId?: string;
    jobId?: string;
  }>;
  encryptedValues: readonly Hex[];
  clearValues: readonly DecryptedValue[];
  permit: DecryptionPermitSummary;
  validationArtifact?: UserDecryptValidationArtifact;
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
