import type { FhevmRuntime } from '../../../types/coreFhevmRuntime.js';
import type {
  BuildWithProofPackedReturnTypeParameters,
  DeserializeFheEncryptionCrsParameters,
  DeserializeFheEncryptionPublicKeyParameters,
  EncryptModuleFactory,
  ParseTFHEProvenCompactCiphertextListParameters,
  SerializeFheEncryptionCrsParameters,
  SerializeFheEncryptionKeyParameters,
  SerializeFheEncryptionPublicKeyParameters,
} from '../types.js';
import {
  buildWithProofPacked,
  deserializeFheEncryptionCrs,
  deserializeFheEncryptionPublicKey,
  parseTFHEProvenCompactCiphertextList,
  serializeFheEncryptionCrs,
  serializeFheEncryptionKey,
  serializeFheEncryptionPublicKey,
} from './api-p.js';
import { getTfheModuleInfo, initTfheModule } from './init-p.js';

////////////////////////////////////////////////////////////////////////////////
// encryptModule
////////////////////////////////////////////////////////////////////////////////

export const encryptModule: EncryptModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    encrypt: Object.freeze({
      initTfheModule: () => initTfheModule(runtime),
      getTfheModuleInfo: () => getTfheModuleInfo(),
      parseTFHEProvenCompactCiphertextList: (args: ParseTFHEProvenCompactCiphertextListParameters) =>
        parseTFHEProvenCompactCiphertextList(runtime, args),
      buildWithProofPacked: (args: BuildWithProofPackedReturnTypeParameters) => buildWithProofPacked(runtime, args),
      serializeFheEncryptionKey: (args: SerializeFheEncryptionKeyParameters) =>
        serializeFheEncryptionKey(runtime, args),
      serializeFheEncryptionPublicKey: (args: SerializeFheEncryptionPublicKeyParameters) =>
        serializeFheEncryptionPublicKey(runtime, args),
      serializeFheEncryptionCrs: (args: SerializeFheEncryptionCrsParameters) =>
        serializeFheEncryptionCrs(runtime, args),
      deserializeFheEncryptionPublicKey: (args: DeserializeFheEncryptionPublicKeyParameters) =>
        deserializeFheEncryptionPublicKey(runtime, args),
      deserializeFheEncryptionCrs: (args: DeserializeFheEncryptionCrsParameters) =>
        deserializeFheEncryptionCrs(runtime, args),
    }),
  });
};
