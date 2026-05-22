import type {
  ChecksummedAddress,
  Uint64BigInt,
  Bytes,
  Bytes32Hex,
  Bytes32,
  Uint64,
  Bytes21Hex,
  BytesHex,
} from '../types/primitives.js';
import type { ZkProofLike, ZkProof } from '../types/zkProof-p.js';
import type { ErrorMetadataParams } from '../base/errors/ErrorBase.js';
import type { EncryptionBits, FheTypeId } from '../types/fheType.js';
import type { ParseTFHEProvenCompactCiphertextListModuleFunction } from '../modules/encrypt/types.js';
import type { InputHandle } from '../types/encryptedTypes-p.js';
import {
  addressToChecksummedAddress,
  assertIsAddress,
  assertIsChecksummedAddress,
  checksummedAddressToBytes20,
} from '../base/address.js';
import { bytes32ToHex, bytesToHexLarge, concatBytes, hexToBytes32, toBytes } from '../base/bytes.js';
import { assertIsUint64, assertIsUint8, asUint64BigInt, uint64ToBytes32 } from '../base/uint.js';
import { ZkProofError } from '../errors/ZkProofError.js';
import { asEncryptionBits, assertIsEncryptionBitsArray, fheTypeIdFromEncryptionBits } from '../handle/FheType.js';
import { buildHandle } from '../handle/FhevmHandle.js';
import { keccak_256 } from '@noble/hashes/sha3.js';
import { InvalidTypeError } from '../base/errors/InvalidTypeError.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('ZkProof.token');
const GET_UNSAFE_RAW_BYTES_FUNC = Symbol('ZkProof.getUnsafeRawBytes');

////////////////////////////////////////////////////////////////////////////////
// ZkProof
////////////////////////////////////////////////////////////////////////////////

export const FHEVM_HANDLE_RAW_CT_HASH_DOMAIN_SEPARATOR = 'ZK-w_rct';
export const FHEVM_HANDLE_HASH_DOMAIN_SEPARATOR = 'ZK-w_hdl';

class ZkProofImpl implements ZkProof {
  readonly #chainId: Uint64BigInt;
  readonly #aclContractAddress: ChecksummedAddress;
  readonly #contractAddress: ChecksummedAddress;
  readonly #userAddress: ChecksummedAddress;
  readonly #ciphertextWithZkProof: Bytes; // Never empty
  readonly #encryptionBits: readonly EncryptionBits[]; // Can be empty
  readonly #fheTypeIds: readonly FheTypeId[]; // Can be empty
  #inputHandles: InputHandle[] | undefined;

