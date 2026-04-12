import type { Bytes20, Bytes32, UintNumber } from '../types/primitives.js';
import type { ZkProof } from '../types/zkProof.js';
import { assert } from '../base/errors/InternalError.js';
import { isUint64, uint256ToBytes32 } from '../base/uint.js';
import { isAddress } from '../base/address.js';
import { hexToBytes20 } from '../base/bytes.js';
import { ZkProofError } from '../errors/ZkProofError.js';
import { TypedValueArrayBuilder } from '../base/typedValue.js';
import type {
  TypedValue,
  Uint32ValueLike,
  Uint64ValueLike,
  Uint128ValueLike,
  Uint256ValueLike,
  Uint8ValueLike,
  Uint16ValueLike,
  BoolValueLike,
  AddressValueLike,
} from '../types/primitives.js';
import { toZkProof } from './ZkProof-p.js';
import {
  encryptionBitsFromFheType,
  fheTypeNameFromTypeName,
} from '../handle/FheType.js';
import type { EncryptionBits, FheType } from '../types/fheType.js';
import type { Fhevm } from '../types/coreFhevmClient.js';
import type { ZkProofBuilder } from '../types/zkProofBuilder.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import { fetchFheEncryptionKeyWasm } from '../key/fetchFheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('ZkProofBuilder.token');

////////////////////////////////////////////////////////////////////////////////

export const TFHE_CRS_BITS_CAPACITY = 2048 as UintNumber;
export const TFHE_ZKPROOF_CIPHERTEXT_CAPACITY = 256 as UintNumber;

////////////////////////////////////////////////////////////////////////////////
// ZkProofBuilder
////////////////////////////////////////////////////////////////////////////////

class ZkProofBuilderImpl implements ZkProofBuilder {
  #totalBits: number = 0;
  readonly #bits: EncryptionBits[] = [];
  readonly #bitsCapacity: UintNumber;
  readonly #ciphertextCapacity: UintNumber;
  readonly #builder = new TypedValueArrayBuilder();

  constructor(
    privateToken: symbol,
    parameters: {
      readonly ciphertextCapacity: UintNumber;
      readonly bitsCapacity: UintNumber;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }

    this.#bitsCapacity = parameters.bitsCapacity;
    this.#ciphertextCapacity = parameters.ciphertextCapacity;
  }

  //////////////////////////////////////////////////////////////////////////////
  // Public API
  //////////////////////////////////////////////////////////////////////////////

  public get count(): number {
    return this.#bits.length;
  }

  public get totalBits(): number {
    return this.#totalBits;
  }

