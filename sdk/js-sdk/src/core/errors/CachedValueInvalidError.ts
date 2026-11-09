import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { FhevmCacheItem } from '../types/cache.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// CachedValueInvalidError
////////////////////////////////////////////////////////////////////////////////

export type CachedValueInvalidErrorType = CachedValueInvalidError & {
  name: 'CachedValueInvalidError';
};

export type CachedValueInvalidErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name' | 'message'> & {
    readonly item: FhevmCacheItem;
    readonly invalidVersion: string;
    readonly message?: string | undefined;
  }
>;

/**
 * Thrown when a request failed because a cached value went stale (e.g. an
 * input proof rejected after a key rotation) and the SDK did not recover:
 * `autoRefreshOnInvalidity` is `false`, or the retry failed on a stale value
 * again. Call `forceRefresh(item, { invalidVersion })`, then retry.
 *
 * `forceRefresh` also throws it when the value read again still has
 * `invalidVersion`: refreshing cannot help.
 */
export class CachedValueInvalidError extends FhevmErrorBase {
  readonly #item: FhevmCacheItem;
  readonly #invalidVersion: string;

  constructor(params: CachedValueInvalidErrorParams) {
    const { item, invalidVersion, ...baseParams } = params;
    super({
      ...baseParams,
      name: 'CachedValueInvalidError',
      message: params.message ?? `Cached ${item} (version ${invalidVersion}) is no longer valid.`,
    });
    this.#item = item;
    this.#invalidVersion = invalidVersion;
  }

  /** The cached value that went stale. */
  public get item(): FhevmCacheItem {
    return this.#item;
  }

  /**
   * The version of the cached value that went stale, to pass to `forceRefresh`.
   * Not the SDK version (`version`).
   */
  public get invalidVersion(): string {
    return this.#invalidVersion;
  }
}
