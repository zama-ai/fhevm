import type { InputHandle } from './encryptedTypes-p.js';
import type { EncryptionBits } from './fheType.js';
import type { Bytes, Bytes32Hex, BytesHex, ChecksummedAddress, Uint64BigInt } from './primitives.js';

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

  // Private functions
  getInputHandles(): readonly InputHandle[];
  getExtraData(): BytesHex;
}

////////////////////////////////////////////////////////////////////////////////
//
// SolanaZkProof — RFC-021 bytes32 host-identity counterpart of ZkProof. Kept as a
// parallel type so the EVM ZkProof's ChecksummedAddress surface stays untouched.
//
////////////////////////////////////////////////////////////////////////////////

export interface SolanaZkProofLike {
  readonly chainId: bigint | number;
  readonly aclContractAddress: string;
  readonly contractAddress: string;
  readonly userAddress: string;
  readonly ciphertextWithZkProof: Uint8Array | string;
  readonly encryptionBits: readonly EncryptionBits[];
}

export interface SolanaZkProof {
  readonly chainId: Uint64BigInt;
  readonly aclContractAddress: Bytes32Hex;
  readonly contractAddress: Bytes32Hex;
  readonly userAddress: Bytes32Hex;
  readonly ciphertextWithZkProof: Bytes;
  readonly encryptionBits: readonly EncryptionBits[];

  getInputHandles(): readonly InputHandle[];
}
