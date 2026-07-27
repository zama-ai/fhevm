import type { Bytes65Hex } from '../../../types/primitives.js';
import type { RelayerFetchPublicDecryptPayload } from '../../../types/relayer-p.js';
import type { FetchPublicDecryptResult } from '../../../types/relayer.js';
import type { FetchPublicDecryptParameters, FetchPublicDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { ensure0x } from '../../../base/string.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';

//////////////////////////////////////////////////////////////////////////////
// fetchPublicDecrypt
//////////////////////////////////////////////////////////////////////////////

export async function fetchPublicDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchPublicDecryptParameters,
): Promise<FetchPublicDecryptReturnType> {
  const { options, payload } = parameters;

  // Convert payload argument to relayer payload
  const p: RelayerFetchPublicDecryptPayload = {
    ciphertextHandles: payload.orderedHandles.map((h) => {
      return h.bytes32Hex;
    }),
    extraData: payload.extraData,
  };

  const hasAuth: boolean = options?.auth !== undefined;
  const relayerBaseUrl: URL = validateRelayerBaseUrl(relayerClient.chain.fhevm.relayerUrl, hasAuth);
  const url = buildRelayerUrlString(relayerBaseUrl, 'v2/public-decrypt');

  const request = new RelayerAsyncRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url,
    payload: p,
    options,
  });

  const result = (await request.run()) as FetchPublicDecryptResult;

  return {
    orderedAbiEncodedClearValues: ensure0x(result.decryptedValue),
    kmsPublicDecryptEip712Signatures: result.signatures.map(ensure0x) as Bytes65Hex[],
    extraData: result.extraData,
  };
}
