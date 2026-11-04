import type { Mock } from 'vitest';
import type { RelayerHttpOperation } from '../RelayerHttpAttempt.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';
import { MAX_TIMEOUT_MS } from '../relayerHttpSignal.js';
import { abortableSleep } from '../../../../base/timeout.js';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.phases.test.ts
////////////////////////////////////////////////////////////////////////////////

// Pass-through by default; the "between the wait and the next fetch" tests hook one call.
vi.mock('../../../../base/timeout.js', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../../../../base/timeout.js')>();
  return { ...actual, abortableSleep: vi.fn(actual.abortableSleep) };
});

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';
const SHARES = [{ payload: 'ab'.repeat(32), signature: 'cd'.repeat(65), extraData: '0x00' }];
const TIMEOUT_MS = 5000;
const RETRY_DELAY_MS = 100;

type RequestParams = ConstructorParameters<typeof RelayerHttpRequest<'USER_DECRYPT'>>[0];
type Options = NonNullable<RequestParams['options']>;

// Test-local override (design §4.5): retries every relayer-http error after RETRY_DELAY_MS, up to the ceiling.
class ScriptedRequest<O extends RelayerHttpOperation> extends RelayerHttpRequest<O> {
  public readonly decisions: number[] = [];
  protected override _retryDecision(_error: RelayerHttpResponseError, attempt: number): number | undefined {
    this.decisions.push(attempt);
    return RETRY_DELAY_MS;
  }
}

function scripted(options: Options): ScriptedRequest<'USER_DECRYPT'> {
  return new ScriptedRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {}, options });
}

type Step = { status: number; body: string } | { reject: unknown } | 'pending' | 'stalled';

const S503: Step = {
  status: 503,
  body: JSON.stringify({ code: 'overloaded', message: 'busy', requestId: VALID_REQUEST_ID }),
};
const S200: Step = {
  status: 200,
  body: JSON.stringify({ status: 'succeeded', requestId: VALID_REQUEST_ID, result: { result: SHARES } }),
};
const NETWORK: Step = { reject: new TypeError('fetch failed') };

