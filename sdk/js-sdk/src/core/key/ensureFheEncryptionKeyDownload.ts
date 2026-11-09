import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase, OptionalNativeClient, ResolvedFhevmOptions } from '../types/coreFhevmClient.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { FetchFheEncryptionKeyBytesParameters } from '../modules/relayer/types.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { FheEncryptionKeySource } from '../types/fheEncryptionKey.js';
import { isBytes32Hex, unsafeBytesEquals } from '../base/bytes.js';
import { uint256ToBytes32Hex } from '../base/uint.js';
import { VersionedValueCache, type CachedValue } from '../caching/versionedValueCache.js';
import { getCacheTtl } from '../caching/cacheOptions-p.js';
import { FheEncryptionKeyVerificationError } from '../errors/FheEncryptionKeyVerificationError.js';
import {
  getProvidedFheEncryptionKey,
  getProvidedFheEncryptionKeyError,
  releaseProvidedFheEncryptionKey,
} from '../runtime/CoreFhevm-p.js';
import {
  readActiveFheEncryptionKeyIds,
  requiresKeyVerification,
  resolveKmsGeneration,
  verifyFheEncryptionKeyBytes,
} from './verifyFheEncryptionKeyBytes-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain, FhevmRuntime, OptionalNativeClient>;

type Parameters = {
  readonly fhevmContext: FhevmClientFrozenContext;
  readonly options?: RelayerKeyUrlOptions | undefined;
};

/** The key bytes, checked for the client, and their cache version. */
type ClientFheEncryptionKey = {
  readonly bytes: FheEncryptionKeyBytes;
  readonly version: string;
};

type ActiveIds = { readonly keyId: bigint; readonly crsId: bigint };

////////////////////////////////////////////////////////////////////////////////
//
// Key material cache: one key per slot (see getFheEncryptionKeySlot). A read
// reads the active ids on KMSGeneration first and keeps the cached key when
// the ids are unchanged; else it downloads the key. A rotation the relayer has
// not followed yet fails the read and leaves the cached key in place: hex ids
// are compared before the download, label ids are checked after it, before
// caching.
// Clients that do not check keys download again when the TTL runs out.
//
////////////////////////////////////////////////////////////////////////////////

class KeyMaterialCache extends VersionedValueCache<Context, Parameters, FheEncryptionKeyBytes> {
  protected override getSlotKey(context: Context): string {
    return getFheEncryptionKeySlot(context);
  }

  protected override async fetch(
    context: Context,
    parameters: Parameters,
    cached: CachedValue<FheEncryptionKeyBytes> | undefined,
  ): Promise<{ readonly version: string; readonly value: FheEncryptionKeyBytes }> {
    if (!requiresKeyVerification(context.runtime)) {
      const bytes = await _downloadFheEncryptionKeyBytes(context, parameters, undefined);
      return { version: _bytesVersion(bytes), value: bytes };
    }
    const activeIds = await readActiveFheEncryptionKeyIds(context);
    const version = fheEncryptionKeyVersion(activeIds);
    if (cached?.version === version) {
      return cached;
    }
    const bytes = await _downloadFheEncryptionKeyBytes(context, parameters, activeIds);
    if (isBytes32Hex(bytes.publicKeyBytes.id) && isBytes32Hex(bytes.crsBytes.id)) {
      return { version, value: bytes };
    }
    return { version: await _admitLabelIdKey(context, bytes), value: bytes };
  }
}

const keyMaterialCache = new KeyMaterialCache();

/**
 * Checks a key served under label ids before it is cached (the check is
 * memoized for this client) and versions it by the active ids it matched.
 *
 * @throws {FheEncryptionKeyVerificationError} when it is not the active key
 * (e.g. the relayer has not followed a rotation yet). Nothing is cached.
 */
