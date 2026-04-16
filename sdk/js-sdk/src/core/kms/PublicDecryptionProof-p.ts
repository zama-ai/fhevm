import type { KmsSignersContext } from '../types/kmsSignersContext.js';
import type { Bytes65Hex, BytesHex } from '../types/primitives.js';
import type { PublicDecryptionProof } from '../types/publicDecryptionProof-p.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { ClearValueType, SolidityPrimitiveTypeName } from '../types/fheType.js';
import type { NonEmptyReadonlyArray } from '../types/utils.js';
import type { ClearValue, Handle } from '../types/encryptedTypes-p.js';
import { concatBytesHex } from '../base/bytes.js';
import { abiEncodeClearValues, createClearValueArray } from '../handle/ClearValue.js';
import { toClearValueType } from '../handle/FheType.js';
import { kmsSignersContextToExtraData } from '../host-contracts/KmsSignersContext-p.js';
import { verifyKmsPublicDecryptEip712 } from './verifyKmsPublicDecryptEip712-p.js';

//////////////////////////////////////////////////////////////////////////////

const PRIVATE_TOKEN = Symbol('PublicDecryptionProof.token');

//////////////////////////////////////////////////////////////////////////////
// PublicDecryptionProof class
//////////////////////////////////////////////////////////////////////////////

/**
 * @internal
 */
export class PublicDecryptionProofImpl implements PublicDecryptionProof {
  // numSigners + KMS signatures + extraData
  readonly #decryptionProof: BytesHex;
  readonly #orderedClearValues: NonEmptyReadonlyArray<ClearValue>;
  readonly #orderedAbiEncodedClearValues: BytesHex;
  // legacy is '0x' (not '0x00')
  readonly #extraData: BytesHex;

  constructor(
    privateToken: symbol,
    parameters: {
      readonly decryptionProof: BytesHex;
      readonly orderedClearValues: readonly ClearValue[];
      readonly orderedAbiEncodedClearValues: BytesHex;
      readonly extraData: BytesHex;
    },
  ) {
    if (privateToken !== PRIVATE_TOKEN) {
      throw new Error('Unauthorized');
    }
    if (parameters.orderedClearValues.length === 0) {
      throw new Error('PublicDecryptionProof requires at least one clear value');
    }
    this.#decryptionProof = parameters.decryptionProof;
    this.#orderedClearValues = Object.freeze([...parameters.orderedClearValues]) as NonEmptyReadonlyArray<ClearValue>;
    this.#extraData = parameters.extraData;
    this.#orderedAbiEncodedClearValues = parameters.orderedAbiEncodedClearValues;
  }

  //////////////////////////////////////////////////////////////////////////////
  // Getters
  //////////////////////////////////////////////////////////////////////////////

  public get decryptionProof(): BytesHex {
    return this.#decryptionProof;
  }

  public get orderedClearValues(): NonEmptyReadonlyArray<ClearValue> {
    return this.#orderedClearValues;
  }

  public get orderedAbiEncodedClearValues(): BytesHex {
    return this.#orderedAbiEncodedClearValues;
  }

  public get extraData(): BytesHex {
    return this.#extraData;
  }
}

Object.freeze(PublicDecryptionProofImpl);
Object.freeze(PublicDecryptionProofImpl.prototype);

//////////////////////////////////////////////////////////////////////////////
// Factory
//////////////////////////////////////////////////////////////////////////////

type Context = {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
};

type Parameters = {
  readonly originToken: symbol;
  readonly orderedHandles: readonly Handle[];
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly kmsPublicDecryptEIP712Signatures: readonly Bytes65Hex[];
  readonly kmsSignersContext: KmsSignersContext;
};

/**
 * Builds a {@link PublicDecryptionProof} from KMS signatures.
 *
 * @internal
 */
export async function createPublicDecryptionProof(
  context: Context,
  parameters: Parameters,
): Promise<PublicDecryptionProof> {
  // Always verify KMS signatures
  await verifyKmsPublicDecryptEip712(context, parameters);

  const {
    orderedHandles: orderedEncryptedValues,
    orderedAbiEncodedClearValues,
    kmsPublicDecryptEIP712Signatures,
    kmsSignersContext,
    originToken,
  } = parameters;

  //////////////////////////////////////////////////////////////////////////////
  // Compute extraData using KmsSignersContext
  //////////////////////////////////////////////////////////////////////////////

  const extraData = kmsSignersContextToExtraData(kmsSignersContext);

  ////////////////////////////////////////////////////////////////////////////
  //
  // Warning!!!! Do not use '0x00' here!! Only '0x' is permitted!
  //
  ////////////////////////////////////////////////////////////////////////////

  const signedExtraData = extraData === ('0x00' as BytesHex) ? ('0x' as BytesHex) : extraData;

  //////////////////////////////////////////////////////////////////////////////
  // Compute the proof as numSigners + KMS signatures + extraData
  //////////////////////////////////////////////////////////////////////////////

  const packedNumSigners: BytesHex = context.runtime.ethereum.encodePacked({
    types: ['uint8'],
    values: [kmsPublicDecryptEIP712Signatures.length],
  });

  const packedSignatures = context.runtime.ethereum.encodePacked({
    types: Array(kmsPublicDecryptEIP712Signatures.length).fill('bytes') as string[],
    values: kmsPublicDecryptEIP712Signatures,
  });

  const decryptionProof: BytesHex = concatBytesHex([packedNumSigners, packedSignatures, signedExtraData]);

  //////////////////////////////////////////////////////////////////////////////
  // Deserialize ordered decrypted result
  //////////////////////////////////////////////////////////////////////////////

  const orderedAbiTypes: SolidityPrimitiveTypeName[] = orderedEncryptedValues.map((h) => h.solidityPrimitiveTypeName);

  const decoded = context.runtime.ethereum.decode({
    types: orderedAbiTypes,
    encodedData: orderedAbiEncodedClearValues,
  });

  if (decoded.length !== orderedEncryptedValues.length) {
    throw new Error('Invalid decrypted result.');
  }

  const orderedValues: ClearValueType[] = orderedEncryptedValues.map((h, index) =>
    toClearValueType(h.fheType, decoded[index]),
  );

  const orderedDecryptedFhevmHandles = createClearValueArray({
    orderedHandles: orderedEncryptedValues,
    orderedValues,
    originToken,
  });

  const orderedAbiEncodedDecryptedFhevmHandles = abiEncodeClearValues(context, {
    orderedClearValues: orderedDecryptedFhevmHandles,
  });

  return new PublicDecryptionProofImpl(PRIVATE_TOKEN, {
    decryptionProof: decryptionProof,
    orderedClearValues: orderedDecryptedFhevmHandles,
    orderedAbiEncodedClearValues: orderedAbiEncodedDecryptedFhevmHandles.abiEncodedClearValues,
    extraData: signedExtraData,
  });
}
