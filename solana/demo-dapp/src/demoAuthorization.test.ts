import { describe, expect, test, vi } from 'vitest';

import { demoApiFetch, demoFaucetFetch } from './demoAuthorization';

describe('browser demo requests', () => {
  test('calls the same-origin dApp API without browser credentials or redirects', async () => {
    const fetcher = vi.fn().mockResolvedValue(new Response('{}'));
    await demoApiFetch('/api/demo-config', { headers: { accept: 'application/json' } }, fetcher);

    expect(fetcher).toHaveBeenCalledWith('/api/demo-config', {
      headers: { accept: 'application/json' },
      credentials: 'omit',
      redirect: 'error',
    });
    const headers = new Headers(fetcher.mock.calls[0]?.[1].headers);
    expect(headers.has('authorization')).toBe(false);
    expect(headers.has('x-fhevm-demo-boot-id')).toBe(false);
  });

  test.each([
    ['/airdrop-sol', '/api/demo-faucet/airdrop-sol'],
    ['/mint-usdc', '/api/demo-faucet/mint-usdc'],
  ] as const)('routes %s through the same-origin dApp server', async (path, expectedUrl) => {
    const fetcher = vi.fn().mockResolvedValue(new Response('{}'));
    await demoFaucetFetch(path, { method: 'POST' }, fetcher);
    expect(fetcher).toHaveBeenCalledWith(expectedUrl, {
      method: 'POST',
      credentials: 'omit',
      redirect: 'error',
    });
  });
});