async function _admitLabelIdKey(context: Context, bytes: FheEncryptionKeyBytes): Promise<string> {
  try {
    const matchedIds = await _verifyFheEncryptionKeyBytesForClient(context, bytes);
    return matchedIds === undefined ? _bytesVersion(bytes) : fheEncryptionKeyVersion(matchedIds);
  } catch (e) {
    if (!(e instanceof FheEncryptionKeyVerificationError)) {
      throw e;
    }
    throw new FheEncryptionKeyVerificationError({
      message:
        'The relayer does not serve the active FHE public key and CRS (its key fails the check against ' +
        'KMSGeneration): a key rotation may be in progress. Nothing was cached: retry later.',
      cause: e,
    });
  }
}

/** The cache version of key material: `keyId:crsId`, as 32-byte hex. */
export function fheEncryptionKeyVersion(ids: ActiveIds): string {
  return `${uint256ToBytes32Hex(ids.keyId)}:${uint256ToBytes32Hex(ids.crsId)}`;
}

/**
 * Reads the version of the active key material (its ids on KMSGeneration),
 * without downloading or caching it.
 */
export async function readFheEncryptionKeyVersion(context: Context): Promise<string> {
  return fheEncryptionKeyVersion(await readActiveFheEncryptionKeyIds(context));
}

/** The version of bytes no client checks: the relayer's ids, as served. */
function _bytesVersion(bytes: FheEncryptionKeyBytes): string {
  const normalize = (id: string): string => (isBytes32Hex(id) ? id.toLowerCase() : id);
  return `${normalize(bytes.publicKeyBytes.id)}:${normalize(bytes.crsBytes.id)}`;
}

/**
 * Reads the key material again (see `VersionedValueCache.refresh`). Each client
 * checks the key on use.
 *
 * @returns the cache version of the current key.
 * @internal
 */
export async function refreshFheEncryptionKey(
  context: Context,
  parameters: Parameters & { readonly invalidVersion: string | undefined },
): Promise<string> {
  const { version } = await keyMaterialCache.refresh(context, parameters, parameters.invalidVersion);
  return version;
}

/** For tests and harnesses: the key cache, by slot. */
export const fheEncryptionKeyCacheForTests = {
  peek: (slot: string): CachedValue<FheEncryptionKeyBytes> | undefined => keyMaterialCache.peek(slot),
  /** Stores `bytes` unchecked, versioned by their served ids. */
  set: (slot: string, bytes: FheEncryptionKeyBytes): void => {
    if (!slot.startsWith(`${bytes.metadata.relayerUrl}|`)) {
      throw new Error(
        `FheEncryptionKey relayerUrl mismatch: "${bytes.metadata.relayerUrl}" is not the slot's relayer.`,
      );
    }
    keyMaterialCache.set(slot, _bytesVersion(bytes), bytes);
  },
  clear: (): void => {
    keyMaterialCache.clear();
  },
};

////////////////////////////////////////////////////////////////////////////////

/**
 * The client's slot in the global key cache: the relayer URL, then what the
 * key is checked against:
 * - `<relayer URL>|kmsgen:<chain id>:<address>`: the KMSGeneration contract;
 * - `<relayer URL>|unverified`: nothing (see
 *   `skipsFheEncryptionKeyVerification`); clients that check keys never
 *   read it.
 *
 * Clients share a slot only when they name the same relayer and
 * KMSGeneration, so a wrong setting evicts only its own slot. A client
 * whose RPC returns other state for that same contract (e.g. a fork)
 * still shares the slot and can evict it. Neither suffix contains `|`, so
 * two different slots never give the same string.
 *
 * @throws {FhevmConfigError} when the client cannot check key material.
 * @internal
 */
