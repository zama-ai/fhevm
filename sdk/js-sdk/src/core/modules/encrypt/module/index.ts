import type { FhevmRuntime } from '../../../types/coreFhevmRuntime.js';
import type {
  BuildWithProofPackedParameters,
  DeserializeFheEncryptionCrsParameters,
  DeserializeFheEncryptionPublicKeyParameters,
  EncryptModuleFactory,
  GetTfheModuleInfoParameters,
  ParseTFHEProvenCompactCiphertextListParameters,
} from '../types.js';
import {
  buildWithProofPacked,
  deserializeFheEncryptionCrs,
  deserializeFheEncryptionPublicKey,
  parseTFHEProvenCompactCiphertextList,
} from './api-p.js';
import { getTfheModuleInfo, initTfheModule } from './init-p.js';

////////////////////////////////////////////////////////////////////////////////
// encryptModule
////////////////////////////////////////////////////////////////////////////////

export const encryptModule: EncryptModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    encrypt: Object.freeze({
      initTfheModule: async () => {
        await initTfheModule(runtime);
      },
      getTfheModuleInfo: (args: GetTfheModuleInfoParameters) => getTfheModuleInfo(args),
      parseTFHEProvenCompactCiphertextList: (args: ParseTFHEProvenCompactCiphertextListParameters) =>
        parseTFHEProvenCompactCiphertextList(runtime, args),
      buildWithProofPacked: (args: BuildWithProofPackedParameters) => buildWithProofPacked(runtime, args),
      deserializeFheEncryptionPublicKey: (args: DeserializeFheEncryptionPublicKeyParameters) =>
        deserializeFheEncryptionPublicKey(runtime, args),
      deserializeFheEncryptionCrs: (args: DeserializeFheEncryptionCrsParameters) =>
        deserializeFheEncryptionCrs(runtime, args),
    }),
  });
};
