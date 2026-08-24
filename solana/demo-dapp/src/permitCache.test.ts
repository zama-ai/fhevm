import { afterEach, describe, expect, test, vi } from 'vitest';

import type { SolanaPermitSession } from '@fhevm/sdk/solana';
import {
  PERMIT_REUSE_SAFETY_MARGIN_SECONDS,
  clearPermitCache,
  permitSessionFor,
  type PermitCacheKey,
} from './permitCache';

const NOW = 1_700_000_000n;

const sessionWithWindow = (startTimestamp: bigint, durationSeconds: bigint): SolanaPermitSession =>
  ({ signedPermit: { fields: { startTimestamp, durationSeconds } } }) as unknown as SolanaPermitSession;

const signerOf = (session: SolanaPermitSession) => vi.fn(() => Promise.resolve(session));

const key = (overrides: Partial<PermitCacheKey> = {}): PermitCacheKey => ({
  walletAddress: 'A1iceWa11etAddress11111111111111111111111111',
  chainId: '42',
  domainKey: '0xd0',
  kmsContextId: '0xc0',
  kmsEpochId: '0xe0',
  ...overrides,
});

afterEach(() => {
  clearPermitCache();
});

describe('permitSessionFor', () => {
  // The review's acceptance shape: two user decrypts, one wallet confirmation.
  test('answers two reveals under one key with one signature', async () => {
    const sign = signerOf(sessionWithWindow(NOW, 3_600n));

    const first = await permitSessionFor(key(), sign, NOW);
    const second = await permitSessionFor(key(), sign, NOW + 10n);

    expect(sign).toHaveBeenCalledTimes(1);
    expect(second).toBe(first);
  });

  test.each([
    ['wallet', { walletAddress: 'BobWa11etAddress1111111111111111111111111111' }],
    ['chain', { chainId: '43' }],
    ['domain', { domainKey: '0xd1' }],
    ['KMS context', { kmsContextId: '0xc1' }],
    ['KMS epoch', { kmsEpochId: '0xe1' }],
  ] as const)('a different %s is a different permit', async (_name, overrides) => {
    const sign = signerOf(sessionWithWindow(NOW, 3_600n));

    await permitSessionFor(key(), sign, NOW);
    await permitSessionFor(key(overrides), sign, NOW);

    expect(sign).toHaveBeenCalledTimes(2);
  });

  test('signs again once the window is inside the safety margin', async () => {
    const duration = 3_600n;
    const sign = signerOf(sessionWithWindow(NOW, duration));

    await permitSessionFor(key(), sign, NOW);
    // Still comfortably inside the window: reused.
    await permitSessionFor(key(), sign, NOW + duration - PERMIT_REUSE_SAFETY_MARGIN_SECONDS - 1n);
    expect(sign).toHaveBeenCalledTimes(1);
    // At the margin, a decrypt started now could outlive the permit: signed again.
    await permitSessionFor(key(), sign, NOW + duration - PERMIT_REUSE_SAFETY_MARGIN_SECONDS);
    expect(sign).toHaveBeenCalledTimes(2);
  });

  test('does not reuse a permit whose window has not started', async () => {
    const sign = signerOf(sessionWithWindow(NOW + 100n, 3_600n));

    await permitSessionFor(key(), sign, NOW);
    await permitSessionFor(key(), sign, NOW);

    expect(sign).toHaveBeenCalledTimes(2);
  });

  test('validity is read from the signed fields, not from when it was cached', async () => {
    // A permit signed with a window that is already spent must not be served even once.
    const spent = sessionWithWindow(NOW - 7_200n, 3_600n);
    const fresh = sessionWithWindow(NOW, 3_600n);
    const sign = vi
      .fn<() => Promise<SolanaPermitSession>>()
      .mockResolvedValueOnce(spent)
      .mockResolvedValueOnce(fresh);

    await permitSessionFor(key(), sign, NOW);
    const served = await permitSessionFor(key(), sign, NOW);

    expect(sign).toHaveBeenCalledTimes(2);
    expect(served).toBe(fresh);
  });

  test('clearPermitCache drops every cached permit', async () => {
    const sign = signerOf(sessionWithWindow(NOW, 3_600n));

    await permitSessionFor(key(), sign, NOW);
    clearPermitCache();
    await permitSessionFor(key(), sign, NOW);

    expect(sign).toHaveBeenCalledTimes(2);
  });
});
