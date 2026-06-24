import type { TfheVersion } from '../../../wasm/tfhe/TfheApi.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type {
  FheEncryptionCrs,
  FheEncryptionCrsBrand,
  FheEncryptionKeyBytes,
  FheEncryptionPublicKey,
  FheEncryptionPublicKeyBrand,
} from '../../types/fheEncryptionKey.js';
import type { Bytes, BytesHex, UintNumber } from '../../types/primitives.js';
import type {
  BuildWithProofPackedSeededParameters,
  BuildWithProofPackedParameters,
  BuildWithProofPackedReturnType,
  ParseTFHEProvenCompactCiphertextListParameters,
  ParseTFHEProvenCompactCiphertextListReturnType,
  SerializeFheEncryptionCrsParameters,
  SerializeFheEncryptionCrsReturnType,
  SerializeFheEncryptionKeyParameters,
  SerializeFheEncryptionKeyReturnType,
  SerializeFheEncryptionPublicKeyParameters,
  SerializeFheEncryptionPublicKeyReturnType,
  EncryptModuleFactory,
  DeserializeFheEncryptionPublicKeyParameters as DeserializeFheEncryptionPublicKeyParameters,
  DeserializeFheEncryptionPublicKeyReturnType as DeserializeFheEncryptionPublicKeyReturnType,
  DeserializeFheEncryptionCrsParameters,
  DeserializeFheEncryptionCrsReturnType,
  CanBuildWithProofPackedSeededParameters,
  CanBuildWithProofPackedSeededReturnType,
} from './types.js';
import { concatBytes, hexToBytes32 } from '../../base/bytes.js';
import { remove0x } from '../../base/string.js';
import { typedValueToBytes32Hex } from '../../base/typedValue.js';
import { MAX_UINT16, numberToBytes2 } from '../../base/uint.js';

////////////////////////////////////////////////////////////////////////////////

const GET_NATIVE_FUNC = Symbol('CleartextTFHELib.getNative');
const PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN = Symbol('CleartextTFHELib.token');

////////////////////////////////////////////////////////////////////////////////
// TfheCompactPublicKeyImpl
////////////////////////////////////////////////////////////////////////////////

class CleartextTfheCompactPublicKeyImpl implements FheEncryptionPublicKey {
  declare readonly [FheEncryptionPublicKeyBrand]: never;

  readonly #id: string;
  readonly #tfheCompactPublicKeyWasmType: Bytes;
  readonly #tfheVersion: TfheVersion;

