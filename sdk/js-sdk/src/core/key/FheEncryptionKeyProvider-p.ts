import type { Auth } from '../types/auth.js';
import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type {
  FheEncryptionKeyBytes,
  FheEncryptionKeyDigests,
  FheEncryptionKeyMetadata,
} from '../types/fheEncryptionKey.js';
import type { RelayerKeyUrlOptions } from '../types/relayer.js';
import type { ConfiguredFheEncryptionKeyTrust } from '../chains/configuredFheEncryptionKeyTrust-p.js';
import type { FheEncryptionKeyTrustReader } from '../host-contracts/readFheEncryptionKeyTrustSnapshot-p.js';
import type { FheEncryptionKeyPolicy, ResolvedFheEncryptionKeyPolicy } from './FheEncryptionKeyPolicy-p.js';
import { sha256 } from '@noble/hashes/sha2.js';
import { bytesToHexNo0x } from '../base/bytes.js';
import { AuthenticatedFheEncryptionKeyBytesCache, globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import {
  authenticateFheEncryptionKeyBytes,
  resolveFheEncryptionKeyTrust,
} from './authenticateFheEncryptionKeyBytes.js';
import { cloneFheEncryptionKeyBytes } from './cloneFheEncryptionKeyBytes.js';
import { createFheEncryptionKeyCacheIdentity, resolveFheEncryptionKeyPolicy } from './FheEncryptionKeyPolicy-p.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import {
  assertFheEncryptionKeyTrustSnapshotStillActive,
  readFheEncryptionKeyTrustSnapshot,
} from '../host-contracts/readFheEncryptionKeyTrustSnapshot-p.js';

////////////////////////////////////////////////////////////////////////////////

export type FheEncryptionKeyProviderParameters = {
  readonly options?: RelayerKeyUrlOptions | undefined;
  readonly ignoreCache?: boolean | undefined;
};

type RelayerRequestSnapshot = {
  readonly options: RelayerKeyUrlOptions;
  readonly cacheKey: string;
  readonly useProviderScopedCache: boolean;
};

/**
 * Narrow capability consumed by key-fetch and proof-building code.
 *
 * The name describes the real-encryption boundary. Cleartext test mode returns
 * cloned, cleartext-scoped bytes without cryptographic authentication; those
 * bytes cannot share a cache scope with real encryption.
 */
export type FheEncryptionKeyProvider = {
  readonly getAuthenticatedBytes: (parameters?: FheEncryptionKeyProviderParameters) => Promise<FheEncryptionKeyBytes>;
};

/**
 * Compiles chain, runtime, host trust, and the factory's public key options into
 * one narrow capability. Callers cannot recover the native host client or mutate
 * the selected policy through this object.
 */
export function createFheEncryptionKeyProvider(parameters: {
  readonly chain: FhevmChain | undefined;
  readonly runtime: FhevmRuntime;
  readonly policy: FheEncryptionKeyPolicy;
  readonly configuredTrustReader?: FheEncryptionKeyTrustReader | undefined;
}): FheEncryptionKeyProvider {
  const { chain, runtime, policy, configuredTrustReader } = parameters;
  const providerScopedBytesCache = new AuthenticatedFheEncryptionKeyBytesCache();

  return Object.freeze({
    getAuthenticatedBytes: async (
      fetchParameters: FheEncryptionKeyProviderParameters = {},
    ): Promise<FheEncryptionKeyBytes> => {
      if (chain === undefined) {
        throw new FhevmConfigError({ message: 'Cannot fetch an FHE encryption key without a configured chain.' });
      }

      const relayerUrl = chain.fhevm.relayerUrl;
      const metadata = Object.freeze({ chainId: chain.id, relayerUrl });
      const request = createRelayerRequestSnapshot(runtime.config.auth, fetchParameters.options);
      const readConfiguredTrustSnapshot =
        configuredTrustReader === undefined
          ? undefined
          : (trust: ConfiguredFheEncryptionKeyTrust) => readFheEncryptionKeyTrustSnapshot(trust, configuredTrustReader);
      const resolvedPolicy = await resolveFheEncryptionKeyPolicy(policy, metadata, readConfiguredTrustSnapshot);

      if (resolvedPolicy.mode === 'missingTrust') {
        throw new FhevmConfigError({
          message:
            `Cannot authenticate FHE encryption-key material for chain ${metadata.chainId.toString()}: ` +
            'no KMSGeneration contract or application trust anchor is configured. ' +
            'Configure fheEncryptionKeyTrust or provide trusted pinned fheEncryptionKey bytes.',
        });
      }

      const { scopeKey, identityKey } = createFheEncryptionKeyCacheIdentity(metadata, resolvedPolicy);
      const cache =
        request.useProviderScopedCache && resolvedPolicy.mode !== 'pinned'
          ? providerScopedBytesCache
          : globalFheEncryptionKeyCache;
      const cacheScopeKey = cache === providerScopedBytesCache ? `${scopeKey}|request=${request.cacheKey}` : scopeKey;
      // A pinned bundle was cloned and hashed at policy creation. It has no
      // external source to refetch, so ignoreCache deliberately reuses its entry.
      const ignoreCache = resolvedPolicy.mode === 'pinned' ? false : fetchParameters.ignoreCache;

      const bytes = await cache.getOrCreate({
        scopeKey: cacheScopeKey,
        identityKey,
        metadata,
        ignoreCache,
        fetcher: async () => {
          if (resolvedPolicy.mode === 'pinned') {
            await revalidateResolvedFheEncryptionKeyPolicy(resolvedPolicy, metadata, configuredTrustReader);
            return resolvedPolicy.key;
          }

          const keyBytes = await fetchRelayerBytes(runtime, chain, relayerUrl, request.options);
          if (resolvedPolicy.mode === 'cleartext') {
            return cloneFheEncryptionKeyBytes(keyBytes);
          }

          const authenticated = authenticateFheEncryptionKeyBytes(
            keyBytes,
            resolvedPolicy.expectedDigests,
            metadata.chainId,
          );
          await revalidateResolvedFheEncryptionKeyPolicy(resolvedPolicy, metadata, configuredTrustReader);
          return authenticated;
        },
        onRejected: (error) => {
          logFetchRejection(runtime, metadata, error);
        },
      });
      await revalidateResolvedFheEncryptionKeyPolicy(resolvedPolicy, metadata, configuredTrustReader);
      return bytes;
    },
  });
}

