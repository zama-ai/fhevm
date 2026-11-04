import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerAsyncRequest } from './RelayerAsyncRequest.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/RelayerAsyncRequest.query.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer.example.com/v2/public-decrypt';
const QUERY = { zama_sdk_version: '1.2.3', zama_sdk_operation: 'decrypt values' };
const QUERY_STRING = 'zama_sdk_version=1.2.3&zama_sdk_operation=decrypt%20values';

function newRequest(): RelayerAsyncRequest {
  return new RelayerAsyncRequest({
    relayerOperation: 'PUBLIC_DECRYPT',
    url: RELAYER_URL,
    payload: {},
    options: { fetchRetries: 1, query: QUERY },
  });
}

// `options.query` exists for proxies that strip custom headers before they reach the relayer.
describe('RelayerAsyncRequest query option', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('appends `options.query` to the submit url', async () => {
    const fetchMock = vi.fn().mockImplementation(() => Promise.resolve(new Response('', { status: 401 })));
    global.fetch = fetchMock;

    await expect(newRequest().run()).rejects.toThrow();
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(fetchMock).toHaveBeenCalledWith(`${RELAYER_URL}?${QUERY_STRING}`, expect.anything());
  });

  it('appends `options.query` to the poll url', async () => {
    vi.useFakeTimers();
    const queued = JSON.stringify({ status: 'queued', requestId: 'req-1', result: { jobId: 'job-1' } });
    const fetchMock = vi
      .fn()
      .mockImplementationOnce(() =>
        Promise.resolve(new Response(queued, { status: 202, headers: { 'Retry-After': '1' } })),
      )
      .mockImplementation(() => Promise.resolve(new Response('', { status: 401 })));
    global.fetch = fetchMock;

    const run = newRequest().run();
    const rejects = expect(run).rejects.toThrow();
    await vi.advanceTimersByTimeAsync(1000);
    await rejects;

    expect(fetchMock).toHaveBeenCalledTimes(2);
    expect(fetchMock).toHaveBeenLastCalledWith(`${RELAYER_URL}/job-1?${QUERY_STRING}`, expect.anything());
  });

  it('drops non-string query values and ignores later changes to the caller object', async () => {
    const fetchMock = vi.fn().mockImplementation(() => Promise.resolve(new Response('', { status: 401 })));
    global.fetch = fetchMock;

    // Plain JS callers may pass anything; only string values must be sent.
    const query = { a: '1', b: undefined, c: 1 } as unknown as Record<string, string>;
    const request = new RelayerAsyncRequest({
      relayerOperation: 'PUBLIC_DECRYPT',
      url: RELAYER_URL,
      payload: {},
      options: { fetchRetries: 1, query },
    });
    query['a'] = 'changed';

    await expect(request.run()).rejects.toThrow();
    expect(fetchMock).toHaveBeenCalledWith(`${RELAYER_URL}?a=1`, expect.anything());
  });
});
