import type { RelayerFetchOptions } from "../../modules/relayer/types.js";
import type {
  Fhevm,
  OptionalNativeClient,
} from "../../types/coreFhevmClient.js";
import type { WithEncryptAndRelayer } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";
import type { GlobalFhePkeParamsBytes } from "../../types/globalFhePkeParams.js";

////////////////////////////////////////////////////////////////////////////////

export type FetchGlobalFhePkeParamsBytesParameters = {
  readonly options?: RelayerFetchOptions;
  readonly ignoreCache?: boolean | undefined;
};

export type FetchGlobalFhePkeParamsBytesReturnType = GlobalFhePkeParamsBytes;

////////////////////////////////////////////////////////////////////////////////

/**
 * Module-level cache keyed by relayer URL.
 * Stores the in-flight or resolved promise to avoid duplicate fetches
 * and race conditions when multiple concurrent calls are made.
 */
// eslint-disable-next-line @typescript-eslint/naming-convention
const __globalFhePkeParamsGlobalCache = new Map<
  string,
  Promise<GlobalFhePkeParamsBytes>
>();

/**
 * Clears all entries from the GlobalFhePkeParams cache.
 */
export function clearGlobalFhePkeParamsCache(): void {
  __globalFhePkeParamsGlobalCache.clear();
}

/**
 * Removes a specific relayer URL entry from the GlobalFhePkeParams cache.
 */
export function deleteGlobalFhePkeParamsCache(relayerUrl: string): boolean {
  return __globalFhePkeParamsGlobalCache.delete(relayerUrl);
}

////////////////////////////////////////////////////////////////////////////////

export async function fetchGlobalFhePkeParamsBytes(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer, OptionalNativeClient>,
  parameters?: FetchGlobalFhePkeParamsBytesParameters | undefined,
): Promise<FetchGlobalFhePkeParamsBytesReturnType> {
  if (parameters?.ignoreCache !== true) {
    // 1. Check if already stored in cache
    const cached = __globalFhePkeParamsGlobalCache.get(
      fhevm.chain.fhevm.relayerUrl,
    );
    if (cached !== undefined) {
      return cached;
    }
  }

  // 2. Create and cache the promise immediately to prevent race conditions.
  // The result is always cached, even when ignoreCache is true,
  // so that future callers benefit from the fresh fetch.
  const promise = _fetchGlobalFhePkeParamsBytes(
    fhevm,
    parameters?.options,
  ).catch((err: unknown) => {
    // Only remove from cache if this promise is still the cached one.
    // A concurrent deleteGlobalFhePkeParamsCache + re-fetch may have replaced it.
    if (
      __globalFhePkeParamsGlobalCache.get(fhevm.chain.fhevm.relayerUrl) ===
      promise
    ) {
      __globalFhePkeParamsGlobalCache.delete(fhevm.chain.fhevm.relayerUrl);
    }
    throw err;
  });

  // save in cache even if `ignoreCache === true`
  __globalFhePkeParamsGlobalCache.set(fhevm.chain.fhevm.relayerUrl, promise);

  return promise;
}

////////////////////////////////////////////////////////////////////////////////

async function _fetchGlobalFhePkeParamsBytes(
  fhevm: Fhevm<FhevmChain, WithEncryptAndRelayer, OptionalNativeClient>,
  options?: RelayerFetchOptions,
): Promise<GlobalFhePkeParamsBytes> {
  const paramsBytes = await fhevm.runtime.relayer.fetchGlobalFhePkeParamsBytes(
    { relayerUrl: fhevm.chain.fhevm.relayerUrl },
    { options },
  );

  return paramsBytes;
}
