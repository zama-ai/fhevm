/* eslint-disable @typescript-eslint/require-await */
import type { TfheVersion } from '../../../wasm/tfhe/TfheApi.js';
import { concatBytes, hexToBytes32, unsafeBytesEquals } from '../../base/bytes.js';
import { remove0x } from '../../base/string.js';
import { typedValueToBytes32Hex } from '../../base/typedValue.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type {
  FheEncryptionCrs,
  FheEncryptionCrsBrand,
  FheEncryptionPublicKey,
  FheEncryptionPublicKeyBrand,
} from '../../types/fheEncryptionKey.js';
import type { BytesHex, UintNumber } from '../../types/primitives.js';
import { CANONICAL_WASM_VERSIONS } from '../../runtime/WasmVersions-p.js';
import type {
  BuildWithProofPackedParameters,
  BuildWithProofPackedReturnType,
  ParseTFHEProvenCompactCiphertextListParameters,
  ParseTFHEProvenCompactCiphertextListReturnType,
  EncryptModuleFactory,
  DeserializeFheEncryptionPublicKeyParameters as DeserializeFheEncryptionPublicKeyParameters,
  DeserializeFheEncryptionPublicKeyReturnType as DeserializeFheEncryptionPublicKeyReturnType,
  DeserializeFheEncryptionCrsParameters,
  DeserializeFheEncryptionCrsReturnType,
} from './types.js';

////////////////////////////////////////////////////////////////////////////////

const PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN = Symbol('CleartextTFHELib.token');

////////////////////////////////////////////////////////////////////////////////
// TfheCompactPublicKeyImpl
////////////////////////////////////////////////////////////////////////////////

class CleartextTfheCompactPublicKeyImpl implements FheEncryptionPublicKey {
  declare readonly [FheEncryptionPublicKeyBrand]: never;

  readonly #id: string;
  readonly #tfheVersion: TfheVersion;

  constructor(token: symbol, id: string, tfheVersion: TfheVersion) {
    if (token !== PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#id = id;
    this.#tfheVersion = tfheVersion;
  }

  public get id(): string {
    return this.#id;
  }

  public get tfheVersion(): TfheVersion {
    return this.#tfheVersion;
  }
}

////////////////////////////////////////////////////////////////////////////////
// TfheCompactPkeCrsImpl
////////////////////////////////////////////////////////////////////////////////

class CleartextTfheCompactPkeCrsImpl implements FheEncryptionCrs {
  declare readonly [FheEncryptionCrsBrand]: never;

  readonly #id: string;
  readonly #tfheVersion: TfheVersion;
  readonly #capacity: UintNumber;

