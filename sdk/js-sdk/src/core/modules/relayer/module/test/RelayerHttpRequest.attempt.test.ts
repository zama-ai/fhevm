import type { Mock } from 'vitest';
import type { Logger } from '../../../../types/logger.js';
import type { RelayerHttpAttemptParams, RelayerHttpOperation } from '../RelayerHttpAttempt.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpAttempt } from '../RelayerHttpAttempt.js';
import { RelayerHttpFetchError } from '../../../../errors/RelayerHttpFetchError.js';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpResultError } from '../../../../errors/RelayerHttpResultError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../../errors/RelayerHttpUnexpectedResponseError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.attempt.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '9b2c1f4e-3a5d-4e6f-8a7b-1c2d3e4f5a6b';
const MAX_BODY_TEXT_LENGTH = 10 * 1024 * 1024;

const USER_DECRYPT_SHARES = [{ payload: 'ab'.repeat(32), signature: 'cd'.repeat(65), extraData: '0x00' }];
const PUBLIC_DECRYPT_RESULT = {
  decryptedValue: '00'.repeat(31) + '2a',
  signatures: ['ab'.repeat(65)],
  extraData: '0x00',
};

function successBody(operation: RelayerHttpOperation): string {
  const result = operation === 'USER_DECRYPT' ? { result: USER_DECRYPT_SHARES } : PUBLIC_DECRYPT_RESULT;
  return JSON.stringify({ status: 'succeeded', requestId: VALID_REQUEST_ID, result });
}

function errorBody(): string {
  return JSON.stringify({ code: 'overloaded', message: 'busy', requestId: VALID_REQUEST_ID });
}

type TestLogger = Logger & { debug: Mock; warn: Mock; error: Mock };
function makeLogger(): TestLogger {
  return { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
}

// A fresh Response per fetch call: a body reads once.
function respondWith(body: () => string | ReadableStream<Uint8Array>, status: number): Mock {
  return vi.fn(() => Promise.resolve(new Response(body(), { status })));
}

// Never answers on its own; rejects with signal.reason once the given signal aborts, as fetch does.
function pendingUntilAbortFetch(): Mock {
  return vi.fn(
    (_url: unknown, init?: RequestInit) =>
      new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener('abort', () => reject(init.signal?.reason), { once: true });
      }),
  );
}

// Headers at once; the body stream errors with signal.reason once the signal aborts.
function stalledBodyFetch(): Mock {
  return vi.fn((_url: unknown, init?: RequestInit) => {
    const body = new ReadableStream<Uint8Array>({
      start(controller) {
        init?.signal?.addEventListener('abort', () => controller.error(init.signal?.reason), { once: true });
      },
    });
    return Promise.resolve(new Response(body, { status: 200 }));
  });
}

const BASE_INIT: Readonly<Omit<RequestInit, 'signal'>> = Object.freeze({
  method: 'POST',
  headers: Object.freeze({ 'content-type': 'application/json', 'x-test': 'yes' }),
  body: '{"a":1}',
});

function makeAttempt<O extends RelayerHttpOperation>(
  overrides: Partial<RelayerHttpAttemptParams<O>> & { relayerOperation: O },
): { attempt: RelayerHttpAttempt<O>; logger: TestLogger; controller: AbortController } {
  const logger = makeLogger();
  const controller = new AbortController();
  const params: RelayerHttpAttemptParams<O> = {
    url: RELAYER_URL,
    query: undefined,
    init: BASE_INIT,
    signal: controller.signal,
    timedOutAfterMs: () => undefined,
    fetchRetries: 0,
    fetchRetryDelayInMilliseconds: undefined,
    logger,
    ...overrides,
  };
  return { attempt: new RelayerHttpAttempt(params), logger, controller };
}

const OPERATIONS: RelayerHttpOperation[] = ['USER_DECRYPT', 'PUBLIC_DECRYPT'];

