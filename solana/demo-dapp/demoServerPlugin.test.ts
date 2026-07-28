import type { IncomingMessage } from 'node:http';

import { describe, expect, test } from 'vitest';

import { hasDemoPageContext, runSerialized, runSingleFlight } from './demoServerPlugin';

const request = (
  overrides: Partial<{
    readonly method: string;
    readonly remoteAddress: string;
    readonly host: string;
    readonly referer: string | null;
    readonly fetchSite: string;
    readonly fetchDest: string;
    readonly origin: string | null;
  }> = {},
): Pick<IncomingMessage, 'headers' | 'method' | 'socket'> =>
  ({
    method: overrides.method ?? 'GET',
    socket: { remoteAddress: overrides.remoteAddress ?? '127.0.0.1' },
    headers: {
      host: overrides.host ?? '127.0.0.1:5173',
      ...(overrides.referer === null ? {} : { referer: overrides.referer ?? 'http://127.0.0.1:5173/' }),
      'sec-fetch-site': overrides.fetchSite ?? 'same-origin',
      'sec-fetch-dest': overrides.fetchDest ?? 'empty',
      ...(overrides.origin === null ? {} : { origin: overrides.origin ?? 'http://127.0.0.1:5173' }),
    },
  }) as Pick<IncomingMessage, 'headers' | 'method' | 'socket'>;

describe('local demo page boundary', () => {
  test('accepts the exact local dApp page', () => {
    expect(hasDemoPageContext(request())).toBe(true);
    expect(hasDemoPageContext(request({ remoteAddress: '::1' }))).toBe(true);
    expect(hasDemoPageContext(request({ referer: 'http://127.0.0.1:5173/vault?view=private' }))).toBe(true);
  });

  test.each([
    ['a non-loopback peer', { remoteAddress: '192.0.2.1' }],
    ['a different host', { host: 'localhost:5173' }],
    ['a missing referrer', { referer: null }],
    ['a different referrer origin', { referer: 'http://127.0.0.1:4173/' }],
    ['an HTTPS referrer', { referer: 'https://127.0.0.1:5173/' }],
    ['a userinfo referrer', { referer: 'http://user@127.0.0.1:5173/' }],
    ['an origin-suffix referrer', { referer: 'http://127.0.0.1.evil:5173/' }],
    ['a cross-site fetch', { fetchSite: 'cross-site' }],
    ['a navigation', { fetchDest: 'document' }],
    ['a different origin', { origin: 'http://127.0.0.1:4173' }],
    ['a POST without an origin', { method: 'POST', origin: null }],
  ] as const)('rejects %s', (_name, overrides) => {
    expect(hasDemoPageContext(request(overrides))).toBe(false);
  });

  test('accepts a missing Origin header on same-origin GET requests', () => {
    expect(hasDemoPageContext(request({ origin: null }))).toBe(true);
  });
});

describe('local keeper single-flight', () => {
  test('shares one operation across concurrent callers and permits a later lifecycle recheck', async () => {
    const operations = new Map<string, Promise<string>>();
    let starts = 0;
    let resolve!: (value: string) => void;
    const pending = new Promise<string>((resolvePromise) => {
      resolve = resolvePromise;
    });
    const start = () => {
      starts += 1;
      return pending;
    };

    const first = runSingleFlight(operations, 'deposit:batch:dispatch', start);
    const second = runSingleFlight(operations, 'deposit:batch:dispatch', start);
    expect(starts).toBe(1);

    resolve('confirmed');
    await expect(Promise.all([first, second])).resolves.toEqual(['confirmed', 'confirmed']);
    await expect(runSingleFlight(operations, 'deposit:batch:dispatch', async () => 'already advanced')).resolves.toBe(
      'already advanced',
    );
  });

  test('serializes distinct batch preparations that share one registry', async () => {
    const queue = { tail: Promise.resolve() };
    const order: string[] = [];
    let releaseFirst!: () => void;
    const firstPending = new Promise<void>((resolve) => {
      releaseFirst = resolve;
    });
    const first = runSerialized(queue, async () => {
      order.push('deposit:start');
      await firstPending;
      order.push('deposit:end');
      return 'deposit';
    });
    const second = runSerialized(queue, async () => {
      order.push('redeem:start');
      order.push('redeem:end');
      return 'redeem';
    });

    await Promise.resolve();
    expect(order).toEqual(['deposit:start']);
    releaseFirst();
    await expect(Promise.all([first, second])).resolves.toEqual(['deposit', 'redeem']);
    expect(order).toEqual(['deposit:start', 'deposit:end', 'redeem:start', 'redeem:end']);
  });
});
