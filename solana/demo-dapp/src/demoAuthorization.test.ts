import { beforeEach, describe, expect, test, vi } from 'vitest';

import { demoApiFetch, demoFaucetFetch } from './demoAuthorization';

beforeEach(() => {
  vi.stubGlobal('window', { location: { origin: 'http://127.0.0.1:5173' } });
});

describe('browser demo requests', () => {
  test('uses the exact same-origin dApp API without browser credentials', async () => {
    const fetcher = vi.fn().mockResolvedValue(new Response('{}'));
    await demoApiFetch('/api/demo-session', { headers: { accept: 'application/json' } }, fetcher);

    expect(fetcher).toHaveBeenCalledWith('http://127.0.0.1:5173/api/demo-session', {
      headers: { accept: 'application/json' },
      credentials: 'omit',
      redirect: 'error',
    });
    const headers = new Headers(fetcher.mock.calls[0]?.[1].headers);
    expect(headers.has('authorization')).toBe(false);
    expect(headers.has('x-fhevm-demo-boot-id')).toBe(false);
  });

  test.each([
    ['/airdrop-sol', 'http://127.0.0.1:5173/api/demo-faucet/airdrop-sol'],
    ['/mint-usdc', 'http://127.0.0.1:5173/api/demo-faucet/mint-usdc'],
  ] as const)('routes %s through the same-origin dApp server', async (path, expectedUrl) => {
    const fetcher = vi.fn().mockResolvedValue(new Response('{}'));
    await demoFaucetFetch(path, { method: 'POST' }, fetcher);
    expect(fetcher).toHaveBeenCalledWith(expectedUrl, {
      method: 'POST',
      credentials: 'omit',
      redirect: 'error',
    });
  });

  test('refuses to send demo requests from another page origin', () => {
    vi.stubGlobal('window', { location: { origin: 'http://localhost:5173' } });
    expect(() => demoApiFetch('/api/demo-session')).toThrow(
      'refusing local demo request from http://localhost:5173',
    );
  });
});