  constructor(token: symbol, id: string, tfheVersion: TfheVersion, publicEncKeyMlKem512Wasm: Bytes) {
    if (token !== PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#id = id;
    this.#tfheVersion = tfheVersion;
    this.#tfheCompactPublicKeyWasmType = publicEncKeyMlKem512Wasm;
  }

  public get id(): string {
    return this.#id;
  }

  public get tfheVersion(): TfheVersion {
    return this.#tfheVersion;
  }

  public static [GET_NATIVE_FUNC](key: unknown, token: symbol): Bytes {
    if (token !== PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    if (!(key instanceof CleartextTfheCompactPublicKeyImpl)) {
      throw new Error('Unauthorized');
    }
    return key.#tfheCompactPublicKeyWasmType;
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
  readonly #compactPublicKeyWasmType: Bytes;

  constructor(
    token: symbol,
    id: string,
    tfheVersion: TfheVersion,
    capacity: UintNumber,
    compactPublicKeyWasmType: Bytes,
  ) {
    if (token !== PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    this.#id = id;
    this.#tfheVersion = tfheVersion;
    this.#capacity = capacity;
    this.#compactPublicKeyWasmType = compactPublicKeyWasmType;
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

  public static [GET_NATIVE_FUNC](key: unknown, token: symbol): Bytes {
    if (token !== PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    if (!(key instanceof CleartextTfheCompactPkeCrsImpl)) {
      throw new Error('Unauthorized');
    }
    return key.#compactPublicKeyWasmType;
  }
}

////////////////////////////////////////////////////////////////////////////////
// parseTFHEProvenCompactCiphertextList
////////////////////////////////////////////////////////////////////////////////

/* eslint-disable @typescript-eslint/require-await */
export async function parseTFHEProvenCompactCiphertextList(
  _parameters: ParseTFHEProvenCompactCiphertextListParameters,
): Promise<ParseTFHEProvenCompactCiphertextListReturnType> {
  throw new Error('Not yet implemented');
}

////////////////////////////////////////////////////////////////////////////////
// canBuildWithProofPackedSeeded
////////////////////////////////////////////////////////////////////////////////

export async function canBuildWithProofPackedSeeded(
  _parameters: CanBuildWithProofPackedSeededParameters,
): Promise<CanBuildWithProofPackedSeededReturnType> {
  return true;
}

////////////////////////////////////////////////////////////////////////////////
// buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

export async function buildWithProofPacked(
  parameters: BuildWithProofPackedParameters,
): Promise<BuildWithProofPackedReturnType> {
  return _buildWithProofPacked({ ...parameters, seed: crypto.getRandomValues(new Uint8Array(new ArrayBuffer(8))) });
}

////////////////////////////////////////////////////////////////////////////////
// buildWithProofPackedSeeded
////////////////////////////////////////////////////////////////////////////////

export async function buildWithProofPackedSeeded(
  parameters: BuildWithProofPackedSeededParameters,
): Promise<BuildWithProofPackedReturnType> {
  return _buildWithProofPacked(parameters);
}

////////////////////////////////////////////////////////////////////////////////
// _buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

async function _buildWithProofPacked(
  parameters: BuildWithProofPackedSeededParameters,
): Promise<BuildWithProofPackedReturnType> {
  const { fheEncryptionKey: publicEncryptionParams, metaData, typedValues, extraData, seed } = parameters;

  const MIN_SEED_BIT_LEN = 32;
  const MAX_SEED_BIT_LEN = 64;

  // seed must be at least 32-bits long and should not exceed 64-bits
  if (seed.length * 8 < MIN_SEED_BIT_LEN || seed.length * 8 > MAX_SEED_BIT_LEN) {
    throw new Error('Invalid seed length');
  }

  const tfheCompactPublicKeyImpl = publicEncryptionParams.publicKey;
  const tfheCompactPkeCrsImpl = publicEncryptionParams.crs;

  if (!(tfheCompactPublicKeyImpl instanceof CleartextTfheCompactPublicKeyImpl)) {
    throw new Error('Invalid tfhePublicKey');
  }

  if (!(tfheCompactPkeCrsImpl instanceof CleartextTfheCompactPkeCrsImpl)) {
    throw new Error('Invalid tfheCrs');
  }

  // mock accepts large arrays even if theoretically, the max len should be 256
  if (typedValues.length > MAX_UINT16 + 1) {
    throw new Error('Invalid typedValues length');
  }

  const typedValuesBytes32HexNo0x = typedValues.map(typedValueToBytes32Hex).map(remove0x).join('');
  const cleartextExtraData = `0x${typedValuesBytes32HexNo0x}${remove0x(extraData)}` as BytesHex;

  // Per typedValue: index (2) || seed || metaData || value bytes (32).
  const perValueBlobs = typedValues.map((tv, index) => {
    const indexBytes = numberToBytes2(index);
    const valueBytes = hexToBytes32(typedValueToBytes32Hex(tv));
    return concatBytes(indexBytes, parameters.seed, metaData, valueBytes);
  });

  const ciphertextWithZKProofBytes = concatBytes(...perValueBlobs);

  return {
    ciphertextWithZKProofBytes,
    extraData: cleartextExtraData,
    tfheVersion: parameters.tfheVersion,
  };
}

////////////////////////////////////////////////////////////////////////////////
// serializeFheEncryptionKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionKey(
  parameters: SerializeFheEncryptionKeyParameters,
): Promise<SerializeFheEncryptionKeyReturnType> {
  const { fheEncryptionKey: publicEncryptionParams } = parameters;

  const tfheCompactPublicKeyImpl = publicEncryptionParams.publicKey;
  const tfheCompactPkeCrsImpl = publicEncryptionParams.crs;

  if (!(tfheCompactPublicKeyImpl instanceof CleartextTfheCompactPublicKeyImpl)) {
    throw new Error('Invalid tfhePublicKey');
  }
  if (!(tfheCompactPkeCrsImpl instanceof CleartextTfheCompactPkeCrsImpl)) {
    throw new Error('Invalid tfheCrs');
  }

  const tfhePublicKeyBytes: Bytes = CleartextTfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
    tfheCompactPublicKeyImpl,
    PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN,
  );
  const tfheCrsBytes: Bytes = CleartextTfheCompactPkeCrsImpl[GET_NATIVE_FUNC](
    tfheCompactPkeCrsImpl,
    PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN,
  );

  const keyBytes: FheEncryptionKeyBytes = Object.freeze({
    publicKeyBytes: Object.freeze({
      id: publicEncryptionParams.publicKey.id,
      bytes: tfhePublicKeyBytes,
    }),
    crsBytes: Object.freeze({
      id: publicEncryptionParams.crs.id,
      capacity: publicEncryptionParams.crs.capacity,
      bytes: tfheCrsBytes,
    }),
    metadata: Object.freeze({
      relayerUrl: publicEncryptionParams.metadata.relayerUrl,
      chainId: publicEncryptionParams.metadata.chainId,
    }),
  });

  return keyBytes;
}

////////////////////////////////////////////////////////////////////////////////
// serializeFheEncryptionPublicKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionPublicKey(
  parameters: SerializeFheEncryptionPublicKeyParameters,
): Promise<SerializeFheEncryptionPublicKeyReturnType> {
  const { publicKey: tfhePublicKey } = parameters;

  const tfheCompactPublicKeyImpl = tfhePublicKey;

  if (!(tfheCompactPublicKeyImpl instanceof CleartextTfheCompactPublicKeyImpl)) {
    throw new Error('Invalid tfhePublicKey');
  }

  const tfhePublicKeyBytes: Bytes = CleartextTfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
    tfheCompactPublicKeyImpl,
    PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN,
  );

  return Object.freeze({
    id: tfhePublicKey.id,
    bytes: tfhePublicKeyBytes,
    tfheVersion: parameters.tfheVersion,
  });
}

////////////////////////////////////////////////////////////////////////////////
// serializeFheEncryptionCrs
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionCrs(
  parameters: SerializeFheEncryptionCrsParameters,
): Promise<SerializeFheEncryptionCrsReturnType> {
  const { crs: tfheCrs } = parameters;

  const tfheCompactPkeCrsImpl = tfheCrs;

  if (!(tfheCompactPkeCrsImpl instanceof CleartextTfheCompactPkeCrsImpl)) {
    throw new Error('Invalid tfheCrs');
  }

  const tfheCrsBytes: Bytes = CleartextTfheCompactPkeCrsImpl[GET_NATIVE_FUNC](
    tfheCompactPkeCrsImpl,
    PRIVATE_CLEARTEXT_TFHE_LIB_TOKEN,
  );

  return Object.freeze({
    id: tfheCrs.id,
    capacity: tfheCrs.capacity,
    bytes: tfheCrsBytes,
    tfheVersion: parameters.tfheVersion,
  });
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
    parameters.tfheVersion,
    globalFheCrsBytes.capacity,
    globalFheCrsBytes.bytes,
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
    parameters.tfheVersion,
    globalFhePublicKeyBytes.bytes,
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
      buildWithProofPackedSeeded,
      canBuildWithProofPackedSeeded,
      serializeFheEncryptionKey,
      serializeFheEncryptionPublicKey,
      serializeFheEncryptionCrs,
      deserializeFheEncryptionPublicKey,
      deserializeFheEncryptionCrs,
    }),
  });
};
