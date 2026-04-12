import type { FhevmRuntime } from '../../../types/coreFhevmRuntime.js';
import type {
  DecryptAndReconstructParameters,
  DecryptAndReconstructUserParameters,
  DecryptModuleFactory,
  DeserializeTkmsPrivateKeyParameters,
  GetTkmsPublicKeyHexParameters,
  SerializeTkmsPrivateKeyParameters,
  UserDecryptModuleFactory,
  UserDecryptModuleParameters,
  VerifyTkmsPrivateKeyParameters,
} from '../types.js';
import {
  decryptAndReconstruct,
  deserializeTkmsPrivateKey,
  generateTkmsPrivateKey,
  getTkmsPublicKeyHex,
  serializeTkmsPrivateKey,
  verifyTkmsPrivateKey,
} from './api-p.js';
import { getTkmsModuleInfo, initTkmsModule } from './init-p.js';

//////////////////////////////////////////////////////////////////////////////
// decryptModule
//////////////////////////////////////////////////////////////////////////////

export const decryptModule: DecryptModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    decrypt: Object.freeze({
      initTkmsModule: () => initTkmsModule(runtime),
      getTkmsModuleInfo: () => getTkmsModuleInfo(),
      decryptAndReconstruct: (args: DecryptAndReconstructParameters) =>
        decryptAndReconstruct(runtime, args),
      generateTkmsPrivateKey: () => generateTkmsPrivateKey(runtime),
      serializeTkmsPrivateKey: (args: SerializeTkmsPrivateKeyParameters) =>
        serializeTkmsPrivateKey(runtime, args),
      deserializeTkmsPrivateKey: (args: DeserializeTkmsPrivateKeyParameters) =>
        deserializeTkmsPrivateKey(runtime, args),
      getTkmsPublicKeyHex: (args: GetTkmsPublicKeyHexParameters) =>
        getTkmsPublicKeyHex(runtime, args),
      verifyTkmsPrivateKey: (args: VerifyTkmsPrivateKeyParameters) => {
        verifyTkmsPrivateKey(runtime, args);
      },
    }),
  });
};

//////////////////////////////////////////////////////////////////////////////
// userDecryptModule
//////////////////////////////////////////////////////////////////////////////

export const userDecryptModule: UserDecryptModuleFactory = (
  runtime: FhevmRuntime,
  parameters: UserDecryptModuleParameters,
) => {
  const { privateKey } = parameters;
  return Object.freeze({
    userDecrypt: Object.freeze({
      initTkmsModule: () => initTkmsModule(runtime),
      getTkmsModuleInfo: () => getTkmsModuleInfo(),
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