/** Rejects real-key policy fields at every cleartext JavaScript factory boundary. */
export function assertCleartextFheEncryptionKeyOptions(options: unknown): void {
  if (
    options !== null &&
    (typeof options === 'object' || typeof options === 'function') &&
    ('fheEncryptionKeyTrust' in options || 'fheEncryptionKey' in options)
  ) {
    throw new FhevmConfigError({
      message: 'Cleartext clients do not accept fheEncryptionKeyTrust or fheEncryptionKey options.',
    });
  }
}

function fetchRelayerBytes(
  runtime: FhevmRuntime,
  chain: FhevmChain,
  relayerUrl: string,
  options: RelayerKeyUrlOptions,
): Promise<FheEncryptionKeyBytes> {
  return runtime.relayer.fetchFheEncryptionKeyBytes({ relayerUrl, chainId: chain.id }, { options });
}

async function revalidateResolvedFheEncryptionKeyPolicy(
  policy: ResolvedFheEncryptionKeyPolicy,
  metadata: FheEncryptionKeyMetadata,
  configuredTrustReader: FheEncryptionKeyTrustReader | undefined,
): Promise<void> {
  if (policy.mode === 'kmsGeneration' || (policy.mode === 'pinned' && policy.configuredTrust !== undefined)) {
    if (configuredTrustReader === undefined) {
      throw new FhevmConfigError({
        message: 'Configured KMSGeneration FHE encryption-key trust requires an authenticated host-chain client.',
      });
    }
    const configuredTrust = policy.mode === 'kmsGeneration' ? policy.trust : policy.configuredTrust;
    if (configuredTrust === undefined) {
      return;
    }
    if (!('publicKeyId' in policy.expectedDigests) || !('crsId' in policy.expectedDigests)) {
      throw new FhevmConfigError({
        message: 'Configured KMSGeneration FHE encryption-key trust resolved without active key ids.',
      });
    }
    await assertFheEncryptionKeyTrustSnapshotStillActive(
      configuredTrust,
      configuredTrustReader,
      policy.expectedDigests,
    );
    return;
  }

  if ((policy.mode === 'relayer' || policy.mode === 'pinned') && typeof policy.trust === 'function') {
    const latest = await resolveFheEncryptionKeyTrust(policy.trust, metadata);
    assertSameFheEncryptionKeyDigests(policy.expectedDigests, latest, metadata);
  }
}

function assertSameFheEncryptionKeyDigests(
  expected: FheEncryptionKeyDigests,
  latest: FheEncryptionKeyDigests,
  metadata: FheEncryptionKeyMetadata,
): void {
  if (expected.publicKeyDigest !== latest.publicKeyDigest || expected.crsDigest !== latest.crsDigest) {
    throw new FhevmConfigError({
      message: `FHE encryption-key trust rotated while key material was being resolved for chain ${metadata.chainId.toString()}.`,
    });
  }
}

