import type {
  FetchUserDecryptParameters,
  FetchUserDecryptParametersV1,
  FetchUserDecryptParametersV2,
  FetchUserDecryptReturnType,
  RelayerClientWithRuntime,
} from '../types.js';
import { getResolvedProtocolVersion } from '../../../runtime/CoreFhevm-p.js';
import { isSemverStrictlyBefore } from '../../../base/semver.js';
import { fetchUserDecryptV1 } from './fetchUserDecryptV1.js';
import { fetchUserDecryptV2 } from './fetchUserDecryptV2.js';

//////////////////////////////////////////////////////////////////////////////
// fetchUserDecrypt
//////////////////////////////////////////////////////////////////////////////

export async function fetchUserDecrypt(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchUserDecryptParameters,
): Promise<FetchUserDecryptReturnType> {
  const protocolVersion = getResolvedProtocolVersion(relayerClient);
  if (protocolVersion === undefined) {
    throw new Error(
      'Unable to resolve protocol version from context, ensure proper initialization of the FhevmRuntime and FhevmChain.',
    );
  }

  if (isSemverStrictlyBefore(protocolVersion.version, '0.14.0')) {
    return await fetchUserDecryptV1(relayerClient, parameters as FetchUserDecryptParametersV1);
  }

  return await fetchUserDecryptV2(relayerClient, parameters as FetchUserDecryptParametersV2);
}
