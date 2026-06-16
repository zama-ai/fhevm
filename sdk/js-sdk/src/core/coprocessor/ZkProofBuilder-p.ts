import type { BytesHex, ChecksummedAddress, UintNumber } from '../types/primitives.js';
import type { ZkProof } from '../types/zkProof-p.js';
import type { EncryptionBits, FheType } from '../types/fheType.js';
import type { ZkProofBuilder } from '../types/zkProofBuilder.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
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
import type { SolanaZkProof } from './SolanaZkProof-p.js';
import { assert } from '../base/errors/InternalError.js';
import { isUint64 } from '../base/uint.js';
import { asBytesHex } from '../base/bytes.js';
import { ZkProofError } from '../errors/ZkProofError.js';
import { buildInputProofMetaData, isSolanaHostChainId } from './buildInputProofMetaData-p.js';
import { toSolanaZkProof } from './SolanaZkProof-p.js';
import { createTypedValue, TypedValueArrayBuilder } from '../base/typedValue.js';
import { toZkProof } from './ZkProof-p.js';
import { encryptionBitsFromFheType, fheTypeNameFromTypeName } from '../handle/FheType.js';
import { fetchFheEncryptionKeyWasm } from '../key/fetchFheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: WithEncrypt;
};

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
    context: Context,
    {
      contractAddress,
      userAddress,
      extraData,
    }: {
      readonly contractAddress: string;
      readonly userAddress: string;
      readonly extraData: string;
    },
  ): Promise<ZkProof> {
    const {
      chainId,
      aclContractAddress,
      ciphertextWithZkProof,
      extraData: finalExtraData,
    } = await this.#encodeAndProve(context, contractAddress, userAddress, asBytesHex(extraData));

    if (isSolanaHostChainId(chainId)) {
      throw new ZkProofError({
        message: 'Use buildSolana() for Solana host chains',
      });
    }

    return toZkProof(
      {
        chainId: BigInt(chainId),
        aclContractAddress,
        contractAddress,
        userAddress,
        ciphertextWithZkProof,
        encryptionBits: this.#bits,
      },
      finalExtraData,
      { copy: false }, // Take ownership
    );
  }

  /**
   * Solana counterpart of {@link build}: produces a {@link SolanaZkProof} bound to
   * RFC-021 bytes32 host identities and the 128-byte aux layout. The proof-generation
   * core is shared with {@link build}; only the aux layout (selected by host chain
   * type inside `buildInputProofMetaData`) and the returned proof type differ.
   */
  public async buildSolana(
    context: Context,
    {
      contractAddress,
      userAddress,
    }: {
      readonly contractAddress: string;
      readonly userAddress: string;
    },
  ): Promise<SolanaZkProof> {
    // Solana input-proof extraData is fixed (`0x00`): the host binding lives in the 128-byte aux
    // (`buildInputProofMetaData`), and SolanaZkProof carries no extraData field.
    const { chainId, aclContractAddress, ciphertextWithZkProof } = await this.#encodeAndProve(
      context,
      contractAddress,
      userAddress,
      asBytesHex('0x00'),
    );

    if (!isSolanaHostChainId(chainId)) {
      throw new ZkProofError({
        message: 'buildSolana() requires a Solana host chain (RFC-021 chain-type bit)',
      });
    }

    return toSolanaZkProof(
      {
        chainId: BigInt(chainId),
        aclContractAddress,
        contractAddress,
        userAddress,
        ciphertextWithZkProof,
        encryptionBits: this.#bits,
      },
      { copy: false }, // Take ownership
    );
  }

  //////////////////////////////////////////////////////////////////////////////
  // Private helpers
  //////////////////////////////////////////////////////////////////////////////

  /**
   * Shared proof-generation core for {@link build} and {@link buildSolana}: validates
   * inputs, assembles the host-appropriate input-proof aux, and produces the packed
   * proven ciphertext. The aux layout is chosen by host chain type inside
   * `buildInputProofMetaData`; the caller wraps the result in the matching proof type.
   */
  async #encodeAndProve(
    context: Context,
    contractAddress: string,
    userAddress: string,
    extraData: BytesHex,
  ): Promise<{
    readonly chainId: bigint | number;
    readonly aclContractAddress: string;
    readonly ciphertextWithZkProof: Uint8Array;
    readonly extraData: BytesHex;
  }> {
    // Fetch the FheEncryptionKey (in wasm format) from the global cache.
    const fheEncryptionKeyWasm = await fetchFheEncryptionKeyWasm(context);

    if (this.#totalBits === 0) {
      throw new ZkProofError({
        message: `Encrypted input must contain at least one value`,
      });
    }

    // should be guaranteed at this point
    assert(this.#totalBits <= this.#bitsCapacity);

    const aclContractAddress = context.chain.fhevm.contracts.acl.address;
    const chainId = context.chain.id;

    if (!isUint64(chainId)) {
      throw new ZkProofError({
        message: `Invalid chain ID uint64: ${chainId}`,
      });
    }

    // Prover side of the input-proof auxiliary data. It MUST agree with the
    // coprocessor zkproof-worker verifier byte-for-byte: EVM hosts use the
    // 92-byte 20-byte-address layout, Solana hosts (RFC-021) the 128-byte bytes32
    // layout. Per-host identity validation happens inside the helper.
    const metaData = buildInputProofMetaData({
      chainId,
      contractAddress,
      userAddress,
      aclContractAddress,
    });

    const { ciphertextWithZKProofBytes, extraData: finalExtraData } =
      await context.runtime.encrypt.buildWithProofPacked({
        typedValues: [...this.#builder.build()],
        fheEncryptionKey: fheEncryptionKeyWasm,
        metaData,
        extraData,
      });

    return {
      chainId,
      aclContractAddress,
      ciphertextWithZkProof: ciphertextWithZKProofBytes,
      extraData: finalExtraData,
    };
  }

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

//////////////////////////////////////////////////////////////////////////////

export async function createZkProof(
  context: Context,
  parameters: {
    readonly values: readonly TypedValue[];
    readonly contractAddress: ChecksummedAddress;
    readonly userAddress: ChecksummedAddress;
    readonly extraData: BytesHex;
  },
): Promise<ZkProof> {
  const { contractAddress, userAddress, values, extraData } = parameters;

  const builder = new ZkProofBuilderImpl(PRIVATE_TOKEN, {
    ciphertextCapacity: TFHE_ZKPROOF_CIPHERTEXT_CAPACITY,
    bitsCapacity: TFHE_CRS_BITS_CAPACITY,
  });

  for (const value of values) {
    builder.addTypedValue(createTypedValue(value));
  }

  const zkProof = await builder.build(context, {
    contractAddress,
    userAddress,
    extraData,
  });

  return zkProof;
}
