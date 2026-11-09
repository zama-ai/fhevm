import type { Mock, MockInstance } from 'vitest';
import type { Logger } from '../../../../types/logger.js';
import type { RelayerHttpOperation } from '../RelayerHttpAttempt.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';
import { MAX_TIMEOUT_MS } from '../relayerHttpSignal.js';
import { RelayerHttpFetchError } from '../../../../errors/RelayerHttpFetchError.js';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpResultError } from '../../../../errors/RelayerHttpResultError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../../errors/RelayerHttpUnexpectedResponseError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.leaks.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';
const SHARES = [{ payload: 'ab'.repeat(32), signature: 'cd'.repeat(65), extraData: '0x00' }];

type RequestParams = ConstructorParameters<typeof RelayerHttpRequest<'USER_DECRYPT'>>[0];
type Options = NonNullable<RequestParams['options']>;
type Decide = (error: RelayerHttpResponseError, attempt: number) => unknown;

// Test-local override (design §4.5): delegates the decision to a finite script.
function scriptedRequestClass(decide: Decide) {
  return class ScriptedRequest<O extends RelayerHttpOperation> extends RelayerHttpRequest<O> {
    protected override _retryDecision(error: RelayerHttpResponseError, attempt: number): number | undefined {
      return decide(error, attempt) as number | undefined;
    }
  };
}

function scriptOf(script: readonly unknown[]): Decide {
  let index = 0;
  return () => script[index++];
}

type Step = { status: number; body: string } | { reject: unknown } | { brokenBody: Error } | 'pending' | 'stalled';

const TIMEOUT_MS = 60_000;
const MAX_BODY_TEXT_LENGTH = 10 * 1024 * 1024;

const S503: Step = {
  status: 503,
  body: JSON.stringify({ code: 'overloaded', message: 'busy', requestId: VALID_REQUEST_ID }),
};
const S200: Step = {
  status: 200,
  body: JSON.stringify({ status: 'succeeded', requestId: VALID_REQUEST_ID, result: { result: SHARES } }),
};

// Listeners added by the fetch stubs are test plumbing, not the code under test.
const stubListeners = new WeakSet<object>();
function stubListener(fn: () => void): () => void {
  stubListeners.add(fn);
  return fn;
}

// A fresh Response per call, following a finite list of steps; a call past the end fails loudly.
function fetchSequence(steps: Step[]): Mock {
  let index = 0;
  return vi.fn((_url: unknown, init?: RequestInit) => {
    const step = steps[index++];
    if (step === undefined) {
      return Promise.reject(new Error(`fetch script exhausted (call ${index})`));
    }
    const signal = init?.signal ?? undefined;
    if (step === 'pending') {
      return new Promise<Response>((_resolve, reject) => {
        signal?.addEventListener(
          'abort',
          stubListener(() => reject(signal.reason)),
          { once: true },
        );
      });
    }
    if (step === 'stalled') {
      const body = new ReadableStream<Uint8Array>({
        start(controller) {
          signal?.addEventListener(
            'abort',
            stubListener(() => controller.error(signal.reason)),
            { once: true },
          );
        },
      });
      return Promise.resolve(new Response(body, { status: 200 }));
    }
    if ('reject' in step) {
      return Promise.reject(step.reject);
    }
    if ('brokenBody' in step) {
      const failing = step.brokenBody;
      const body = new ReadableStream<Uint8Array>({
        pull(controller) {
          controller.error(failing);
        },
      });
      return Promise.resolve(new Response(body, { status: 200 }));
    }
    return Promise.resolve(new Response(step.body, { status: step.status }));
  });
}

// AbortSignal.timeout() on the fake clock: same caller-owned TimeoutError reason, no wall-clock wait.
function fakeAbortSignalTimeout(ms: number): AbortSignal {
  const controller = new AbortController();
  setTimeout(() => controller.abort(new DOMException('The operation timed out.', 'TimeoutError')), ms);
  return controller.signal;
}

type ListenerTracker = {
  addSpy: MockInstance<EventTarget['addEventListener']>;
  removeSpy: MockInstance<EventTarget['removeEventListener']>;
  // Listeners (on any AbortSignal) neither removed nor consumed by a `once` abort dispatch.
  unreleased: () => number;
};

