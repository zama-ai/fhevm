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
  DeserializeFheEncryptionCrsParameters,
  DeserializeFheEncryptionCrsReturnType,
  DeserializeFheEncryptionPublicKeyParameters,
  DeserializeFheEncryptionPublicKeyReturnType,
  CanBuildWithProofPackedSeededParameters,
  CanBuildWithProofPackedSeededReturnType,
} from '../types.js';
import type {
  FheEncryptionCrs,
  FheEncryptionCrsBrand,
  FheEncryptionKeyBytes,
  FheEncryptionPublicKey,
  FheEncryptionPublicKeyBrand,
} from '../../../types/fheEncryptionKey.js';
import type { Bytes, UintNumber } from '../../../types/primitives.js';
import type { FheTypeId } from '../../../types/fheType.js';
import type { FhevmRuntime } from '../../../types/coreFhevmRuntime.js';
import type {
  CompactPkeCrs,
  TfheCompactPublicKey,
  ProvenCompactCiphertextList,
  CompactCiphertextListBuilder,
  TfheVersion,
} from '../../../../wasm/tfhe/TfheApi.js';
import { isNonEmptyString } from '../../../base/string.js';
import { hexToBytesFaster } from '../../../base/bytes.js';
import { encryptionBitsFromFheTypeId, isFheTypeId } from '../../../handle/FheType.js';
import { EncryptionError } from '../../../errors/EncryptionError.js';
import { getErrorMessage } from '../../../base/errors/utils.js';
import { initTfheModule } from './init-p.js';
import { assertIsTypedValue } from '../../../base/typedValue.js';
import { isSemverAtLeast } from '../../../base/semver.js';

////////////////////////////////////////////////////////////////////////////////

const GET_NATIVE_FUNC = Symbol('TFHELib.getNative');
const PRIVATE_TFHE_LIB_TOKEN = Symbol('TFHELib.token');

////////////////////////////////////////////////////////////////////////////////
//
// TFHELib
//
////////////////////////////////////////////////////////////////////////////////

export const SERIALIZED_SIZE_LIMIT_CIPHERTEXT = BigInt(1024 * 1024 * 512);
export const SERIALIZED_SIZE_LIMIT_PK = BigInt(1024 * 1024 * 512);
export const SERIALIZED_SIZE_LIMIT_CRS = BigInt(1024 * 1024 * 512);

const MIN_TFHE_VERSION_WITH_SEEDED_PROOF = '1.6.0';

// Cache only the capability result by version; do not retain key-backed WASM objects here.
const cacheCanBuildWithProofPackedSeededByTfheVersion = new Map<TfheVersion, boolean>();

////////////////////////////////////////////////////////////////////////////////
// TfheCompactPublicKeyImpl
////////////////////////////////////////////////////////////////////////////////

class TfheCompactPublicKeyImpl implements FheEncryptionPublicKey {
  declare readonly [FheEncryptionPublicKeyBrand]: never;

  readonly #id: string;
  readonly #tfheCompactPublicKeyWasmType: TfheCompactPublicKey;
  readonly #tfheVersion: TfheVersion;

  constructor(token: symbol, id: string, tfheVersion: TfheVersion, publicEncKeyMlKem512Wasm: TfheCompactPublicKey) {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
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

  public static [GET_NATIVE_FUNC](key: unknown, token: symbol, tfheVersion: TfheVersion): TfheCompactPublicKey {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    if (!(key instanceof TfheCompactPublicKeyImpl)) {
      throw new Error('Unauthorized');
    }
    if (tfheVersion !== key.#tfheVersion) {
      throw new Error('TfheVersion mismatch');
    }
    return key.#tfheCompactPublicKeyWasmType;
  }
}

////////////////////////////////////////////////////////////////////////////////
// TfheCompactPkeCrsImpl
////////////////////////////////////////////////////////////////////////////////

class TfheCompactPkeCrsImpl implements FheEncryptionCrs {
  declare readonly [FheEncryptionCrsBrand]: never;

  readonly #id: string;
  readonly #capacity: UintNumber;
  readonly #compactPublicKeyWasmType: CompactPkeCrs;
  readonly #tfheVersion: TfheVersion;

