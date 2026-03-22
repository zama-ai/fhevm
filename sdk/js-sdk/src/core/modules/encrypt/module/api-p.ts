import type {
  BuildWithProofPackedReturnTypeParameters,
  BuildWithProofPackedReturnType,
  ParseTFHEProvenCompactCiphertextListParameters,
  ParseTFHEProvenCompactCiphertextListReturnType,
  SerializeGlobalFheCrsParameters,
  SerializeGlobalFheCrsReturnType,
  SerializeGlobalFhePkeParamsParameters,
  SerializeGlobalFhePkeParamsReturnType,
  SerializeGlobalFhePublicKeyParameters,
  SerializeGlobalFhePublicKeyReturnType,
  DeserializeGlobalFheCrsParameters,
  DeserializeGlobalFheCrsReturnType,
  DeserializeGlobalFhePublicKeyParameters,
  DeserializeGlobalFhePublicKeyReturnType,
} from "../types.js";
import type {
  GlobalFheCrs,
  GlobalFheCrsBrand,
  GlobalFheCrsBytes,
  GlobalFhePkeParamsBytes,
  GlobalFhePublicKey,
  GlobalFhePublicKeyBrand,
  GlobalFhePublicKeyBytes,
} from "../../../types/globalFhePkeParams.js";
import type { CompactCiphertextListBuilder } from "../../../../wasm/tfhe/tfhe.v1.5.3.js";
import {
  TfheCompactPublicKey,
  CompactPkeCrs,
  ProvenCompactCiphertextList,
  CompactCiphertextList,
  ZkComputeLoad,
} from "../../../../wasm/tfhe/tfhe.v1.5.3.js";
import type { Bytes, UintNumber } from "../../../types/primitives.js";
import { isNonEmptyString } from "../../../base/string.js";
import { hexToBytesFaster } from "../../../base/bytes.js";
import type { FheTypeId } from "../../../types/fheType.js";
import {
  encryptionBitsFromFheTypeId,
  isFheTypeId,
} from "../../../handle/FheType.js";
import { EncryptionError } from "../../../errors/EncryptionError.js";
import { getErrorMessage } from "../../../base/errors/utils.js";
import { initTfheModule } from "./init-p.js";
import type { FhevmRuntime } from "../../../types/coreFhevmRuntime.js";

////////////////////////////////////////////////////////////////////////////////

const GET_NATIVE_FUNC = Symbol("TFHELib.getNative");
const PRIVATE_TFHE_LIB_TOKEN = Symbol("TFHELib.token");

////////////////////////////////////////////////////////////////////////////////
//
// TFHELib
//
////////////////////////////////////////////////////////////////////////////////

export const SERIALIZED_SIZE_LIMIT_CIPHERTEXT = BigInt(1024 * 1024 * 512);
export const SERIALIZED_SIZE_LIMIT_PK = BigInt(1024 * 1024 * 512);
export const SERIALIZED_SIZE_LIMIT_CRS = BigInt(1024 * 1024 * 512);

////////////////////////////////////////////////////////////////////////////////
// TfheCompactPublicKeyImpl
////////////////////////////////////////////////////////////////////////////////

class TfheCompactPublicKeyImpl implements GlobalFhePublicKey {
  declare readonly [GlobalFhePublicKeyBrand]: never;

  readonly #id: string;
  readonly #tfheCompactPublicKeyWasmType: TfheCompactPublicKey;

  constructor(
    token: symbol,
    id: string,
    publicEncKeyMlKem512Wasm: TfheCompactPublicKey,
  ) {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
      throw new Error("Unauthorized");
    }
    this.#id = id;
    this.#tfheCompactPublicKeyWasmType = publicEncKeyMlKem512Wasm;
  }

  public get id(): string {
    return this.#id;
  }

  public static [GET_NATIVE_FUNC](
    key: unknown,
    token: symbol,
  ): TfheCompactPublicKey {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
      throw new Error("Unauthorized");
    }
    if (!(key instanceof TfheCompactPublicKeyImpl)) {
      throw new Error("Unauthorized");
    }
    return key.#tfheCompactPublicKeyWasmType;
  }
}

////////////////////////////////////////////////////////////////////////////////
// TfheCompactPkeCrsImpl
////////////////////////////////////////////////////////////////////////////////

class TfheCompactPkeCrsImpl implements GlobalFheCrs {
  declare readonly [GlobalFheCrsBrand]: never;

  readonly #id: string;
  readonly #capacity: UintNumber;
  readonly #compactPublicKeyWasmType: CompactPkeCrs;

