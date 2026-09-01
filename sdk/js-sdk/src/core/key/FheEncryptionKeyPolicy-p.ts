import type { FhevmChain } from '../types/fhevmChain.js';
import type {
  FheEncryptionKeyBytes,
  FheEncryptionKeyDigests,
  FheEncryptionKeyMetadata,
  FheEncryptionKeyTrust,
} from '../types/fheEncryptionKey.js';
import type { FheEncryptionKeyTrustSnapshot } from '../host-contracts/readFheEncryptionKeyTrustSnapshot-p.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import {
  assertFheEncryptionKeyDigestsMatch,
  computeFheEncryptionKeyDigests,
  normalizeFheEncryptionKeyTrust,
  resolveFheEncryptionKeyTrust,
} from './authenticateFheEncryptionKeyBytes.js';
import { cloneFheEncryptionKeyBytes } from './cloneFheEncryptionKeyBytes.js';
import {
  getConfiguredFheEncryptionKeyTrust,
  type ConfiguredFheEncryptionKeyTrust,
} from '../chains/configuredFheEncryptionKeyTrust-p.js';

export type FheEncryptionKeyPolicy =
  | { readonly mode: 'cleartext' }
  | { readonly mode: 'missingTrust' }
  | { readonly mode: 'kmsGeneration'; readonly trust: ConfiguredFheEncryptionKeyTrust }
  | { readonly mode: 'relayer'; readonly trust: FheEncryptionKeyTrust }
  | {
      readonly mode: 'pinned';
      readonly key: FheEncryptionKeyBytes;
      readonly digests: FheEncryptionKeyDigests;
      readonly trust: FheEncryptionKeyTrust | undefined;
      readonly configuredTrust: ConfiguredFheEncryptionKeyTrust | undefined;
    };

export type ResolvedFheEncryptionKeyPolicy =
  | { readonly mode: 'cleartext' }
  | { readonly mode: 'missingTrust' }
  | {
      readonly mode: 'kmsGeneration';
      readonly trust: ConfiguredFheEncryptionKeyTrust;
      readonly expectedDigests: FheEncryptionKeyTrustSnapshot;
    }
  | {
      readonly mode: 'relayer';
      readonly trust: FheEncryptionKeyTrust;
      readonly expectedDigests: FheEncryptionKeyDigests;
    }
  | {
      readonly mode: 'pinned';
      readonly key: FheEncryptionKeyBytes;
      readonly expectedDigests: FheEncryptionKeyDigests | FheEncryptionKeyTrustSnapshot;
      readonly trust: FheEncryptionKeyTrust | undefined;
      readonly configuredTrust: ConfiguredFheEncryptionKeyTrust | undefined;
    };

export function createFheEncryptionKeyPolicy(
  parameters?: {
    readonly fheEncryptionKeyTrust?: FheEncryptionKeyTrust | undefined;
    readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  },
  chain?: FhevmChain,
): FheEncryptionKeyPolicy {
  const configuredTrust = getConfiguredFheEncryptionKeyTrust(chain);
  if (configuredTrust !== undefined && parameters?.fheEncryptionKeyTrust !== undefined) {
    throw new FhevmConfigError({
      message:
        'Chains with a configured KMSGeneration contract authenticate FHE encryption-key material on-chain ' +
        'and do not accept application digest overrides.',
    });
  }

  const trust = normalizeFheEncryptionKeyTrust(parameters?.fheEncryptionKeyTrust);
  if (parameters?.fheEncryptionKey !== undefined) {
    const key = cloneFheEncryptionKeyBytes(parameters.fheEncryptionKey);
    return Object.freeze({
      mode: 'pinned',
      key,
      digests: computeFheEncryptionKeyDigests(key),
      trust,
      configuredTrust,
    });
  }

  if (configuredTrust !== undefined) {
    return Object.freeze({ mode: 'kmsGeneration', trust: configuredTrust });
  }

  return trust === undefined ? Object.freeze({ mode: 'missingTrust' }) : Object.freeze({ mode: 'relayer', trust });
}

export function createCleartextFheEncryptionKeyPolicy(): FheEncryptionKeyPolicy {
  return Object.freeze({ mode: 'cleartext' });
}

