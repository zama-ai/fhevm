import type { BytesHex, TypedValue } from '../../types/primitives.js';
import type { EncryptionBits, FheTypeId } from '../../types/fheType.js';
import type {
  FheEncryptionCrs,
  FheEncryptionCrsBytes,
  FheEncryptionKeyWasm,
  FheEncryptionKeyBytes,
  FheEncryptionPublicKey,
  FheEncryptionPublicKeyBytes,
} from '../../types/fheEncryptionKey.js';
import type { Prettify } from '../../types/utils.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';

////////////////////////////////////////////////////////////////////////////////
//
// EncryptModule
//
////////////////////////////////////////////////////////////////////////////////

/*

WASM compilation (how to get WebAssembly.Module):

| wasmUrl   | Result                                                         |
|-----------|----------------------------------------------------------------|
| defined   | Compile from URL (isomorphicCompileWasm)                       |
| undefined | Compile from embedded base64 (isomorphicCompileWasmFromBase64) |

Worker creation (how to spawn thread pool workers):

| workerUrl | Result                                    |
|-----------|-------------------------------------------|
| defined   | Direct URL → fetch+blob → embedded base64 |
| undefined | Embedded base64 worker                    |

*/

////////////////////////////////////////////////////////////////////////////////
// initTfheModuleFunction
////////////////////////////////////////////////////////////////////////////////

export type InitTfheModuleFunction = {
  initTfheModule(): Promise<void>;
};

////////////////////////////////////////////////////////////////////////////////
// getTfheModuleInfoFunction
////////////////////////////////////////////////////////////////////////////////

/**
 * Information about the running TFHE module configuration.
 */
export type TfheModuleInfo = {
  /**
   * Number of WASM worker threads.
   * `0` means single-threaded mode.
   */
  readonly numberOfThreads: number;
  /**
   * URL used to fetch the TFHE WASM binary.
   * `undefined` means the base64-embedded fallback is used.
   */
  readonly wasmUrl: URL | undefined;
  /**
   * URL used to load the TFHE worker script.
   * `undefined` means the base64-embedded fallback is used.
   */
  readonly workerUrl: URL | undefined;
  /**
   * Whether the environment supports multi-threading.
   * - `undefined` — user explicitly requested single-threaded mode.
   * - `true` — multi-threading is supported and active.
   * - `false` — multi-threading was requested but unavailable; fell back to single-threaded.
   */
  readonly threadsAvailable: boolean | undefined;
};

export type GetTfheModuleInfoReturnType = TfheModuleInfo | undefined;

export type GetTfheModuleInfoFunction = {
  /**
   * Returns {@link TfheModuleInfo} when the module is initialized,
   * or `undefined` if the module has not completed initialization.
   */
  getTfheModuleInfo(): GetTfheModuleInfoReturnType;
};

////////////////////////////////////////////////////////////////////////////////
// 1. parseTFHEProvenCompactCiphertextList
////////////////////////////////////////////////////////////////////////////////

export type ParseTFHEProvenCompactCiphertextListParameters = {
  readonly ciphertextWithZkProof: Uint8Array | string;
};

export type ParseTFHEProvenCompactCiphertextListReturnType = {
  fheTypeIds: FheTypeId[];
  encryptionBits: EncryptionBits[];
};

