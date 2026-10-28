import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { FetchFheEncryptionKeyBytesParameters } from '../modules/relayer/types.js';
import type { FheEncryptionKeyEntry } from './FheEncryptionKeyCache-p.js';
import { getFheEncryptionKeyCacheKey, globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { assertKeyVerificationConfig, verifyFheEncryptionKeyBytes } from './verifyFheEncryptionKeyBytes-p.js';
import { getProvidedFheEncryptionKey, releaseProvidedFheEncryptionKey } from '../runtime/CoreFhevm-p.js';
import { unsafeBytesEquals } from '../base/bytes.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Downloads the key / CRS bytes through the relayer, then checks them against
 * KMSGeneration. Bytes that fail the check are never returned, so never
 * cached nor used.
 */
export async function fetchVerifiedFheEncryptionKeyBytes(
  context: FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: FetchFheEncryptionKeyBytesParameters,
): Promise<FheEncryptionKeyBytes> {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  const bytes = await context.runtime.relayer.fetchFheEncryptionKeyBytes(
    { relayerUrl, chainId: context.chain.id },
    // Merged here, so that no fetch path can omit the runtime auth.
    // Caller-provided options override it.
    { ...parameters, options: { auth: context.runtime.config.auth, ...parameters.options } },
  );

  if (context.runtime.relayer.servesPlaceholderFheEncryptionKey !== true) {
    await verifyFheEncryptionKeyBytes(context, bytes);
  }
  return bytes;
}

/**
 * The client's entry in the global cache (see
 * {@link getFheEncryptionKeyCacheKey}), filled by `fetcher` if the slot is
 * empty.
 *
 * Caller-provided bytes (`options.fheEncryptionKey`) are checked once, whatever
 * the cache holds: bytes equal to the cached ones are verified already (no RPC
 * call); others are checked against KMSGeneration. Checked bytes fill an empty
 * slot; a cached entry is never replaced (refreshing it is the cache's job).
 * Released once checked, whether the check passes or not.
 *
 * @throws {FhevmConfigError} when the client cannot verify key material.
 * @throws {FheEncryptionKeyVerificationError} when the provided bytes fail the check.
 */
export async function getFheEncryptionKeyEntry(
  context: FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  parameters: {
    readonly fetcher: () => Promise<FheEncryptionKeyBytes>;
  },
): Promise<FheEncryptionKeyEntry> {
  // The client's own setup, even when another client filled the cache.
  if (context.runtime.relayer.servesPlaceholderFheEncryptionKey !== true) {
    assertKeyVerificationConfig(context);
  }

  const cache = globalFheEncryptionKeyCache;
  const slot = {
    owner: context.runtime,
    key: getFheEncryptionKeyCacheKey(context),
    relayerUrl: context.chain.fhevm.relayerUrl,
  };

  const provided = getProvidedFheEncryptionKey(context);
  if (provided === undefined) {
    return cache.getOrCreate({ ...slot, fill: parameters.fetcher });
  }

  try {
    await _checkProvidedFheEncryptionKey(context, provided, slot.key);
  } finally {
    releaseProvidedFheEncryptionKey(context, provided);
  }
  return cache.getOrCreate({ ...slot, fill: () => Promise.resolve(provided) });
}

// The check of each client's provided bytes (its own copy): concurrent calls
// on a client share one set of KMSGeneration reads and one hash.
const providedChecks = new WeakMap<FheEncryptionKeyBytes, Promise<void>>();

function _checkProvidedFheEncryptionKey(
  context: FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>,
  provided: FheEncryptionKeyBytes,
  cacheKey: string,
): Promise<void> {
  let check = providedChecks.get(provided);
  if (check === undefined) {
    check = (async () => {
      if (context.runtime.relayer.servesPlaceholderFheEncryptionKey === true) {
        return;
      }
      const cached = await globalFheEncryptionKeyCache.get(cacheKey)?.bytes.catch(() => undefined);
      if (cached !== undefined && _sameFheEncryptionKeyBytes(provided, cached)) {
        return;
      }
      await verifyFheEncryptionKeyBytes(context, provided);
    })();
    providedChecks.set(provided, check);
  }
  return check;
}

/** Ids aside: a label id and a 32-byte hex id can name the same bytes. */
function _sameFheEncryptionKeyBytes(a: FheEncryptionKeyBytes, b: FheEncryptionKeyBytes): boolean {
  return (
    a === b ||
    (unsafeBytesEquals(a.publicKeyBytes.bytes, b.publicKeyBytes.bytes) &&
      unsafeBytesEquals(a.crsBytes.bytes, b.crsBytes.bytes))
  );
}
