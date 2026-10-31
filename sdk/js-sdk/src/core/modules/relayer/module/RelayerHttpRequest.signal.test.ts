import type { MockInstance } from 'vitest';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from './RelayerHttpRequest.js';
import { RelayerHttpTimeoutError } from '../../../errors/RelayerHttpTimeoutError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/RelayerHttpRequest.signal.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';

function successBody(): string {
  return JSON.stringify({
    status: 'succeeded',
    requestId: VALID_REQUEST_ID,
    result: [{ payload: 'ab'.repeat(32), signature: 'cd'.repeat(65), extraData: '0x00' }],
  });
}

function errorBody(): string {
  return JSON.stringify({ code: 'acl_denied', message: 'nope', requestId: VALID_REQUEST_ID });
}

// A caller-supplied AbortSignal may be long-lived (e.g. shared across many requests),
// so RelayerHttpRequest must not leave its own 'abort' listener attached once a
// request settles — only `once: true` self-removal (i.e. the signal actually
// firing) does not cover the success/non-aborted-failure paths.
describe('RelayerHttpRequest external signal cleanup', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('removes its abort listener from the caller-supplied signal on success', async () => {
    global.fetch = vi.fn().mockResolvedValue(new Response(successBody(), { status: 200 }));

    const controller = new AbortController();
    const addSpy = vi.spyOn(controller.signal, 'addEventListener');
    const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal },
    });

    await request.run();

    expect(addSpy).toHaveBeenCalledTimes(1);
    const [, handler] = addSpy.mock.calls[0]!;
    expect(removeSpy).toHaveBeenCalledTimes(1);
    expect(removeSpy).toHaveBeenCalledWith('abort', handler);
  });

  it('removes its abort listener from the caller-supplied signal on a (non-abort) failure', async () => {
    global.fetch = vi.fn().mockResolvedValue(new Response(errorBody(), { status: 403 }));

    const controller = new AbortController();
    const addSpy = vi.spyOn(controller.signal, 'addEventListener');
    const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal },
    });

    await expect(request.run()).rejects.toThrow();

    expect(addSpy).toHaveBeenCalledTimes(1);
    const [, handler] = addSpy.mock.calls[0]!;
    expect(removeSpy).toHaveBeenCalledTimes(1);
    expect(removeSpy).toHaveBeenCalledWith('abort', handler);
  });

  // Bug: _createCombinedSignal() runs before the try/finally, so a throw while
  // building the request init (here JSON.stringify on a BigInt) skips cleanup().
  // Both tests below share this setup; each checks one half of the leak.
  // Fix-agnostic: whichever way the fix goes (signal created later, or the build
  // moved inside the try), no listener and no timer may outlive run().
  async function runWithThrowingBuild(): Promise<{
    addSpy: MockInstance<AbortSignal['addEventListener']>;
    removeSpy: MockInstance<AbortSignal['removeEventListener']>;
  }> {
    vi.useFakeTimers();
    const fetchMock = vi.fn();
    global.fetch = fetchMock;

    const controller = new AbortController();
    const addSpy = vi.spyOn(controller.signal, 'addEventListener');
    const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: { value: 1n },
      options: { signal: controller.signal, timeout: 60_000 },
    });

    await expect(request.run()).rejects.toThrow();
    expect(fetchMock).not.toHaveBeenCalled();

    return { addSpy, removeSpy };
  }

  it('leaves no timer pending when building the request throws', async () => {
    await runWithThrowingBuild();

    expect(vi.getTimerCount()).toBe(0);
  });

  it('leaves no listener on the caller-supplied signal when building the request throws', async () => {
    const { addSpy, removeSpy } = await runWithThrowingBuild();

    expect(removeSpy).toHaveBeenCalledTimes(addSpy.mock.calls.length);
    for (const [type, handler] of addSpy.mock.calls) {
      expect(removeSpy).toHaveBeenCalledWith(type, handler);
    }
  });

  // The timeout timer must never outlive run(), however the request settles.
  it.each([
    {
      outcome: 'success',
      fetch: (): typeof fetch => vi.fn().mockResolvedValue(new Response(successBody(), { status: 200 })),
      abort: undefined,
    },
    {
      outcome: 'an error response',
      fetch: (): typeof fetch => vi.fn().mockResolvedValue(new Response(errorBody(), { status: 403 })),
      abort: undefined,
    },
    {
      outcome: 'a caller cancellation',
      fetch: pendingUntilAbortFetch,
      abort: 'user left',
    },
  ])('leaves no timer pending after $outcome', async ({ fetch: makeFetch, abort }) => {
    vi.useFakeTimers();
    global.fetch = makeFetch();

    const controller = new AbortController();
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal, timeout: 60_000 },
    });

    const run = request.run().catch((e: unknown) => e);
    if (abort !== undefined) {
      controller.abort(abort);
    }
    await run;

    expect(vi.getTimerCount()).toBe(0);
  });

  it('does not touch the signal at all when none is provided', async () => {
    global.fetch = vi.fn().mockResolvedValue(new Response(successBody(), { status: 200 }));

    const request = new RelayerHttpRequest({ url: RELAYER_URL, payload: {} });

    await expect(request.run()).resolves.toBeDefined();
  });
});