// Tracks every listener added to any AbortSignal: the caller's, the combined one, and abortableSleep's.
// A listener is released when removed from the same signal, or when added with `once` and that signal fired.
function trackSignalListeners(): ListenerTracker {
  const addSpy = vi.spyOn(AbortSignal.prototype, 'addEventListener');
  const removeSpy = vi.spyOn(AbortSignal.prototype, 'removeEventListener');
  const unreleased = (): number =>
    addSpy.mock.calls.filter(([type, listener, options], i) => {
      if (listener === null || stubListeners.has(listener)) {
        return false;
      }
      const target = addSpy.mock.contexts[i] as AbortSignal;
      const removed = removeSpy.mock.calls.some(
        ([rType, rListener], j) => removeSpy.mock.contexts[j] === target && rType === type && rListener === listener,
      );
      const once = typeof options === 'object' && options.once === true;
      return !removed && !(once && target.aborted);
    }).length;
  return { addSpy, removeSpy, unreleased };
}

type Exit = {
  readonly name: string;
  readonly steps: Step[];
  readonly decide?: () => Decide;
  readonly options?: Options;
  readonly payload?: Record<string, unknown>;
  readonly logger?: Logger;
  // Fake time the run needs: until `abort` is applied, or until it settles (our-timeout exits:
  // exactly the deadline). Resources are inspected right after, with no further advance.
  readonly advanceMs: number;
  readonly abort?: unknown;
  readonly preAborted?: boolean;
  // The run's outcome, so a leak check cannot pass on an unintended path.
  readonly expected: (outcome: unknown, fetchCalls: number) => void;
};

function resolvesShares(fetches: number): Exit['expected'] {
  return (outcome, fetchCalls) => {
    expect(outcome).toEqual(SHARES);
    expect(fetchCalls).toBe(fetches);
  };
}

function rejectsWith(type: abstract new (...args: never[]) => unknown, fetches: number): Exit['expected'] {
  return (outcome, fetchCalls) => {
    expect(outcome).toBeInstanceOf(type);
    expect(fetchCalls).toBe(fetches);
  };
}

function rejectsWithValue(value: unknown, fetches: number): Exit['expected'] {
  return (outcome, fetchCalls) => {
    expect(outcome).toBe(value);
    expect(fetchCalls).toBe(fetches);
  };
}

function rejectsWithMessage(message: string, fetches: number): Exit['expected'] {
  return (outcome, fetchCalls) => {
    expect(outcome).toBeInstanceOf(Error);
    expect((outcome as Error).message).toContain(message);
    expect(fetchCalls).toBe(fetches);
  };
}

const BODY_READ_ERROR = new Error('connection reset mid-body');

