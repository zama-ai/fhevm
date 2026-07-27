import { beforeEach, describe, expect, test, vi } from 'vitest';

import { consumeDemoLaunchAuthorization, demoApiFetch, demoFaucetFetch } from './demoAuthorization';

const bootId = '123e4567-e89b-42d3-a456-426614174000';
const token = 'A'.repeat(43);

beforeEach(() => {
  vi.stubGlobal('window', { location: { origin: 'http://127.0.0.1:5173' } });
  consumeDemoLaunchAuthorization({ hash: '', pathname: '/', search: '' }, { replaceState: vi.fn() });
});

describe('browser demo authorization', () => {
  test('keeps a valid launch capability only in memory and scrubs the fragment', () => {
    const replaceState = vi.fn();
    consumeDemoLaunchAuthorization(
      { hash: `#boot=${bootId}&token=${token}`, pathname: '/vault', search: '?view=private' },
      { replaceState },
    );
    expect(replaceState).toHaveBeenCalledWith(null, '', '/vault?view=private');
  });

  test('injects authorization only into exact demo API and faucet origins', async () => {
    consumeDemoLaunchAuthorization(
      { hash: `#boot=${bootId}&token=${token}`, pathname: '/', search: '' },
      { replaceState: vi.fn() },
    );
    const fetcher = vi.fn().mockResolvedValue(new Response('{}'));
    await demoApiFetch('/api/demo-session', {}, fetcher);
    await demoFaucetFetch('/mint-usdc', { method: 'POST' }, fetcher);
    for (const [url, init] of fetcher.mock.calls as [string, RequestInit][]) {
      expect(new URL(url).origin).toMatch(/^http:\/\/127\.0\.0\.1:(5173|8090)$/);
      const headers = new Headers(init.headers);
      expect(headers.get('authorization')).toBe(`Bearer ${token}`);
      expect(headers.get('x-fhevm-demo-boot-id')).toBe(bootId);
    }
  });

  test('fails closed after an invalid or missing launch fragment', () => {
    consumeDemoLaunchAuthorization(
      { hash: '#boot=old&token=wrong', pathname: '/', search: '' },
      { replaceState: vi.fn() },
    );
    expect(() => demoApiFetch('/api/demo-session')).toThrow('current demo launch URL');
    expect(() => demoFaucetFetch('/airdrop-sol')).toThrow('current demo launch URL');
  });
});