// A fetch that never answers on its own: it rejects with signal.reason once the
// request's signal aborts, as the real fetch does.
function pendingUntilAbortFetch(): typeof fetch {
  return vi.fn(
    (_url: unknown, init?: RequestInit) =>
      new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener(
          'abort',
          () => {
            reject(init.signal?.reason);
          },
          { once: true },
        );
      }),
  ) as unknown as typeof fetch;
}

// A fetch that answers with headers at once, but whose body never finishes: the
// body stream errors with signal.reason once the request's signal aborts, as the
// real fetch does when aborted mid-body.
function stalledBodyFetch(): typeof fetch {
  return vi.fn((_url: unknown, init?: RequestInit) => {
    const body = new ReadableStream<Uint8Array>({
      start(controller) {
        init?.signal?.addEventListener(
          'abort',
          () => {
            controller.error(init.signal?.reason);
          },
          { once: true },
        );
      },
    });
    return Promise.resolve(new Response(body, { status: 200 }));
  }) as unknown as typeof fetch;
}

// Bug: aborts are classified by the error's `name` alone. fetch rejects with
// signal.reason itself, so the classification depends on what the caller passed
// to abort(), not on which mechanism actually aborted the request.
describe('RelayerHttpRequest abort classification', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('rethrows the caller reason when the caller signal is already aborted with a string', async () => {
    const fetchMock = vi.fn();
    global.fetch = fetchMock;

    const controller = new AbortController();
    controller.abort('gone');
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal },
    });

    expect(await request.run().catch((e: unknown) => e)).toBe('gone');
    expect(fetchMock).not.toHaveBeenCalled();
  });

  // throwIfAborted() bypasses fetchWithRetry's catch, so null reaches the
  // RelayerHttpRequest catch directly instead of crashing in fetchWithRetry first.
  it('rethrows the caller reason when the caller signal is already aborted with null', async () => {
    const fetchMock = vi.fn();
    global.fetch = fetchMock;

    const controller = new AbortController();
    controller.abort(null);
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal },
    });

    const error = await request.run().catch((e: unknown) => e);

    expect(fetchMock).not.toHaveBeenCalled();
    expect(error).toBeNull();
  });

  it('rethrows the caller reason when the caller aborts with a string while the body is being read', async () => {
    global.fetch = stalledBodyFetch();

    const controller = new AbortController();
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal },
    });

    const run = request.run().catch((e: unknown) => e);
    controller.abort('user left');

    expect(await run).toBe('user left');
  });

  // During the retry delay, abortableSleep rejects with a wrapper Error named
  // 'AbortError' (the caller's reason is only its `cause`). Only a fix that
  // rethrows signal.reason, not the caught error, returns the caller's reason here.
  it('rethrows the caller reason when the caller aborts with a string during the delay between retries', async () => {
    vi.useFakeTimers();
    const fetchMock = vi.fn().mockRejectedValue(new TypeError('fetch failed'));
    global.fetch = fetchMock;

    const controller = new AbortController();
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal, fetchRetries: 3, fetchRetryDelayInMilliseconds: 1000 },
    });

    const run = request.run().catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(10);
    controller.abort('user left');

    expect(await run).toBe('user left');
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  // The caller's own timeout (e.g. AbortSignal.timeout()) aborts with a reason
  // named 'TimeoutError' too. It is the caller's, not ours, so it must come back
  // unchanged, never as RelayerHttpTimeoutError, in every phase. A manual
  // controller stands in for AbortSignal.timeout() to keep the timing exact.

  it("rethrows the caller's own TimeoutError reason while the body is being read", async () => {
    global.fetch = stalledBodyFetch();

    const controller = new AbortController();
    const callerReason = new DOMException('Caller timed out', 'TimeoutError');
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal, timeout: 60_000 },
    });

    const run = request.run().catch((e: unknown) => e);
    controller.abort(callerReason);

    expect(await run).toBe(callerReason);
  });

  it("rethrows the caller's own TimeoutError reason during the delay between retries", async () => {
    vi.useFakeTimers();
    const fetchMock = vi.fn().mockRejectedValue(new TypeError('fetch failed'));
    global.fetch = fetchMock;

    const controller = new AbortController();
    const callerReason = new DOMException('Caller timed out', 'TimeoutError');
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal, timeout: 60_000, fetchRetries: 3, fetchRetryDelayInMilliseconds: 1000 },
    });

    const run = request.run().catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(10);
    controller.abort(callerReason);

    expect(await run).toBe(callerReason);
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  // Regression guards for the fix: our own options.timeout must keep producing
  // RelayerHttpTimeoutError, whichever phase it fires in. These pass today.

  it('reports our own timeout as RelayerHttpTimeoutError while fetch is in flight', async () => {
    vi.useFakeTimers();
    global.fetch = pendingUntilAbortFetch();

    const request = new RelayerHttpRequest({ url: RELAYER_URL, payload: {}, options: { timeout: 500 } });

    const run = request.run().catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(500);

    const error = await run;
    expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
    expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(500);
  });

  it('reports our own timeout as RelayerHttpTimeoutError during the delay between retries', async () => {
    vi.useFakeTimers();
    const fetchMock = vi.fn().mockRejectedValue(new TypeError('fetch failed'));
    global.fetch = fetchMock;

    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { timeout: 500, fetchRetries: 3, fetchRetryDelayInMilliseconds: 1000 },
    });

    const run = request.run().catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(500);

    expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
    expect(fetchMock).toHaveBeenCalledTimes(1);
  });

  it('reports our own timeout as RelayerHttpTimeoutError while the body is being read', async () => {
    vi.useFakeTimers();
    global.fetch = stalledBodyFetch();

    const request = new RelayerHttpRequest({ url: RELAYER_URL, payload: {}, options: { timeout: 500 } });

    const run = request.run().catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(500);

    expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
  });

  it('rethrows the caller reason when the caller aborts with a string', async () => {
    global.fetch = pendingUntilAbortFetch();

    const controller = new AbortController();
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal },
    });

    const run = request.run().catch((e: unknown) => e);
    controller.abort('user left');

    expect(await run).toBe('user left');
  });

  it('rethrows the caller reason when the caller aborts with null', async () => {
    global.fetch = pendingUntilAbortFetch();

    const controller = new AbortController();
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal },
    });

    const run = request.run().catch((e: unknown) => e);
    controller.abort(null);

    expect(await run).toBeNull();
  });

  it("does not report the caller's own AbortSignal.timeout() as RelayerHttpTimeoutError", async () => {
    global.fetch = pendingUntilAbortFetch();

    const callerSignal = AbortSignal.timeout(10);
    const request = new RelayerHttpRequest({
      url: RELAYER_URL,
      payload: {},
      options: { signal: callerSignal, timeout: 60_000 },
    });

    const error = await request.run().catch((e: unknown) => e);

    expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
    expect(error).toBe(callerSignal.reason);
  });
});