// A fresh Response per call, following a finite list of steps; a call past the end fails loudly.
function fetchSequence(steps: Step[]): Mock {
  let index = 0;
  return vi.fn((_url: unknown, init?: RequestInit) => {
    const step = steps[index++];
    if (step === undefined) {
      return Promise.reject(new Error(`fetch script exhausted (call ${index})`));
    }
    if (step === 'pending') {
      return new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener('abort', () => reject(init.signal?.reason), { once: true });
      });
    }
    if (step === 'stalled') {
      const body = new ReadableStream<Uint8Array>({
        start(controller) {
          init?.signal?.addEventListener('abort', () => controller.error(init.signal?.reason), { once: true });
        },
      });
      return Promise.resolve(new Response(body, { status: 200 }));
    }
    if ('reject' in step) {
      return Promise.reject(step.reject);
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

type Phase = {
  readonly phase: string;
  readonly steps: Step[];
  // Fake time elapsed when the phase is reached, and fetch calls made by then.
  readonly reachedAtMs: number;
  readonly fetchCalls: number;
  readonly decisions: number[];
};

// Each phase is reached with the retry machinery active (attempt 2 follows a retried 503).
// The network-retry delay is longer than the timeout so our timer fires inside it.
const PHASES: Phase[] = [
  { phase: 'during the fetch of attempt 1', steps: ['pending'], reachedAtMs: 0, fetchCalls: 1, decisions: [] },
  {
    phase: 'during the fetch of attempt 2',
    steps: [S503, 'pending'],
    reachedAtMs: RETRY_DELAY_MS,
    fetchCalls: 2,
    decisions: [1],
  },
  {
    phase: 'during the network-retry delay of attempt 2',
    steps: [S503, NETWORK],
    reachedAtMs: RETRY_DELAY_MS,
    fetchCalls: 2,
    decisions: [1],
  },
  {
    phase: 'while reading the body of attempt 2',
    steps: [S503, 'stalled'],
    reachedAtMs: RETRY_DELAY_MS,
    fetchCalls: 2,
    decisions: [1],
  },
  { phase: 'during the wait between attempts', steps: [S503, S200], reachedAtMs: 10, fetchCalls: 1, decisions: [1] },
];

const PHASE_OPTIONS: Options = { fetchRetries: 3, fetchRetryDelayInMilliseconds: 10 * TIMEOUT_MS };

describe('RelayerHttpRequest abort in every phase', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.mocked(abortableSleep).mockReset();
    vi.useRealTimers();
  });

  describe.each([
    ['a string', 'user left'],
    ['null', null],
    ['an Error', new Error('cancelled')],
    ['a caller DOMException TimeoutError', new DOMException('Caller timed out', 'TimeoutError')],
  ])('caller abort with %s', (_label, reason) => {
    // Caller aborts in any phase of a retrying run: run() rejects with the caller's exact reason,
    // never RelayerHttpTimeoutError; no extra fetch, timer and listener cleaned up.
    it.each(PHASES)('$phase → the reason unchanged', async ({ steps, reachedAtMs, fetchCalls, decisions }) => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(steps);
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
      const request = scripted({ ...PHASE_OPTIONS, signal: controller.signal, timeout: TIMEOUT_MS });

      const run = request.run().then(
        () => 'resolved',
        (e: unknown) => e,
      );
      await vi.advanceTimersByTimeAsync(reachedAtMs);
      expect(global.fetch).toHaveBeenCalledTimes(fetchCalls);
      controller.abort(reason);

      const error = await run;
      expect(error).toBe(reason);
      expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
      expect(global.fetch).toHaveBeenCalledTimes(fetchCalls);
      expect(request.decisions).toEqual(decisions);
      expect(vi.getTimerCount()).toBe(0);
      expect(addSpy).toHaveBeenCalledTimes(1);
      expect(removeSpy).toHaveBeenCalledWith('abort', addSpy.mock.calls[0]![1]);
    });
  });

  describe('our timer', () => {
    // Our timeout fires in any phase but the wait (covered below): RelayerHttpTimeoutError, nothing left behind.
    it.each(PHASES.filter(({ phase }) => !phase.includes('wait')))(
      '$phase → RelayerHttpTimeoutError',
      async ({ steps, reachedAtMs, fetchCalls, decisions }) => {
        vi.useFakeTimers();
        global.fetch = fetchSequence(steps);
        const controller = new AbortController();
        const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
        const request = scripted({ ...PHASE_OPTIONS, signal: controller.signal, timeout: TIMEOUT_MS });

        const run = request.run().catch((e: unknown) => e);
        await vi.advanceTimersByTimeAsync(reachedAtMs);
        expect(global.fetch).toHaveBeenCalledTimes(fetchCalls);
        await vi.advanceTimersByTimeAsync(TIMEOUT_MS - reachedAtMs);

        const error = await run;
        expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
        expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(TIMEOUT_MS);
        expect((error as RelayerHttpTimeoutError).url).toBe(RELAYER_URL);
        expect(global.fetch).toHaveBeenCalledTimes(fetchCalls);
        expect(request.decisions).toEqual(decisions);
        expect(vi.getTimerCount()).toBe(0);
        expect(removeSpy).toHaveBeenCalledTimes(1);
      },
    );

    // Our deadline hits during the wait between attempts: RelayerHttpTimeoutError, no second fetch.
    //   fetch 503 ──► warn 'Retrying in' (clock jumps near deadline) ──► wait ──► deadline ──► reject
    it('during the wait between attempts → RelayerHttpTimeoutError', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503, S200]);
      const warn = vi.fn((_message: string) => {
        if (_message.includes('Retrying in')) {
          vi.advanceTimersByTime(TIMEOUT_MS - RETRY_DELAY_MS / 2);
        }
      });
      const request = new ScriptedRequest({
        relayerOperation: 'USER_DECRYPT',
        url: RELAYER_URL,
        payload: {},
        options: { timeout: TIMEOUT_MS },
        logger: { debug: undefined, warn, error: vi.fn() },
      });

      const run = request.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      expect(vi.getTimerCount()).toBe(2);
      await vi.advanceTimersByTimeAsync(RETRY_DELAY_MS / 2);

      expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  // S14: the abort lands after the wait resolved, before the next attempt's fetch.
  describe('between the end of the wait and the next fetch', () => {
    async function hookAfterWait(afterSleep: () => void): Promise<void> {
      const actual = await vi.importActual<typeof import('../../../../base/timeout.js')>('../../../../base/timeout.js');
      vi.mocked(abortableSleep).mockImplementationOnce(async (ms, signal) => {
        await actual.abortableSleep(ms, signal);
        afterSleep();
      });
    }

    // Caller aborts right after the wait ends, before the next fetch: run() rejects with the reason, 1 fetch only.
    //   fetch 503 ──► wait ends ──► caller aborts ──► no 2nd fetch ──► reject reason
    it.each([
      ['a string', 'user left'],
      ['null', null],
      ['a RelayerHttpResponseError', new RelayerHttpResponseError({ url: RELAYER_URL, status: 503 })],
    ])('caller abort with %s → the reason, no second fetch', async (_label, reason) => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503, S200]);
      const controller = new AbortController();
      await hookAfterWait(() => controller.abort(reason));
      const request = scripted({ signal: controller.signal, timeout: TIMEOUT_MS });

      const run = request.run().then(
        () => 'resolved',
        (e: unknown) => e,
      );
      await vi.advanceTimersByTimeAsync(RETRY_DELAY_MS);

      expect(await run).toBe(reason);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(request.decisions).toEqual([1]);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Our deadline passes right after the wait ends, before the next fetch: RelayerHttpTimeoutError, 1 fetch only.
    it('our timer → RelayerHttpTimeoutError, no second fetch', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503, S200]);
      await hookAfterWait(() => vi.advanceTimersByTime(TIMEOUT_MS - RETRY_DELAY_MS));
      const request = scripted({ timeout: TIMEOUT_MS });

      const run = request.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(RETRY_DELAY_MS);

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(TIMEOUT_MS);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('caller signal already aborted', () => {
    // Caller signal already aborted: run() rejects with its reason, no fetch, no retry decision, nothing left behind.
    it.each([
      ['a string', 'gone'],
      ['null', null],
      ['an Error', new Error('gone')],
      ['a caller DOMException TimeoutError', new DOMException('Caller timed out', 'TimeoutError')],
    ])('with %s → the reason, no fetch, no decision, nothing left behind', async (_label, reason) => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503]);
      const callerSignal = AbortSignal.abort(reason);
      const addSpy = vi.spyOn(callerSignal, 'addEventListener');
      const removeSpy = vi.spyOn(callerSignal, 'removeEventListener');
      const request = scripted({ signal: callerSignal, timeout: TIMEOUT_MS });

      expect(
        await request.run().then(
          () => 'resolved',
          (e: unknown) => e,
        ),
      ).toBe(reason);
      expect(global.fetch).not.toHaveBeenCalled();
      expect(request.decisions).toEqual([]);
      expect(vi.getTimerCount()).toBe(0);
      expect(removeSpy.mock.calls.length).toBeGreaterThanOrEqual(addSpy.mock.calls.length);
    });

    // abort() without a reason: run() rejects with the platform's AbortError, unchanged.
    it('without a reason → the platform AbortError instance, unchanged', async () => {
      global.fetch = fetchSequence([S503]);
      const controller = new AbortController();
      controller.abort();

      const error = await scripted({ signal: controller.signal })
        .run()
        .catch((e: unknown) => e);

      expect(error).toBe(controller.signal.reason);
      expect((error as DOMException).name).toBe('AbortError');
    });
  });

  // Tie-break documentation (fake-timer FIFO, not a design rule): the timer armed first fires first.
  describe('caller abort vs our timer at the same instant (during fetch)', () => {
    // Caller abort and our timeout due at the same instant, caller's timer armed first: the caller's reason wins.
    it('the caller timer armed first wins: caller reason', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(['pending']);
      const controller = new AbortController();
      setTimeout(() => controller.abort('caller first'), 500);

      const run = scripted({ signal: controller.signal, timeout: 500 })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(500);

      expect(await run).toBe('caller first');
    });

    // Same instant, our timer armed first: RelayerHttpTimeoutError wins.
    it('our timer armed first wins: RelayerHttpTimeoutError', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(['pending']);
      const controller = new AbortController();

      const run = scripted({ signal: controller.signal, timeout: 500 })
        .run()
        .catch((e: unknown) => e);
      setTimeout(() => controller.abort('caller second'), 500);
      await vi.advanceTimersByTimeAsync(500);

      expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
    });

    // The caller's own AbortSignal.timeout() fires: its reason comes back, never RelayerHttpTimeoutError.
    it("the caller's own AbortSignal.timeout() is never reported as our timeout", async () => {
      vi.useFakeTimers();
      vi.spyOn(AbortSignal, 'timeout').mockImplementation(fakeAbortSignalTimeout);
      global.fetch = fetchSequence(['pending']);
      const callerSignal = AbortSignal.timeout(10);

      const run = scripted({ signal: callerSignal, timeout: 60_000 })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);
      const error = await run;

      expect(error).toBe(callerSignal.reason);
      expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
    });
  });

  describe('timeout bounds', () => {
    // No timeout option: no timer is armed; only the caller's abort ends the run.
    it('without a timeout: no timer is armed during the run', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(['pending']);
      const controller = new AbortController();

      const run = scripted({ signal: controller.signal })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      expect(vi.getTimerCount()).toBe(0);
      controller.abort('done');

      expect(await run).toBe('done');
    });

    // timeout 1 ms: RelayerHttpTimeoutError after 1 ms.
    it('timeout 1 ms → RelayerHttpTimeoutError after 1 ms', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(['pending']);

      const run = scripted({ timeout: 1 })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(1);

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(1);
    });

    // timeout MAX_TIMEOUT_MS: still pending 1 ms before the deadline, RelayerHttpTimeoutError exactly at it.
    it('timeout MAX_TIMEOUT_MS fires exactly at the deadline, not 1 ms early', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(['pending']);
      let settled = false;

      const run = scripted({ timeout: MAX_TIMEOUT_MS })
        .run()
        .catch((e: unknown) => e)
        .finally(() => {
          settled = true;
        });
      await vi.advanceTimersByTimeAsync(MAX_TIMEOUT_MS - 1);
      expect(settled).toBe(false);
      await vi.advanceTimersByTimeAsync(1);

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(MAX_TIMEOUT_MS);
    });

    // The timeout bounds the whole run: two 503s with waits, then a hung fetch, all stopped at 250 ms.
    it('bounds the whole run across attempts and waits', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503, S503, 'pending']);
      const request = scripted({ timeout: 250 });

      const run = request.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(250);

      expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
      expect(global.fetch).toHaveBeenCalledTimes(3);
      expect(request.decisions).toEqual([1, 2]);
    });

    // The system clock jumps 10 minutes ahead: the timeout is unaffected and the retry succeeds.
    it('a system-clock jump does not shorten nor extend the timeout', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503, S200]);
      const request = scripted({ timeout: 1000 });

      const run = request.run();
      vi.setSystemTime(Date.now() + 10 * 60_000);
      await vi.advanceTimersByTimeAsync(RETRY_DELAY_MS);

      await expect(run).resolves.toEqual(SHARES);
      expect(global.fetch).toHaveBeenCalledTimes(2);
    });
  });
});
