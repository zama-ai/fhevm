import type { EncryptionBits } from "./fheType.js";
import type { FhevmHandle } from "./fhevmHandle.js";
import type { Bytes, ChecksummedAddress, Uint64BigInt } from "./primitives.js";

////////////////////////////////////////////////////////////////////////////////
//
// ZkProof
//
////////////////////////////////////////////////////////////////////////////////

export interface ZkProofLike {
  readonly chainId: bigint | number;
  readonly aclContractAddress: string;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly ciphertextWithZkProof: Uint8Array | string;
  readonly encryptionBits?: readonly number[];
}

export interface ZkProof {
  readonly chainId: Uint64BigInt;
  readonly aclContractAddress: ChecksummedAddress;
  readonly contractAddress: ChecksummedAddress;
  readonly userAddress: ChecksummedAddress;
  readonly ciphertextWithZkProof: Bytes;
  readonly encryptionBits: readonly EncryptionBits[];

  getFhevmHandles(): readonly FhevmHandle[];
}
