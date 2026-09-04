import type { FhevmRuntime } from '../../../types/coreFhevmRuntime.js';
import type {
  DecryptAndReconstructParameters,
  DecryptModuleFactory,
  DeserializeTkmsPrivateKeyParameters,
  GenerateTkmsPrivateKeyParameters,
  GetTkmsModuleInfoParameters,
  GetTkmsPublicKeyHexParameters,
  InitTkmsModuleParameters,
  SerializeTkmsPrivateKeyParameters,
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
      initTkmsModule: async (args: InitTkmsModuleParameters) => {
        await initTkmsModule(runtime, args);
      },
      getTkmsModuleInfo: (args: GetTkmsModuleInfoParameters) => getTkmsModuleInfo(args),
      decryptAndReconstruct: (args: DecryptAndReconstructParameters) => decryptAndReconstruct(runtime, args),
      generateTkmsPrivateKey: (args: GenerateTkmsPrivateKeyParameters) => generateTkmsPrivateKey(runtime, args),
      serializeTkmsPrivateKey: (args: SerializeTkmsPrivateKeyParameters) => serializeTkmsPrivateKey(runtime, args),
      deserializeTkmsPrivateKey: (args: DeserializeTkmsPrivateKeyParameters) =>
        deserializeTkmsPrivateKey(runtime, args),
      getTkmsPublicKeyHex: (args: GetTkmsPublicKeyHexParameters) => getTkmsPublicKeyHex(runtime, args),
      verifyTkmsPrivateKey: (args: VerifyTkmsPrivateKeyParameters) => {
        verifyTkmsPrivateKey(runtime, args);
      },
    }),
  });
};