const EXITS: Exit[] = [
  { name: 'success', steps: [S200], advanceMs: 0, expected: resolvesShares(1) },
  {
    name: 'a retried success',
    steps: [S503, S503, S200],
    decide: () => scriptOf([10, 10]),
    advanceMs: 20,
    expected: resolvesShares(3),
  },
  {
    name: 'a RelayerHttpResponseError (default, no retry)',
    steps: [S503],
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpResponseError, 1),
  },
  {
    name: 'a retried RelayerHttpResponseError',
    steps: [S503, S503],
    decide: () => scriptOf([10, undefined]),
    advanceMs: 10,
    expected: rejectsWith(RelayerHttpResponseError, 2),
  },
  {
    name: 'a RelayerHttpFetchError (network)',
    steps: [{ reject: new TypeError('x') }],
    options: { fetchRetries: 0 },
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpFetchError, 1),
  },
  {
    name: 'a RelayerHttpFetchError (non-JSON)',
    steps: [{ status: 502, body: '<html/>' }],
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpFetchError, 1),
  },
  {
    name: 'a RelayerHttpUnexpectedResponseError',
    steps: [{ status: 500, body: '{}' }],
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpUnexpectedResponseError, 1),
  },
  {
    name: 'a RelayerHttpResultError',
    steps: [{ status: 200, body: '{}' }],
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpResultError, 1),
  },
  {
    name: 'a retry then another error type',
    steps: [S503, { status: 200, body: '{}' }],
    decide: () => scriptOf([10]),
    advanceMs: 10,
    expected: rejectsWith(RelayerHttpResultError, 2),
  },
  {
    name: 'a caller abort during fetch',
    steps: ['pending'],
    advanceMs: 0,
    abort: 'user left',
    expected: rejectsWithValue('user left', 1),
  },
  {
    name: 'a caller abort while reading the body',
    steps: ['stalled'],
    advanceMs: 0,
    abort: 'user left',
    expected: rejectsWithValue('user left', 1),
  },
  {
    name: 'a caller abort during the network-retry delay',
    steps: [{ reject: new TypeError('x') }],
    options: { fetchRetries: 3, fetchRetryDelayInMilliseconds: 1000 },
    advanceMs: 10,
    abort: null,
    expected: rejectsWithValue(null, 1),
  },
  {
    name: 'a caller abort during the wait',
    steps: [S503, S200],
    decide: () => scriptOf([1000]),
    advanceMs: 10,
    abort: 'user left',
    expected: rejectsWithValue('user left', 1),
  },
  {
    name: 'a caller signal already aborted',
    steps: [S200],
    advanceMs: 0,
    preAborted: true,
    expected: rejectsWithValue('gone', 0),
  },
  {
    name: 'our timeout during fetch',
    steps: ['pending'],
    advanceMs: TIMEOUT_MS,
    expected: rejectsWith(RelayerHttpTimeoutError, 1),
  },
  {
    name: 'our timeout during fetch of a retried attempt',
    steps: [S503, 'pending'],
    decide: () => scriptOf([10]),
    advanceMs: TIMEOUT_MS,
    expected: rejectsWith(RelayerHttpTimeoutError, 2),
  },
  {
    name: 'our timeout while reading the body',
    steps: ['stalled'],
    advanceMs: TIMEOUT_MS,
    expected: rejectsWith(RelayerHttpTimeoutError, 1),
  },
  {
    name: 'an oversized body (over 10 MiB characters)',
    steps: [{ status: 200, body: `"${'a'.repeat(MAX_BODY_TEXT_LENGTH - 1)}"` }],
    advanceMs: 0,
    expected: (outcome, fetchCalls) => {
      expect(MAX_BODY_TEXT_LENGTH).toBe(10_485_760);
      expect(outcome).toBeInstanceOf(RelayerHttpFetchError);
      expect((outcome as Error).message).toContain('limit 10485760');
      expect((outcome as RelayerHttpFetchError).status).toBe(200);
      expect(fetchCalls).toBe(1);
    },
  },
  {
    name: 'a body-read failure on a live signal',
    steps: [{ brokenBody: BODY_READ_ERROR }],
    advanceMs: 0,
    expected: (outcome, fetchCalls) => {
      expect(outcome).toBeInstanceOf(RelayerHttpFetchError);
      expect((outcome as Error).cause).toBe(BODY_READ_ERROR);
      expect((outcome as RelayerHttpFetchError).status).toBe(200);
      expect(fetchCalls).toBe(1);
    },
  },
  {
    name: 'a build throw (BigInt payload)',
    steps: [S200],
    payload: { big: 1n },
    advanceMs: 0,
    expected: rejectsWith(TypeError, 0),
  },
  {
    name: 'a build throw (invalid auth header value)',
    steps: [S200],
    options: { auth: { type: 'BearerToken', token: 'a\nb' } },
    advanceMs: 0,
    expected: rejectsWithMessage('CR, LF, or NUL', 0),
  },
  {
    name: 'a decide throw',
    steps: [S503],
    decide: () => () => {
      throw new Error('decide failed');
    },
    advanceMs: 0,
    expected: rejectsWithMessage('decide failed', 1),
  },
  {
    name: 'a ceiling stop',
    steps: [S503, S503, S503, S503, S503],
    decide: () => () => 1,
    advanceMs: 10,
    expected: rejectsWith(RelayerHttpResponseError, 5),
  },
  {
    name: 'a no-fit stop',
    steps: [S503, S200],
    decide: () => () => 60_000,
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpResponseError, 1),
  },
  {
    name: 'an invalid-delay stop',
    steps: [S503, S200],
    decide: () => () => Number.NaN,
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpResponseError, 1),
  },
  {
    name: 'an over-range delay stop',
    steps: [S503, S200],
    decide: () => () => MAX_TIMEOUT_MS + 1,
    advanceMs: 0,
    expected: rejectsWith(RelayerHttpResponseError, 1),
  },
  {
    name: 'a throwing logger warn during a retry (swallowed)',
    steps: [S503, S200],
    decide: () => scriptOf([10]),
    logger: {
      debug: undefined,
      warn: () => {
        throw new Error('warn hook failed');
      },
      error: undefined,
    },
    advanceMs: 10,
    expected: resolvesShares(2),
  },
];