  constructor(
    token: symbol,
    id: string,
    capacity: UintNumber,
    compactPublicKeyWasmType: CompactPkeCrs,
  ) {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
      throw new Error("Unauthorized");
    }
    this.#id = id;
    this.#capacity = capacity;
    this.#compactPublicKeyWasmType = compactPublicKeyWasmType;
  }

  public get id(): string {
    return this.#id;
  }

  public get capacity(): UintNumber {
    return this.#capacity;
  }

  public static [GET_NATIVE_FUNC](key: unknown, token: symbol): CompactPkeCrs {
    if (token !== PRIVATE_TFHE_LIB_TOKEN) {
      throw new Error("Unauthorized");
    }
    if (!(key instanceof TfheCompactPkeCrsImpl)) {
      throw new Error("Unauthorized");
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
  await initTfheModule(runtime);

  const { ciphertextWithZkProof: ciphertextWithZKProof } = parameters;
  if ((ciphertextWithZKProof as unknown) == null) {
    throw new EncryptionError({
      message: `ciphertextWithZKProof argument is null or undefined.`,
    });
  }
  if (
    !(ciphertextWithZKProof instanceof Uint8Array) &&
    !isNonEmptyString(ciphertextWithZKProof)
  ) {
    throw new EncryptionError({
      message: `Invalid ciphertextWithZKProof argument.`,
    });
  }

  const ciphertext: Uint8Array =
    typeof ciphertextWithZKProof === "string"
      ? hexToBytesFaster(ciphertextWithZKProof, { strict: true })
      : ciphertextWithZKProof;

  let listWasm: ProvenCompactCiphertextList;
  try {
    listWasm = ProvenCompactCiphertextList.safe_deserialize(
      ciphertext,
      SERIALIZED_SIZE_LIMIT_CIPHERTEXT,
    );
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
// buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

export async function buildWithProofPacked(
  runtime: FhevmRuntime,
  parameters: BuildWithProofPackedReturnTypeParameters,
): Promise<BuildWithProofPackedReturnType> {
  await initTfheModule(runtime);

  const { publicEncryptionParams, metaData, typedValues } = parameters;

  const tfheCompactPublicKeyImpl = publicEncryptionParams.publicKey;
  const tfheCompactPkeCrsImpl = publicEncryptionParams.crs;

  if (!(tfheCompactPublicKeyImpl instanceof TfheCompactPublicKeyImpl)) {
    throw new Error("Invalid tfhePublicKey");
  }
  if (!(tfheCompactPkeCrsImpl instanceof TfheCompactPkeCrsImpl)) {
    throw new Error("Invalid tfheCrs");
  }

  let tfheProvenCompactCiphertextList: ProvenCompactCiphertextList | undefined;

  let ciphertextWithZKProofBytes: Uint8Array | undefined;
  let fheCompactCiphertextListBuilderWasm:
    | CompactCiphertextListBuilder
    | undefined;

  try {
    const tfheCompactPublicKeyWasm: TfheCompactPublicKey =
      TfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
        tfheCompactPublicKeyImpl,
        PRIVATE_TFHE_LIB_TOKEN,
      );
    const compactPkeCrsWasm: CompactPkeCrs = TfheCompactPkeCrsImpl[
      GET_NATIVE_FUNC
    ](tfheCompactPkeCrsImpl, PRIVATE_TFHE_LIB_TOKEN);

    fheCompactCiphertextListBuilderWasm = CompactCiphertextList.builder(
      tfheCompactPublicKeyWasm,
    );

    for (const typedValue of typedValues) {
      switch (typedValue.type) {
        case "uint8":
          fheCompactCiphertextListBuilderWasm.push_u8(typedValue.value);
          break;
        case "uint16":
          fheCompactCiphertextListBuilderWasm.push_u16(typedValue.value);
          break;
        case "uint32":
          fheCompactCiphertextListBuilderWasm.push_u32(typedValue.value);
          break;
        case "uint64":
          fheCompactCiphertextListBuilderWasm.push_u64(typedValue.value);
          break;
        case "uint128":
          fheCompactCiphertextListBuilderWasm.push_u128(typedValue.value);
          break;
        case "uint256":
          fheCompactCiphertextListBuilderWasm.push_u256(typedValue.value);
          break;
        case "bool":
          fheCompactCiphertextListBuilderWasm.push_boolean(typedValue.value);
          break;
        case "address":
          fheCompactCiphertextListBuilderWasm.push_u160(
            BigInt(typedValue.value),
          );
          break;
      }
    }

    tfheProvenCompactCiphertextList =
      fheCompactCiphertextListBuilderWasm.build_with_proof_packed(
        compactPkeCrsWasm,
        metaData,
        ZkComputeLoad.Verify,
      );

    ciphertextWithZKProofBytes = tfheProvenCompactCiphertextList.safe_serialize(
      SERIALIZED_SIZE_LIMIT_CIPHERTEXT,
    );

    return ciphertextWithZKProofBytes;
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
// serializeGlobalFhePkeParams
////////////////////////////////////////////////////////////////////////////////

export async function serializeGlobalFhePkeParams(
  runtime: FhevmRuntime,
  parameters: SerializeGlobalFhePkeParamsParameters,
): Promise<SerializeGlobalFhePkeParamsReturnType> {
  await initTfheModule(runtime);

  const { globalFhePkeParams: publicEncryptionParams } = parameters;

  const tfheCompactPublicKeyImpl = publicEncryptionParams.publicKey;
  const tfheCompactPkeCrsImpl = publicEncryptionParams.crs;

  if (!(tfheCompactPublicKeyImpl instanceof TfheCompactPublicKeyImpl)) {
    throw new Error("Invalid tfhePublicKey");
  }
  if (!(tfheCompactPkeCrsImpl instanceof TfheCompactPkeCrsImpl)) {
    throw new Error("Invalid tfheCrs");
  }

  const tfhePublicKeyBytes: Bytes = TfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
    tfheCompactPublicKeyImpl,
    PRIVATE_TFHE_LIB_TOKEN,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_PK);
  const tfheCrsBytes: Bytes = TfheCompactPkeCrsImpl[GET_NATIVE_FUNC](
    tfheCompactPkeCrsImpl,
    PRIVATE_TFHE_LIB_TOKEN,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_CRS);

  return Object.freeze({
    publicKeyBytes: Object.freeze({
      id: publicEncryptionParams.publicKey.id,
      bytes: tfhePublicKeyBytes,
    }),
    crsBytes: Object.freeze({
      id: publicEncryptionParams.crs.id,
      capacity: publicEncryptionParams.crs.capacity,
      bytes: tfheCrsBytes,
    }),
  }) as GlobalFhePkeParamsBytes;
}

////////////////////////////////////////////////////////////////////////////////
// serializeTfhePublicKey
////////////////////////////////////////////////////////////////////////////////

export async function serializeGlobalFhePublicKey(
  runtime: FhevmRuntime,
  parameters: SerializeGlobalFhePublicKeyParameters,
): Promise<SerializeGlobalFhePublicKeyReturnType> {
  await initTfheModule(runtime);

  const { globalFhePublicKey: tfhePublicKey } = parameters;

  const tfheCompactPublicKeyImpl = tfhePublicKey;

  if (!(tfheCompactPublicKeyImpl instanceof TfheCompactPublicKeyImpl)) {
    throw new Error("Invalid tfhePublicKey");
  }

  const tfhePublicKeyBytes: Bytes = TfheCompactPublicKeyImpl[GET_NATIVE_FUNC](
    tfheCompactPublicKeyImpl,
    PRIVATE_TFHE_LIB_TOKEN,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_PK);

  return Object.freeze({
    id: tfhePublicKey.id,
    bytes: tfhePublicKeyBytes,
  }) as GlobalFhePublicKeyBytes;
}

////////////////////////////////////////////////////////////////////////////////
// serializeGlobalFheCrs
////////////////////////////////////////////////////////////////////////////////

export async function serializeGlobalFheCrs(
  runtime: FhevmRuntime,
  parameters: SerializeGlobalFheCrsParameters,
): Promise<SerializeGlobalFheCrsReturnType> {
  await initTfheModule(runtime);

  const { globalFheCrs: tfheCrs } = parameters;

  const tfheCompactPkeCrsImpl = tfheCrs;

  if (!(tfheCompactPkeCrsImpl instanceof TfheCompactPkeCrsImpl)) {
    throw new Error("Invalid tfheCrs");
  }

  const tfheCrsBytes: Bytes = TfheCompactPkeCrsImpl[GET_NATIVE_FUNC](
    tfheCompactPkeCrsImpl,
    PRIVATE_TFHE_LIB_TOKEN,
  ).safe_serialize(SERIALIZED_SIZE_LIMIT_CRS);

  return Object.freeze({
    id: tfheCrs.id,
    capacity: tfheCrs.capacity,
    bytes: tfheCrsBytes,
  }) as GlobalFheCrsBytes;
}

////////////////////////////////////////////////////////////////////////////////
// deserializeGlobalFhePkeParams
////////////////////////////////////////////////////////////////////////////////

export async function deserializeGlobalFheCrs(
  runtime: FhevmRuntime,
  parameters: DeserializeGlobalFheCrsParameters,
): Promise<DeserializeGlobalFheCrsReturnType> {
  await initTfheModule(runtime);

  const { globalFheCrsBytes } = parameters;

  const compactPkeCrsWasm: CompactPkeCrs = CompactPkeCrs.safe_deserialize(
    globalFheCrsBytes.bytes,
    SERIALIZED_SIZE_LIMIT_CRS,
  );

  return new TfheCompactPkeCrsImpl(
    PRIVATE_TFHE_LIB_TOKEN,
    globalFheCrsBytes.id,
    globalFheCrsBytes.capacity,
    compactPkeCrsWasm,
  );
}

////////////////////////////////////////////////////////////////////////////////
// deserializeGlobalFhePkeParams
////////////////////////////////////////////////////////////////////////////////

export async function deserializeGlobalFhePublicKey(
  runtime: FhevmRuntime,
  parameters: DeserializeGlobalFhePublicKeyParameters,
): Promise<DeserializeGlobalFhePublicKeyReturnType> {
  await initTfheModule(runtime);

  const { globalFhePublicKeyBytes } = parameters;

  const tfheCompactPublicKeyWasm: TfheCompactPublicKey =
    TfheCompactPublicKey.safe_deserialize(
      globalFhePublicKeyBytes.bytes,
      SERIALIZED_SIZE_LIMIT_PK,
    );

  return new TfheCompactPublicKeyImpl(
    PRIVATE_TFHE_LIB_TOKEN,
    globalFhePublicKeyBytes.id,
    tfheCompactPublicKeyWasm,
  );
}