  constructor(
    token: symbol,
    id: string,
    tfheVersion: TfheVersion,
    capacity: UintNumber,
    compactPublicKeyWasmType: CompactPkeCrs,
  ) {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
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

  public static [GET_NATIVE_FUNC](key: unknown, token: symbol, tfheVersion: TfheVersion): CompactPkeCrs {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
      throw new Error('Unauthorized');
    }
    if (!(key instanceof TfheCompactPkeCrsImpl)) {
      throw new Error('Unauthorized');
    }
    if (tfheVersion !== key.#tfheVersion) {
      throw new Error('TfheVersion mismatch');
    }
    return key.#compactPublicKeyWasmType;
  }
}

////////////////////////////////////////////////////////////////////////////////
// parseTFHEProvenCompactCiphertextList
////////////////////////////////////////////////////////////////////////////////

export async function parseTFHEProvenCompactCiphertextList(
  runtime: FhevmRuntime,
  parameters: ParseTFHEProvenCompactCiphertextListParameters,
): Promise<ParseTFHEProvenCompactCiphertextListReturnType> {
  const tfheLib = await initTfheModule(runtime, { tfheVersion: parameters.tfheVersion });

  const { ciphertextWithZkProof: ciphertextWithZKProof } = parameters;
  if ((ciphertextWithZKProof as unknown) == null) {
    throw new EncryptionError({
      message: `ciphertextWithZKProof argument is null or undefined.`,
    });
  }
  if (!(ciphertextWithZKProof instanceof Uint8Array) && !isNonEmptyString(ciphertextWithZKProof)) {
    throw new EncryptionError({
      message: `Invalid ciphertextWithZKProof argument.`,
    });
  }

  const ciphertext: Uint8Array =
    typeof ciphertextWithZKProof === 'string'
      ? hexToBytesFaster(ciphertextWithZKProof, { strict: true })
      : ciphertextWithZKProof;

  let listWasm: ProvenCompactCiphertextList;
  try {
    listWasm = tfheLib.ProvenCompactCiphertextList.safe_deserialize(ciphertext, SERIALIZED_SIZE_LIMIT_CIPHERTEXT);
  } catch (e) {
    throw new EncryptionError({
      message: `Invalid ciphertextWithZKProof bytes. ${getErrorMessage(e)}.`,
    });
  }

  const fheTypeIds: FheTypeId[] = [];

  try {
    const len = listWasm.len();

    for (let i = 0; i < len; ++i) {
      const v = listWasm.get_kind_of(i);
      if (!isFheTypeId(v)) {
        throw new EncryptionError({
          message: `Invalid FheTypeId: ${v}`,
        });
      }
      fheTypeIds.push(v);
    }

    return {
      fheTypeIds,
      encryptionBits: fheTypeIds.map(encryptionBitsFromFheTypeId),
    };
  } finally {
    listWasm.free();
  }
}

////////////////////////////////////////////////////////////////////////////////
// canBuildWithProofPackedSeeded
////////////////////////////////////////////////////////////////////////////////

export async function canBuildWithProofPackedSeeded(
  runtime: FhevmRuntime,
  parameters: CanBuildWithProofPackedSeededParameters,
): Promise<CanBuildWithProofPackedSeededReturnType> {
  const cached = cacheCanBuildWithProofPackedSeededByTfheVersion.get(parameters.tfheVersion);
  if (cached !== undefined) {
    return cached;
  }

  if (!isSemverAtLeast(parameters.tfheVersion, MIN_TFHE_VERSION_WITH_SEEDED_PROOF)) {
    cacheCanBuildWithProofPackedSeededByTfheVersion.set(parameters.tfheVersion, false);
    return false;
  }

  const tfheLib = await initTfheModule(runtime, { tfheVersion: parameters.tfheVersion });
  const canBuild = _tfheLibHasSeededBuilderMethod(tfheLib);

  cacheCanBuildWithProofPackedSeededByTfheVersion.set(parameters.tfheVersion, canBuild);

  return canBuild;
}

////////////////////////////////////////////////////////////////////////////////
// buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

export async function buildWithProofPacked(
  runtime: FhevmRuntime,
  parameters: BuildWithProofPackedParameters,
): Promise<BuildWithProofPackedReturnType> {
  return _buildWithProofPacked(runtime, { ...parameters, seed: new Uint8Array() });
}

////////////////////////////////////////////////////////////////////////////////
// buildWithProofPackedSeeded
////////////////////////////////////////////////////////////////////////////////

export async function buildWithProofPackedSeeded(
  runtime: FhevmRuntime,
  parameters: BuildWithProofPackedSeededParameters,
): Promise<BuildWithProofPackedReturnType> {
  // TFHE >= 1.6.0 may support seeded proof generation; the WASM method check below remains authoritative.
  if (!isSemverAtLeast(parameters.tfheVersion, MIN_TFHE_VERSION_WITH_SEEDED_PROOF)) {
    throw new Error(
      `TFHE ${parameters.tfheVersion} does not support seeded proof generation: expected >= ${MIN_TFHE_VERSION_WITH_SEEDED_PROOF}`,
    );
  }

  if (parameters.seed.length < 4 || parameters.seed.length > 8) {
    // Seed length must be 32 to 64 bits inclusive.
    throw new Error('Invalid seed length: expected 4 to 8 bytes');
  }

  return _buildWithProofPacked(runtime, parameters);
}

////////////////////////////////////////////////////////////////////////////////
// _buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

type SeededCompactCiphertextListBuilder = CompactCiphertextListBuilder & {
  build_with_proof_packed_seeded: (
    crs: Parameters<CompactCiphertextListBuilder['build_with_proof_packed']>[0],
    metadata: Parameters<CompactCiphertextListBuilder['build_with_proof_packed']>[1],
    computeLoad: Parameters<CompactCiphertextListBuilder['build_with_proof_packed']>[2],
    seed: Uint8Array,
  ) => ProvenCompactCiphertextList;
};

function _tfheLibHasSeededBuilderMethod(tfheLib: unknown): boolean {
  const compactCiphertextListBuilderClass = (
    tfheLib as {
      readonly CompactCiphertextListBuilder?: {
        readonly prototype?: {
          readonly build_with_proof_packed_seeded?: unknown;
        };
      };
    }
  ).CompactCiphertextListBuilder;

  return typeof compactCiphertextListBuilderClass?.prototype?.build_with_proof_packed_seeded === 'function';
}

// available in tfhe v1.6 and higher.
function _hasSeededMethod(builder: CompactCiphertextListBuilder): builder is SeededCompactCiphertextListBuilder {
  return typeof (builder as { build_with_proof_packed_seeded?: unknown }).build_with_proof_packed_seeded === 'function';
}

async function _buildWithProofPacked(
  runtime: FhevmRuntime,
  parameters: BuildWithProofPackedSeededParameters,
): Promise<BuildWithProofPackedReturnType> {
  const tfheLib = await initTfheModule(runtime, { tfheVersion: parameters.tfheVersion });

  const { fheEncryptionKey: publicEncryptionParams, metaData, typedValues, extraData, seed } = parameters;

  const tfheCompactPublicKeyImpl = publicEncryptionParams.publicKey;
  const tfheCompactPkeCrsImpl = publicEncryptionParams.crs;

  if (!(tfheCompactPublicKeyImpl instanceof TfheCompactPublicKeyImpl)) {
    throw new Error('Invalid tfhePublicKey');
  }

  if (!(tfheCompactPkeCrsImpl instanceof TfheCompactPkeCrsImpl)) {
    throw new Error('Invalid tfheCrs');
  }

  let tfheProvenCompactCiphertextList: ProvenCompactCiphertextList | undefined;

  let ciphertextWithZKProofBytes: Uint8Array | undefined;
  let fheCompactCiphertextListBuilderWasm: CompactCiphertextListBuilder | undefined;

  try {
    const tfheCompactPublicKeyWasm: TfheCompactPublicKey = TfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
      tfheCompactPublicKeyImpl,
      PRIVATE_TFHE_LIB_TOKEN,
      parameters.tfheVersion,
    );
    const compactPkeCrsWasm: CompactPkeCrs = TfheCompactPkeCrsImpl[GET_NATIVE_FUNC](
      tfheCompactPkeCrsImpl,
      PRIVATE_TFHE_LIB_TOKEN,
      parameters.tfheVersion,
    );

    fheCompactCiphertextListBuilderWasm = tfheLib.CompactCiphertextList.builder(tfheCompactPublicKeyWasm);

    for (const typedValue of typedValues) {
      assertIsTypedValue(typedValue, {});
      switch (typedValue.type) {
        case 'uint8':
          fheCompactCiphertextListBuilderWasm.push_u8(typedValue.value);
          break;
        case 'uint16':
          fheCompactCiphertextListBuilderWasm.push_u16(typedValue.value);
          break;
        case 'uint32':
          fheCompactCiphertextListBuilderWasm.push_u32(typedValue.value);
          break;
        case 'uint64':
          fheCompactCiphertextListBuilderWasm.push_u64(typedValue.value);
          break;
        case 'uint128':
          fheCompactCiphertextListBuilderWasm.push_u128(typedValue.value);
          break;
        case 'uint256':
          fheCompactCiphertextListBuilderWasm.push_u256(typedValue.value);
          break;
        case 'bool':
          fheCompactCiphertextListBuilderWasm.push_boolean(typedValue.value);
          break;
        case 'address':
          fheCompactCiphertextListBuilderWasm.push_u160(BigInt(typedValue.value));
          break;
      }
    }

    if (seed.length > 0) {
      if (!_hasSeededMethod(fheCompactCiphertextListBuilderWasm)) {
        throw new Error(`TFHE ${parameters.tfheVersion} does not support seeded proof generation`);
      }

      tfheProvenCompactCiphertextList = fheCompactCiphertextListBuilderWasm.build_with_proof_packed_seeded(
        compactPkeCrsWasm,
        metaData,
        tfheLib.ZkComputeLoad.Verify,
        seed,
      );
    } else {
      tfheProvenCompactCiphertextList = fheCompactCiphertextListBuilderWasm.build_with_proof_packed(
        compactPkeCrsWasm,
        metaData,
        tfheLib.ZkComputeLoad.Verify,
      );
    }

    ciphertextWithZKProofBytes = tfheProvenCompactCiphertextList.safe_serialize(SERIALIZED_SIZE_LIMIT_CIPHERTEXT);

    return Object.freeze({
      ciphertextWithZKProofBytes,
      extraData,
      tfheVersion: parameters.tfheVersion,
    });
  } finally {
    try {
      if (tfheProvenCompactCiphertextList !== undefined) {
        tfheProvenCompactCiphertextList.free();
      }
    } catch {
      //
    }

    try {
      if (fheCompactCiphertextListBuilderWasm !== undefined) {
        fheCompactCiphertextListBuilderWasm.free();
      }
    } catch {
      //
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// serializeFheEncryptionKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionKey(
  runtime: FhevmRuntime,
  parameters: SerializeFheEncryptionKeyParameters,
): Promise<SerializeFheEncryptionKeyReturnType> {
  const tfheVersion = parameters.tfheVersion;
  await initTfheModule(runtime, { tfheVersion });

  const { fheEncryptionKey: publicEncryptionParams } = parameters;

  const tfheCompactPublicKeyImpl = publicEncryptionParams.publicKey;
  const tfheCompactPkeCrsImpl = publicEncryptionParams.crs;

  if (!(tfheCompactPublicKeyImpl instanceof TfheCompactPublicKeyImpl)) {
    throw new Error('Invalid tfhePublicKey');
  }
  if (!(tfheCompactPkeCrsImpl instanceof TfheCompactPkeCrsImpl)) {
    throw new Error('Invalid tfheCrs');
  }

  const tfhePublicKeyBytes: Bytes = TfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
    tfheCompactPublicKeyImpl,
    PRIVATE_TFHE_LIB_TOKEN,
    tfheVersion,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_PK);
  const tfheCrsBytes: Bytes = TfheCompactPkeCrsImpl[GET_NATIVE_FUNC](
    tfheCompactPkeCrsImpl,
    PRIVATE_TFHE_LIB_TOKEN,
    tfheVersion,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_CRS);

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
// serializeTfhePublicKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionPublicKey(
  runtime: FhevmRuntime,
  parameters: SerializeFheEncryptionPublicKeyParameters,
): Promise<SerializeFheEncryptionPublicKeyReturnType> {
  const tfheVersion = parameters.tfheVersion;
  await initTfheModule(runtime, { tfheVersion });

  const { publicKey: tfhePublicKey } = parameters;

  const tfheCompactPublicKeyImpl = tfhePublicKey;

  if (!(tfheCompactPublicKeyImpl instanceof TfheCompactPublicKeyImpl)) {
    throw new Error('Invalid tfhePublicKey');
  }

  const tfhePublicKeyBytes: Bytes = TfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
    tfheCompactPublicKeyImpl,
    PRIVATE_TFHE_LIB_TOKEN,
    tfheVersion,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_PK);

