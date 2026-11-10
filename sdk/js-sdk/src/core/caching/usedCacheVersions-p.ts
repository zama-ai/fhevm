import type { FhevmCacheName } from '../types/cache.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * The version of each cached value a request used, recorded as it reads them,
 * so a failure names the version it used even if another request replaced it.
 */
export type UsedCacheVersions = Partial<Record<FhevmCacheName, string>>;

/** Records the version of a cached value the request used (no-op without `usedVersions` or `version`). */
export function recordUsedCacheVersion(
  usedVersions: UsedCacheVersions | undefined,
  cacheName: FhevmCacheName,
  version: string | undefined,
): void {
  if (usedVersions !== undefined && version !== undefined) {
    usedVersions[cacheName] = version;
  }
}
