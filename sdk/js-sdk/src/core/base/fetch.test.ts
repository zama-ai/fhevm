import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { getResponseBytes, fetchWithRetry, normalizeHeaders } from './fetch.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/base/fetch.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('fetchBytes', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
  });

  //////////////////////////////////////////////////////////////////////////////

  it('fetches bytes using arrayBuffer method', async () => {
    const testData = new Uint8Array([1, 2, 3, 4, 5]);
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      arrayBuffer: vi.fn().mockResolvedValue(testData.buffer),
    });

    const response = await fetch('https://example.com/data');
    const result = await getResponseBytes(response);

    expect(result).toEqual(testData);
    expect(global.fetch).toHaveBeenCalledWith('https://example.com/data');
  });

  //////////////////////////////////////////////////////////////////////////////

  it('fetches bytes using bytes method when available', async () => {
    const testData = new Uint8Array([1, 2, 3, 4, 5]);
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      bytes: vi.fn().mockResolvedValue(testData),
    });

    const response = await fetch('https://example.com/data');
    const result = await getResponseBytes(response);

    expect(result).toEqual(testData);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('fetchWithRetry', () => {
  const originalFetch = global.fetch;

  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    global.fetch = originalFetch;
    vi.useRealTimers();
  });

  //////////////////////////////////////////////////////////////////////////////
  // Successful fetch
  //////////////////////////////////////////////////////////////////////////////

  it('returns response on successful fetch', async () => {
    const mockResponse = { ok: true, status: 200 };
    global.fetch = vi.fn().mockResolvedValue(mockResponse);

    const response = await fetchWithRetry({ url: 'https://example.com' });

    expect(response).toBe(mockResponse);
    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  it('returns response on HTTP error (does not retry 4xx/5xx)', async () => {
    const mockResponse = { ok: false, status: 500 };
    global.fetch = vi.fn().mockResolvedValue(mockResponse);

    const response = await fetchWithRetry({ url: 'https://example.com' });

    expect(response).toBe(mockResponse);
    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Retry behavior
  //////////////////////////////////////////////////////////////////////////////

  it('retries on network error and succeeds', async () => {
    const mockResponse = { ok: true, status: 200 };
    const networkError = new TypeError('fetch failed');

    let callCount = 0;
    global.fetch = vi.fn().mockImplementation(() => {
      callCount++;
      if (callCount < 3) {
        return Promise.reject(networkError);
      }
      return Promise.resolve(mockResponse);
    });

    const promise = fetchWithRetry({
      url: 'https://example.com',
      retries: 3,
      retryDelayMs: 100,
    });

    // First attempt fails, wait for retry delay
    await vi.advanceTimersByTimeAsync(100);
    // Second attempt fails, wait for retry delay
    await vi.advanceTimersByTimeAsync(100);
    // Third attempt succeeds

    const response = await promise;

    expect(response).toBe(mockResponse);
    expect(global.fetch).toHaveBeenCalledTimes(3);
  });

  it('throws last error after all retries exhausted', async () => {
    vi.useRealTimers(); // Use real timers for this test

    const networkError = new Error('network failure');
    global.fetch = vi.fn().mockRejectedValue(networkError);

    await expect(
      fetchWithRetry({
        url: 'https://example.com',
        retries: 2,
        retryDelayMs: 100,
      }),
    ).rejects.toThrow('network failure');

    expect(global.fetch).toHaveBeenCalledTimes(3); // initial + 2 retries
  });

  it('does not retry on AbortError', async () => {
    const abortError = new DOMException('Aborted', 'AbortError');
    global.fetch = vi.fn().mockRejectedValue(abortError);

    await expect(fetchWithRetry({ url: 'https://example.com', retries: 3 })).rejects.toThrow(abortError);

    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Abort signal
  //////////////////////////////////////////////////////////////////////////////

  it('throws immediately if signal already aborted', async () => {
    const controller = new AbortController();
    controller.abort();

    global.fetch = vi.fn().mockResolvedValue({ ok: true });

    await expect(
      fetchWithRetry({
        url: 'https://example.com',
        init: { signal: controller.signal },
      }),
    ).rejects.toThrow();

    expect(global.fetch).not.toHaveBeenCalled();
  });

  //////////////////////////////////////////////////////////////////////////////
  // Default values
  //////////////////////////////////////////////////////////////////////////////

  it('uses default retries (3) when not specified', async () => {
    vi.useRealTimers(); // Use real timers for this test

    const networkError = new Error('network failure');
    global.fetch = vi.fn().mockRejectedValue(networkError);

    await expect(
      fetchWithRetry({
        url: 'https://example.com',
        retryDelayMs: 100,
      }),
    ).rejects.toThrow();

    expect(global.fetch).toHaveBeenCalledTimes(4); // initial + 3 retries
  });

  //////////////////////////////////////////////////////////////////////////////
  // Parameter clamping
  //////////////////////////////////////////////////////////////////////////////

  it('clamps retries to max 1000', async () => {
    const mockResponse = { ok: true, status: 200 };
    global.fetch = vi.fn().mockResolvedValue(mockResponse);

    // This should not cause issues - just verify it doesn't throw
    const response = await fetchWithRetry({
      url: 'https://example.com',
      retries: 5000,
    });

    expect(response).toBe(mockResponse);
  });

  it('clamps negative retries to 0', async () => {
    const networkError = new TypeError('fetch failed');
    global.fetch = vi.fn().mockRejectedValue(networkError);

    await expect(
      fetchWithRetry({
        url: 'https://example.com',
        retries: -5,
      }),
    ).rejects.toThrow();

    // With 0 retries, only 1 attempt
    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  it('clamps retryDelayMs to min 100ms', async () => {
    const networkError = new TypeError('fetch failed');
    const mockResponse = { ok: true, status: 200 };

    let callCount = 0;
    global.fetch = vi.fn().mockImplementation(() => {
      callCount++;
      if (callCount === 1) {
        return Promise.reject(networkError);
      }
      return Promise.resolve(mockResponse);
    });

    const promise = fetchWithRetry({
      url: 'https://example.com',
      retries: 1,
      retryDelayMs: 10, // Should be clamped to 100
    });

    // Advance by 50ms - should not have retried yet
    await vi.advanceTimersByTimeAsync(50);
    expect(global.fetch).toHaveBeenCalledTimes(1);

    // Advance to 100ms - should retry now
    await vi.advanceTimersByTimeAsync(50);

    const response = await promise;
    expect(response).toBe(mockResponse);
    expect(global.fetch).toHaveBeenCalledTimes(2);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Request init passthrough
  //////////////////////////////////////////////////////////////////////////////

  it('passes init options to fetch', async () => {
    const mockResponse = { ok: true, status: 200 };
    global.fetch = vi.fn().mockResolvedValue(mockResponse);

    const init: RequestInit = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ test: true }),
    };

    await fetchWithRetry({ url: 'https://example.com', init });

    expect(global.fetch).toHaveBeenCalledWith('https://example.com', init);
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('normalizeHeaders', () => {
  //////////////////////////////////////////////////////////////////////////////
  // Cases that return a value
  //////////////////////////////////////////////////////////////////////////////

  const cases: ReadonlyArray<{ name: string; input: unknown; expected: Record<string, string> }> = [
    // Non-object inputs → {}
    { name: 'undefined', input: undefined, expected: {} },
    { name: 'null', input: null, expected: {} },
    { name: 'number', input: 42, expected: {} },
    { name: 'string', input: 'foo', expected: {} },
    { name: 'boolean', input: true, expected: {} },
    { name: 'array (empty)', input: [], expected: {} },
    { name: 'array (entries-shaped)', input: [['a', 'b']], expected: {} },

    // Empty / simple objects
    { name: 'empty object', input: {}, expected: {} },
    { name: 'single string entry', input: { foo: 'bar' }, expected: { foo: 'bar' } },

    // Key lower-casing
    {
      name: 'mixed-case keys lower-cased',
      input: { 'Content-Type': 'application/json' },
      expected: { 'content-type': 'application/json' },
    },
    { name: 'all-upper key', input: { AUTHORIZATION: 'Bearer x' }, expected: { authorization: 'Bearer x' } },

    // Non-string values dropped — one row per `typeof` category
    { name: 'undefined value dropped', input: { foo: undefined, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'null value dropped', input: { foo: null, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'number value dropped', input: { foo: 42, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'NaN value dropped', input: { foo: NaN, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'Infinity value dropped', input: { foo: Infinity, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'bigint value dropped', input: { foo: 10n, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'boolean true dropped', input: { foo: true, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'boolean false dropped', input: { foo: false, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'symbol value dropped', input: { foo: Symbol('s'), bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'function value dropped', input: { foo: () => {}, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'async function value dropped', input: { foo: async () => {}, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'class value dropped', input: { foo: class Foo {}, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'object value dropped', input: { foo: { nested: 1 }, bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'array value dropped', input: { foo: [1, 2, 3], bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'Date value dropped', input: { foo: new Date(), bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'Map value dropped', input: { foo: new Map(), bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'Set value dropped', input: { foo: new Set(), bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'RegExp value dropped', input: { foo: /x/, bar: 'keep' }, expected: { bar: 'keep' } },
    {
      name: 'Uint8Array value dropped',
      input: { foo: new Uint8Array([1, 2]), bar: 'keep' },
      expected: { bar: 'keep' },
    },
    { name: 'String wrapper object dropped', input: { foo: new String('x'), bar: 'keep' }, expected: { bar: 'keep' } },
    { name: 'all values dropped → empty result', input: { a: 1, b: null, c: undefined }, expected: {} },
    { name: 'empty-string value kept', input: { foo: '' }, expected: { foo: '' } },
    { name: 'whitespace-only string kept', input: { foo: '   ' }, expected: { foo: '   ' } },
    { name: 'numeric-looking string kept', input: { foo: '42' }, expected: { foo: '42' } },

    // Null-prototype objects pass through (typeof === 'object', not an array)
    {
      name: 'null-prototype object',
      input: Object.assign(Object.create(null) as Record<string, unknown>, { Foo: 'bar' }),
      expected: { foo: 'bar' },
    },
  ];

  it.each(cases)('$name', ({ input, expected }) => {
    expect(normalizeHeaders(input)).toEqual(expected);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Cases that throw — duplicate keys after lower-casing
  //////////////////////////////////////////////////////////////////////////////

  const throwingCases: ReadonlyArray<{ name: string; input: unknown; errorMatch: RegExp }> = [
    {
      name: 'two keys differing only in case',
      input: { Foo: 'a', foo: 'b' },
      errorMatch: /keys 'Foo' and 'foo' both lower-case to 'foo'/,
    },
    {
      name: 'collision when second value is non-string (check runs before filter)',
      input: { Authorization: 'Bearer x', authorization: undefined },
      errorMatch: /keys 'Authorization' and 'authorization' both lower-case to 'authorization'/,
    },
    {
      name: 'three-way collision reports first duplicate',
      input: { 'Content-Type': 'a', 'content-type': 'b', 'CONTENT-TYPE': 'c' },
      errorMatch: /keys 'Content-Type' and 'content-type' both lower-case to 'content-type'/,
    },

    // Invalid header name (RFC 7230 token violations)
    {
      name: 'CRLF injection in name',
      input: { 'X-Bad\r\nInject': 'x' },
      errorMatch: /invalid header name/,
    },
    {
      name: 'space in name',
      input: { 'X Bad': 'x' },
      errorMatch: /invalid header name/,
    },
    {
      name: 'colon in name',
      input: { 'X:Bad': 'x' },
      errorMatch: /invalid header name/,
    },
    {
      name: 'tab in name',
      input: { 'X\tBad': 'x' },
      errorMatch: /invalid header name/,
    },
    {
      name: 'non-ASCII (unicode) in name',
      input: { 'X-Café': 'x' },
      errorMatch: /invalid header name/,
    },
    {
      name: 'empty-string name',
      input: { '': 'x' },
      errorMatch: /invalid header name/,
    },
    {
      name: 'parentheses in name',
      input: { 'X-(paren)': 'x' },
      errorMatch: /invalid header name/,
    },

    // Invalid header value (CRLF / NUL injection)
    {
      name: 'CR in value',
      input: { 'x-foo': 'bad\rinject' },
      errorMatch: /CR, LF, or NUL/,
    },
    {
      name: 'LF in value',
      input: { 'x-foo': 'bad\ninject' },
      errorMatch: /CR, LF, or NUL/,
    },
    {
      name: 'CRLF in value (header smuggling)',
      input: { 'x-foo': 'bad\r\nAuthorization: Bearer evil' },
      errorMatch: /CR, LF, or NUL/,
    },
    {
      name: 'NUL byte in value',
      input: { 'x-foo': 'bad\0inject' },
      errorMatch: /CR, LF, or NUL/,
    },
  ];

  it.each(throwingCases)('throws: $name', ({ input, errorMatch }) => {
    expect(() => normalizeHeaders(input)).toThrow(errorMatch);
  });

  //////////////////////////////////////////////////////////////////////////////
  // Valid uncommon name characters — confirm the regex doesn't over-reject
  //////////////////////////////////////////////////////////////////////////////

  const validUncommonNames = [
    'x-token-1.0', // dot + digit
    'x_test',
    'x-test!',
    'x#test',
    'x-test*',
    'x-test^h',
    'x-test|h',
    'x-test~h',
    'x-test`h',
    "x-test'h",
  ];

  it.each(validUncommonNames)('accepts RFC 7230 token char: %s', (name) => {
    expect(normalizeHeaders({ [name]: 'ok' })).toEqual({ [name.toLowerCase()]: 'ok' });
  });

  //////////////////////////////////////////////////////////////////////////////
  // Result is a fresh object (not the input)
  //////////////////////////////////////////////////////////////////////////////

  it('returns a fresh object — mutating the result does not affect the input', () => {
    const input = { Foo: 'bar' };
    const result = normalizeHeaders(input) as Record<string, string>;
    result.foo = 'mutated';
    expect(input).toEqual({ Foo: 'bar' });
  });
});
