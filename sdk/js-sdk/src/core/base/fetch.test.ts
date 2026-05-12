import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { getResponseBytes, fetchWithRetry } from './fetch.js';

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
