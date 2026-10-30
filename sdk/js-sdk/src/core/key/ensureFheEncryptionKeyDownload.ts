import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FetchFheEncryptionKeyBytesParameters } from '../modules/relayer/types.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { CacheEntry } from './FheEncryptionKeyCache-p.js';
import { unsafeBytesEquals } from '../base/bytes.js';
import { FheEncryptionKeyVerificationError } from '../errors/FheEncryptionKeyVerificationError.js';
import {
  getProvidedFheEncryptionKey,
  getProvidedFheEncryptionKeyError,
  releaseProvidedFheEncryptionKey,
} from '../runtime/CoreFhevm-p.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import {
  assertKeyVerificationConfig,
  requiresKeyVerification,
  verifyFheEncryptionKeyBytes,
} from './verifyFheEncryptionKeyBytes-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>;

/** A cache entry and its bytes, checked for the client. */
type CheckedFheEncryptionKey = {
  readonly key: string;
  readonly entry: CacheEntry;
  readonly bytes: FheEncryptionKeyBytes;
};

////////////////////////////////////////////////////////////////////////////////

/**
 * The client's slot in the global key cache: the relayer URL, or
 * `placeholder:<relayer URL>` for a relayer module that serves placeholder key
 * material, so a client that checks keys never reads unchecked ones.
 *
 * @internal
 */
export function getFheEncryptionKeySlot(context: {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
}): string {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  return requiresKeyVerification(context.runtime) ? relayerUrl : `placeholder:${relayerUrl}`;
}

/**
 * Starts the FHE encryption key download into the global cache, or joins the
 * one in flight, then checks the key against `KMSGeneration` for this client,
 * with its own settings (once per client and key). Builds the relayer request
 * (e.g. auth) for both the bytes and the encryption paths. Does not convert
 * the key.
 *
 * A caller-provided key (`options.fheEncryptionKey`) replaces the download:
 * - equal to the cached key, once downloaded: the cached key is used, checked
 *   for this client;
 * - otherwise (including while a download is in flight): checked, then cached,
 *   replacing a cached key, which is stale (only one key is active at a time),
 *   unless the cache holds the same key by then: that one is used, as above.
 *
 * Outcomes of the check:
 * - pass: a provided key is cached and the client releases it;
 * - {@link FheEncryptionKeyVerificationError}: a downloaded key leaves the
 *   cache, the next call downloads again; the client drops a provided key and
 *   keeps only the error: later calls throw it, with no RPC call and no
 *   download;
 * - any other error (e.g. network): the key stays as is, the next call checks
 *   it again.
 *
 * Callers use the returned entry, the one checked, never the slot's current
 * one: another entry may have taken the slot since.
 *
 * @throws {FhevmConfigError} when the client cannot check key material.
 * @throws {FheEncryptionKeyVerificationError} when the key fails the check.
 * @internal
 */
export async function ensureFheEncryptionKeyDownload(
  context: Context,
  parameters: {
    readonly fhevmContext: FhevmClientFrozenContext;
    readonly options?: RelayerKeyUrlOptions | undefined;
    readonly ignoreCache?: boolean | undefined;
  },
): Promise<CheckedFheEncryptionKey> {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  const runtime = context.runtime;
  const logger = runtime.config.logger;
  // A provided key that failed the check: only its error is left.
  const providedError = getProvidedFheEncryptionKeyError(context);
  if (providedError !== undefined) {
    throw providedError;
  }

  const key = getFheEncryptionKeySlot(context);

  // The client's own setup, even when another client filled the cache.
  if (requiresKeyVerification(runtime)) {
    assertKeyVerificationConfig(context);
  }

  const provided = getProvidedFheEncryptionKey(context);
  if (provided !== undefined) {
    return _useProvidedFheEncryptionKey(context, provided, key);
  }

  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerParameters: FetchFheEncryptionKeyBytesParameters = {
    ...parameters,
    options: { auth: context.runtime.config.auth, ...parameters.options },
  };

  // Shared by every client on the slot: a download in flight is joined, and
  // runs with the request options (e.g. auth, abort signal, retries) of the
  // client that started it.
  const entry = globalFheEncryptionKeyCache.ensureBytes({
    owner: runtime,
    key,
    fetcher: async () => {
      logger?.debug?.(
        `Fetching FHE encryption key/CRS bytes from relayer "${relayerUrl}" (chainId=${context.chain.id}).`,
      );
      try {
        const bytes = await runtime.relayer.fetchFheEncryptionKeyBytes(
          { relayerUrl, chainId: context.chain.id },
          relayerParameters,
        );
        logger?.debug?.(`Fetched FHE encryption key/CRS bytes from relayer "${relayerUrl}".`);
        return bytes;
      } catch (e) {
        logger?.error?.(`Failed to fetch FHE encryption key/CRS bytes from relayer "${relayerUrl}".`, e);
        throw e;
      }
    },
    metadata: { chainId: context.chain.id, relayerUrl },
  });

  return _checkCachedFheEncryptionKey(context, key, entry);
}

////////////////////////////////////////////////////////////////////////////////