export async function resolveFheEncryptionKeyPolicy(
  policy: FheEncryptionKeyPolicy,
  metadata: FheEncryptionKeyMetadata,
  readConfiguredTrust?: (trust: ConfiguredFheEncryptionKeyTrust) => Promise<FheEncryptionKeyTrustSnapshot>,
): Promise<ResolvedFheEncryptionKeyPolicy> {
  switch (policy.mode) {
    case 'cleartext':
    case 'missingTrust':
      return policy;
    case 'kmsGeneration':
      if (readConfiguredTrust === undefined) {
        throw new FhevmConfigError({
          message: 'Configured KMSGeneration FHE encryption-key trust requires an authenticated host-chain client.',
        });
      }
      return Object.freeze({
        mode: 'kmsGeneration',
        trust: policy.trust,
        expectedDigests: await readConfiguredTrust(policy.trust),
      });
    case 'relayer':
      return Object.freeze({
        mode: 'relayer',
        trust: policy.trust,
        expectedDigests: await resolveFheEncryptionKeyTrust(policy.trust, metadata),
      });
    case 'pinned': {
      const expectedDigests = await resolvePinnedFheEncryptionKeyDigests(policy, metadata, readConfiguredTrust);
      assertPinnedFheEncryptionKeyMatches(policy, expectedDigests, metadata);
      return Object.freeze({
        mode: 'pinned',
        key: policy.key,
        expectedDigests,
        trust: policy.trust,
        configuredTrust: policy.configuredTrust,
      });
    }
  }
}

async function resolvePinnedFheEncryptionKeyDigests(
  policy: Extract<FheEncryptionKeyPolicy, { readonly mode: 'pinned' }>,
  metadata: FheEncryptionKeyMetadata,
  readConfiguredTrust?: (trust: ConfiguredFheEncryptionKeyTrust) => Promise<FheEncryptionKeyTrustSnapshot>,
): Promise<FheEncryptionKeyDigests | FheEncryptionKeyTrustSnapshot> {
  if (policy.configuredTrust !== undefined) {
    if (readConfiguredTrust === undefined) {
      throw new FhevmConfigError({
        message: 'Configured KMSGeneration FHE encryption-key trust requires an authenticated host-chain client.',
      });
    }
    return readConfiguredTrust(policy.configuredTrust);
  }
  return policy.trust === undefined ? policy.digests : resolveFheEncryptionKeyTrust(policy.trust, metadata);
}

function assertPinnedFheEncryptionKeyMatches(
  policy: Extract<FheEncryptionKeyPolicy, { readonly mode: 'pinned' }>,
  expectedDigests: FheEncryptionKeyDigests | FheEncryptionKeyTrustSnapshot,
  metadata: FheEncryptionKeyMetadata,
): void {
  try {
    assertFheEncryptionKeyDigestsMatch(policy.digests, expectedDigests, metadata.chainId);
  } catch (cause) {
    const trustSource =
      policy.configuredTrust === undefined ? 'fheEncryptionKeyTrust' : 'chain-configured KMSGeneration trust';
    throw new FhevmConfigError({
      message: `fheEncryptionKey does not match ${trustSource} for chain ${metadata.chainId.toString()}.`,
      cause: cause instanceof Error ? cause : new Error(String(cause)),
    });
  }
}

export function createFheEncryptionKeyCacheIdentity(
  metadata: FheEncryptionKeyMetadata,
  policy: ResolvedFheEncryptionKeyPolicy,
): { readonly scopeKey: string; readonly identityKey: string } {
  const scopeKey = JSON.stringify({
    mode: policy.mode,
    chainId: metadata.chainId,
    relayerUrl: metadata.relayerUrl,
  });
  const identityKey =
    policy.mode === 'cleartext'
      ? 'cleartext'
      : policy.mode === 'missingTrust'
        ? 'missingTrust'
        : JSON.stringify({
            publicKeyId:
              'publicKeyId' in policy.expectedDigests ? policy.expectedDigests.publicKeyId.toString() : undefined,
            crsId: 'crsId' in policy.expectedDigests ? policy.expectedDigests.crsId.toString() : undefined,
            publicKeyDigest: policy.expectedDigests.publicKeyDigest,
            crsDigest: policy.expectedDigests.crsDigest,
          });
  return { scopeKey, identityKey };
}
