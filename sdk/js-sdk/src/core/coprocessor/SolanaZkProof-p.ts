import type { Bytes, Bytes32Hex, Uint64BigInt } from '../types/primitives.js';
import type { EncryptionBits, FheTypeId } from '../types/fheType.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import type { SolanaZkProof, SolanaZkProofLike } from '../types/zkProof-p.js';

import { keccak_256 } from '@noble/hashes/sha3.js';

import { asBytes32Hex, bytes32ToHex, concatBytes, hexToBytes32, toBytes } from '../base/bytes.js';
import { asUint64BigInt, uint256ToBytes32 } from '../base/uint.js';
import { fheTypeIdFromEncryptionBits } from '../handle/FheType.js';
import { buildHandle } from '../handle/FhevmHandle.js';
import { ZkProofError } from '../errors/ZkProofError.js';
import { FHEVM_HANDLE_HASH_DOMAIN_SEPARATOR, FHEVM_HANDLE_RAW_CT_HASH_DOMAIN_SEPARATOR } from './ZkProof-p.js';

export type { SolanaZkProof, SolanaZkProofLike } from '../types/zkProof-p.js';

const PRIVATE_TOKEN = Symbol('SolanaZkProof.token');

class SolanaZkProofImpl implements SolanaZkProof {
  readonly #chainId: Uint64BigInt;
  readonly #aclContractAddress: Bytes32Hex;
  readonly #contractAddress: Bytes32Hex;
  readonly #userAddress: Bytes32Hex;
  readonly #ciphertextWithZkProof: Bytes;
  readonly #encryptionBits: readonly EncryptionBits[];
  readonly #fheTypeIds: readonly FheTypeId[];
  #inputHandles: InputHandle[] | undefined;

  constructor(
    privateToken: symbol,
    params: {
      readonly chainId: Uint64BigInt;
      readonly aclContractAddress: Bytes32Hex;
      readonly contractAddress: Bytes32Hex;
      readonly userAddress: Bytes32Hex;
      readonly ciphertextWithZkProof: Bytes;
      readonly encryptionBits: readonly EncryptionBits[];
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#chainId = params.chainId;
    this.#aclContractAddress = params.aclContractAddress;
    this.#contractAddress = params.contractAddress;
    this.#userAddress = params.userAddress;
    this.#ciphertextWithZkProof = params.ciphertextWithZkProof;
    this.#encryptionBits = Object.freeze([...params.encryptionBits]);
    this.#fheTypeIds = Object.freeze(this.#encryptionBits.map(fheTypeIdFromEncryptionBits));
    Object.freeze(this);
  }

  public get chainId(): Uint64BigInt {
    return this.#chainId;
  }
  public get aclContractAddress(): Bytes32Hex {
    return this.#aclContractAddress;
  }
  public get contractAddress(): Bytes32Hex {
    return this.#contractAddress;
  }
  public get userAddress(): Bytes32Hex {
    return this.#userAddress;
  }
  public get ciphertextWithZkProof(): Bytes {
    return new Uint8Array(this.#ciphertextWithZkProof);
  }
  public get encryptionBits(): readonly EncryptionBits[] {
    return this.#encryptionBits;
  }

  public getInputHandles(): readonly InputHandle[] {
    if (this.#inputHandles === undefined) {
      this.#inputHandles = solanaInputHandles({
        ciphertextWithZkProof: this.#ciphertextWithZkProof,
        aclContractAddress: this.#aclContractAddress,
        chainId: this.#chainId,
        fheTypeIds: this.#fheTypeIds,
      });
      Object.freeze(this.#inputHandles);
    }
    return this.#inputHandles;
  }
}

/** Validates a {@link SolanaZkProofLike} and builds an immutable {@link SolanaZkProof}. */
export function toSolanaZkProof(proofLike: SolanaZkProofLike, options?: { readonly copy?: boolean }): SolanaZkProof {
  const chainId = asUint64BigInt(proofLike.chainId);
  const aclContractAddress = asBytes32Hex(proofLike.aclContractAddress, {
    subject: 'aclContractAddress',
  });
  const contractAddress = asBytes32Hex(proofLike.contractAddress, {
    subject: 'contractAddress',
  });
  const userAddress = asBytes32Hex(proofLike.userAddress, { subject: 'userAddress' });

  const ciphertextWithZkProof = toBytes(proofLike.ciphertextWithZkProof, {
    subject: 'ciphertextWithZkProof',
    copy: options?.copy !== false,
  });
  if (ciphertextWithZkProof.length === 0) {
    throw new ZkProofError({
      message: 'ciphertextWithZkProof argument should not be empty',
    });
  }

  return new SolanaZkProofImpl(PRIVATE_TOKEN, {
    chainId,
    aclContractAddress,
    contractAddress,
    userAddress,
    ciphertextWithZkProof,
    encryptionBits: proofLike.encryptionBits,
  });
}

/**
 * Derives the input handles a Solana proof's ciphertexts will be bound to. Mirrors
 * `set_ciphertext_metadata` + `finalize_ciphertext` in the coprocessor zkproof-worker
 * for Solana hosts: the handle prehash is
 * `keccak("ZK-w_hdl" || blobHash || index || acl(32) || chainId(32))`, then the
 * trailing metadata (index, chainId, fheType, version) is patched in by `buildHandle`.
 * The only divergence from the EVM derivation is the 32-byte bytes32 ACL identity.
 */
function solanaInputHandles(args: {
  readonly ciphertextWithZkProof: Bytes;
  readonly aclContractAddress: Bytes32Hex;
  readonly chainId: Uint64BigInt;
  readonly fheTypeIds: readonly FheTypeId[];
}): InputHandle[] {
  const encoder = new TextEncoder();
  const blobHash = keccak_256(
    concatBytes(encoder.encode(FHEVM_HANDLE_RAW_CT_HASH_DOMAIN_SEPARATOR), args.ciphertextWithZkProof),
  );

  const handleDomainSep = encoder.encode(FHEVM_HANDLE_HASH_DOMAIN_SEPARATOR);
  const aclBytes32 = hexToBytes32(args.aclContractAddress);
  const chainIdBytes32 = uint256ToBytes32(args.chainId);

  const handles: InputHandle[] = [];
  for (const [index, fheTypeId] of args.fheTypeIds.entries()) {
    const prehash = bytes32ToHex(
      keccak_256(concatBytes(handleDomainSep, blobHash, new Uint8Array([index]), aclBytes32, chainIdBytes32)),
    );
    const hash21 = prehash.slice(0, 2 + 2 * 21);

    handles.push(
      buildHandle({
        hash21,
        chainId: args.chainId,
        fheTypeId,
        index,
      }),
    );
  }

  return handles;
}
