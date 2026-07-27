import { beforeEach, describe, expect, test, vi } from 'vitest';

import {
  consumeDemoLaunchAuthorization,
  demoApiFetch,
  demoFaucetFetch,
  initializeDemoLaunchAuthorization,
} from './demoAuthorization';

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

  test('consumes a launch capability opened in an already-loaded tab', () => {
    const location = { hash: '', pathname: '/', search: '' };
    const replaceState = vi.fn(() => {
      location.hash = '';
    });
    let onHashChange: (() => void) | undefined;
    const removeEventListener = vi.fn();
    initializeDemoLaunchAuthorization({
      location,
      history: { replaceState },
      addEventListener: (_type, listener) => {
        onHashChange = listener;
      },
      removeEventListener,
    });

    location.hash = `#boot=${bootId}&token=${token}`;
    onHashChange?.();

    expect(replaceState).toHaveBeenCalledWith(null, '', '/');
  });

  test('ignores ordinary anchors without revoking authorization or rewriting navigation', async () => {
    const location = { hash: `#boot=${bootId}&token=${token}`, pathname: '/', search: '' };
    const replaceState = vi.fn(() => {
      location.hash = '';
    });
    let onHashChange: (() => void) | undefined;
    initializeDemoLaunchAuthorization({
      location,
      history: { replaceState },
      addEventListener: (_type, listener) => {
        onHashChange = listener;
      },
      removeEventListener: vi.fn(),
    });
    replaceState.mockClear();
    location.hash = '#deposit';
    onHashChange?.();

    const fetcher = vi.fn().mockResolvedValue(new Response('{}'));
    await demoApiFetch('/api/demo-session', {}, fetcher);
    expect(replaceState).not.toHaveBeenCalled();
    expect(new Headers(fetcher.mock.calls[0]?.[1].headers).get('authorization')).toBe(`Bearer ${token}`);
  });

  test('initializes once per browser and disposes its listener', () => {
    const location = { hash: '', pathname: '/', search: '' };
    const addEventListener = vi.fn();
    const removeEventListener = vi.fn();
    const browser = {
      location,
      history: { replaceState: vi.fn() },
      addEventListener,
      removeEventListener,
    };

    const firstDispose = initializeDemoLaunchAuthorization(browser);
    const secondDispose = initializeDemoLaunchAuthorization(browser);

    expect(secondDispose).toBe(firstDispose);
    expect(addEventListener).toHaveBeenCalledTimes(1);
    firstDispose();
    expect(removeEventListener).toHaveBeenCalledTimes(1);
  });

  test('scrubs malformed launch fragments and fails closed', () => {
    const location = { hash: '#boot=invalid&token=invalid', pathname: '/', search: '' };
    const replaceState = vi.fn(() => {
      location.hash = '';
    });
    initializeDemoLaunchAuthorization({
      location,
      history: { replaceState },
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
    });

    expect(replaceState).toHaveBeenCalledWith(null, '', '/');
    expect(() => demoApiFetch('/api/demo-session')).toThrow('current demo launch URL');
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