export type ParseTFHEProvenCompactCiphertextListModuleFunction = {
  parseTFHEProvenCompactCiphertextList(
    parameters: ParseTFHEProvenCompactCiphertextListParameters,
  ): Promise<ParseTFHEProvenCompactCiphertextListReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 2. buildWithProofPacked
////////////////////////////////////////////////////////////////////////////////

export type BuildWithProofPackedParameters = {
  readonly fheEncryptionKey: FheEncryptionKeyWasm;
  readonly typedValues: TypedValue[];
  readonly metaData: Uint8Array;
  readonly extraData: BytesHex;
};

export type BuildWithProofPackedReturnType = {
  readonly ciphertextWithZKProofBytes: Uint8Array;
  readonly extraData: BytesHex;
};

export type BuildWithProofPackedModuleFunction = {
  buildWithProofPacked(parameters: BuildWithProofPackedParameters): Promise<BuildWithProofPackedReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 3. serializeFheEncryptionKey
////////////////////////////////////////////////////////////////////////////////

export type SerializeFheEncryptionKeyParameters = {
  readonly fheEncryptionKey: FheEncryptionKeyWasm;
};

export type SerializeFheEncryptionKeyReturnType = FheEncryptionKeyBytes;

export type SerializeFheEncryptionKeyModuleFunction = {
  serializeFheEncryptionKey(
    parameters: SerializeFheEncryptionKeyParameters,
  ): Promise<SerializeFheEncryptionKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 4. serializeFheEncryptionPublicKey
////////////////////////////////////////////////////////////////////////////////

export type SerializeFheEncryptionPublicKeyParameters = {
  readonly publicKey: FheEncryptionPublicKey;
};

export type SerializeFheEncryptionPublicKeyReturnType = FheEncryptionPublicKeyBytes;

export type SerializeFheEncryptionPublicKeyModuleFunction = {
  serializeFheEncryptionPublicKey(
    parameters: SerializeFheEncryptionPublicKeyParameters,
  ): Promise<SerializeFheEncryptionPublicKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 5. serializeFheEncryptionCrs
////////////////////////////////////////////////////////////////////////////////

export type SerializeFheEncryptionCrsParameters = {
  readonly crs: FheEncryptionCrs;
};

export type SerializeFheEncryptionCrsReturnType = FheEncryptionCrsBytes;

export type SerializeFheEncryptionCrsModuleFunction = {
  serializeFheEncryptionCrs(
    parameters: SerializeFheEncryptionCrsParameters,
  ): Promise<SerializeFheEncryptionCrsReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 6. deserializeFheEncryptionPublicKey
////////////////////////////////////////////////////////////////////////////////

export type DeserializeFheEncryptionPublicKeyParameters = {
  readonly publicKeyBytes: FheEncryptionPublicKeyBytes;
};

export type DeserializeFheEncryptionPublicKeyReturnType = FheEncryptionPublicKey;

export type DeserializeFheEncryptionPublicKeyModuleFunction = {
  deserializeFheEncryptionPublicKey(
    parameters: DeserializeFheEncryptionPublicKeyParameters,
  ): Promise<DeserializeFheEncryptionPublicKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 6. deserializeFheEncryptionCrs
////////////////////////////////////////////////////////////////////////////////

export type DeserializeFheEncryptionCrsParameters = {
  readonly crsBytes: FheEncryptionCrsBytes;
};

export type DeserializeFheEncryptionCrsReturnType = FheEncryptionCrs;

export type DeserializeFheEncryptionCrsModuleFunction = {
  deserializeFheEncryptionCrs(
    parameters: DeserializeFheEncryptionCrsParameters,
  ): Promise<DeserializeFheEncryptionCrsReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// EncryptModule
////////////////////////////////////////////////////////////////////////////////

export type WithEncryptModule = {
  readonly encrypt: EncryptModule;
};

export type EncryptModule = Prettify<
  InitTfheModuleFunction &
    GetTfheModuleInfoFunction &
    ParseTFHEProvenCompactCiphertextListModuleFunction &
    BuildWithProofPackedModuleFunction &
    SerializeFheEncryptionKeyModuleFunction &
    SerializeFheEncryptionPublicKeyModuleFunction &
    SerializeFheEncryptionCrsModuleFunction &
    DeserializeFheEncryptionPublicKeyModuleFunction &
    DeserializeFheEncryptionCrsModuleFunction
>;

export type EncryptModuleFactory = (runtime: FhevmRuntime) => {
  readonly encrypt: EncryptModule;
};