  return Object.freeze({
    id: tfhePublicKey.id,
    bytes: tfhePublicKeyBytes,
    tfheVersion,
  });
}

////////////////////////////////////////////////////////////////////////////////
// serializeGlobalFheCrs
////////////////////////////////////////////////////////////////////////////////

export async function serializeFheEncryptionCrs(
  runtime: FhevmRuntime,
  parameters: SerializeFheEncryptionCrsParameters,
): Promise<SerializeFheEncryptionCrsReturnType> {
  const tfheVersion = parameters.tfheVersion;
  await initTfheModule(runtime, { tfheVersion });

  const { crs: tfheCrs } = parameters;

  const tfheCompactPkeCrsImpl = tfheCrs;

  if (!(tfheCompactPkeCrsImpl instanceof TfheCompactPkeCrsImpl)) {
    throw new Error('Invalid tfheCrs');
  }

  const tfheCrsBytes: Bytes = TfheCompactPkeCrsImpl[GET_NATIVE_FUNC](
    tfheCompactPkeCrsImpl,
    PRIVATE_TFHE_LIB_TOKEN,
    tfheVersion,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_CRS);

  return Object.freeze({
    id: tfheCrs.id,
    capacity: tfheCrs.capacity,
    bytes: tfheCrsBytes,
    tfheVersion,
  });
}

