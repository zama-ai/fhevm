import type { Bytes65Hex } from '../../../types/primitives.js';
import type { RelayerFetchPublicDecryptPayload } from '../../../types/relayer-p.js';
import type { FetchPublicDecryptParameters, FetchPublicDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { ensure0x } from '../../../base/string.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';
import { RelayerHttpRequest } from './RelayerHttpRequest.js';

//////////////////////////////////////////////////////////////////////////////
// fetchPublicDecryptNoGateway
//////////////////////////////////////////////////////////////////////////////

export async function fetchPublicDecryptNoGateway(
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
  const url = buildRelayerUrlString(relayerBaseUrl, 'v4/public-decrypt');

  const request = new RelayerHttpRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url,
    payload: p,
    options,
    logger: relayerClient.runtime.config.logger,
  });

  // TODO !!!
  const result = await request.run();

  return {
    orderedAbiEncodedClearValues: ensure0x(result.decryptedValue),
    kmsPublicDecryptEip712Signatures: result.signatures.map(ensure0x) as Bytes65Hex[],
    extraData: result.extraData,
  };
}
