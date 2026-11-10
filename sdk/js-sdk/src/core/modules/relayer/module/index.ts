import type { RelayerModuleFactory } from '../types.js';
import { fetchCoprocessorSignatures } from './fetchCoprocessorSignatures.js';
import { fetchDelegatedUserDecrypt } from './fetchDelegatedUserDecryptV1.js';
import { fetchPublicDecrypt } from './fetchPublicDecrypt.js';
import { fetchFheEncryptionKeyBytes } from './fetchFheEncryptionKeyBytes.js';
import { fetchFheEncryptionKeySource } from './fetchFheEncryptionKeySource.js';
import { fetchUserDecrypt } from './fetchUserDecrypt.js';
import { fetchFeatures } from './fetchFeatures.js';
import { fetchPadShares } from './fetchPadShares.js';
import { fetchUserDecryptNoGateway } from './fetchUserDecryptNoGateway.js';
import { fetchPublicDecryptNoGateway } from './fetchPublicDecryptNoGateway.js';

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
      fetchPublicDecryptNoGateway,
      fetchUserDecrypt,
      fetchUserDecryptNoGateway,
      fetchDelegatedUserDecrypt,
      fetchFeatures,
      fetchPadShares,
    }),
  });
};
