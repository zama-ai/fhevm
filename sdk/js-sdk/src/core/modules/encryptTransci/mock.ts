import type { FhevmRuntime } from '../../types/coreFhevmRuntime.js';
import type { GetTkmsPublicKeyHexParameters, SerializeTkmsPrivateKeyParameters } from '../decrypt/types.js';
import type { EncryptTransciModuleFactory, ReconstructPadsModuleParameters } from './types.js';
import { reconstructPads } from '../../kms/reconstructPads-p.js';
import { generateTkmsPrivateKey, getTkmsPublicKeyHex, serializeTkmsPrivateKey } from '../decrypt/mock.js';

//////////////////////////////////////////////////////////////////////////////
// encryptTransciModule (cleartext)
//
// No TKMS WASM: the cleartext relayer returns cleartext-encoded pad shares.
// Transport keys are the cleartext decrypt mock's keys, so a key pair
// generated here is interchangeable with one from the cleartext decrypt module.
//////////////////////////////////////////////////////////////////////////////

export const encryptTransciModule: EncryptTransciModuleFactory = (runtime: FhevmRuntime) => {
  return Object.freeze({
    encryptTransci: Object.freeze({
      initTkmsModule: () => Promise.resolve(),
      getTkmsModuleInfo: () => {
        throw new Error('Not yet implemented');
      },
      reconstructPads: (args: ReconstructPadsModuleParameters) => reconstructPads(runtime.ethereum, args),
      generateTkmsPrivateKey: () => generateTkmsPrivateKey(runtime),
      serializeTkmsPrivateKey: (args: SerializeTkmsPrivateKeyParameters) => serializeTkmsPrivateKey(runtime, args),
      getTkmsPublicKeyHex: (args: GetTkmsPublicKeyHexParameters) => getTkmsPublicKeyHex(runtime, args),
    }),
  });
};