export function getFheEncryptionKeySlot(context: {
  readonly chain: FhevmChain;
  readonly runtime: FhevmRuntime;
  readonly options: Pick<ResolvedFhevmOptions, 'keyVerification'>;
}): string {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  if (!requiresKeyVerification(context.runtime)) {
    return `${relayerUrl}|unverified`;
  }
  const { chainId, address } = resolveKmsGeneration(context);
  return `${relayerUrl}|kmsgen:${chainId}:${address}`;
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
 * @throws {FhevmConfigError} when the client cannot check key material.
 * @throws {FheEncryptionKeyVerificationError} when the key fails the check.
 * @internal
 */
export async function ensureFheEncryptionKeyDownload(
  context: Context,
  parameters: Parameters,
): Promise<ClientFheEncryptionKey> {
  // A provided key that failed the check: only its error is left.
  const providedError = getProvidedFheEncryptionKeyError(context);
  if (providedError !== undefined) {
    throw providedError;
  }

  // Checks the client's own setup, even when another client filled the cache.
  const slot = getFheEncryptionKeySlot(context);

  const provided = getProvidedFheEncryptionKey(context);
  if (provided !== undefined) {
    return _useProvidedFheEncryptionKey(context, slot, provided);
  }

  return _checkCachedFheEncryptionKey(
    context,
    slot,
    await keyMaterialCache.get(context, parameters, getCacheTtl(context, 'keyMaterial')),
  );
}

/** Checks a cached key for the client; a key that fails leaves the cache. */
async function _checkCachedFheEncryptionKey(
  context: Context,
  slot: string,
  cached: CachedValue<FheEncryptionKeyBytes>,
): Promise<ClientFheEncryptionKey> {
  try {
    await _verifyFheEncryptionKeyBytesForClient(context, cached.value);
  } catch (e) {
    if (e instanceof FheEncryptionKeyVerificationError) {
      // Clients on this slot check against the same KMSGeneration, so they
      // reject these bytes too (unless their RPC serves other chain state).
      keyMaterialCache.evict(slot, cached.value);
    }
    throw e;
  }
  return { bytes: cached.value, version: cached.version };
}

/**
 * The relayer serves the active key and CRS (compared for 32-byte hex ids;
 * label ids, e.g. the decimal ids of relayer release/0.14.x, are left to the
 * digest check; see planning-blockchain#1790).
 *
 * @throws {FheEncryptionKeyVerificationError} when it serves others (e.g. it
 * has not followed a rotation yet). Nothing is downloaded.
 */
function _assertRelayerServesActiveKey(source: FheEncryptionKeySource, activeIds: ActiveIds): void {
  const servedIds = [source.publicKeySource.id, source.crsSource.id];
  const activeHexIds = [uint256ToBytes32Hex(activeIds.keyId), uint256ToBytes32Hex(activeIds.crsId)];
  if (servedIds.some((id, i) => isBytes32Hex(id) && id.toLowerCase() !== activeHexIds[i])) {
    throw new FheEncryptionKeyVerificationError({
      message: `The relayer does not serve the active FHE public key and CRS yet (relayer: ${servedIds.join(', ')}; active on KMSGeneration: ${activeHexIds.join(', ')}): a key rotation is in progress. Nothing was downloaded: retry later.`,
    });
  }
}

/**
 * Downloads the key bytes. With `activeIds`, first asserts the relayer serves
 * them (a download would otherwise only fail the check).
 */
async function _downloadFheEncryptionKeyBytes(
  context: Context,
  parameters: Parameters,
  activeIds: ActiveIds | undefined,
): Promise<FheEncryptionKeyBytes> {
  const relayerUrl = context.chain.fhevm.relayerUrl;
  const logger = context.runtime.config.logger;
  // Caller-provided options override runtime config defaults (e.g. auth)
  const relayerParameters: FetchFheEncryptionKeyBytesParameters = {
    options: { auth: context.runtime.config.auth, ...parameters.options },
    beforeDownload:
      activeIds === undefined
        ? undefined
        : async (source: FheEncryptionKeySource) => {
            _assertRelayerServesActiveKey(source, activeIds);
            return Promise.resolve();
          },
  };
  logger?.debug?.(`Fetching FHE encryption key/CRS bytes from relayer "${relayerUrl}" (chainId=${context.chain.id}).`);
  try {
    const bytes = await context.runtime.relayer.fetchFheEncryptionKeyBytes(
      { relayerUrl, chainId: context.chain.id },
      relayerParameters,
    );
    if (bytes.metadata.relayerUrl !== relayerUrl) {
      throw new Error(
        `FheEncryptionKey relayerUrl mismatch: expected "${relayerUrl}" but got "${bytes.metadata.relayerUrl}". The FheEncryptionKey must originate from the same relayer.`,
      );
    }
    logger?.debug?.(`Fetched FHE encryption key/CRS bytes from relayer "${relayerUrl}".`);
    return bytes;
  } catch (e) {
    logger?.error?.(`Failed to fetch FHE encryption key/CRS bytes from relayer "${relayerUrl}".`, e);
    throw e;
  }
}

////////////////////////////////////////////////////////////////////////////////

// Each client's check of each key (bytes object), resolving to the active ids
// the key matched (`undefined` when the client does not check keys): concurrent
// calls on a client share one set of KMSGeneration reads. A key that fails the
// check stays failed; any other error (e.g. network) is retried by the next call.
const clientVerifications = new WeakMap<FheEncryptionKeyBytes, WeakMap<Context, Promise<ActiveIds | undefined>>>();

function _verifyFheEncryptionKeyBytesForClient(
  context: Context,
  bytes: FheEncryptionKeyBytes,
): Promise<ActiveIds | undefined> {
  if (!requiresKeyVerification(context.runtime)) {
    return Promise.resolve(undefined);
  }
  const checks = clientVerifications.get(bytes) ?? new WeakMap<Context, Promise<ActiveIds | undefined>>();
  clientVerifications.set(bytes, checks);
  return memoizeUntilRejected(checks, context, () => verifyFheEncryptionKeyBytes(context, bytes), true);
}

////////////////////////////////////////////////////////////////////////////////

// The use of each client's provided key (its own copy): concurrent calls on a
// client share the one in flight. A failed use is forgotten: the client keeps
// the error of a key that fails the check (see ensureFheEncryptionKeyDownload),
// any other error is retried by the next call.
const providedKeyUses = new WeakMap<FheEncryptionKeyBytes, Promise<ClientFheEncryptionKey>>();

function _useProvidedFheEncryptionKey(
  context: Context,
  slot: string,
  provided: FheEncryptionKeyBytes,
): Promise<ClientFheEncryptionKey> {
  return memoizeUntilRejected(
    providedKeyUses,
    provided,
    async (): Promise<ClientFheEncryptionKey> => {
      try {
        let checked: ClientFheEncryptionKey;
        const cached = _cachedSameKey(slot, provided);
        if (cached !== undefined) {
          checked = await _checkCachedFheEncryptionKey(context, slot, cached);
        } else {
          const activeIds = await _verifyFheEncryptionKeyBytesForClient(context, provided);
          // Another client may have cached the same key during the check: use it.
          const cachedNow = _cachedSameKey(slot, provided);
          if (cachedNow !== undefined) {
            checked = await _checkCachedFheEncryptionKey(context, slot, cachedNow);
          } else {
            const version = activeIds === undefined ? _bytesVersion(provided) : fheEncryptionKeyVersion(activeIds);
            keyMaterialCache.set(slot, version, provided);
            checked = { bytes: provided, version };
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

/** The cached key if it is `provided`'s bytes. Never waits for a download. */
function _cachedSameKey(slot: string, provided: FheEncryptionKeyBytes): CachedValue<FheEncryptionKeyBytes> | undefined {
  const cached = keyMaterialCache.peek(slot);
  return cached !== undefined && _sameFheEncryptionKeyBytes(provided, cached.value) ? cached : undefined;
}

/**
 * Memoizes `start()` under `key`: concurrent and later calls share it. A
 * rejection is forgotten, so the next call starts again, except a
 * {@link FheEncryptionKeyVerificationError} when `keepVerificationError`.
 *
 * @internal
 */
export function memoizeUntilRejected<K extends object, T>(
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
