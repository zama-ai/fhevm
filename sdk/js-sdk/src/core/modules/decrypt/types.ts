import type { TkmsPrivateKey } from '../../types/tkms-p.js';
import type { KmsSigncryptedShares } from '../../types/kms.js';
import type { Bytes, BytesHex } from '../../types/primitives.js';
import type { Prettify } from '../../types/utils.js';
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { ClearValue } from '../../types/encryptedTypes.js';

////////////////////////////////////////////////////////////////////////////////
//
// DecryptModule
//
////////////////////////////////////////////////////////////////////////////////

/*

WASM compilation (how to get WebAssembly.Module):

| wasmUrl   | Result                                                         |
|-----------|----------------------------------------------------------------|
| defined   | Compile from URL (isomorphicCompileWasm)                       |
| undefined | Compile from embedded base64 (isomorphicCompileWasmFromBase64) |

*/

////////////////////////////////////////////////////////////////////////////////
// initTkmsModule
////////////////////////////////////////////////////////////////////////////////

export type InitTkmsModuleFunction = {
  initTkmsModule(): Promise<void>;
};

////////////////////////////////////////////////////////////////////////////////
// getTkmsModuleInfoFunction
////////////////////////////////////////////////////////////////////////////////

/**
 * Information about the running TKMS module configuration.
 */
export type TkmsModuleInfo = {
  /**
   * URL used to fetch the TFHE WASM binary.
   * `undefined` means the base64-embedded fallback is used.
   */
  readonly wasmUrl: URL | undefined;
};

export type GetTkmsModuleInfoReturnType = TkmsModuleInfo | undefined;

export type GetTkmsModuleInfoFunction = {
  /**
   * Returns {@link TkmsModuleInfo} when the module is initialized,
   * or `undefined` if the module has not completed initialization.
   */
  getTkmsModuleInfo(): GetTkmsModuleInfoReturnType;
};
////////////////////////////////////////////////////////////////////////////////

type WithTkmsPrivateKey = {
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
// 1. (User) decryptAndReconstruct
////////////////////////////////////////////////////////////////////////////////

export type DecryptAndReconstructUserParameters = DecryptAndReconstructBaseParameters;

export type DecryptAndReconstructUserModuleFunction = {
  decryptAndReconstruct(parameters: DecryptAndReconstructUserParameters): Promise<DecryptAndReconstructReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 2. generateTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type GenerateTkmsPrivateKeyReturnType = TkmsPrivateKey;

export type GenerateTkmsPrivateKeyModuleFunction = {
  generateTkmsPrivateKey(): Promise<GenerateTkmsPrivateKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 3 getTkmsPublicKeyHex
////////////////////////////////////////////////////////////////////////////////

export type GetTkmsPublicKeyHexParameters = WithTkmsPrivateKey;
export type GetTkmsPublicKeyHexReturnType = BytesHex;

export type GetTkmsPublicKeyHexModuleFunction = {
  getTkmsPublicKeyHex(parameters: GetTkmsPublicKeyHexParameters): Promise<GetTkmsPublicKeyHexReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 3. (User) getTkmsPublicKeyHex
////////////////////////////////////////////////////////////////////////////////

export type GetTkmsPublicKeyHexUserModuleFunction = {
  getTkmsPublicKeyHex(): Promise<GetTkmsPublicKeyHexReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 4. serializeTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type SerializeTkmsPrivateKeyParameters = {
  readonly tkmsPrivateKey: TkmsPrivateKey;
};

export type SerializeTkmsPrivateKeyReturnType = Bytes;

export type SerializeTkmsPrivateKeyModuleFunction = {
  serializeTkmsPrivateKey(parameters: SerializeTkmsPrivateKeyParameters): Promise<SerializeTkmsPrivateKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 4. (User) serializeTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type SerializeTkmsPrivateKeyUserModuleFunction = {
  serializeTkmsPrivateKey(): Promise<SerializeTkmsPrivateKeyReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 5. deserializeTkmsPrivateKey
////////////////////////////////////////////////////////////////////////////////

export type DeserializeTkmsPrivateKeyParameters = {
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

export type VerifyTkmsPrivateKeyParameters = {
  readonly tkmsPrivateKey: TkmsPrivateKey;
};

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

////////////////////////////////////////////////////////////////////////////////
// UserDecryptModule
////////////////////////////////////////////////////////////////////////////////

export type WithUserDecryptModule = {
  readonly userDecrypt: UserDecryptModule;
};

export type UserDecryptModuleParameters = {
  readonly privateKey: TkmsPrivateKey;
};

export type UserDecryptModule = Prettify<
  InitTkmsModuleFunction &
    DecryptAndReconstructUserModuleFunction &
    GetTkmsPublicKeyHexUserModuleFunction &
    SerializeTkmsPrivateKeyUserModuleFunction
>;

export type UserDecryptModuleFactory = (
  runtime: FhevmRuntime,
  parameters: UserDecryptModuleParameters,
) => {
  readonly userDecrypt: UserDecryptModule;
};
