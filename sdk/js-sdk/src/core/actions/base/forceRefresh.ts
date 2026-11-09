import type { Fhevm } from '../../types/coreFhevmClient.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { FhevmCacheItem } from '../../types/cache.js';
import { forceRefreshCachedValue } from '../../caching/retryOnStaleCache-p.js';
import { FHEVM_CACHE_ITEMS } from '../../caching/cacheOptions-p.js';
import { initPublicAction } from '../../runtime/CoreFhevm-p.js';
import { FhevmConfigError } from '../../errors/FhevmConfigError.js';

////////////////////////////////////////////////////////////////////////////////

export type ForceRefreshParameters = {
  /**
   * The version that failed the request (`CachedValueInvalidError.invalidVersion`).
   * With it, nothing is fetched when the cache already holds another version;
   * without it, the value is always fetched.
   */
  readonly invalidVersion?: string | undefined;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Fetches a cached value again: the FHE public key and CRS
 * (`'keyMaterial'`), the KMS context (`'kmsContext'`) or the coprocessor
 * signers (`'coprocessorSigners'`). The cache is shared: the new value serves
 * every client. Concurrent calls share one fetch.
 *
 * Resolves once the cache holds a value other than `invalidVersion`; then
 * retry the failed request.
 *
 * @throws {FhevmConfigError} on an unknown `item`.
 * @throws {CachedValueInvalidError} when the value fetched still has
 * `invalidVersion`: refreshing cannot fix the failure.
 * @throws the fetch error, as is.
 */
export async function forceRefresh(
  fhevm: Fhevm<FhevmChain>,
  item: FhevmCacheItem,
  parameters?: ForceRefreshParameters,
): Promise<void> {
  if (!FHEVM_CACHE_ITEMS.includes(item)) {
    throw new FhevmConfigError({
      message: `Unknown cache item ${JSON.stringify(item)}: expected one of ${FHEVM_CACHE_ITEMS.join(', ')}.`,
    });
  }

  const fhevmContext = await initPublicAction(fhevm);

  await forceRefreshCachedValue(fhevm, { item, invalidVersion: parameters?.invalidVersion, fhevmContext });
}