describe('RelayerHttpAttempt', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  describe.each(OPERATIONS)('%s', (relayerOperation) => {
    // A 200 with a valid success body: run() resolves to the validated result, nothing logged.
    it('returns the validated result of a 200 success body', async () => {
      global.fetch = respondWith(() => successBody(relayerOperation), 200);
      const { attempt, logger } = makeAttempt({ relayerOperation });

      const expected = relayerOperation === 'USER_DECRYPT' ? USER_DECRYPT_SHARES : PUBLIC_DECRYPT_RESULT;
      await expect(attempt.run()).resolves.toEqual(expected);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(logger.error).not.toHaveBeenCalled();
    });

    // A relayer-http error body (503): RelayerHttpResponseError with its fields, not logged.
    it('throws RelayerHttpResponseError for a relayer-http error body, WITHOUT logging it', async () => {
      global.fetch = respondWith(errorBody, 503);
      const { attempt, logger } = makeAttempt({ relayerOperation });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpResponseError);
      const responseError = error as RelayerHttpResponseError;
      expect(responseError.url).toBe(RELAYER_URL);
      expect(responseError.status).toBe(503);
      expect(responseError.code).toBe('overloaded');
      expect(responseError.responseMessage).toBe('busy');
      expect(responseError.requestId).toBe(VALID_REQUEST_ID);
      expect(logger.error).not.toHaveBeenCalled();
      expect(logger.warn).not.toHaveBeenCalled();
    });

    // A 502 JSON body that is not a relayer-http error: RelayerHttpUnexpectedResponseError, logged once.
    it('throws a logged RelayerHttpUnexpectedResponseError for a non-relayer-http JSON error body', async () => {
      global.fetch = respondWith(() => JSON.stringify({ error: 'bad gateway' }), 502);
      const { attempt, logger } = makeAttempt({ relayerOperation });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpUnexpectedResponseError);
      expect((error as RelayerHttpUnexpectedResponseError).status).toBe(502);
      expect(logger.error).toHaveBeenCalledTimes(1);
    });

    // A 502 whose body is JSON null: RelayerHttpUnexpectedResponseError, logged once.
    it('throws a logged RelayerHttpUnexpectedResponseError for a non-2xx JSON null body', async () => {
      global.fetch = respondWith(() => 'null', 502);
      const { attempt, logger } = makeAttempt({ relayerOperation });

      await expect(attempt.run()).rejects.toBeInstanceOf(RelayerHttpUnexpectedResponseError);
      expect(logger.error).toHaveBeenCalledTimes(1);
    });

    // A non-JSON body (HTML proxy page), success or error status: logged RelayerHttpFetchError with that status.
    it.each([200, 503])('throws a logged RelayerHttpFetchError for a non-JSON body (status %d)', async (status) => {
      global.fetch = respondWith(() => '<html>proxy</html>', status);
      const { attempt, logger } = makeAttempt({ relayerOperation });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpFetchError);
      expect((error as RelayerHttpFetchError).status).toBe(status);
      expect(logger.error).toHaveBeenCalledTimes(1);
    });

    // A 200 success body without a result: logged RelayerHttpResultError carrying the url.
    it('throws a logged RelayerHttpResultError for a 200 body that misses the schema', async () => {
      global.fetch = respondWith(() => JSON.stringify({ status: 'succeeded', requestId: VALID_REQUEST_ID }), 200);
      const { attempt, logger } = makeAttempt({ relayerOperation });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpResultError);
      expect((error as RelayerHttpResultError).url).toBe(RELAYER_URL);
      expect(logger.error).toHaveBeenCalledTimes(1);
    });

    // fetch rejects (network failure): logged RelayerHttpFetchError with the network error as cause.
    it('throws a logged RelayerHttpFetchError keeping the cause on a network failure', async () => {
      const networkError = new TypeError('fetch failed');
      global.fetch = vi.fn(() => Promise.reject(networkError));
      const { attempt, logger } = makeAttempt({ relayerOperation });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpFetchError);
      expect((error as Error).cause).toBe(networkError);
      expect(logger.error).toHaveBeenCalledTimes(1);
    });
  });

  describe('body size limit (unchanged)', () => {
    // A body one byte over the size limit: RelayerHttpFetchError 'too large', JSON.parse never called.
    it.each([200, 503])('rejects a body over the limit without parsing it (status %d)', async (status) => {
      const body = `"${'a'.repeat(MAX_BODY_TEXT_LENGTH - 1)}"`;
      global.fetch = respondWith(() => body, status);
      const parseSpy = vi.spyOn(JSON, 'parse');
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT' });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpFetchError);
      expect((error as Error).message).toContain('too large');
      expect((error as RelayerHttpFetchError).status).toBe(status);
      expect(parseSpy).not.toHaveBeenCalled();
    });

    // A body exactly at the size limit: it is parsed.
    it('parses a body exactly at the limit', async () => {
      const body = `"${'a'.repeat(MAX_BODY_TEXT_LENGTH - 2)}"`;
      global.fetch = respondWith(() => body, 200);
      const parseSpy = vi.spyOn(JSON, 'parse');
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT' });

      await attempt.run().catch((e: unknown) => e);

      expect(parseSpy).toHaveBeenCalledTimes(1);
    });

    // The body stream errors without any abort: RelayerHttpFetchError with the status and the stream error as cause.
    it('throws RelayerHttpFetchError with status and cause when reading the body fails (not aborted)', async () => {
      const streamError = new TypeError('terminated');
      global.fetch = respondWith(
        () =>
          new ReadableStream<Uint8Array>({
            pull(controller) {
              controller.error(streamError);
            },
          }),
        200,
      );
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT' });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpFetchError);
      expect((error as RelayerHttpFetchError).status).toBe(200);
      expect((error as Error).cause).toBe(streamError);
    });
  });

  describe('request shape', () => {
    // fetch gets the same url, method, headers and body, plus the attempt's signal.
    it('passes init.body unchanged and attaches the given signal', async () => {
      global.fetch = respondWith(() => successBody('USER_DECRYPT'), 200);
      const { attempt, controller } = makeAttempt({ relayerOperation: 'USER_DECRYPT' });

      await attempt.run();

      const [url, init] = (global.fetch as Mock).mock.calls[0]! as [string, RequestInit];
      expect(url).toBe(RELAYER_URL);
      expect(init.body).toBe(BASE_INIT.body);
      expect(init.method).toBe('POST');
      expect(init.headers).toEqual(BASE_INIT.headers);
      expect(init.signal).toBe(controller.signal);
    });

    // A stray signal smuggled into init is ignored: fetch gets the attempt's own signal.
    it('attaches the given signal even if init carries another one at runtime', async () => {
      global.fetch = respondWith(() => successBody('USER_DECRYPT'), 200);
      const stray = new AbortController();
      const init = { ...BASE_INIT, signal: stray.signal } as Readonly<Omit<RequestInit, 'signal'>>;
      const { attempt, controller } = makeAttempt({ relayerOperation: 'USER_DECRYPT', init });

      await attempt.run();

      const [, sentInit] = (global.fetch as Mock).mock.calls[0]! as [string, RequestInit];
      expect(sentInit.signal).toBe(controller.signal);
    });

    // run() leaves the caller's init object untouched (no signal added).
    it('does not mutate the shared init', async () => {
      global.fetch = respondWith(() => successBody('USER_DECRYPT'), 200);
      const init = { ...BASE_INIT, headers: { ...(BASE_INIT.headers as Record<string, string>) } };
      const snapshot = JSON.parse(JSON.stringify(init)) as unknown;
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', init });

      await attempt.run();

      expect(init).toEqual(snapshot);
      expect('signal' in init).toBe(false);
    });

    // A query is appended (encoded) to the fetched url; errors report the url without it.
    it('appends the query to the fetched url but keeps the query-less url in errors', async () => {
      global.fetch = respondWith(errorBody, 503);
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', query: { a: '1', 'b c': 'd&e' } });

      const error = await attempt.run().catch((e: unknown) => e);

      expect((global.fetch as Mock).mock.calls[0]![0]).toBe(`${RELAYER_URL}?a=1&b%20c=d%26e`);
      expect((error as RelayerHttpResponseError).url).toBe(RELAYER_URL);
    });

    // A url that already holds a query: logged RelayerHttpFetchError, no fetch.
    it('wraps the query-append assert on a query-bearing url as a logged RelayerHttpFetchError, without fetching', async () => {
      const fetchMock = vi.fn();
      global.fetch = fetchMock;
      const { attempt, logger } = makeAttempt({ relayerOperation: 'USER_DECRYPT', url: `${RELAYER_URL}?x=1` });

      await expect(attempt.run()).rejects.toBeInstanceOf(RelayerHttpFetchError);
      expect(fetchMock).not.toHaveBeenCalled();
      expect(logger.error).toHaveBeenCalledTimes(1);
    });

    // Two network failures then a 200, with 2 retries allowed: one attempt resolves after 3 fetches.
    it('runs the network retries of fetchWithRetry inside one attempt', async () => {
      vi.useFakeTimers();
      let call = 0;
      global.fetch = vi.fn(() => {
        call += 1;
        return call < 3
          ? Promise.reject(new TypeError('fetch failed'))
          : Promise.resolve(new Response(successBody('USER_DECRYPT'), { status: 200 }));
      });
      const { attempt } = makeAttempt({
        relayerOperation: 'USER_DECRYPT',
        fetchRetries: 2,
        fetchRetryDelayInMilliseconds: 100,
      });

      const run = attempt.run();
      await vi.advanceTimersByTimeAsync(200);

      await expect(run).resolves.toEqual(USER_DECRYPT_SHARES);
      expect(global.fetch).toHaveBeenCalledTimes(3);
    });

    // An unknown relayerOperation at runtime: a plain Error naming it, not a typed relayer error.
    it('rejects an unknown runtime relayerOperation with a defensive throw on a 200 body', async () => {
      global.fetch = respondWith(() => successBody('USER_DECRYPT'), 200);
      const { attempt } = makeAttempt({ relayerOperation: 'INPUT_PROOF' as unknown as 'USER_DECRYPT' });

      const error = await attempt.run().catch((e: unknown) => e);

      // Message unspecified by the design beyond naming the operation (today's text is moved unchanged).
      expect(error).toBeInstanceOf(Error);
      expect((error as Error).message).toContain('INPUT_PROOF');
      expect(error).not.toBeInstanceOf(RelayerHttpResultError);
      expect(error).not.toBeInstanceOf(RelayerHttpFetchError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    // No one-shot guard here (the owner holds it): two run() calls make two fetches.
    it('has no one-shot guard: two run() calls make two fetches', async () => {
      global.fetch = respondWith(() => successBody('USER_DECRYPT'), 200);
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT' });

      await attempt.run();
      await attempt.run();

      expect(global.fetch).toHaveBeenCalledTimes(2);
    });
  });

  describe('owns no resource', () => {
    // Success: no timer created, no listener added on the given signal.
    it('creates no timer and adds no listener on the given signal (success)', async () => {
      vi.useFakeTimers();
      global.fetch = respondWith(() => successBody('USER_DECRYPT'), 200);
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', signal: controller.signal });

      const run = attempt.run();
      expect(vi.getTimerCount()).toBe(0);
      await run;

      expect(vi.getTimerCount()).toBe(0);
      expect(addSpy).not.toHaveBeenCalled();
    });

    // Error response: no timer created, no listener added on the given signal.
    it('creates no timer and adds no listener on the given signal (error response)', async () => {
      vi.useFakeTimers();
      global.fetch = respondWith(errorBody, 503);
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', signal: controller.signal });

      await attempt.run().catch(() => undefined);

      expect(vi.getTimerCount()).toBe(0);
      expect(addSpy).not.toHaveBeenCalled();
    });

    // Network retries with delays, all failing: no timer left, every listener added is removed.
    it('leaves listeners balanced and no timer after a network-retry delay', async () => {
      vi.useFakeTimers();
      global.fetch = vi.fn(() => Promise.reject(new TypeError('fetch failed')));
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
      const { attempt } = makeAttempt({
        relayerOperation: 'USER_DECRYPT',
        signal: controller.signal,
        fetchRetries: 2,
        fetchRetryDelayInMilliseconds: 100,
      });

      const run = attempt.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(200);

      expect(await run).toBeInstanceOf(RelayerHttpFetchError);
      expect(vi.getTimerCount()).toBe(0);
      expect(removeSpy).toHaveBeenCalledTimes(addSpy.mock.calls.length);
    });
  });

  // The attempt classifies an abort with the signal and timedOutAfterMs it was given (S5).
  describe('abort classification with a synthetic signal', () => {
    // Signal already aborted, timedOutAfterMs 500: RelayerHttpTimeoutError(500), no fetch.
    it('given signal already aborted + timedOutAfterMs 500 → RelayerHttpTimeoutError, no fetch', async () => {
      const fetchMock = vi.fn();
      global.fetch = fetchMock;
      const signal = AbortSignal.abort(new DOMException('Timed out', 'TimeoutError'));
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', signal, timedOutAfterMs: () => 500 });

      const error = await attempt.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(500);
      expect(fetchMock).not.toHaveBeenCalled();
    });

    // Signal already aborted, not our timeout: run() rejects with that exact reason, no fetch.
    it.each([
      ['a string', 'gone'],
      ['null', null],
      ['a RelayerHttpResponseError', new RelayerHttpResponseError({ url: RELAYER_URL, status: 503 })],
    ])(
      'given signal already aborted with %s + timedOutAfterMs undefined → that reason, no fetch',
      async (_l, reason) => {
        const fetchMock = vi.fn();
        global.fetch = fetchMock;
        const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', signal: AbortSignal.abort(reason) });

        expect(await attempt.run().catch((e: unknown) => e)).toBe(reason);
        expect(fetchMock).not.toHaveBeenCalled();
      },
    );

    const phases = [
      { phase: 'during fetch', during: '', setup: (): void => void (global.fetch = pendingUntilAbortFetch()) },
      {
        phase: 'while reading the body',
        during: ' while reading the response body',
        setup: (): void => void (global.fetch = stalledBodyFetch()),
      },
    ];

    // Signal aborts mid-request (fetch or body read), timedOutAfterMs 500: RelayerHttpTimeoutError, timeout warned.
    it.each(phases)('aborted $phase + timedOutAfterMs 500 → RelayerHttpTimeoutError', async ({ setup, during }) => {
      vi.useFakeTimers();
      setup();
      let fired: number | undefined;
      const controller = new AbortController();
      const { attempt, logger } = makeAttempt({
        relayerOperation: 'USER_DECRYPT',
        signal: controller.signal,
        timedOutAfterMs: () => fired,
      });

      const run = attempt.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      fired = 500;
      controller.abort(new DOMException('Timed out', 'TimeoutError'));

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(500);
      expect(logger.warn).toHaveBeenCalledWith(`[RelayerHttpRequest] Request to ${RELAYER_URL} timed out${during}.`);
    });

    // Signal aborts mid-request (fetch or body read), timedOutAfterMs undefined: run() rejects with the raw reason.
    it.each(phases)('aborted $phase + timedOutAfterMs undefined → the reason unchanged', async ({ setup }) => {
      vi.useFakeTimers();
      setup();
      const controller = new AbortController();
      const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', signal: controller.signal });

      const run = attempt.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      controller.abort(null);

      expect(await run).toBeNull();
    });

    // Abort during the network-retry delay: our timeout gives RelayerHttpTimeoutError, otherwise the raw reason; no timer left.
    it.each([
      ['timedOutAfterMs 500', 500, RelayerHttpTimeoutError],
      ['timedOutAfterMs undefined', undefined, undefined],
    ])('aborted during the network-retry delay + %s', async (_label, timeoutMs, expectedType) => {
      vi.useFakeTimers();
      global.fetch = vi.fn(() => Promise.reject(new TypeError('fetch failed')));
      let fired: number | undefined;
      const controller = new AbortController();
      const { attempt } = makeAttempt({
        relayerOperation: 'USER_DECRYPT',
        signal: controller.signal,
        timedOutAfterMs: () => fired,
        fetchRetries: 3,
        fetchRetryDelayInMilliseconds: 1000,
      });

      const run = attempt.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);
      fired = timeoutMs;
      controller.abort('user left');

      const error = await run;
      if (expectedType !== undefined) {
        expect(error).toBeInstanceOf(expectedType);
      } else {
        expect(error).toBe('user left');
      }
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });
  });
});

