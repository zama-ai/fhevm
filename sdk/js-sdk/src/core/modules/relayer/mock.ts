import type {
  FetchCoprocessorSignaturesParameters,
  FetchDelegatedUserDecryptParameters,
  FetchFheEncryptionKeySourceParameters,
  FetchPublicDecryptParameters,
  FetchUserDecryptParameters,
  RelayerClient,
  RelayerModuleFactory,
} from './types.js';

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
        _relayerClient: RelayerClient,
        _parameters: FetchFheEncryptionKeySourceParameters,
      ) => {
        throw new Error('Not yet implemented');
      },
      fetchCoprocessorSignatures: (
        _relayerClient: RelayerClient,
        _parameters: FetchCoprocessorSignaturesParameters,
      ) => {
        throw new Error('Not yet implemented');
      },
      fetchPublicDecrypt: (
        _relayerClient: RelayerClient,
        _parameters: FetchPublicDecryptParameters,
      ) => {
        throw new Error('Not yet implemented');
      },
      fetchUserDecrypt: (
        _relayerClient: RelayerClient,
        _parameters: FetchUserDecryptParameters,
      ) => {
        throw new Error('Not yet implemented');
      },
      fetchDelegatedUserDecrypt: (
        _relayerClient: RelayerClient,
        _parameters: FetchDelegatedUserDecryptParameters,
      ) => {
        throw new Error('Not yet implemented');
      },
    }),
  });
};
