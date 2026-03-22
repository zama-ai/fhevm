/*
 * ============================================================================
 * Naming convention
 * ============================================================================
 *
 * Standard (no user variant):
 *
 *   type [FuncName]Parameters  = ...
 *   type [FuncName]ReturnType  = ...
 *
 *   [FuncName]ModuleFunction           [funcName](parameters)      => Promise<ReturnType>
 *
 * With user variant (privateKey in closure vs explicit):
 *
 *   type [FuncName]Parameters  = ... (core params, no privateKey)
 *   type [FuncName]ReturnType  = ...
 *
 *   User (FhevmUserClient — privateKey bound in closure):
 *   [FuncName]UserModuleFunction       [funcName](parameters: [FuncName]Parameters)
 *
 *   Standalone (privateKey explicit):
 *   [FuncName]ModuleFunction           [funcName](parameters: WithTkmsPrivateKey & [FuncName]Parameters)
 *
 * ============================================================================
 */

import type { TkmsPrivateKey } from "../../types/tkms-p.js";
import type { KmsSigncryptedShares } from "../../types/kms.js";
import type { DecryptedFhevmHandle } from "../../types/decryptedFhevmHandle.js";
import type { Bytes, BytesHex } from "../../types/primitives.js";
import type { Prettify } from "../../types/utils.js";
import type { Logger } from "../../types/logger.js";
import type { FhevmRuntime } from "../../types/coreFhevmRuntime.js";

////////////////////////////////////////////////////////////////////////////////
//
// DecryptModule
//
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// setTkmsModuleConfig
////////////////////////////////////////////////////////////////////////////////

/*
WASM compilation (how to get WebAssembly.Module):

| wasmUrl   | Result                                                         |
|-----------|----------------------------------------------------------------|
| defined   | Compile from URL (isomorphicCompileWasm)                       |
| undefined | Compile from embedded base64 (isomorphicCompileWasmFromBase64) |

*/

export type TkmsModuleConfig = {
  readonly locateFile?: ((file: string) => URL) | undefined;
  readonly logger?: Logger | undefined;
};

////////////////////////////////////////////////////////////////////////////////
// initTkmsModule
////////////////////////////////////////////////////////////////////////////////

export type InitTkmsModuleFunction = {
  initTkmsModule(): Promise<void>;
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

export type DecryptAndReconstructParameters = WithTkmsPrivateKey &
  DecryptAndReconstructBaseParameters;
export type DecryptAndReconstructReturnType = readonly DecryptedFhevmHandle[];

export type DecryptAndReconstructModuleFunction = {
  decryptAndReconstruct(
    parameters: DecryptAndReconstructParameters,
  ): Promise<DecryptAndReconstructReturnType>;
};

////////////////////////////////////////////////////////////////////////////////
// 1. (User) decryptAndReconstruct
////////////////////////////////////////////////////////////////////////////////

export type DecryptAndReconstructUserParameters =
  DecryptAndReconstructBaseParameters;

export type DecryptAndReconstructUserModuleFunction = {
  decryptAndReconstruct(
    parameters: DecryptAndReconstructUserParameters,
  ): Promise<DecryptAndReconstructReturnType>;
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
  getTkmsPublicKeyHex(
    parameters: GetTkmsPublicKeyHexParameters,
  ): Promise<GetTkmsPublicKeyHexReturnType>;
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
  serializeTkmsPrivateKey(
    parameters: SerializeTkmsPrivateKeyParameters,
  ): Promise<SerializeTkmsPrivateKeyReturnType>;
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
