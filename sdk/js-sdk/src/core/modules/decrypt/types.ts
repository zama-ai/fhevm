import type { TkmsPrivateKey } from '../../types/tkms-p.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { Bytes, BytesHex } from '../../types/primitives.js';
import type { Prettify } from '../../types/utils.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { ClearValue } from '../../types/encryptedTypes-p.js';
import type { TkmsVersion } from '../../../wasm/tkms/KmsLibApi.js';

////////////////////////////////////////////////////////////////////////////////
//
// DecryptModule
//
////////////////////////////////////////////////////////////////////////////////

/*

WASM compilation (how to get WebAssembly.Module):

| wasmUrl   | Result                                                         |
|-----------|----------------------------------------------------------------|
| defined   | Verify and compile from URL (isomorphicCompileVerifiedWasm)    |
| undefined | Compile from embedded base64 (isomorphicCompileWasmFromBase64) |

*/

type WithTkmsVersion = {
  readonly tkmsVersion: TkmsVersion;
};

////////////////////////////////////////////////////////////////////////////////
// initTkmsModule
////////////////////////////////////////////////////////////////////////////////

export type InitTkmsModuleParameters = {
  readonly tkmsVersion: TkmsVersion;
};

export type InitTkmsModuleFunction = {
  initTkmsModule(parameters: InitTkmsModuleParameters): Promise<void>;
};

////////////////////////////////////////////////////////////////////////////////
// getTkmsModuleInfoFunction
////////////////////////////////////////////////////////////////////////////////

/**
 * Information about the running TKMS module configuration.
 */
export type TkmsModuleInfo = {
  /**
   * URL used to fetch the TKMS WASM binary.
   * `undefined` means the base64-embedded fallback is used.
   */
  readonly wasmUrl: URL | undefined;

  /**
   * Current size of the TKMS WASM linear memory.
   * This is the size of the underlying `WebAssembly.Memory` buffer, not live heap usage.
   */
  readonly memory: {
    /**
     * Current memory buffer size in bytes.
     */
    readonly byteLength: number;
    /**
     * Current memory buffer size in 64 KiB WASM pages.
     */
    readonly pages: number;
  };
};

export type GetTkmsModuleInfoParameters = WithTkmsVersion;

export type GetTkmsModuleInfoReturnType = TkmsModuleInfo | undefined;

export type GetTkmsModuleInfoFunction = {
  /**
   * Returns {@link TkmsModuleInfo} when the module is initialized,
   * or `undefined` if the module has not completed initialization.
   */
  getTkmsModuleInfo(parameters: GetTkmsModuleInfoParameters): Promise<GetTkmsModuleInfoReturnType>;
};

////////////////////////////////////////////////////////////////////////////////

type WithTkmsPrivateKey = WithTkmsVersion & {
  readonly tkmsPrivateKey: TkmsPrivateKey;
};

////////////////////////////////////////////////////////////////////////////////
// 1. decryptAndReconstruct
////////////////////////////////////////////////////////////////////////////////

type DecryptAndReconstructBaseParameters = {
  readonly shares: KmsSigncryptedShares;
};

export type DecryptAndReconstructParameters = WithTkmsPrivateKey & DecryptAndReconstructBaseParameters;
export type DecryptAndReconstructReturnType = readonly ClearValue[];

export type DecryptAndReconstructModuleFunction = {
  decryptAndReconstruct(parameters: DecryptAndReconstructParameters): Promise<DecryptAndReconstructReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 2. generateTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type GenerateTkmsPrivateKeyParameters = WithTkmsVersion;
export type GenerateTkmsPrivateKeyReturnType = TkmsPrivateKey;

export type GenerateTkmsPrivateKeyModuleFunction = {
  generateTkmsPrivateKey(parameters: GenerateTkmsPrivateKeyParameters): Promise<GenerateTkmsPrivateKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 3 getTkmsPublicKeyHex
////////////////////////////////////////////////////////////////////////////////

export type GetTkmsPublicKeyHexParameters = WithTkmsPrivateKey & WithTkmsVersion;
export type GetTkmsPublicKeyHexReturnType = BytesHex;

export type GetTkmsPublicKeyHexModuleFunction = {
  getTkmsPublicKeyHex(parameters: GetTkmsPublicKeyHexParameters): Promise<GetTkmsPublicKeyHexReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 4. serializeTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type SerializeTkmsPrivateKeyParameters = WithTkmsPrivateKey;

export type SerializeTkmsPrivateKeyReturnType = Bytes;

export type SerializeTkmsPrivateKeyModuleFunction = {
  serializeTkmsPrivateKey(parameters: SerializeTkmsPrivateKeyParameters): Promise<SerializeTkmsPrivateKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 5. deserializeTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type DeserializeTkmsPrivateKeyParameters = WithTkmsVersion & {
  readonly tkmsPrivateKeyBytes: Bytes;
};

export type DeserializeTkmsPrivateKeyReturnType = TkmsPrivateKey;

export type DeserializeTkmsPrivateKeyModuleFunction = {
  deserializeTkmsPrivateKey(
    parameters: DeserializeTkmsPrivateKeyParameters,
  ): Promise<DeserializeTkmsPrivateKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 6. verifyTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type VerifyTkmsPrivateKeyParameters = WithTkmsVersion & WithTkmsPrivateKey;

export type VerifyTkmsPrivateKeyModuleFunction = {
  verifyTkmsPrivateKey(parameters: VerifyTkmsPrivateKeyParameters): void;
};

////////////////////////////////////////////////////////////////////////////////
// DecryptModule
////////////////////////////////////////////////////////////////////////////////

export type WithDecryptModule = {
  readonly decrypt: DecryptModule;
};

export type DecryptModule = Prettify<
  InitTkmsModuleFunction &
    GetTkmsModuleInfoFunction &
    DecryptAndReconstructModuleFunction &
    GetTkmsPublicKeyHexModuleFunction &
    GenerateTkmsPrivateKeyModuleFunction &
    SerializeTkmsPrivateKeyModuleFunction &
    DeserializeTkmsPrivateKeyModuleFunction &
    VerifyTkmsPrivateKeyModuleFunction
>;

export type DecryptModuleFactory = (runtime: FhevmRuntime) => {
  readonly decrypt: DecryptModule;
};