  constructor(token: symbol, id: string, tfheVersion: TfheVersion, capacity: UintNumber) {
    if (token !== PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#id = id;
    this.#tfheVersion = tfheVersion;
    this.#capacity = capacity;
  }

  public get id(): string {
    return this.#id;
  }

  public get tfheVersion(): TfheVersion {
    return this.#tfheVersion;
  }

  public get capacity(): UintNumber {
    return this.#capacity;
  }
}

////////////////////////////////////////////////////////////////////////////////
// parseTFHEProvenCompactCiphertextList
////////////////////////////////////////////////////////////////////////////////

export async function parseTFHEProvenCompactCiphertextList(
  _parameters: ParseTFHEProvenCompactCiphertextListParameters,
): Promise<ParseTFHEProvenCompactCiphertextListReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

// Per typedValue: random nonce || metaData || value as a 32-byte big-endian word.
const PACKED_NONCE_LEN = 8;
const PACKED_VALUE_LEN = 32;

export async function buildWithProofPacked(
  parameters: BuildWithProofPackedParameters,
): Promise<BuildWithProofPackedReturnType> {
  const { fheEncryptionKey: publicEncryptionParams, metaData, typedValues, extraData } = parameters;

  const tfheCompactPublicKeyImpl = publicEncryptionParams.publicKey;
  const tfheCompactPkeCrsImpl = publicEncryptionParams.crs;

  if (!(tfheCompactPublicKeyImpl instanceof CleartextTfheCompactPublicKeyImpl)) {
    throw new Error('Invalid tfhePublicKey');
  }
  if (!(tfheCompactPkeCrsImpl instanceof CleartextTfheCompactPkeCrsImpl)) {
    throw new Error('Invalid tfheCrs');
  }

  const typedValuesBytes32HexNo0x = typedValues.map(typedValueToBytes32Hex).map(remove0x).join('');
  const cleartextExtraData = `0x${typedValuesBytes32HexNo0x}${remove0x(extraData)}` as BytesHex;

  const perValueBlobs = typedValues.map((tv) => {
    const nonce = crypto.getRandomValues(new Uint8Array(new ArrayBuffer(PACKED_NONCE_LEN)));
    const valueBytes = hexToBytes32(typedValueToBytes32Hex(tv));
    return concatBytes(nonce, metaData, valueBytes);
  });

  const ciphertextWithZKProofBytes = concatBytes(...perValueBlobs);

  return {
    ciphertextWithZKProofBytes,
    extraData: cleartextExtraData,
    tfheVersion: tfheCompactPublicKeyImpl.tfheVersion,
  };
}

/**
 * The values `buildWithProofPacked` packed with `metaData`, as 32-byte big-endian words in input
 * order, or `undefined` when `ciphertextWithZKProofBytes` is not that packing of `count` values.
 */
export function unpackWithProofPacked(parameters: {
  readonly ciphertextWithZKProofBytes: Uint8Array;
  readonly metaData: Uint8Array;
  readonly count: number;
}): Uint8Array[] | undefined {
  const { ciphertextWithZKProofBytes: packed, metaData, count } = parameters;
  const blobLength = PACKED_NONCE_LEN + metaData.length + PACKED_VALUE_LEN;
  if (packed.length !== count * blobLength) return undefined;
  const values: Uint8Array[] = [];
  for (let start = 0; start < packed.length; start += blobLength) {
    const blob = packed.subarray(start, start + blobLength);
    if (!unsafeBytesEquals(blob.subarray(PACKED_NONCE_LEN, PACKED_NONCE_LEN + metaData.length), metaData)) {
      return undefined;
    }
    values.push(blob.subarray(blobLength - PACKED_VALUE_LEN));
  }
  return values;
}

////////////////////////////////////////////////////////////////////////////////
// deserializeFheEncryptionCrs
////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionCrs(
  parameters: DeserializeFheEncryptionCrsParameters,
): Promise<DeserializeFheEncryptionCrsReturnType> {
  const { crsBytes: globalFheCrsBytes } = parameters;

  return new CleartextTfheCompactPkeCrsImpl(
    PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN,
    globalFheCrsBytes.id,
    CANONICAL_WASM_VERSIONS.tfhe,
    globalFheCrsBytes.capacity,
  );
}

////////////////////////////////////////////////////////////////////////////////
// deserializeFheEncryptionPublicKey
////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionPublicKey(
  parameters: DeserializeFheEncryptionPublicKeyParameters,
): Promise<DeserializeFheEncryptionPublicKeyReturnType> {
  const { publicKeyBytes: globalFhePublicKeyBytes } = parameters;

  return new CleartextTfheCompactPublicKeyImpl(
    PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN,
    globalFhePublicKeyBytes.id,
    CANONICAL_WASM_VERSIONS.tfhe,
  );
}

////////////////////////////////////////////////////////////////////////////////
// mockTfheActions
////////////////////////////////////////////////////////////////////////////////

export const encryptModule: EncryptModuleFactory = (_runtime: FhevmRuntime) => {
  return Object.freeze({
    encrypt: Object.freeze({
      initTfheModule: () => Promise.resolve(),
      getTfheModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      parseTFHEProvenCompactCiphertextList,
      buildWithProofPacked,
      deserializeFheEncryptionPublicKey,
      deserializeFheEncryptionCrs,
    }),
  });
};