function createRelayerRequestSnapshot(
  runtimeAuth: RelayerKeyUrlOptions['auth'] | undefined,
  options: RelayerKeyUrlOptions | undefined,
): RelayerRequestSnapshot {
  const hasOwnAuth = options !== undefined && Object.prototype.hasOwnProperty.call(options, 'auth');
  const auth = cloneAuth(hasOwnAuth ? options.auth : runtimeAuth);
  const headers = cloneHeaders(options?.headers);
  const snapshot: RelayerKeyUrlOptions = {};
  if (auth !== undefined) {
    snapshot.auth = auth;
  }
  if (headers !== undefined) {
    snapshot.headers = headers;
  }
  if (options?.debug !== undefined) {
    snapshot.debug = options.debug;
  }
  if (options?.fetchRetries !== undefined) {
    snapshot.fetchRetries = options.fetchRetries;
  }
  if (options?.fetchRetryDelayInMilliseconds !== undefined) {
    snapshot.fetchRetryDelayInMilliseconds = options.fetchRetryDelayInMilliseconds;
  }
  if (options?.signal !== undefined) {
    snapshot.signal = options.signal;
  }
  if (options?.onProgress !== undefined) {
    snapshot.onProgress = options.onProgress;
  }
  return Object.freeze({
    options: Object.freeze(snapshot),
    cacheKey: createRelayerRequestCacheKey(snapshot),
    useProviderScopedCache: runtimeAuth !== undefined || options !== undefined,
  });
}

function cloneAuth(auth: Auth | undefined): Auth | undefined {
  if (auth === undefined) {
    return undefined;
  }
  switch (auth.type) {
    case 'BearerToken':
      return Object.freeze({ type: 'BearerToken', token: auth.token });
    case 'ApiKeyHeader':
      return auth.header === undefined
        ? Object.freeze({ type: 'ApiKeyHeader', value: auth.value })
        : Object.freeze({ type: 'ApiKeyHeader', header: auth.header, value: auth.value });
    case 'ApiKeyCookie':
      return auth.cookie === undefined
        ? Object.freeze({ type: 'ApiKeyCookie', value: auth.value })
        : Object.freeze({ type: 'ApiKeyCookie', cookie: auth.cookie, value: auth.value });
  }
}

function cloneHeaders(headers: Record<string, string> | undefined): Record<string, string> | undefined {
  if (headers === undefined) {
    return undefined;
  }
  const snapshot: Record<string, string> = {};
  for (const [name, value] of Object.entries(headers).sort(([a], [b]) => a.localeCompare(b))) {
    snapshot[name] = value;
  }
  return Object.freeze(snapshot);
}

const requestObjectCacheIds = new WeakMap<object, number>();
let nextRequestObjectCacheId = 0;
const requestCacheKeyEncoder = new TextEncoder();

function createRelayerRequestCacheKey(options: RelayerKeyUrlOptions): string {
  return JSON.stringify({
    auth: authCacheKey(options.auth),
    headers: headersCacheKey(options.headers),
    debug: options.debug,
    fetchRetries: options.fetchRetries,
    fetchRetryDelayInMilliseconds: options.fetchRetryDelayInMilliseconds,
    signal: options.signal === undefined ? undefined : requestObjectId(options.signal),
    onProgress: options.onProgress === undefined ? undefined : requestObjectId(options.onProgress),
  });
}

function authCacheKey(auth: Auth | undefined): unknown {
  if (auth === undefined) {
    return 'none';
  }
  switch (auth.type) {
    case 'BearerToken':
      return { type: auth.type, token: digestRequestSecret(auth.token) };
    case 'ApiKeyHeader':
      return { type: auth.type, header: auth.header ?? 'x-api-key', value: digestRequestSecret(auth.value) };
    case 'ApiKeyCookie':
      return { type: auth.type, cookie: auth.cookie ?? 'x-api-key', value: digestRequestSecret(auth.value) };
  }
}

function headersCacheKey(headers: Record<string, string> | undefined): unknown {
  if (headers === undefined) {
    return 'none';
  }
  return Object.entries(headers).map(([name, value]) => [name.toLowerCase(), digestRequestSecret(value)] as const);
}

function digestRequestSecret(value: string): string {
  return bytesToHexNo0x(sha256(requestCacheKeyEncoder.encode(value)));
}

function requestObjectId(value: object): string {
  let id = requestObjectCacheIds.get(value);
  if (id === undefined) {
    id = ++nextRequestObjectCacheId;
    requestObjectCacheIds.set(value, id);
  }
  return id.toString();
}

function logFetchRejection(runtime: FhevmRuntime, metadata: FheEncryptionKeyMetadata, error: unknown): void {
  runtime.config.logger?.error?.(
    `FHE encryption-key fetch failed for chain ${metadata.chainId.toString()} ` +
      `at "${metadata.relayerUrl}"; the cache entry was evicted.`,
    error,
  );
}
