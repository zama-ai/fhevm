/* eslint-disable @typescript-eslint/require-await */
import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type {
  DecryptAndReconstructParameters,
  DecryptAndReconstructReturnType,
  DecryptAndReconstructUserParameters,
  DeserializeTkmsPrivateKeyParameters,
  DeserializeTkmsPrivateKeyReturnType,
  GenerateTkmsPrivateKeyReturnType,
  GetTkmsPublicKeyHexParameters,
  GetTkmsPublicKeyHexReturnType,
  SerializeTkmsPrivateKeyParameters,
  SerializeTkmsPrivateKeyReturnType,
  VerifyTkmsPrivateKeyParameters,
  UserDecryptModuleParameters,
  DecryptModuleFactory,
  UserDecryptModuleFactory,
} from './types.js';

////////////////////////////////////////////////////////////////////////////////
// decryptAndReconstruct
////////////////////////////////////////////////////////////////////////////////

export async function decryptAndReconstruct(
  _runtime: FhevmRuntime,
  _parameters: DecryptAndReconstructParameters,
): Promise<DecryptAndReconstructReturnType> {
  throw new Error('Not yet implemented');
}

//////////////////////////////////////////////////////////////////////////////
// generateTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function generateTkmsPrivateKey(
  _runtime: FhevmRuntime,
): Promise<GenerateTkmsPrivateKeyReturnType> {
  throw new Error('Not yet implemented');
}

//////////////////////////////////////////////////////////////////////////////
// getTkmsPublicKeyHex
//////////////////////////////////////////////////////////////////////////////

export async function getTkmsPublicKeyHex(
  _runtime: FhevmRuntime,
  _parameters: GetTkmsPublicKeyHexParameters,
): Promise<GetTkmsPublicKeyHexReturnType> {
  throw new Error('Not yet implemented');
}

//////////////////////////////////////////////////////////////////////////////
// serializeTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function serializeTkmsPrivateKey(
  _runtime: FhevmRuntime,
  _parameters: SerializeTkmsPrivateKeyParameters,
): Promise<SerializeTkmsPrivateKeyReturnType> {
  throw new Error('Not yet implemented');
}

//////////////////////////////////////////////////////////////////////////////
// deserializeTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export async function deserializeTkmsPrivateKey(
  _runtime: FhevmRuntime,
  _parameters: DeserializeTkmsPrivateKeyParameters,
): Promise<DeserializeTkmsPrivateKeyReturnType> {
  throw new Error('Not yet implemented');
}

//////////////////////////////////////////////////////////////////////////////
// verifyTkmsPrivateKey
//////////////////////////////////////////////////////////////////////////////

export function verifyTkmsPrivateKey(
  _runtime: FhevmRuntime,
  _parameters: VerifyTkmsPrivateKeyParameters,
): void {
  throw new Error('Not yet implemented');
}

//////////////////////////////////////////////////////////////////////////////
// decryptActions
//////////////////////////////////////////////////////////////////////////////

export const decryptModule: DecryptModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    decrypt: Object.freeze({
      initTkmsModule: () => Promise.resolve(),
      getTkmsModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      generateTkmsPrivateKey: () => generateTkmsPrivateKey(runtime),
      decryptAndReconstruct: (args: DecryptAndReconstructParameters) =>
        decryptAndReconstruct(runtime, args),
      serializeTkmsPrivateKey: (args: SerializeTkmsPrivateKeyParameters) =>
        serializeTkmsPrivateKey(runtime, args),
      deserializeTkmsPrivateKey: (args: DeserializeTkmsPrivateKeyParameters) =>
        deserializeTkmsPrivateKey(runtime, args),
      verifyTkmsPrivateKey: (args: VerifyTkmsPrivateKeyParameters) => {
        verifyTkmsPrivateKey(runtime, args);
      },
      getTkmsPublicKeyHex: (args: GetTkmsPublicKeyHexParameters) =>
        getTkmsPublicKeyHex(runtime, args),
    }),
  });
};

//////////////////////////////////////////////////////////////////////////////
// userDecryptActions
//////////////////////////////////////////////////////////////////////////////

export const userDecryptModule: UserDecryptModuleFactory = (
  runtime: FhevmRuntime,
  parameters: UserDecryptModuleParameters,
) => {
  const { privateKey } = parameters;
  return Object.freeze({
    userDecrypt: Object.freeze({
      initTkmsModule: () => Promise.resolve(),
      getTkmsModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      decryptAndReconstruct: (args: DecryptAndReconstructUserParameters) =>
        decryptAndReconstruct(runtime, {
          ...args,
          tkmsPrivateKey: privateKey,
        }),
      getTkmsPublicKeyHex: () =>
        getTkmsPublicKeyHex(runtime, {
          tkmsPrivateKey: privateKey,
        }),
      serializeTkmsPrivateKey: () =>
        serializeTkmsPrivateKey(runtime, {
          tkmsPrivateKey: privateKey,
        }),
    }),
  });
};
