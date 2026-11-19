import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { FhevmCacheName } from '../../types/cache.js';
import { refreshCachedValue } from '../../caching/retryOnStaleCache-p.js';
import { FHEVM_CACHE_NAMES } from '../../caching/cacheOptions-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';
import { FhevmConfigError } from '../../errors/FhevmConfigError.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Fetches a cached value again: the FHE public key and CRS
 * (`'keyMaterial'`), the KMS context (`'kmsContext'`) or the coprocessor
 * signers (`'coprocessorSigners'`). The cache is shared: the new value serves
 * every client. Concurrent calls share one fetch.
 *
 * Only reads again: resolves once the read is done, whatever version it
 * found (an older one, from an RPC that lags, is logged and not cached); then
 * retry the failed request. `staleVersion` is the version that failed it
 * (`CachedValueStaleError.staleVersion`): with it, nothing is fetched when the
 * cache already holds a newer version; without it, the value is always
 * fetched. The cache never goes back to an older version.
 *
 * @throws {FhevmConfigError} on an unknown `cacheName`.
 * @throws the fetch error, as is.
 */
export async function forceRefreshCachedValue(
  fhevm: Fhevm<FhevmChain>,
  cacheName: FhevmCacheName,
  staleVersion?: string,
): Promise<void> {
  if (!FHEVM_CACHE_NAMES.includes(cacheName)) {
    throw new FhevmConfigError({
      message: `Unknown cache name ${JSON.stringify(cacheName)}: expected one of ${FHEVM_CACHE_NAMES.join(', ')}.`,
    });
  }

  const fhevmContext = await initPublicAction(fhevm);

  await refreshCachedValue(fhevm, { cacheName, staleVersion, fhevmContext });
}