describe('RelayerHttpAttempt with a throwing logger', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  const brokenBody = (): ReadableStream<Uint8Array> =>
    new ReadableStream<Uint8Array>({
      pull(controller) {
        controller.error(new Error('connection dropped'));
      },
    });

  const loggedFailures: Array<[string, () => Mock, abstract new (...args: never[]) => unknown]> = [
    ['a network failure', () => vi.fn().mockRejectedValue(new TypeError('fetch failed')), RelayerHttpFetchError],
    ['an unreadable body', () => respondWith(brokenBody, 200), RelayerHttpFetchError],
    ['an oversized body', () => respondWith(() => 'x'.repeat(MAX_BODY_TEXT_LENGTH + 1), 200), RelayerHttpFetchError],
    ['a non-JSON body', () => respondWith(() => '<html>bad gateway</html>', 502), RelayerHttpFetchError],
    ['a non-relayer JSON error', () => respondWith(() => '{"oops":1}', 502), RelayerHttpUnexpectedResponseError],
    ['a success body with the wrong shape', () => respondWith(() => '{}', 200), RelayerHttpResultError],
  ];

  // A throwing error hook is swallowed: the attempt still throws its own typed error.
  it.each(loggedFailures)('%s: the typed error is thrown, not the hook error', async (_label, makeFetch, type) => {
    global.fetch = makeFetch();
    const logger = makeLogger();
    logger.error.mockImplementation(() => {
      throw new Error('error hook failed');
    });
    const { attempt } = makeAttempt({ relayerOperation: 'USER_DECRYPT', logger });

    const error = await attempt.run().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(type);
    expect(logger.error).toHaveBeenCalledTimes(1);
  });
});
