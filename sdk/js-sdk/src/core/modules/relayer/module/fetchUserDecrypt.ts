import type { FetchUserDecryptParameters, FetchUserDecryptReturnType, RelayerClientWithRuntime } from '../types.js';
import { fetchUserDecryptV1 } from './fetchUserDecryptV1.js';
import { fetchUserDecryptV2 } from './fetchUserDecryptV2.js';

//////////////////////////////////////////////////////////////////////////////
// fetchUserDecrypt
//////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchUserDecryptParameters,
): Promise<FetchUserDecryptReturnType> {
  if (parameters.version === 1) {
    return await fetchUserDecryptV1(relayerClient, parameters);
  }
  return await fetchUserDecryptV2(relayerClient, parameters);
}