  public getBits(): EncryptionBits[] {
    return [...this.#bits];
  }

  public addTypedValue(typedValue: TypedValue): this {
    this.#builder.addTypedValue(typedValue);
    this.#addType(fheTypeNameFromTypeName(typedValue.type));
    return this;
  }

  public addBool(value: boolean | number | bigint | BoolValueLike): this {
    this.#builder.addBool(value);
    this.#addType(fheTypeNameFromTypeName('bool'));
    return this;
  }

  public addUint8(value: number | bigint | Uint8ValueLike): this {
    this.#builder.addUint8(value);
    this.#addType(fheTypeNameFromTypeName('uint8'));
    return this;
  }

  public addUint16(value: number | bigint | Uint16ValueLike): this {
    this.#builder.addUint16(value);
    this.#addType(fheTypeNameFromTypeName('uint16'));
    return this;
  }

  public addUint32(value: number | bigint | Uint32ValueLike): this {
    this.#builder.addUint32(value);
    this.#addType(fheTypeNameFromTypeName('uint32'));
    return this;
  }

  public addUint64(value: number | bigint | Uint64ValueLike): this {
    this.#builder.addUint64(value);
    this.#addType(fheTypeNameFromTypeName('uint64'));
    return this;
  }

  public addUint128(value: number | bigint | Uint128ValueLike): this {
    this.#builder.addUint128(value);
    this.#addType(fheTypeNameFromTypeName('uint128'));
    return this;
  }

  public addUint256(value: number | bigint | Uint256ValueLike): this {
    this.#builder.addUint256(value);
    this.#addType(fheTypeNameFromTypeName('uint256'));
    return this;
  }

  public addAddress(value: string | AddressValueLike): this {
    this.#builder.addAddress(value);
    this.#addType(fheTypeNameFromTypeName('address'));
    return this;
  }

  public async build(
    fhevm: Fhevm<FhevmChain, WithEncrypt>,
    {
      contractAddress,
      userAddress,
    }: {
      readonly contractAddress: string;
      readonly userAddress: string;
    },
  ): Promise<ZkProof> {
    // Fetch the FheEncryptionKey (in wasm format) from the global cache.
    const fheEncryptionKeyWasm = await fetchFheEncryptionKeyWasm(fhevm);

    if (this.#totalBits === 0) {
      throw new ZkProofError({
        message: `Encrypted input must contain at least one value`,
      });
    }

    // should be guaranteed at this point
    assert(this.#totalBits <= this.#bitsCapacity);

    if (!isAddress(contractAddress)) {
      throw new ZkProofError({
        message: `Invalid contract address: ${contractAddress}`,
      });
    }
    if (!isAddress(userAddress)) {
      throw new ZkProofError({
        message: `Invalid user address: ${userAddress}`,
      });
    }

    const aclContractAddress = fhevm.chain.fhevm.contracts.acl.address;
    const chainId = fhevm.chain.id;

    if (!isAddress(aclContractAddress)) {
      throw new ZkProofError({
        message: `Invalid ACL address: ${aclContractAddress}`,
      });
    }
    if (!isUint64(chainId)) {
      throw new ZkProofError({
        message: `Invalid chain ID uint64: ${chainId}`,
      });
    }

    // Note about hexToBytes(<address>)
    // ================================
    // All addresses are 42 characters long strings.
    // hexToBytes(<42-characters hex string>) always returns a 20-byte long Uint8Array

    // Bytes20
    const contractAddressBytes20: Bytes20 = hexToBytes20(contractAddress);

    // Bytes20
    const userAddressBytes20: Bytes20 = hexToBytes20(userAddress);

    // Bytes20
    const aclContractAddressBytes20: Bytes20 = hexToBytes20(aclContractAddress);

    // Bytes32
    const chainIdBytes32: Bytes32 = uint256ToBytes32(chainId);

    const metaDataLength = 3 * 20 + 32;
    const metaData = new Uint8Array(metaDataLength);

    metaData.set(contractAddressBytes20, 0);
    metaData.set(userAddressBytes20, 20);
    metaData.set(aclContractAddressBytes20, 40);
    metaData.set(chainIdBytes32, 60);

    assert(metaData.length - chainIdBytes32.length === 60);

    const ciphertextWithZKProofBytes: Uint8Array =
      await fhevm.runtime.encrypt.buildWithProofPacked({
        typedValues: [...this.#builder.build()],
        fheEncryptionKey: fheEncryptionKeyWasm,
        metaData,
      });

    return toZkProof(
      {
        chainId: BigInt(chainId),
        aclContractAddress,
        contractAddress,
        userAddress,
        ciphertextWithZkProof: ciphertextWithZKProofBytes,
        encryptionBits: this.#bits,
      },
      { copy: false }, // Take ownership
    );
  }

  //////////////////////////////////////////////////////////////////////////////
  // Private helpers
  //////////////////////////////////////////////////////////////////////////////

  #checkLimit(encryptionBits: EncryptionBits): void {
    if (this.#totalBits + encryptionBits > this.#bitsCapacity) {
      throw new ZkProofError({
        message: `Packing more than ${this.#bitsCapacity.toString()} bits in a single input ciphertext is unsupported`,
      });
    }
    if (this.#bits.length >= this.#ciphertextCapacity) {
      throw new ZkProofError({
        message: `Packing more than ${this.#ciphertextCapacity.toString()} variables in a single input ciphertext is unsupported`,
      });
    }
  }

  #addType(fheTypeName: FheType): void {
    // encryptionBits is guaranteed to be >= 2
    const encryptionBits = encryptionBitsFromFheType(fheTypeName);
    this.#checkLimit(encryptionBits);
    this.#totalBits += encryptionBits;
    this.#bits.push(encryptionBits);
  }
}

//////////////////////////////////////////////////////////////////////////////

export function createZkProofBuilder(): ZkProofBuilder {
  return new ZkProofBuilderImpl(PRIVATE_TOKEN, {
    ciphertextCapacity: TFHE_ZKPROOF_CIPHERTEXT_CAPACITY,
    bitsCapacity: TFHE_CRS_BITS_CAPACITY,
  });
}
