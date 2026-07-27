import { describe, expect, test, vi } from 'vitest';

import type { DemoSession } from './demoSession';
import { withWalletMutationLock } from './mutationLock';

const session = {
  config: { chainId: '42' },
  signer: { address: '11111111111111111111111111111111' },
  assertActive: vi.fn(),
} as unknown as DemoSession;

describe('withWalletMutationLock', () => {
  test('holds a chain-and-wallet-scoped exclusive lock around the action', async () => {
    const request = vi.fn(async (name: string, options: LockOptions, callback: (lock: Lock | null) => Promise<string>) => {
      expect(name).toBe('fhevm-solana-demo:mutation:42:11111111111111111111111111111111');
      expect(options).toEqual({ ifAvailable: true, mode: 'exclusive' });
      return callback({} as Lock);
    });
    await expect(
      withWalletMutationLock(session, async () => 'done', { request } as unknown as LockManager),
    ).resolves.toBe('done');
    expect(session.assertActive).toHaveBeenCalledTimes(2);
  });

  test('fails fast when another tab owns the wallet mutation lock', async () => {
    const request = vi.fn(async (_name: string, _options: LockOptions, callback: (lock: Lock | null) => Promise<string>) =>
      callback(null),
    );
    await expect(
      withWalletMutationLock(session, async () => 'unreachable', { request } as unknown as LockManager),
    ).rejects.toThrow('already has an action running in another tab');
  });

  test('fails closed when the browser cannot coordinate tabs', async () => {
    await expect(withWalletMutationLock(session, async () => 'unreachable', null)).rejects.toThrow(
      'cannot safely coordinate',
    );
  });
});
