import { createKeyPairSignerFromBytes } from '@solana/kit';
import { describe, expect, test } from 'vitest';

import { BURNER_WALLET_STORAGE_KEY, loadOrCreateBurnerSecretKey } from './burnerWallet';

const memoryStorage = (initial: Record<string, string> = {}) => {
  const entries = new Map(Object.entries(initial));
  return {
    getItem: (key: string) => entries.get(key) ?? null,
    setItem: (key: string, value: string) => void entries.set(key, value),
    entries,
  };
};

describe('the demo burner wallet', () => {
  test('generates a kit-compatible keypair once and reuses it on the next load', async () => {
    const storage = memoryStorage();
    const first = await loadOrCreateBurnerSecretKey(storage);
    expect(first).toHaveLength(64);
    // kit verifies that the public half matches the seed; a wrong layout throws here.
    const signer = await createKeyPairSignerFromBytes(first);
    expect(signer.address).toHaveLength(44);

    const second = await loadOrCreateBurnerSecretKey(storage);
    expect(second).toEqual(first);
    expect(storage.entries.size).toBe(1);
  });

  test('replaces a corrupt entry instead of failing the session', async () => {
    const storage = memoryStorage({ [BURNER_WALLET_STORAGE_KEY]: '[1,2,3]' });
    const key = await loadOrCreateBurnerSecretKey(storage);
    expect(key).toHaveLength(64);
    expect(storage.getItem(BURNER_WALLET_STORAGE_KEY)).not.toBe('[1,2,3]');
  });
});