// Runs one exit to completion and asserts its expected outcome.
async function runExit(exit: Exit, controller: AbortController): Promise<void> {
  const fetchMock = fetchSequence(exit.steps);
  global.fetch = fetchMock;
  const Request = exit.decide !== undefined ? scriptedRequestClass(exit.decide()) : RelayerHttpRequest;
  const request = new Request({
    relayerOperation: 'USER_DECRYPT',
    url: RELAYER_URL,
    payload: exit.payload ?? {},
    options: { timeout: TIMEOUT_MS, ...exit.options, signal: controller.signal },
    logger: exit.logger,
  });
  let settled = false;
  const run = request.run().then(
    (value) => value,
    (e: unknown) => e,
  );
  void run.finally(() => {
    settled = true;
  });
  await vi.advanceTimersByTimeAsync(exit.advanceMs);
  if ('abort' in exit) {
    controller.abort(exit.abort);
  }
  // Flush promise jobs only: a forgotten timer must still be pending when inspected.
  await vi.advanceTimersByTimeAsync(0);
  expect(settled).toBe(true);
  exit.expected(await run, fetchMock.mock.calls.length);
}

describe('RelayerHttpRequest leaves nothing behind', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  // For every way a run can end: no timer pending and every abort listener released.
  it.each(EXITS)('after $name: no pending timer, every signal listener released', async (exit) => {
    vi.useFakeTimers();
    const tracker = trackSignalListeners();
    const controller = new AbortController();
    if (exit.preAborted === true) {
      controller.abort('gone');
    }

    await runExit(exit, controller);

    expect(vi.getTimerCount()).toBe(0);
    expect(tracker.unreleased()).toBe(0);
  });

  // For every exit with a live caller signal: exactly one listener added on it, and removed.
  it.each(EXITS.filter((exit) => exit.preAborted !== true))(
    'after $name: exactly one listener added on the caller signal, and removed',
    async (exit) => {
      vi.useFakeTimers();
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

      await runExit(exit, controller);

      expect(addSpy).toHaveBeenCalledTimes(1);
      expect(removeSpy).toHaveBeenCalledWith('abort', addSpy.mock.calls[0]![1]);
    },
  );

  // After a run settled, aborting the caller signal no longer affects the signal given to fetch.
  it('after a settled run, a late caller abort no longer reaches the combined signal', async () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const fetchMock = fetchSequence([S200]);
    global.fetch = fetchMock;

    await new RelayerHttpRequest({
      relayerOperation: 'USER_DECRYPT',
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal, timeout: TIMEOUT_MS },
    }).run();
    const combinedSignal = (fetchMock.mock.calls[0]![1] as RequestInit).signal!;
    controller.abort('late');

    expect(combinedSignal).not.toBe(controller.signal);
    expect(combinedSignal.aborted).toBe(false);
  });

  // A fetch or body read that ignores the signal (non-compliant polyfill):
  // our timeout still removes the caller listener.
  it.each([
    { name: 'fetch', response: () => new Promise<Response>(() => {}) },
    {
      name: 'the body read',
      response: () => Promise.resolve(new Response(new ReadableStream<Uint8Array>(), { status: 200 })),
    },
  ])(
    'after our timeout fires on a hung $name that ignores the signal: the caller listener is removed',
    async ({ response }) => {
      vi.useFakeTimers();
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
      global.fetch = vi.fn(response);

      void new RelayerHttpRequest({
        relayerOperation: 'USER_DECRYPT',
        url: RELAYER_URL,
        payload: {},
        options: { signal: controller.signal, timeout: TIMEOUT_MS },
      })
        .run()
        .catch(() => undefined);
      await vi.advanceTimersByTimeAsync(TIMEOUT_MS);

      expect(addSpy).toHaveBeenCalledTimes(1);
      expect(removeSpy).toHaveBeenCalledWith('abort', addSpy.mock.calls[0]![1]);
    },
  );

  // Same hung fetch or body read, but the caller aborts: our timeout timer is cleared at once.
  it.each([
    { name: 'fetch', response: () => new Promise<Response>(() => {}) },
    {
      name: 'the body read',
      response: () => Promise.resolve(new Response(new ReadableStream<Uint8Array>(), { status: 200 })),
    },
  ])('after a caller abort on a hung $name that ignores the signal: our timer is cleared', async ({ response }) => {
    vi.useFakeTimers();
    const controller = new AbortController();
    global.fetch = vi.fn(response);

    void new RelayerHttpRequest({
      relayerOperation: 'USER_DECRYPT',
      url: RELAYER_URL,
      payload: {},
      options: { signal: controller.signal, timeout: MAX_TIMEOUT_MS },
    })
      .run()
      .catch(() => undefined);
    await vi.advanceTimersByTimeAsync(0);
    controller.abort('gone');
    // Flush promise jobs only: the timer must not be left pending until MAX_TIMEOUT_MS.
    await vi.advanceTimersByTimeAsync(0);

    expect(vi.getTimerCount()).toBe(0);
  });

  // Caller's AbortSignal.timeout() expired before run(): its reason, no fetch, nothing left behind.
  it("a caller's AbortSignal.timeout() already expired before run(): its reason, no fetch, nothing left behind", async () => {
    vi.useFakeTimers();
    vi.spyOn(AbortSignal, 'timeout').mockImplementation(fakeAbortSignalTimeout);
    const callerSignal = AbortSignal.timeout(10);
    vi.advanceTimersByTime(10);
    expect(callerSignal.aborted).toBe(true);
    const tracker = trackSignalListeners();
    const fetchMock = fetchSequence([S200]);
    global.fetch = fetchMock;

    const run = new RelayerHttpRequest({
      relayerOperation: 'USER_DECRYPT',
      url: RELAYER_URL,
      payload: {},
      options: { signal: callerSignal, timeout: TIMEOUT_MS },
    })
      .run()
      .catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(0);

    const error = await run;
    expect(error).toBe(callerSignal.reason);
    expect((error as DOMException).name).toBe('TimeoutError');
    expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
    expect(fetchMock).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
    expect(tracker.unreleased()).toBe(0);
    const onCaller = (spy: { mock: { contexts: unknown[] } }): number =>
      spy.mock.contexts.filter((target) => target === callerSignal).length;
    expect(onCaller(tracker.removeSpy)).toBeGreaterThanOrEqual(onCaller(tracker.addSpy));
  });

  // Every exit run 4 times on one shared caller signal: listeners balanced, no timer left.
  it('many sequential runs sharing one caller signal: listeners balanced, no timer', async () => {
    vi.useFakeTimers();
    // One prototype-level tracker sees every signal; the caller's calls are picked out by `this`.
    const tracker = trackSignalListeners();
    const controller = new AbortController();
    const onCaller = (spy: { mock: { contexts: unknown[] } }): number =>
      spy.mock.contexts.filter((target) => target === controller.signal).length;
    const outcomes = EXITS.filter((exit) => !('abort' in exit) && exit.preAborted !== true);

    for (let round = 0; round < 4; round++) {
      for (const exit of outcomes) {
        await runExit(exit, controller);
      }
    }

    const runs = 4 * outcomes.length;
    expect(onCaller(tracker.addSpy)).toBe(runs);
    expect(onCaller(tracker.removeSpy)).toBe(runs);
    expect(vi.getTimerCount()).toBe(0);
    expect(tracker.unreleased()).toBe(0);
  });

  // 25 concurrent runs, each retrying once: all succeed, no timer, every listener released.
  it('many concurrent retrying runs: no timer, every listener released', async () => {
    vi.useFakeTimers();
    const tracker = trackSignalListeners();
    // Each run (identified by its payload) gets a 503 first, then a 200.
    const seen = new Set<string>();
    global.fetch = vi.fn((_url: unknown, init?: RequestInit) => {
      const body = String(init?.body);
      const step = seen.has(body) ? S200 : S503;
      seen.add(body);
      return Promise.resolve(new Response(step.body, { status: step.status }));
    });
    const controllers = Array.from({ length: 25 }, () => new AbortController());

    const runs = controllers.map((controller, i) => {
      const Request = scriptedRequestClass(scriptOf([1 + i]));
      return new Request({
        relayerOperation: 'USER_DECRYPT',
        url: RELAYER_URL,
        payload: { i },
        options: { signal: controller.signal, timeout: TIMEOUT_MS },
      })
        .run()
        .catch((e: unknown) => e);
    });
    // The slowest run waits 25 ms; advance exactly that before inspecting.
    await vi.advanceTimersByTimeAsync(25);

    expect(await Promise.all(runs)).toEqual(controllers.map(() => SHARES));
    expect(global.fetch).toHaveBeenCalledTimes(50);
    expect(vi.getTimerCount()).toBe(0);
    expect(tracker.unreleased()).toBe(0);
  });
});
