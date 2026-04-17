import type { RelayerModuleFactory } from '../types.js';
import { fetchCoprocessorSignatures } from './fetchCoprocessorSignatures.js';
import { fetchDelegatedUserDecrypt } from './fetchDelegatedUserDecrypt.js';
import { fetchPublicDecrypt } from './fetchPublicDecrypt.js';
import { fetchFheEncryptionKeyBytes } from './fetchFheEncryptionKeyBytes.js';
import { fetchFheEncryptionKeySource } from './fetchFheEncryptionKeySource.js';
import { fetchUserDecrypt } from './fetchUserDecrypt.js';

////////////////////////////////////////////////////////////////////////////////
// relayerModule
////////////////////////////////////////////////////////////////////////////////

export const relayerModule: RelayerModuleFactory = () => {
  return Object.freeze({
    relayer: Object.freeze({
      fetchFheEncryptionKeySource,
      fetchFheEncryptionKeyBytes,
      fetchCoprocessorSignatures,
      fetchPublicDecrypt,
      fetchUserDecrypt,
      fetchDelegatedUserDecrypt,
    }),
  });
};
