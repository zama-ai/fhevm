import type { FetchFeaturesParameters, FetchFeaturesReturnType, RelayerClientWithRuntime } from '../types.js';
import { RelayerResponseErrorBase } from '../../../errors/RelayerResponseErrorBase.js';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';
import { buildRelayerUrlString, validateRelayerBaseUrl } from './relayerUrl.js';

//////////////////////////////////////////////////////////////////////////////
// fetchFeatures
//////////////////////////////////////////////////////////////////////////////

/**
 * Probes whether the relayer exposes the unified user-decrypt route, and reports it as
 * `supportsRouteV3`.
 *
 * There is no dedicated feature endpoint, so we detect the route by POSTing an **empty probe
 * payload** to `v3/user-decrypt` and reading the outcome. Routing is path-based, so the body never
 * decides existence:
 *
 * - **Route absent** → a user-decrypt POST does not expect a 404, so the request layer surfaces it
 *   as `status: 404` → `supportsRouteV3: false`.
 * - **Route present** → the relayer answers the (invalid) probe with a 400 (or 429 / 5xx). Any such
 *   response proves the endpoint exists → `supportsRouteV3: true`.
 * - **Auth rejected (401 / 403)** → the key is missing, bad, or expired (or an edge proxy blocked
 *   the request). That is not a capability answer, so we **fail loudly** (rethrow) rather than guess.
 *   `ensureRelayerFeatures` clears its in-flight promise on this rejection, so a retry with a valid
 *   key resolves cleanly.
 *
 * `options.auth` is forwarded so the probe can authenticate against relayers that require a key
 * (e.g. mainnet); the resulting capability is itself auth-independent.
 *
 * Any non-response failure (network, DNS, timeout) is likewise rethrown so the caller fails loudly
 * and retries instead of caching a wrong answer.
 */
export async function fetchFeatures(
  relayerClient: RelayerClientWithRuntime,
  parameters: FetchFeaturesParameters,
): Promise<FetchFeaturesReturnType> {
  const { options } = parameters;

  const hasAuth: boolean = options?.auth !== undefined;
  const relayerBaseUrl: URL = validateRelayerBaseUrl(relayerClient.chain.fhevm.relayerUrl, hasAuth);
  const url = buildRelayerUrlString(relayerBaseUrl, 'v3/user-decrypt');

  const request = new RelayerAsyncRequest({
    relayerOperation: 'USER_DECRYPT',
    url,
    // Deliberately-empty probe: a present route rejects it with a 400 (proving it exists); an absent
    // route 404s. We never expect this request to succeed.
    payload: {},
    options,
  });

  try {
    await request.run();
    // Not expected from an empty payload — but if the route accepts it, it certainly exists.
    return { supportsRouteV3: true };
  } catch (error) {
    if (error instanceof RelayerResponseErrorBase) {
      if (error.status === 404) {
        // RelayerResponseStatusError(404)
        return { supportsRouteV3: false };
      }
      // Auth rejected (missing / bad / expired key) or edge-blocked → fail loudly instead of
      // reporting a capability: the caller supplied a key we could not authenticate with.
      if (error.status === 401 || error.status === 403) {
        throw error;
      }
      // Any other relayer response (400 rejects the empty probe, 429, 5xx) proves the route exists.
      return { supportsRouteV3: true };
    }
    // Non-response failure (network / timeout): can't determine support — fail loudly.
    throw error;
  }
}