////////////////////////////////////////////////////////////////////////////////
// deserializeFheEncryptionCrs
////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionCrs(
  runtime: FhevmRuntime,
  parameters: DeserializeFheEncryptionCrsParameters,
): Promise<DeserializeFheEncryptionCrsReturnType> {
  const tfheVersion = parameters.tfheVersion;
  const tfheLib = await initTfheModule(runtime, { tfheVersion });

  const { crsBytes: globalFheCrsBytes } = parameters;

  const compactPkeCrsWasm: CompactPkeCrs = tfheLib.CompactPkeCrs.safe_deserialize(
    globalFheCrsBytes.bytes,
    SERIALIZED_SIZE_LIMIT_CRS,
  );

  return new TfheCompactPkeCrsImpl(
    PRIVATE_TFHE_LIB_TOKEN,
    globalFheCrsBytes.id,
    tfheVersion,
    globalFheCrsBytes.capacity,
    compactPkeCrsWasm,
  );
}

////////////////////////////////////////////////////////////////////////////////
// deserializeFheEncryptionPublicKey
////////////////////////////////////////////////////////////////////////////////

export async function deserializeFheEncryptionPublicKey(
  runtime: FhevmRuntime,
  parameters: DeserializeFheEncryptionPublicKeyParameters,
): Promise<DeserializeFheEncryptionPublicKeyReturnType> {
  const tfheVersion = parameters.tfheVersion;
  const tfheLib = await initTfheModule(runtime, { tfheVersion });

  const { publicKeyBytes: globalFhePublicKeyBytes } = parameters;

  const tfheCompactPublicKeyWasm: TfheCompactPublicKey = tfheLib.TfheCompactPublicKey.safe_deserialize(
    globalFhePublicKeyBytes.bytes,
    SERIALIZED_SIZE_LIMIT_PK,
  );

  return new TfheCompactPublicKeyImpl(
    PRIVATE_TFHE_LIB_TOKEN,
    globalFhePublicKeyBytes.id,
    tfheVersion,
    tfheCompactPublicKeyWasm,
  );
}