  // In theory, 'extraData' is not part of the ZkProof
  readonly #extraData: BytesHex;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly chainId: Uint64BigInt;
      readonly aclContractAddress: ChecksummedAddress;
      readonly contractAddress: ChecksummedAddress;
      readonly userAddress: ChecksummedAddress;
      readonly ciphertextWithZkProof: Bytes;
      readonly encryptionBits: readonly EncryptionBits[];
      readonly extraData: BytesHex;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#chainId = parameters.chainId;
    this.#aclContractAddress = parameters.aclContractAddress;
    this.#contractAddress = parameters.contractAddress;
    this.#userAddress = parameters.userAddress;
    this.#ciphertextWithZkProof = parameters.ciphertextWithZkProof;
    this.#encryptionBits = Object.freeze([...parameters.encryptionBits]);
    this.#fheTypeIds = Object.freeze(this.#encryptionBits.map((w) => fheTypeIdFromEncryptionBits(asEncryptionBits(w))));
    this.#extraData = parameters.extraData;
    Object.freeze(this);
  }

  //////////////////////////////////////////////////////////////////////////////
  // Instance Getters
  //////////////////////////////////////////////////////////////////////////////

  /** The chain ID where this proof is valid. */
  public get chainId(): Uint64BigInt {
    return this.#chainId;
  }

  /** The ACL contract address associated with this proof. */
  public get aclContractAddress(): ChecksummedAddress {
    return this.#aclContractAddress;
  }

  /** The target contract address associated with this proof. */
  public get contractAddress(): ChecksummedAddress {
    return this.#contractAddress;
  }

  /** The user address associated with this proof. */
  public get userAddress(): ChecksummedAddress {
    return this.#userAddress;
  }

  /** The ciphertext with Zk proof (guaranteed non-empty). Returns a copy. */
  public get ciphertextWithZkProof(): Bytes {
    return new Uint8Array(this.#ciphertextWithZkProof);
  }

  /** The encryption bit sizes for each encrypted value in the proof. */
  public get encryptionBits(): readonly EncryptionBits[] {
    return this.#encryptionBits;
  }

  /** The FHE type IDs corresponding to each encrypted value. */
  public get fheTypeIds(): readonly FheTypeId[] {
    return this.#fheTypeIds;
  }

  /**
   * Returns the raw internal bytes without copying.
   * WARNING: Do not mutate the returned array - this would violate immutability.
   * Use `ciphertextWithZkProof` getter if you need a safe copy.
   */
  public [GET_UNSAFE_RAW_BYTES_FUNC](token: symbol): Bytes {
    if (token !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    return this.#ciphertextWithZkProof;
  }

  /**
   * Returns a safe string representation for debugging.
   * Does not expose ciphertext content - only metadata.
   */
  public toString(): string {
    return `ZkProof(chainId=${String(this.#chainId)}, contract=${this.#contractAddress}, user=${this.#userAddress}, types=${this.#fheTypeIds.length}, bytes=${String(this.#ciphertextWithZkProof.length)})`;
  }

  public getInputHandles(): readonly InputHandle[] {
    if (this.#inputHandles === undefined) {
      this.#inputHandles = _zkProofToInputHandles({
        ciphertextWithZkProof: this.#ciphertextWithZkProof,
        aclContractAddress: this.#aclContractAddress,
        fheTypeIds: this.#fheTypeIds,
        chainId: this.#chainId,
      });
      Object.freeze(this.#inputHandles);
    }
    return this.#inputHandles;
  }

  public getExtraData(): BytesHex {
    return this.#extraData;
  }

  //////////////////////////////////////////////////////////////////////////////
  // JSON
  //////////////////////////////////////////////////////////////////////////////

  /**
   * Serializes the ZkProof to a JSON-compatible object.
   * Ciphertext is hex-encoded, chainId is converted to number if safe.
   * @returns A plain object suitable for JSON.stringify.
   */
  public toJSON(): Omit<ZkProofLike, 'encryptionBits'> & {
    fheTypeIds: readonly FheTypeId[];
    encryptionBits: readonly EncryptionBits[];
  } {
    return {
      chainId: this.#chainId <= Number.MAX_SAFE_INTEGER ? Number(this.#chainId) : this.#chainId,
      aclContractAddress: this.#aclContractAddress,
      contractAddress: this.#contractAddress,
      userAddress: this.#userAddress,
      ciphertextWithZkProof: bytesToHexLarge(this.#ciphertextWithZkProof),
      encryptionBits: this.#encryptionBits,
      fheTypeIds: this.#fheTypeIds,
    };
  }
}

////////////////////////////////////////////////////////////////////////////////
// Freeze
////////////////////////////////////////////////////////////////////////////////

Object.freeze(ZkProofImpl);
Object.freeze(ZkProofImpl.prototype);

////////////////////////////////////////////////////////////////////////////////
// Public API: createZkProof
////////////////////////////////////////////////////////////////////////////////

export function isZkProof(value: unknown): value is ZkProof {
  return value instanceof ZkProofImpl;
}

export function assertIsZkProof(
  value: unknown,
  options: { readonly subject?: string; readonly metaMessages?: string[] | undefined } & ErrorMetadataParams,
): asserts value is ZkProof {
  if (!isZkProof(value)) {
    throw new InvalidTypeError(
      {
        subject: options.subject,
        type: typeof value,
        expectedType: 'ZkProof',
        metaMessages: options.metaMessages,
      },
      options,
    );
  }
}

/**
 * @internal
 * Creates a ZkProof from loose input types.
 * Validates and normalizes all fields.
 *
 * If `ciphertextWithZkProof` is a hex string, it will be converted to a new Uint8Array.
 * If it is already a Uint8Array:
 * - By default, a defensive copy is made, allowing the caller to retain the original.
 * - With `noCopy: true`, the instance takes ownership — callers must not mutate it afterward.
 * @param zkProofLike - The loose input to validate and normalize (see {@link ZkProofLike}).
 * @param options - Optional settings. Set `options.copy` to `false` to skip copying the
 *   `ciphertextWithZkProof` Uint8Array (takes ownership). Defaults to `true` (copy by default).
 * @throws A {@link ZkProofError} if ciphertextWithZkProof is invalid or empty.
 * @throws A {@link InvalidTypeError} if any field fails validation.
 */
export async function toZkProof(
  zkProofLike: ZkProofLike,
  extraData: BytesHex,
  options?: {
    readonly zkProofParser?: ParseTFHEProvenCompactCiphertextListModuleFunction;
    readonly copy?: boolean;
  },
): Promise<ZkProof> {
  if (zkProofLike instanceof ZkProofImpl) {
    return zkProofLike;
  }

  // Validate arguments
  assertIsUint64(zkProofLike.chainId, {});
  const chainId = BigInt(zkProofLike.chainId) as Uint64BigInt;

  assertIsAddress(zkProofLike.aclContractAddress, {});
  assertIsAddress(zkProofLike.contractAddress, {});
  assertIsAddress(zkProofLike.userAddress, {});

  const aclContractAddress: ChecksummedAddress = addressToChecksummedAddress(zkProofLike.aclContractAddress);
  const contractAddress: ChecksummedAddress = addressToChecksummedAddress(zkProofLike.contractAddress);
  const userAddress: ChecksummedAddress = addressToChecksummedAddress(zkProofLike.userAddress);

  // Validate and normalize ciphertextWithZkProof
  const ciphertextWithZkProof = toBytes(zkProofLike.ciphertextWithZkProof, {
    subject: 'ciphertextWithZkProof',
    copy: options?.copy !== false,
  });

  if (ciphertextWithZkProof.length === 0) {
    throw new ZkProofError({
      message: 'ciphertextWithZkProof argument should not be empty',
    });
  }

  // Validation of packed variable count and total bits is handled by
  // parseTFHEProvenCompactCiphertextList, which deserializes and validates
  // the ciphertext structure via the TFHE WASM module.
  const encryptionBits = await _getOrParseEncryptionBits(
    zkProofLike.encryptionBits,
    ciphertextWithZkProof,
    options?.zkProofParser,
  );

  return new ZkProofImpl(PRIVATE_TOKEN, {
    chainId,
    aclContractAddress,
    contractAddress,
    userAddress,
    ciphertextWithZkProof,
    encryptionBits,
    extraData,
  });
}

////////////////////////////////////////////////////////////////////////////////
// Public API: zkProofToFhevmHandles
////////////////////////////////////////////////////////////////////////////////

export async function zkProofToExternalEncryptedValues(
  zkProofLike: ZkProofLike,
  options?: {
    readonly version?: number;
    readonly zkProofParser?: ParseTFHEProvenCompactCiphertextListModuleFunction;
  },
): Promise<readonly InputHandle[]> {
  if (zkProofLike instanceof ZkProofImpl) {
    return zkProofLike.getInputHandles();
  }

  assertIsChecksummedAddress(zkProofLike.aclContractAddress, {
    subject: 'aclContractAddress',
  });

  const encryptionBits = await _getOrParseEncryptionBits(
    zkProofLike.encryptionBits,
    zkProofLike.ciphertextWithZkProof,
    options?.zkProofParser,
  );

  const ciphertextWithZkProof = toBytes(zkProofLike.ciphertextWithZkProof, {
    subject: 'ciphertextWithZkProof',
  });

  const fheTypeIds = encryptionBits.map((w) => fheTypeIdFromEncryptionBits(w));

  assertIsUint8(fheTypeIds.length, {});

  return _zkProofToInputHandles({
    ciphertextWithZkProof: ciphertextWithZkProof,
    aclContractAddress: zkProofLike.aclContractAddress,
    fheTypeIds,
    chainId: asUint64BigInt(zkProofLike.chainId, { subject: 'chainId' }),
  });
}

////////////////////////////////////////////////////////////////////////////////
// Private Helpers
////////////////////////////////////////////////////////////////////////////////

/**
 * Asserts that two encryption bits arrays are equal (same length and values).
 * @param actual - The actual encryption bits array.
 * @param expected - The expected encryption bits array.
 * @throws ZkProofError if there's a count or type mismatch.
 */
function _assertEncryptionBitsMatch(actual: readonly EncryptionBits[], expected: readonly EncryptionBits[]): void {
  if (actual.length !== expected.length) {
    throw new ZkProofError({
      message: `Encryption count mismatch, expected ${expected.length}, got ${actual.length}.`,
    });
  }

  for (let i = 0; i < actual.length; ++i) {
    if (actual[i] !== expected[i]) {
      throw new ZkProofError({
        message: `Encryption type mismatch at index ${i}.`,
      });
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

function _zkProofToInputHandles(
  args: {
    readonly ciphertextWithZkProof: Bytes;
    readonly aclContractAddress: ChecksummedAddress;
    readonly fheTypeIds: readonly FheTypeId[];
    readonly chainId: Uint64BigInt;
  },
  options?: {
    readonly version?: number;
  },
): InputHandle[] {
  const encoder = new TextEncoder();
  const domainSepBytes = encoder.encode(FHEVM_HANDLE_RAW_CT_HASH_DOMAIN_SEPARATOR);

  const blobHashBytes32Hex: Bytes32Hex = bytes32ToHex(
    keccak_256(concatBytes(domainSepBytes, args.ciphertextWithZkProof)),
  );

  const inputHandles: InputHandle[] = [];
  for (const [i, fheTypeId] of args.fheTypeIds.entries()) {
    const hash21 = _computeInputHash21(hexToBytes32(blobHashBytes32Hex), args.aclContractAddress, args.chainId, i);

    inputHandles.push(
      buildHandle({
        hash21,
        chainId: args.chainId,
        fheTypeId,
        ...(options?.version !== undefined ? { version: options.version } : {}),
        index: i,
      }),
    );
  }

  return inputHandles; // not readonly
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Returns the encryption bits from a ZkProofLike.
 * If `encryptionBits` is provided, validates and returns it.
 * Otherwise, parses the ciphertext to extract the encryption bits.
 */
async function _getOrParseEncryptionBits(
  encryptionBits: readonly number[] | undefined,
  ciphertextWithZkProof: Uint8Array | string,
  zkProofParser?: ParseTFHEProvenCompactCiphertextListModuleFunction,
): Promise<readonly EncryptionBits[]> {
  // Case 1: encryptionBits provided — validate, and verify against parsed if possible
  if (encryptionBits != null) {
    assertIsEncryptionBitsArray(encryptionBits, {
      subject: 'encryptionBits',
    });

    if (zkProofParser != null) {
      const parsed = await zkProofParser.parseTFHEProvenCompactCiphertextList({
        ciphertextWithZkProof: ciphertextWithZkProof,
      });
      _assertEncryptionBitsMatch(parsed.encryptionBits, encryptionBits);
    }

    return encryptionBits;
  }

  // Case 2: encryptionBits not provided — extract if parse function available
  if (zkProofParser != null) {
    const parsed = await zkProofParser.parseTFHEProvenCompactCiphertextList({
      ciphertextWithZkProof: ciphertextWithZkProof,
    });
    return parsed.encryptionBits;
  }

  // Case 3: no encryptionBits and no way to extract them
  throw new ZkProofError({ message: 'Missing encryption bits' });
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Computes the 21-byte handle hash for an encrypted input.
 *
 * handle_hash = "ZK-w_hdl" (8 bytes) + blobHash (32 bytes) + index (1 byte) + aclAddress (20 bytes) + chainId (32 bytes)
 *
 * Reference implementation (Rust):
 * ```rust
 * const HANDLE_HASH_DOMAIN_SEPARATOR: [u8; 8] = *b"ZK-w_hdl";
 *
 * let mut handle_hash = Keccak256::new();
 * handle_hash.update(HANDLE_HASH_DOMAIN_SEPARATOR);
 * handle_hash.update(blob_hash);
 * handle_hash.update([ct_idx as u8]);
 * handle_hash.update(
 *     Address::from_str(&aux_data.acl_contract_address)
 *         .expect("valid acl_contract_address")
 *         .into_array(),
 * );
 * handle_hash.update(chain_id_bytes);
 * let mut handle = handle_hash.finalize().to_vec();
 * assert_eq!(handle.len(), 32);
 * ```
 *
 * @see https://github.com/zama-ai/fhevm/blob/322748569f0b5eb100b8cc1a58f691cfe88d17c4/coprocessor/fhevm-engine/zkproof-worker/src/verifier.rs#L538
 * @internal
 */
function _computeInputHash21(
  blobHashBytes32: Bytes32,
  aclAddress: ChecksummedAddress,
  chainId: Uint64,
  index: number,
): Bytes21Hex {
  const encryptionIndexByte1 = new Uint8Array([index]);
  const aclContractAddressBytes20 = checksummedAddressToBytes20(aclAddress);
  const chainIdBytes32 = uint64ToBytes32(chainId);

  const encoder = new TextEncoder();
  const domainSepBytes = encoder.encode(FHEVM_HANDLE_HASH_DOMAIN_SEPARATOR);

  const hashBytes32Hex = bytes32ToHex(
    keccak_256(
      concatBytes(domainSepBytes, blobHashBytes32, encryptionIndexByte1, aclContractAddressBytes20, chainIdBytes32),
    ),
  );

  // Truncate to 21 bytes (0x + 42 hex chars)
  return hashBytes32Hex.slice(0, 2 + 2 * 21) as Bytes21Hex;
}

/**
 * @internal
 */
export function zkProofGetUnsafeRawBytes(zkProof: ZkProof): Bytes {
  if (!(zkProof instanceof ZkProofImpl)) {
    throw new Error('Unauthorized');
  }
  return zkProof[GET_UNSAFE_RAW_BYTES_FUNC](PRIVATE_TOKEN);
}
