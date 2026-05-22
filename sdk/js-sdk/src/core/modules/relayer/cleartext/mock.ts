import type { UintNumber } from '../../../types/primitives.js';
import type {
  FetchFheEncryptionKeyBytesParameters,
  FetchFheEncryptionKeyBytesReturnType,
  FetchFheEncryptionKeySourceParameters,
  RelayerClient,
  RelayerModuleFactory,
} from '../types.js';
import { createDeadbeefBytes } from '../../../base/bytes.js';
import { fetchUserDecrypt } from './fetchUserDecrypt.js';
import { fetchPublicDecrypt } from './fetchPublicDecrypt.js';
import { fetchDelegatedUserDecrypt } from './fetchDelegatedUserDecrypt.js';
import { fetchCoprocessorSignatures } from './fetchCoprocessorSignatures.js';

////////////////////////////////////////////////////////////////////////////////
// relayerModule
////////////////////////////////////////////////////////////////////////////////

export const relayerModule: RelayerModuleFactory = () => {
  return Object.freeze({
    relayer: Object.freeze({
      fetchFheEncryptionKeySource: (
        _relayerClient: RelayerClient,
        _parameters: FetchFheEncryptionKeySourceParameters,
      ) => {
        throw new Error('Not yet implemented');
      },
      fetchFheEncryptionKeyBytes: (
        relayerClient: RelayerClient,
        _parameters: FetchFheEncryptionKeyBytesParameters,
      ): Promise<FetchFheEncryptionKeyBytesReturnType> => {
        return Promise.resolve({
          publicKeyBytes: {
            id: 'fhe-public-key-data-id',
            bytes: createDeadbeefBytes(256),
          },
          crsBytes: {
            bytes: createDeadbeefBytes(256),
            capacity: 2048 as UintNumber,
            id: 'crs-data-id',
          },
          metadata: {
            relayerUrl: relayerClient.relayerUrl,
            chainId: relayerClient.chainId,
          },
        });
      },
      fetchCoprocessorSignatures,
      fetchPublicDecrypt,
      fetchUserDecrypt,
      fetchDelegatedUserDecrypt,
    }),
  });
};
