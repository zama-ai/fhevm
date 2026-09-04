import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import { describe, expect, it, vi } from 'vitest';
import { AuthenticatedFheEncryptionKeyBytesCache } from './FheEncryptionKeyCache-p.js';

const METADATA = { chainId: 1, relayerUrl: 'https://relayer.example' } as const;

describe('AuthenticatedFheEncryptionKeyBytesCache', () => {
  it('shares one authenticated promise for the current identity', async () => {
    const cache = new AuthenticatedFheEncryptionKeyBytesCache();
    const fetcher = vi.fn(async () => keyBytes(1));

    const first = cache.getOrCreate({ scopeKey: 'scope', identityKey: 'v1', metadata: METADATA, fetcher });
    const second = cache.getOrCreate({ scopeKey: 'scope', identityKey: 'v1', metadata: METADATA, fetcher });

    expect(second).toBe(first);
    await expect(first).resolves.toBeDefined();
    expect(fetcher).toHaveBeenCalledOnce();
  });

  it('evicts rejected and superseded identities', async () => {
    const cache = new AuthenticatedFheEncryptionKeyBytesCache();
    const rejected = vi.fn(async () => Promise.reject(new Error('bad key')));

    await expect(
      cache.getOrCreate({ scopeKey: 'scope', identityKey: 'v1', metadata: METADATA, fetcher: rejected }),
    ).rejects.toThrow('bad key');
    expect(cache.size).toBe(0);

    await cache.getOrCreate({
      scopeKey: 'scope',
      identityKey: 'v1',
      metadata: METADATA,
      fetcher: async () => keyBytes(1),
    });
    await cache.getOrCreate({
      scopeKey: 'scope',
      identityKey: 'v2',
      metadata: METADATA,
      fetcher: async () => keyBytes(2),
    });
    expect(cache.size).toBe(1);
  });

  it('does not let rejection diagnostics create an unhandled rejection', async () => {
    const cache = new AuthenticatedFheEncryptionKeyBytesCache();
    const onRejected = vi.fn(() => {
      throw new Error('application logger failed');
    });

    await expect(
      cache.getOrCreate({
        scopeKey: 'scope',
        identityKey: 'v1',
        metadata: METADATA,
        fetcher: async () => Promise.reject(new Error('bad key')),
        onRejected,
      }),
    ).rejects.toThrow('bad key');
    await vi.waitFor(() => expect(onRejected).toHaveBeenCalledOnce());
    expect(cache.size).toBe(0);
  });

  it('does not cancel an in-flight caller when the identity is force-refreshed', async () => {
    const cache = new AuthenticatedFheEncryptionKeyBytesCache();
    let resolveFirst!: (bytes: FheEncryptionKeyBytes) => void;
    const first = cache.getOrCreate({
      scopeKey: 'scope',
      identityKey: 'v1',
      metadata: METADATA,
      fetcher: () =>
        new Promise((resolve) => {
          resolveFirst = resolve;
        }),
    });
    const refreshed = cache.getOrCreate({
      scopeKey: 'scope',
      identityKey: 'v1',
      metadata: METADATA,
      ignoreCache: true,
      fetcher: async () => keyBytes(2),
    });

    await Promise.resolve();
    resolveFirst(keyBytes(1));
    await expect(Promise.all([first, refreshed])).resolves.toHaveLength(2);
  });

  it('bounds strong global byte retention to four trust scopes', async () => {
    const cache = new AuthenticatedFheEncryptionKeyBytesCache();
    for (let index = 0; index < 8; index += 1) {
      await cache.getOrCreate({
        scopeKey: `scope-${index.toString()}`,
        identityKey: 'v1',
        metadata: METADATA,
        fetcher: async () => keyBytes(index),
      });
    }
    expect(cache.size).toBe(4);
  });
});

function keyBytes(seed: number): FheEncryptionKeyBytes {
  return {
    publicKeyBytes: { id: 'key', bytes: new Uint8Array([seed]) },
    crsBytes: { id: 'crs', capacity: 2048, bytes: new Uint8Array([seed]) },
    metadata: METADATA,
  };
}