// Each client's check of each key (bytes object): concurrent calls on a
// client share one set of KMSGeneration reads. A key that fails the check stays
// failed; any other error (e.g. network) is retried by the next call.
const clientChecks = new WeakMap<FheEncryptionKeyBytes, WeakMap<Context, Promise<void>>>();

function _checkFheEncryptionKeyBytes(context: Context, bytes: FheEncryptionKeyBytes): Promise<void> {
  if (!requiresKeyVerification(context.runtime)) {
    return Promise.resolve();
  }
  const checks = clientChecks.get(bytes) ?? new WeakMap<Context, Promise<void>>();
  clientChecks.set(bytes, checks);
  return _memoizeCheck(checks, context, () => verifyFheEncryptionKeyBytes(context, bytes).then(() => undefined), true);
}

async function _checkCachedFheEncryptionKey(
  context: Context,
  key: string,
  entry: CacheEntry,
): Promise<CheckedFheEncryptionKey> {
  const bytes = await globalFheEncryptionKeyCache.entryBytes(entry);
  try {
    await _checkFheEncryptionKeyBytes(context, bytes);
  } catch (e) {
    if (e instanceof FheEncryptionKeyVerificationError) {
      // Accepted trade-off: a client with a wrong KMSGeneration address evicts
      // good bytes, and the other clients download them again. Not reachable
      // with the shipped chain definitions.
      globalFheEncryptionKeyCache.evict(key, entry);
    }
    throw e;
  }
  return { key, entry, bytes };
}

////////////////////////////////////////////////////////////////////////////////

// The use of each client's provided key (its own copy): concurrent calls on a
// client share the one in flight. A failed use is forgotten: the client keeps
// the error of a key that fails the check (see ensureFheEncryptionKeyDownload),
// any other error is retried by the next call.
const providedChecks = new WeakMap<FheEncryptionKeyBytes, Promise<CheckedFheEncryptionKey>>();

function _useProvidedFheEncryptionKey(
  context: Context,
  provided: FheEncryptionKeyBytes,
  key: string,
): Promise<CheckedFheEncryptionKey> {
  return _memoizeCheck(
    providedChecks,
    provided,
    async (): Promise<CheckedFheEncryptionKey> => {
      try {
        const cachedEntry = await _getSameCachedEntry(key, provided);
        let checked: CheckedFheEncryptionKey;
        if (cachedEntry !== undefined) {
          checked = await _checkCachedFheEncryptionKey(context, key, cachedEntry);
        } else {
          await _checkFheEncryptionKeyBytes(context, provided);
          // Another client may have cached the same key during the check: use
          // its entry rather than orphan it.
          const sameEntry = await _getSameCachedEntry(key, provided);
          if (sameEntry !== undefined) {
            checked = await _checkCachedFheEncryptionKey(context, key, sameEntry);
          } else {
            const entry = globalFheEncryptionKeyCache.replaceBytes(
              context.runtime,
              key,
              context.chain.fhevm.relayerUrl,
              provided,
            );
            checked = { key, entry, bytes: provided };
          }
        }
        // The cache holds the key from now on.
        releaseProvidedFheEncryptionKey(context, provided);
        return checked;
      } catch (e) {
        if (e instanceof FheEncryptionKeyVerificationError) {
          // The client keeps only the error, not the bytes.
          releaseProvidedFheEncryptionKey(context, provided, e);
        }
        throw e;
      }
    },
    false,
  );
}

/**
 * The slot's entry if it holds `provided`'s bytes. Compared only with a cached
 * key already downloaded: never waits for another client's download.
 */
async function _getSameCachedEntry(key: string, provided: FheEncryptionKeyBytes): Promise<CacheEntry | undefined> {
  const entry = globalFheEncryptionKeyCache.get(key);
  if (entry?.resolvedKind === undefined) {
    return undefined;
  }
  const cached = await globalFheEncryptionKeyCache.entryBytes(entry);
  return _sameFheEncryptionKeyBytes(provided, cached) ? entry : undefined;
}

/**
 * Memoizes `start()` under `key`: concurrent and later calls share it. A
 * rejection is forgotten, so the next call starts again, except a
 * {@link FheEncryptionKeyVerificationError} when `keepVerificationError`.
 */
function _memoizeCheck<K extends object, T>(
  memo: WeakMap<K, Promise<T>>,
  key: K,
  start: () => Promise<T>,
  keepVerificationError: boolean,
): Promise<T> {
  const memoized = memo.get(key);
  if (memoized !== undefined) {
    return memoized;
  }
  const check = start();
  memo.set(key, check);
  check.catch((e: unknown) => {
    if (!(keepVerificationError && e instanceof FheEncryptionKeyVerificationError) && memo.get(key) === check) {
      memo.delete(key);
    }
  });
  return check;
}

/**
 * Ids aside: a label id and a 32-byte hex id can name the same bytes. The
 * check against the active ids decides (a relayer serving the right bytes under
 * a wrong id fails the check for clients reusing that entry).
 */
function _sameFheEncryptionKeyBytes(a: FheEncryptionKeyBytes, b: FheEncryptionKeyBytes): boolean {
  return (
    a === b ||
    (unsafeBytesEquals(a.publicKeyBytes.bytes, b.publicKeyBytes.bytes) &&
      unsafeBytesEquals(a.crsBytes.bytes, b.crsBytes.bytes))
  );
}
