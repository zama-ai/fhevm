import type { Auth } from '../../../../types/auth.js';
import type { Logger } from '../../../../types/logger.js';
import type { RelayerCommonOptions } from '../../../../types/relayer.js';
import type { RelayerHttpOperation } from '../RelayerHttpAttempt.js';

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { InvalidPropertyError } from '../../../../base/errors/InvalidPropertyError.js';
import { RelayerHttpFetchError } from '../../../../errors/RelayerHttpFetchError.js';
import {
  KNOWN_RELAYER_HTTP_ERROR_CODES,
  RelayerHttpResponseError,
} from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpResultError } from '../../../../errors/RelayerHttpResultError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../../errors/RelayerHttpUnexpectedResponseError.js';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';

const URL_ = 'https://relayer.test/v2/public-decrypt';
const WAIT_PHASE = ' while waiting to retry';
const MAX_TIMEOUT_MS = 2_147_483_647;
const MAX_BODY_TEXT_LENGTH = 10 * 1024 * 1024;
// The content type a platform Response gives a string body.
const TEXT_PLAIN = 'text/plain;charset=UTF-8';
const PUBLIC_RESULT = { signatures: ['ab'.repeat(65)], decryptedValue: '00'.repeat(32), extraData: '0x00' };
const OK_TEXT = JSON.stringify({
  status: 'succeeded',
  requestId: '01890a5d-ac96-7000-8000-000000000000',
  result: PUBLIC_RESULT,
});
const ERROR_TEXT = JSON.stringify({
  code: 'overloaded',
  message: 'busy',
  requestId: '01890a5d-ac96-7000-8000-000000000001',
});

type FetchFn = (input: string | URL | Request, init?: RequestInit) => Promise<Response>;
type Script = (error: RelayerHttpResponseError, attempt: number) => number | undefined;
type Step = (init?: RequestInit) => Response | Promise<Response>;
type Outcome = { ok: true; value: unknown; timers: number } | { ok: false; reason: unknown; timers: number };

// The relayer issues UUIDv7 request ids; k picks a distinct, valid one.
function rid(k: number): string {
  return `01890a5d-ac96-7000-8000-${`${k}`.padStart(12, '0')}`;
}

// Body read is supplied by the test: resolved text, or a hook run when the body is read.
class TextResponse extends Response {
  readonly #read: () => Promise<string>;

  constructor(text: string, status: number, contentType = 'application/json', read?: () => Promise<string>) {
    super(text, { status, headers: { 'content-type': contentType } });
    this.#read = read ?? (() => Promise.resolve(text));
  }

  override text(): Promise<string> {
    return this.#read();
  }
}

function publicOk(): Response {
  return new TextResponse(OK_TEXT, 200);
}

function relayerError(): Response {
  return new TextResponse(ERROR_TEXT, 503);
}

const ok: Step = () => new TextResponse(OK_TEXT, 200);

function padded(text: string, length: number): string {
  return text + ' '.repeat(length - text.length);
}

function deferred<T>() {
  let resolve: (value: T) => void = () => undefined;
  const promise = new Promise<T>((r) => (resolve = r));
  return { promise, resolve };
}

function untilAborted(signal: AbortSignal | null | undefined): Promise<never> {
  return new Promise<never>((_resolve, reject) => {
    if (!signal) return;
    if (signal.aborted) return reject(signal.reason);
    signal.addEventListener('abort', () => reject(signal.reason), { once: true });
  });
}

function makeLogger(events: string[] = []) {
  const warn = vi.fn((message: string) => void events.push(`warn:${message}`));
  const error = vi.fn((message: string, _cause: unknown) => void events.push(`error:${message}`));
  const logger: Logger = { debug: undefined, warn, error };
  return { logger, warn, error };
}

function stubFetch(...steps: Step[]) {
  const fetchMock = vi.fn<FetchFn>((_input, init) => {
    const step = steps.shift();
    if (step === undefined) throw new Error('test: unexpected fetch');
    return Promise.resolve(step(init));
  });
  vi.stubGlobal('fetch', fetchMock);
  return fetchMock;
}

class ScriptedRequest extends RelayerHttpRequest<'PUBLIC_DECRYPT'> {
  public script: Script = () => undefined;

  protected override _retryDecision(error: RelayerHttpResponseError, attempt: number): number | undefined {
    return this.script(error, attempt);
  }
}

function scripted(logger: Logger, options: RelayerCommonOptions, script: Script = () => undefined): ScriptedRequest {
  const request = new ScriptedRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: URL_, payload: {}, options, logger });
  request.script = script;
  return request;
}

function capture(promise: Promise<unknown>): Promise<Outcome> {
  return promise.then(
    (value) => ({ ok: true as const, value, timers: vi.getTimerCount() }),
    (reason: unknown) => ({ ok: false as const, reason, timers: vi.getTimerCount() }),
  );
}

function rejected(outcome: Outcome): unknown {
  if (outcome.ok) throw new Error('test: expected a rejection');
  return outcome.reason;
}

// One request on a fresh caller signal; at settlement, zero timers and the one caller listener removed.
function setup(timeout = 100, options: RelayerCommonOptions = {}) {
  const controller = new AbortController();
  const add = vi.spyOn(controller.signal, 'addEventListener');
  const remove = vi.spyOn(controller.signal, 'removeEventListener');
  const log = { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
  const decision = vi.fn<Script>(() => 20);
  const request = scripted(log, { signal: controller.signal, timeout, fetchRetries: 0, ...options }, decision);
  function settled() {
    expect(vi.getTimerCount()).toBe(0);
    const additions = add.mock.calls.filter(([name]) => name === 'abort');
    const removals = remove.mock.calls.filter(([name]) => name === 'abort');
    expect(additions).toHaveLength(1);
    expect(removals).toHaveLength(1);
    expect(removals[0]?.[1]).toBe(additions[0]?.[1]);
  }
  function run() {
    return request.run().then(
      (value) => {
        settled();
        return { ok: true as const, value };
      },
      (error: unknown) => {
        settled();
        return { ok: false as const, error };
      },
    );
  }
  return { controller, request, decision, log, run };
}

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
  vi.restoreAllMocks();
});

describe('abort', () => {
  function expectTimeout(reason: unknown, timeoutMs: number) {
    expect(reason).toBeInstanceOf(RelayerHttpTimeoutError);
    expect((reason as RelayerHttpTimeoutError).timeoutMs).toBe(timeoutMs);
    expect((reason as RelayerHttpTimeoutError).url).toBe(URL_);
  }

  const callerReasons: Array<[string, () => unknown]> = [
    ['a string', () => 'caller gave up'],
    ['null', () => null],
    ['a caller TimeoutError', () => new DOMException('caller timeout', 'TimeoutError')],
    ['an Error', () => new Error('caller error')],
  ];

  describe('caller already aborted', () => {
    const reasons: Array<[string, (c: AbortController) => void]> = [
      ['a string', (c) => c.abort('early')],
      ['null', (c) => c.abort(null)],
      ['no reason (AbortError)', (c) => c.abort()],
    ];

    // Caller signal already aborted: run() rejects with that exact reason and never fetches.
    it.each(reasons)('rejects with the reason (%s) without fetching', async (_label, abort) => {
      const fetchMock = stubFetch(publicOk);
      const controller = new AbortController();
      abort(controller);

      const outcome = await capture(scripted(makeLogger().logger, { signal: controller.signal }).run());

      expect(rejected(outcome)).toBe(controller.signal.reason);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).not.toHaveBeenCalled();
    });
    // Pre-aborted caller with a 1 ms timeout configured: rejects with the caller's reason, not a timeout error.
    it('a pre-aborted caller is not a timeout, even with a timeout configured', async () => {
      stubFetch();
      const controller = new AbortController();
      controller.abort('gave up');
      const request = new RelayerHttpRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { signal: controller.signal, timeout: 1, fetchRetries: 0 },
        logger: makeLogger().logger,
      });

      const reason = rejected(await capture(request.run()));

      expect(reason).toBe('gave up');
      expect(reason).not.toBeInstanceOf(RelayerHttpTimeoutError);
    });

    const buildThrows: Array<[string, Record<string, unknown>, RelayerCommonOptions]> = [
      ['an unserializable payload', { big: 1n }, {}],
      ['an invalid auth header', {}, { auth: { type: 'BearerToken', token: 'a\nb' } }],
    ];

    // Pre-aborted caller and a request that would fail to build: the caller's reason wins, no fetch.
    it.each(buildThrows)(
      'the reason wins over a build throw (%s), without fetching',
      async (_label, payload, options) => {
        const fetchMock = stubFetch(publicOk);
        const controller = new AbortController();
        controller.abort('gave up');
        const request = new RelayerHttpRequest({
          relayerOperation: 'PUBLIC_DECRYPT',
          url: URL_,
          payload,
          options: { ...options, signal: controller.signal },
          logger: makeLogger().logger,
        });

        const outcome = await capture(request.run());

        expect(rejected(outcome)).toBe('gave up');
        expect(outcome.timers).toBe(0);
        expect(fetchMock).not.toHaveBeenCalled();
      },
    );
  });

  describe('abort during fetch', () => {
    // Caller aborts while fetch hangs: run() rejects with the caller's own reason, one warn, no error log.
    it.each(callerReasons)('caller abort with %s rejects with that reason and one warn', async (_label, make) => {
      const fetchMock = stubFetch((init) => untilAborted(init?.signal));
      const { logger, warn, error } = makeLogger();
      const controller = new AbortController();
      const reason = make();

      const out = capture(scripted(logger, { signal: controller.signal, timeout: 5000 }).run());
      await vi.advanceTimersByTimeAsync(100);
      controller.abort(reason);
      const outcome = await out;

      expect(rejected(outcome)).toBe(reason);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).toHaveBeenCalledTimes(1);
      expect(warn.mock.calls[0]?.[0]).toMatch(/^\[RelayerHttpRequest\]/);
      expect(error).not.toHaveBeenCalled();
    });

    // Fetch hangs with a 500 ms timeout: still pending at 499 ms, RelayerHttpTimeoutError at 500 ms.
    it('our timer rejects with RelayerHttpTimeoutError at the deadline, not before', async () => {
      stubFetch((init) => untilAborted(init?.signal));
      const { logger, warn } = makeLogger();
      const events: string[] = [];

      const out = capture(scripted(logger, { timeout: 500 }).run()).then((o) => (events.push('settled'), o));
      await vi.advanceTimersByTimeAsync(499);
      expect(events).toEqual([]);
      await vi.advanceTimersByTimeAsync(1);
      const outcome = await out;

      expectTimeout(rejected(outcome), 500);
      expect(outcome.timers).toBe(0);
      expect(warn).toHaveBeenCalledTimes(1);
    });

    // Caller aborts with an old RelayerHttpResponseError as reason: rejected with that same object,
    // never handed to the retry decision, one warn.
    it('a caller reason that is itself a RelayerHttpResponseError passes through by identity, with a single warn line', async () => {
      const first = new RelayerHttpRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: URL_, payload: {} });
      stubFetch(relayerError);
      const earlier = rejected(await capture(first.run()));
      expect(earlier).toBeInstanceOf(RelayerHttpResponseError);

      const controller = new AbortController();
      stubFetch((init) => untilAborted(init?.signal));
      const { logger, warn, error } = makeLogger();
      const decide = vi.fn<Script>(() => 1);
      const out = capture(scripted(logger, { signal: controller.signal, fetchRetries: 0 }, decide).run());
      await vi.advanceTimersByTimeAsync(0);
      controller.abort(earlier);

      expect(rejected(await out)).toBe(earlier);
      expect(decide).not.toHaveBeenCalled();
      expect(warn).toHaveBeenCalledTimes(1);
      expect(error).not.toHaveBeenCalled();
    });
  });

  describe('abort during the body read', () => {
    function slowBody(init?: RequestInit): Response {
      return new TextResponse(OK_TEXT, 200, undefined, () => untilAborted(init?.signal));
    }

    // Caller aborts while the body is being read: rejects with the caller's reason; the warn names the body-read phase.
    it.each(callerReasons)('caller abort with %s rejects with the reason', async (_label, make) => {
      stubFetch(slowBody);
      const { logger, warn } = makeLogger();
      const controller = new AbortController();
      const reason = make();

      const out = capture(scripted(logger, { signal: controller.signal }).run());
      await vi.advanceTimersByTimeAsync(10);
      controller.abort(reason);
      const outcome = await out;

      expect(rejected(outcome)).toBe(reason);
      expect(warn).toHaveBeenCalledTimes(1);
      expect(warn.mock.calls[0]?.[0]).toMatch(/^\[RelayerHttpRequest\]/);
      expect(warn.mock.calls[0]?.[0]).toContain(' while reading the response body');
    });

    // Body read hangs past the 300 ms timeout: RelayerHttpTimeoutError; the warn names the body-read phase.
    it('our timer rejects with RelayerHttpTimeoutError', async () => {
      stubFetch(slowBody);
      const { logger, warn } = makeLogger();

      const out = capture(scripted(logger, { timeout: 300 }).run());
      await vi.advanceTimersByTimeAsync(300);
      const outcome = await out;

      expectTimeout(rejected(outcome), 300);
      expect(outcome.timers).toBe(0);
      expect(warn).toHaveBeenCalledTimes(1);
      expect(warn.mock.calls[0]?.[0]).toContain(' while reading the response body');
    });
  });

  describe('abort from hooks at the stop', () => {
    // Retry decision aborts the caller then asks for a delay: rejects with the abort reason, no second fetch.
    it('abort inside a decision returning a delay: the abort wins and no second fetch starts', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const controller = new AbortController();
      const decide: Script = () => {
        controller.abort('abort with delay');
        return 50;
      };

      const out = capture(scripted(makeLogger().logger, { signal: controller.signal }, decide).run());
      await vi.advanceTimersByTimeAsync(100);
      const outcome = await out;

      expect(rejected(outcome)).toBe('abort with delay');
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });
  });

  describe('abort during the wait between attempts', () => {
    // Caller aborts during the wait between attempts: rejects with the reason, no second fetch, wait-phase warn.
    it.each(callerReasons)('caller abort with %s rejects with the reason, no second fetch', async (_label, make) => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const { logger, warn, error } = makeLogger();
      const controller = new AbortController();
      const reason = make();

      const out = capture(scripted(logger, { signal: controller.signal, timeout: 10_000 }, () => 1000).run());
      await vi.advanceTimersByTimeAsync(400);
      controller.abort(reason);
      const outcome = await out;

      expect(rejected(outcome)).toBe(reason);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).toHaveBeenCalledTimes(2);
      expect(warn.mock.calls[1]?.[0]).toMatch(/^\[RelayerHttpRequest\]/);
      expect(warn.mock.calls[1]?.[0]).toContain(WAIT_PHASE);
      expect(error).not.toHaveBeenCalled();
    });

    // Abort reason whose name getter throws, during the wait: still rejects with that reason at once.
    it('caller abort with a reason whose name getter throws rejects with the reason, no second fetch', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const controller = new AbortController();
      const reason = {
        get name(): string {
          throw new Error('hostile name getter');
        },
      };

      let outcome: Outcome | undefined;
      void capture(scripted(makeLogger().logger, { signal: controller.signal }, () => 1000).run()).then(
        (settled) => (outcome = settled),
      );
      await vi.advanceTimersByTimeAsync(400);
      controller.abort(reason);
      // Flush promise jobs only: the wait's own timer must not be what settles the run.
      await vi.advanceTimersByTimeAsync(0);

      expect(outcome).toBeDefined();
      expect(rejected(outcome!)).toBe(reason);
      expect(outcome!.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // Timeout expires during the wait between attempts: RelayerHttpTimeoutError, no second fetch.
    //   fetch ──► 503 ──► retry warn (clock +600 ms) ──► wait 500 ms ──► t=1000 timeout ──► rejects
    it('our timer during the wait rejects with RelayerHttpTimeoutError', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const events: string[] = [];
      let advanced = false;
      // The retry warn spends 600 ms after the remaining-time check, so the sleep ends past the deadline.
      const warn = vi.fn((message: string) => {
        events.push(`warn:${message}`);
        if (!advanced) {
          advanced = true;
          vi.advanceTimersByTime(600);
        }
      });
      const logger: Logger = { debug: undefined, warn, error: vi.fn() };

      const out = capture(scripted(logger, { timeout: 1000 }, () => 500).run());
      await vi.advanceTimersByTimeAsync(399);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      await vi.advanceTimersByTimeAsync(1);
      const outcome = await out;

      expectTimeout(rejected(outcome), 1000);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).toHaveBeenCalledTimes(2);
      expect(warn.mock.calls[1]?.[0]).toContain(WAIT_PHASE);
    });

    // Caller abort and the wait's timer due at the same instant, abort armed first: the wait rejects with the reason.
    //   503 ──► warn arms abort@200 ──► wait arms timer@200 ──► t=200 abort fires first ──► rejects
    it('a caller abort armed before the wait and due when it ends is seen by the wait', async () => {
      const controller = new AbortController();
      const fetchMock = stubFetch(relayerError, publicOk);
      const events: string[] = [];
      // Armed from the retry warn, before the wait's own timer, for the same due time.
      const warn = vi.fn((message: string) => {
        events.push(message);
        if (events.length === 1) setTimeout(() => controller.abort('at due time'), 200);
      });
      const logger: Logger = { debug: undefined, warn, error: vi.fn() };

      const out = capture(scripted(logger, { signal: controller.signal }, () => 200).run());
      await vi.advanceTimersByTimeAsync(300);
      const outcome = await out;

      expect(rejected(outcome)).toBe('at due time');
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).toHaveBeenCalledTimes(2);
      expect(events[1]).toContain(WAIT_PHASE);
    });
  });

  describe('abort between the end of the wait and the next fetch', () => {
    // Caller aborts after the wait's timer fired, before the next fetch: rejects with the reason, no second fetch.
    //   503 ──► wait 200 ms ──► timer fires (sync advance) ──► caller aborts ──► next attempt rejects
    it.each(callerReasons)(
      'caller abort with %s: the next attempt rejects with the reason before fetching',
      async (_label, make) => {
        const fetchMock = stubFetch(relayerError, publicOk);
        const { logger, warn, error } = makeLogger();
        const controller = new AbortController();
        const reason = make();

        const out = capture(scripted(logger, { signal: controller.signal }, () => 200).run());
        await vi.advanceTimersByTimeAsync(100);
        expect(warn).toHaveBeenCalledTimes(1);
        expect(vi.getTimerCount()).toBeGreaterThan(0);
        vi.advanceTimersByTime(100);
        controller.abort(reason);
        const outcome = await out;

        expect(rejected(outcome)).toBe(reason);
        expect(outcome.timers).toBe(0);
        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(warn).toHaveBeenCalledTimes(2);
        expect(warn.mock.calls[1]?.[0]).toMatch(/^\[RelayerHttpRequest\]/);
        expect(warn.mock.calls[1]?.[0]).not.toContain(WAIT_PHASE);
        expect(error).not.toHaveBeenCalled();
      },
    );

    // Wait ends at 999 ms, timeout fires at 1000 ms before the next fetch: RelayerHttpTimeoutError, one fetch.
    it('our timer due just after the wait ends: RelayerHttpTimeoutError before the next fetch', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const { logger, warn } = makeLogger();

      // The wait ends at 999 ms, our timer fires at 1000 ms, in one synchronous advance.
      const out = capture(scripted(logger, { timeout: 1000 }, () => 999).run());
      await vi.advanceTimersByTimeAsync(500);
      expect(warn).toHaveBeenCalledTimes(1);
      vi.advanceTimersByTime(500);
      const outcome = await out;

      expectTimeout(rejected(outcome), 1000);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).toHaveBeenCalledTimes(2);
      expect(warn.mock.calls[1]?.[0]).not.toContain(WAIT_PHASE);
    });
  });

  describe('abort during the network-retry delay', () => {
    function networkThenOk() {
      return stubFetch(() => Promise.reject(new TypeError('network down')), publicOk);
    }
    const retryOptions: RelayerCommonOptions = { fetchRetries: 1, fetchRetryDelayInMilliseconds: 10_000 };

    // Caller aborts during the network-retry delay: settles at once with the reason, no second fetch, no decision.
    it.each(callerReasons)(
      'caller abort with %s settles at once with the reason, no second fetch',
      async (_label, make) => {
        const fetchMock = networkThenOk();
        const decide = vi.fn<Script>(() => 10);
        const controller = new AbortController();
        const reason = make();
        const events: string[] = [];

        const out = capture(
          scripted(makeLogger().logger, { ...retryOptions, signal: controller.signal }, decide).run(),
        );
        void out.then(() => events.push('settled'));
        await vi.advanceTimersByTimeAsync(100);
        expect([fetchMock.mock.calls.length, events]).toEqual([1, []]);
        expect(vi.getTimerCount()).toBeGreaterThan(0);
        controller.abort(reason);
        await vi.advanceTimersByTimeAsync(1);
        expect(events).toEqual(['settled']);
        const outcome = await out;

        expect(rejected(outcome)).toBe(reason);
        expect(outcome.timers).toBe(0);
        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(decide).not.toHaveBeenCalled();
      },
    );

    // Abort reason whose name getter throws, during the network-retry delay: settles at once with that reason.
    it('caller abort with a reason whose name getter throws settles at once with the reason, no second fetch', async () => {
      const fetchMock = networkThenOk();
      const decide = vi.fn<Script>(() => 10);
      const controller = new AbortController();
      const reason = {
        get name(): string {
          throw new Error('hostile name getter');
        },
      };

      let outcome: Outcome | undefined;
      void capture(scripted(makeLogger().logger, { ...retryOptions, signal: controller.signal }, decide).run()).then(
        (settled) => (outcome = settled),
      );
      await vi.advanceTimersByTimeAsync(100);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      controller.abort(reason);
      // Flush promise jobs only: the 10 s retry delay must not be what settles the run.
      await vi.advanceTimersByTimeAsync(0);

      expect(outcome).toBeDefined();
      expect(rejected(outcome!)).toBe(reason);
      expect(outcome!.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(decide).not.toHaveBeenCalled();
    });

    // Timeout expires during a 10 s network-retry delay: RelayerHttpTimeoutError at 500 ms, no second fetch.
    it('our timer during the network-retry delay rejects with RelayerHttpTimeoutError', async () => {
      const fetchMock = networkThenOk();
      const events: string[] = [];

      const out = capture(scripted(makeLogger().logger, { ...retryOptions, timeout: 500 }).run());
      void out.then(() => events.push('settled'));
      await vi.advanceTimersByTimeAsync(499);
      expect([fetchMock.mock.calls.length, events]).toEqual([1, []]);
      await vi.advanceTimersByTimeAsync(1);
      const outcome = await out;

      expectTimeout(rejected(outcome), 500);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });
  });

  describe('failures on a live signal are not aborts', () => {
    // Fetch rejects with an AbortError while the caller signal is live: a RelayerHttpFetchError, not an abort.
    it('a fetch rejecting with an AbortError while the signal is live is a fetch error', async () => {
      const controller = new AbortController();
      stubFetch(() => Promise.reject(new DOMException('aborted elsewhere', 'AbortError')));

      const outcome = await capture(
        scripted(makeLogger().logger, { signal: controller.signal, fetchRetries: 0 }).run(),
      );

      expect(rejected(outcome)).toBeInstanceOf(RelayerHttpFetchError);
      expect(controller.signal.aborted).toBe(false);
      expect(outcome.timers).toBe(0);
    });

    // Body read of a 503 fails while the signal is live: RelayerHttpFetchError, retry decision never called.
    it('a body read rejecting while the signal is live is a fetch error, never a decision', async () => {
      const fetchMock = stubFetch(
        () => new TextResponse(ERROR_TEXT, 503, undefined, () => Promise.reject(new TypeError('reset'))),
      );
      const decide = vi.fn<Script>(() => 10);

      const outcome = await capture(scripted(makeLogger().logger, { timeout: 5000 }, decide).run());

      expect(rejected(outcome)).toBeInstanceOf(RelayerHttpFetchError);
      expect(decide).not.toHaveBeenCalled();
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(outcome.timers).toBe(0);
    });
  });
  describe('abort precedence and lifetime', () => {
    for (const phase of ['fetch', 'body'] as const) {
      // Caller aborts while blocked in fetch or in the body read: rejects with the exact reason (any type),
      // one warn, no decision, listener and timers cleaned up.
      it.each(['caller', null, new DOMException('caller timeout', 'TimeoutError'), { source: 'caller' }])(
        `preserves caller reason during ${phase}: %j`,
        async (reason) => {
          const context = setup();
          const entered = deferred<void>();
          const events: string[] = [];
          stubFetch((init) => {
            const blocked = () =>
              new Promise<never>((_resolve, reject) => {
                events.push(phase);
                entered.resolve();
                init?.signal?.addEventListener(
                  'abort',
                  () => {
                    events.push('signal');
                    reject(init.signal?.reason);
                  },
                  { once: true },
                );
              });
            if (phase === 'fetch') return blocked();
            return new TextResponse(OK_TEXT, 200, undefined, blocked);
          });
          const pending = context.run();
          await entered.promise;
          events.push('caller');
          context.controller.abort(reason);
          const outcome = await pending;
          expect(outcome.ok).toBe(false);
          if (!outcome.ok) expect(outcome.error).toBe(reason);
          events.push('settled');
          expect(events).toEqual([phase, 'caller', 'signal', 'settled']);
          expect(context.log.warn).toHaveBeenCalledTimes(1);
          expect(context.decision).not.toHaveBeenCalled();
        },
      );
    }

    // Caller abort and timeout both due at 100 ms: whichever timer was armed first wins.
    //   caller-first: abort armed ──► run() arms timeout ──► t=100 ──► caller reason
    //   timer-first:  run() arms timeout ──► abort armed ──► t=100 ──► RelayerHttpTimeoutError
    it.each(['caller-first', 'timer-first'])('keeps the first abort at the same deadline: %s', async (order) => {
      const context = setup();
      const events: string[] = [];
      const entered = deferred<void>();
      const reason = { source: 'caller' };
      const abort = () => {
        events.push('caller-timer');
        context.controller.abort(reason);
      };
      const fetchMock = stubFetch(
        (init) =>
          new Promise<Response>((_resolve, reject) => {
            events.push('fetch');
            entered.resolve();
            init?.signal?.addEventListener(
              'abort',
              () => {
                events.push('combined-abort');
                reject(init.signal?.reason);
              },
              { once: true },
            );
          }),
      );
      if (order === 'caller-first') setTimeout(abort, 100);
      const pending = context.run();
      await entered.promise;
      if (order === 'timer-first') setTimeout(abort, 100);
      vi.advanceTimersByTime(100);
      events.push('timers-finished');
      const outcome = await pending;
      expect(outcome.ok).toBe(false);
      if (!outcome.ok) {
        if (order === 'caller-first') expect(outcome.error).toBe(reason);
        else expect(outcome.error).toBeInstanceOf(RelayerHttpTimeoutError);
      }
      expect(events).toEqual(
        order === 'caller-first'
          ? ['fetch', 'caller-timer', 'combined-abort', 'timers-finished']
          : ['fetch', 'combined-abort', 'caller-timer', 'timers-finished'],
      );
      expect(context.log.warn).toHaveBeenCalledTimes(1);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // Shared signal aborted during a first request: a later request on it rejects with the same reason, no fetch.
    it('a later request on an aborted shared signal rejects without another fetch', async () => {
      const context = setup();
      const entered = deferred<void>();
      const reason = { source: 'shared' };
      const fetchMock = stubFetch(
        (init) =>
          new Promise<Response>((_resolve, reject) => {
            entered.resolve();
            init?.signal?.addEventListener('abort', () => reject(init.signal?.reason), { once: true });
          }),
      );
      const first = context.run();
      await entered.promise;
      context.controller.abort(reason);
      const firstOutcome = await first;
      expect(firstOutcome.ok).toBe(false);
      if (!firstOutcome.ok) expect(firstOutcome.error).toBe(reason);
      const later = new ScriptedRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { signal: context.controller.signal, fetchRetries: 0 },
      });
      const laterDecision = vi.fn<Script>(() => 20);
      later.script = laterDecision;
      await expect(later.run()).rejects.toBe(reason);
      expect(laterDecision).not.toHaveBeenCalled();
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });
  });
});

describe('leaks', () => {
  const pending: Step = (init) => untilAborted(init?.signal);

  type Scenario = {
    name: string;
    steps: Step[] | ((controller: AbortController) => Step[]);
    script?: Script;
    timeout?: number;
    fetchRetries?: number;
    fetchRetryDelayInMilliseconds?: number;
    payload?: Record<string, unknown>;
    logger?: () => Logger;
    // Drives time and the caller signal until the run settles.
    drive?: (controller: AbortController) => Promise<void>;
    expectOk: boolean;
  };

  const advance = (ms: number) => async () => void (await vi.advanceTimersByTimeAsync(ms));
  const networkError: Step = () => Promise.reject(new TypeError('net'));

  // The first retry warn spends 600 ms after the remaining-time check, so our timer fires during the wait.
  function slowRetryWarnLogger(): Logger {
    let spent = false;
    const warn = () => {
      if (spent) return;
      spent = true;
      vi.advanceTimersByTime(600);
    };
    return { debug: undefined, warn, error: vi.fn() };
  }

  const scenarios: Scenario[] = [
    { name: 'first-try success', steps: [ok], expectOk: true },
    { name: 'first-try success with a timeout', steps: [ok], timeout: 5000, expectOk: true },
    { name: 'default stop on a relayer-http error', steps: [relayerError], timeout: 5000, expectOk: false },
    {
      name: 'retried success',
      steps: [relayerError, ok],
      script: () => 100,
      timeout: 5000,
      drive: advance(100),
      expectOk: true,
    },
    {
      name: 'retried failure (script ends)',
      steps: [relayerError, relayerError],
      script: (_e, n) => (n === 1 ? 100 : undefined),
      timeout: 5000,
      drive: advance(100),
      expectOk: false,
    },
    {
      name: 'attempt ceiling',
      steps: [relayerError, relayerError, relayerError, relayerError, relayerError],
      script: () => 10,
      timeout: 5000,
      drive: advance(100),
      expectOk: false,
    },
    { name: 'invalid delay', steps: [relayerError], script: () => 0, timeout: 5000, expectOk: false },
    {
      name: 'delay not fitting the remaining time',
      steps: [relayerError],
      script: () => 5000,
      timeout: 5000,
      expectOk: false,
    },
    {
      name: 'throwing decision',
      steps: [relayerError],
      script: () => {
        throw new Error('decide');
      },
      timeout: 5000,
      expectOk: false,
    },
    {
      name: 'non-JSON body',
      steps: [() => new TextResponse('nope', 502, 'text/plain')],
      timeout: 5000,
      expectOk: false,
    },
    { name: 'non-relayer JSON error', steps: [() => new TextResponse('{"x":1}', 500)], timeout: 5000, expectOk: false },
    {
      name: 'result schema mismatch',
      steps: [
        () =>
          new TextResponse(
            '{"status":"succeeded","requestId":"01890a5d-ac96-7000-8000-000000000000","result":{}}',
            200,
          ),
      ],
      timeout: 5000,
      expectOk: false,
    },
    {
      name: 'network error',
      steps: [() => Promise.reject(new TypeError('net'))],
      timeout: 5000,
      fetchRetries: 0,
      expectOk: false,
    },
    { name: 'request init build throws', steps: [ok], payload: { big: 1n }, timeout: 5000, expectOk: false },
    {
      name: 'caller abort during fetch',
      steps: [pending],
      timeout: 5000,
      drive: async (c) => {
        await vi.advanceTimersByTimeAsync(10);
        c.abort('stop');
      },
      expectOk: false,
    },
    {
      name: 'caller abort during the body read',
      steps: [(init) => new TextResponse(OK_TEXT, 200, undefined, () => untilAborted(init?.signal))],
      timeout: 5000,
      drive: async (c) => {
        await vi.advanceTimersByTimeAsync(10);
        c.abort('stop');
      },
      expectOk: false,
    },
    {
      name: 'caller abort during the wait',
      steps: [relayerError, ok],
      script: () => 1000,
      timeout: 5000,
      drive: async (c) => {
        await vi.advanceTimersByTimeAsync(500);
        c.abort('stop');
      },
      expectOk: false,
    },
    {
      name: 'caller abort between the end of the wait and the next fetch',
      steps: [relayerError, ok],
      script: () => 200,
      timeout: 5000,
      drive: async (c) => {
        await vi.advanceTimersByTimeAsync(100);
        vi.advanceTimersByTime(100);
        c.abort('stop');
      },
      expectOk: false,
    },
    {
      name: 'caller abort during the network-retry delay',
      steps: [networkError, ok],
      fetchRetries: 1,
      fetchRetryDelayInMilliseconds: 10_000,
      timeout: 5000,
      drive: async (c) => {
        await vi.advanceTimersByTimeAsync(100);
        c.abort('stop');
      },
      expectOk: false,
    },
    { name: 'our timer during fetch', steps: [pending], timeout: 300, drive: advance(300), expectOk: false },
    {
      name: 'our timer during the wait',
      steps: [relayerError, ok],
      script: () => 500,
      timeout: 1000,
      logger: slowRetryWarnLogger,
      drive: advance(400),
      expectOk: false,
    },
    {
      name: 'our timer during the network-retry delay',
      steps: [networkError, ok],
      fetchRetries: 1,
      fetchRetryDelayInMilliseconds: 10_000,
      timeout: 500,
      drive: advance(500),
      expectOk: false,
    },
  ];

  function fetchFor(steps: Step[]) {
    const queue = [...steps];
    return vi.fn<FetchFn>((_input, init) => {
      const step = queue.shift();
      if (step === undefined) throw new Error('test: unexpected fetch');
      return Promise.resolve(step(init));
    });
  }

  // Invokes run() synchronously; a synchronous throw becomes a rejection.
  function callCapturing(call: () => Promise<unknown>): Promise<unknown> {
    try {
      return call();
    } catch (reason: unknown) {
      return Promise.reject(reason);
    }
  }

  describe('resources while running', () => {
    function abortListeners(signal: AbortSignal) {
      const add = vi.spyOn(signal, 'addEventListener');
      const remove = vi.spyOn(signal, 'removeEventListener');
      const ofType = (spy: typeof add | typeof remove) => spy.mock.calls.filter(([type]) => type === 'abort').length;
      return () => [ofType(add), ofType(remove)];
    }

    // While the fetch is pending: one timer only if a timeout is set, one caller listener; removed after abort.
    it.each([
      [5000, 1],
      [undefined, 0],
    ])('timeout %s: %i timer and one caller listener while the fetch is pending', async (timeout, timers) => {
      const controller = new AbortController();
      const listeners = abortListeners(controller.signal);
      vi.stubGlobal('fetch', fetchFor([pending]));
      const options: RelayerCommonOptions = { signal: controller.signal };
      if (timeout !== undefined) options.timeout = timeout;
      const request = new ScriptedRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: URL_, payload: {}, options });

      const out = request.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);
      const running = { timers: vi.getTimerCount(), listeners: listeners() };
      controller.abort('done');
      await out;

      expect(running).toEqual({ timers, listeners: [1, 0] });
      expect(listeners()).toEqual([1, 1]);
    });
  });

  describe('resources at settlement', () => {
    // Every outcome path (success, failures, aborts, timeouts): no timer left and the one caller listener removed.
    it.each(scenarios)('$name: zero timers and the caller listener removed once', async (scenario) => {
      const controller = new AbortController();
      const add = vi.spyOn(controller.signal, 'addEventListener');
      const remove = vi.spyOn(controller.signal, 'removeEventListener');
      const abortListeners = (spy: typeof add | typeof remove) =>
        spy.mock.calls.filter(([type]) => type === 'abort').map(([, listener]) => listener);
      const steps = typeof scenario.steps === 'function' ? scenario.steps(controller) : scenario.steps;
      vi.stubGlobal('fetch', fetchFor(steps));
      const logger: Logger = scenario.logger?.() ?? { debug: undefined, warn: vi.fn(), error: vi.fn() };
      const options: RelayerCommonOptions = { signal: controller.signal };
      if (scenario.timeout !== undefined) options.timeout = scenario.timeout;
      if (scenario.fetchRetries !== undefined) options.fetchRetries = scenario.fetchRetries;
      if (scenario.fetchRetryDelayInMilliseconds !== undefined) {
        options.fetchRetryDelayInMilliseconds = scenario.fetchRetryDelayInMilliseconds;
      }
      const request = new ScriptedRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: scenario.payload ?? { handles: ['0x01'] },
        options,
        logger,
      });
      if (scenario.script) request.script = scenario.script;

      const out = request.run().then(
        () => ({ ok: true, timers: vi.getTimerCount(), added: abortListeners(add), removed: abortListeners(remove) }),
        () => ({ ok: false, timers: vi.getTimerCount(), added: abortListeners(add), removed: abortListeners(remove) }),
      );
      if (scenario.drive) await scenario.drive(controller);
      const snapshot = await out;

      expect.soft(snapshot.ok, 'settled ok').toBe(scenario.expectOk);
      expect.soft(snapshot.timers, 'timers at settlement').toBe(0);
      expect.soft(snapshot.added.length, 'listeners added').toBe(1);
      expect.soft(snapshot.removed.length, 'listeners removed').toBe(1);
      expect.soft(snapshot.removed[0] === snapshot.added[0], 'same listener removed').toBe(true);
    });

    // Pre-aborted caller signal: rejects with its reason, no fetch, no timer, no listener left behind.
    it('a pre-aborted caller signal leaves no timer and no listener', async () => {
      const controller = new AbortController();
      controller.abort('early');
      const add = vi.spyOn(controller.signal, 'addEventListener');
      const remove = vi.spyOn(controller.signal, 'removeEventListener');
      const fetchMock = fetchFor([ok]);
      vi.stubGlobal('fetch', fetchMock);
      const request = new ScriptedRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { signal: controller.signal, timeout: 5000 },
      });

      const settled = await request.run().then(
        () => ({ rejectedWith: undefined, timers: vi.getTimerCount() }),
        (reason: unknown) => ({ rejectedWith: reason, timers: vi.getTimerCount() }),
      );

      const added = add.mock.calls.filter(([type]) => type === 'abort').map(([, listener]) => listener);
      const removed = remove.mock.calls.filter(([type]) => type === 'abort').map(([, listener]) => listener);
      expect.soft(settled.rejectedWith, 'rejection').toBe('early');
      expect.soft(settled.timers, 'timers at settlement').toBe(0);
      expect.soft(fetchMock.mock.calls.length, 'fetch calls').toBe(0);
      expect.soft(added.length, 'listeners added').toBeLessThanOrEqual(1);
      expect.soft(added.filter((l) => !removed.includes(l)).length, 'listeners left').toBe(0);
    });

    // No caller signal, retry then failure: no timer left at settlement.
    it('a run without a caller signal leaves no timer after a retried failure', async () => {
      vi.stubGlobal('fetch', fetchFor([relayerError, relayerError]));
      const request = new ScriptedRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { timeout: 5000 },
      });
      request.script = (_e, n) => (n === 1 ? 100 : undefined);

      const out = request.run().then(
        () => -1,
        () => vi.getTimerCount(),
      );
      await vi.advanceTimersByTimeAsync(100);

      expect(await out).toBe(0);
    });

    // Successful run with a timeout: the timeout never fires afterwards (no warn, no timer).
    it('the timeout does not fire after a successful run settles', async () => {
      vi.stubGlobal('fetch', fetchFor([ok]));
      const warn = vi.fn();
      const request = new ScriptedRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { timeout: 1000 },
        logger: { debug: undefined, warn, error: vi.fn() },
      });

      await expect(request.run()).resolves.toEqual(PUBLIC_RESULT);
      await vi.advanceTimersByTimeAsync(5000);

      expect(warn).not.toHaveBeenCalled();
      expect(vi.getTimerCount()).toBe(0);
    });
    // After settlement, a caller abort is not forwarded to the fetch signal and logs nothing.
    it('the caller listener is gone: a later abort is not forwarded to the fetch signal and logs nothing', async () => {
      const controller = new AbortController();
      const add = vi.spyOn(controller.signal, 'addEventListener');
      const remove = vi.spyOn(controller.signal, 'removeEventListener');
      const abortListeners = (spy: typeof add | typeof remove) =>
        spy.mock.calls.filter(([type]) => type === 'abort').map(([, listener]) => listener);
      const fetchMock = fetchFor([ok]);
      vi.stubGlobal('fetch', fetchMock);
      const warn = vi.fn();
      const error = vi.fn();
      const request = new RelayerHttpRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { signal: controller.signal, timeout: 1000, fetchRetries: 0 },
        logger: { debug: vi.fn(), warn, error },
      });

      await request.run().catch(() => undefined);
      expect(abortListeners(remove)).toEqual(abortListeners(add));
      const fetchSignal = fetchMock.mock.calls[0]?.[1]?.signal;
      expect(fetchSignal?.aborted).toBe(false);

      controller.abort('later');
      await vi.advanceTimersByTimeAsync(0);

      expect(fetchSignal?.aborted).toBe(false);
      expect(warn).not.toHaveBeenCalled();
      expect(error).not.toHaveBeenCalled();
    });

    // A second run() on a settled request rejects and adds no listener or timer.
    it('a second run() does not add a listener or a timer', async () => {
      const controller = new AbortController();
      const add = vi.spyOn(controller.signal, 'addEventListener');
      const remove = vi.spyOn(controller.signal, 'removeEventListener');
      const abortListeners = (spy: typeof add | typeof remove) =>
        spy.mock.calls.filter(([type]) => type === 'abort').map(([, listener]) => listener);
      vi.stubGlobal('fetch', fetchFor([ok]));
      const request = new RelayerHttpRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { signal: controller.signal, timeout: 100, fetchRetries: 0 },
        logger: { debug: vi.fn(), warn: vi.fn(), error: vi.fn() },
      });
      await request.run().catch(() => undefined);

      const second = await callCapturing(() => request.run()).then(
        () => ({ ok: true, timers: vi.getTimerCount(), added: abortListeners(add), removed: abortListeners(remove) }),
        () => ({ ok: false, timers: vi.getTimerCount(), added: abortListeners(add), removed: abortListeners(remove) }),
      );

      expect(second.ok).toBe(false);
      expect(second.timers).toBe(0);
      expect(second.added).toHaveLength(1);
      expect(second.removed).toHaveLength(1);
    });
  });
});

describe('races', () => {
  // Records the settlement as an event: fulfilled, the caller reason, or the error class.
  function recordSettlement(run: Promise<unknown>, events: string[], reason: unknown): Promise<void> {
    return run.then(
      () => void events.push('fulfilled'),
      (e: unknown) => {
        if (e === reason) return void events.push('rejected:reason');
        if (e instanceof RelayerHttpTimeoutError) return void events.push('rejected:timeout');
        if (e instanceof RelayerHttpResponseError) return void events.push('rejected:response-error');
        events.push('rejected:other');
      },
    );
  }

  describe('caller abort and our timer at the same instant', () => {
    // Caller abort and timeout both due at 500 ms, caller scheduled first: rejects with the caller's reason.
    it('the caller abort scheduled first wins', async () => {
      stubFetch((init) => untilAborted(init?.signal));
      const controller = new AbortController();
      setTimeout(() => controller.abort('caller first'), 500);

      const out = scripted(makeLogger().logger, { signal: controller.signal, timeout: 500 }).run();
      const settled = Promise.allSettled([out]);
      await vi.advanceTimersByTimeAsync(500);
      const [result] = await settled;

      expect(result).toEqual({ status: 'rejected', reason: 'caller first' });
    });

    // Timeout armed by run() before a caller abort due at the same 500 ms: RelayerHttpTimeoutError wins.
    it('our timer, armed first, wins over a caller abort due at the same time', async () => {
      stubFetch((init) => untilAborted(init?.signal));
      const controller = new AbortController();

      const out = scripted(makeLogger().logger, { signal: controller.signal, timeout: 500 }).run();
      setTimeout(() => controller.abort('caller second'), 500);
      const settled = Promise.allSettled([out]);
      await vi.advanceTimersByTimeAsync(500);
      const [result] = await settled;

      expect(result?.status).toBe('rejected');
      expect((result as PromiseRejectedResult).reason).toBeInstanceOf(RelayerHttpTimeoutError);
    });
  });

  describe('abort issued right after run() returns', () => {
    // Caller aborts synchronously right after run() returns: rejects with the reason, no timer left.
    it('synchronously: the run rejects with the reason', async () => {
      stubFetch((init) => untilAborted(init?.signal));
      const controller = new AbortController();
      const events: string[] = [];

      const done = recordSettlement(scripted(makeLogger().logger, { signal: controller.signal }).run(), events, 'now');
      controller.abort('now');
      await done;

      expect(events).toEqual(['rejected:reason']);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Caller aborts one microtask after run() returns: rejects with the reason, no timer left.
    it('in the next microtask: the run rejects with the reason', async () => {
      stubFetch((init) => untilAborted(init?.signal));
      const controller = new AbortController();
      const events: string[] = [];

      const done = recordSettlement(scripted(makeLogger().logger, { signal: controller.signal }).run(), events, 'soon');
      queueMicrotask(() => controller.abort('soon'));
      await done;

      expect(events).toEqual(['rejected:reason']);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('gated orderings after the response arrived', () => {
    // Abort arrives after run() already rejected with a response error: the error stands, no extra log.
    it('settlement observed first, abort after: the response error stands and nothing more is logged', async () => {
      stubFetch(() => new TextResponse(ERROR_TEXT, 503));
      const controller = new AbortController();
      const events: string[] = [];
      const { logger, warn, error } = makeLogger();

      await recordSettlement(scripted(logger, { signal: controller.signal }).run(), events, 'late');
      controller.abort('late');
      events.push('abort');
      await vi.advanceTimersByTimeAsync(10);

      expect(events).toEqual(['rejected:response-error', 'abort']);
      expect(warn).not.toHaveBeenCalled();
      expect(error).toHaveBeenCalledTimes(1);
    });
  });

  describe('forced orderings', () => {
    // Fetch aborts the caller then rejects with a network error: the caller's reason wins.
    it('abort inside fetch that then rejects with another error: the caller reason wins', async () => {
      const controller = new AbortController();
      stubFetch(() => {
        controller.abort('caller reason');
        return Promise.reject(new TypeError('network'));
      });

      await expect(scripted(makeLogger().logger, { signal: controller.signal }).run()).rejects.toBe('caller reason');
    });

    // Caller aborts in the .then of a successful run: the result stands, nothing logged.
    it('abort after settlement, in the same tick: the result stands and nothing is logged', async () => {
      stubFetch(() => new TextResponse(OK_TEXT, 200));
      const controller = new AbortController();
      const events: string[] = [];
      const { logger, warn } = makeLogger(events);

      const value = await scripted(logger, { signal: controller.signal })
        .run()
        .then((v) => {
          events.push('settled');
          controller.abort('too late');
          events.push('aborted');
          return v;
        });
      await vi.advanceTimersByTimeAsync(10);

      expect(value).toEqual(PUBLIC_RESULT);
      expect(events).toEqual(['settled', 'aborted']);
      expect(warn).not.toHaveBeenCalled();
    });

    // One 503 retry: events happen in strict order.
    //   fetch ──► decide(1) ──► warn ──► wait 100 ms ──► fetch ──► settled
    it('a retry follows a strict order: fetch, decide, warn, wait, fetch, settle', async () => {
      const events: string[] = [];
      const steps = [() => new TextResponse(ERROR_TEXT, 503), () => new TextResponse(OK_TEXT, 200)];
      vi.stubGlobal(
        'fetch',
        vi.fn<FetchFn>(() => {
          events.push('fetch');
          return Promise.resolve(steps.shift()!());
        }),
      );
      const logger: Logger = { debug: undefined, warn: () => events.push('warn'), error: () => events.push('error') };
      const decide: Script = (_e, attempt) => (events.push(`decide:${attempt}`), 100);

      const out = scripted(logger, { timeout: 1000 }, decide)
        .run()
        .then(() => events.push('settled'));
      await vi.advanceTimersByTimeAsync(99);
      events.push('t=99');
      await vi.advanceTimersByTimeAsync(1);
      await out;

      expect(events).toEqual(['fetch', 'decide:1', 'warn', 't=99', 'fetch', 'settled']);
    });
  });
});

describe('reentrancy', () => {
  function quietLogger(): Logger {
    return { debug: undefined, warn: vi.fn(), error: vi.fn() };
  }

  // Reentrancy variant: options first, with a quiet logger by default.
  function scripted(options: RelayerCommonOptions = {}, logger: Logger = quietLogger()): ScriptedRequest {
    return new ScriptedRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: URL_, payload: {}, options, logger });
  }

  // Runs a call that may throw synchronously or reject; returns the rejection or throw.
  async function failureOf(call: () => Promise<unknown>): Promise<unknown> {
    try {
      await call();
    } catch (e) {
      return e;
    }
    throw new Error('test: expected a failure');
  }

  function listenerSpies(signal: AbortSignal) {
    const add = vi.spyOn(signal, 'addEventListener');
    const remove = vi.spyOn(signal, 'removeEventListener');
    const abortCalls = (spy: typeof add | typeof remove) => spy.mock.calls.filter(([type]) => type === 'abort');
    return { added: () => abortCalls(add).length, removed: () => abortCalls(remove).length };
  }

  describe('second run()', () => {
    // Second run() while the first is in flight: rejects without fetching; the first still resolves.
    it('rejects while the first run is pending, without fetching; the first run completes', async () => {
      let release: (r: Response) => void = () => undefined;
      const fetchMock = stubFetch(() => new Promise<Response>((resolve) => (release = resolve)));
      const request = scripted();

      const first = request.run();
      const second = await failureOf(() => request.run());
      release(publicOk());

      expect(second).toBeInstanceOf(Error);
      await expect(first).resolves.toEqual(PUBLIC_RESULT);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // Second run() after a success: rejects without a new fetch.
    it('rejects after a successful run, without fetching again', async () => {
      const fetchMock = stubFetch(publicOk, publicOk);
      const request = scripted();
      await request.run();

      expect(await failureOf(() => request.run())).toBeInstanceOf(Error);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // Second run() after a failure: rejects with a new error, not the first failure, no new fetch.
    it('rejects after a failed run, with an error that is not the first failure', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const request = scripted();
      const firstFailure = await failureOf(() => request.run());

      const secondFailure = await failureOf(() => request.run());

      expect(firstFailure).toBeInstanceOf(RelayerHttpResponseError);
      expect(secondFailure).toBeInstanceOf(Error);
      expect(secondFailure).not.toBe(firstFailure);
      expect(secondFailure).not.toBeInstanceOf(RelayerHttpResponseError);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // Repeated run() calls on a used request: the same non-empty error message each time.
    it('gives the same message on every rejected second call', async () => {
      stubFetch(publicOk);
      const request = scripted();
      await request.run();

      const a = (await failureOf(() => request.run())) as Error;
      const b = (await failureOf(() => request.run())) as Error;

      expect(a.message).toBe(b.message);
      expect(a.message.length).toBeGreaterThan(0);
    });

    // A rejected second run() adds no caller listener and no timer.
    it('does not touch the caller signal listeners', async () => {
      stubFetch(publicOk);
      const controller = new AbortController();
      const spies = listenerSpies(controller.signal);
      const request = scripted({ signal: controller.signal });
      await request.run();
      const [added, removed] = [spies.added(), spies.removed()];

      await failureOf(() => request.run());

      expect([spies.added(), spies.removed()]).toEqual([added, removed]);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('re-entry from hooks', () => {
    // run() re-entered from inside fetch: the inner call rejects, the outer run resolves.
    it('run() called from inside fetch rejects; the outer run completes', async () => {
      let inner: unknown = 'not called';
      const request = scripted();
      const fetchMock = stubFetch(() => {
        inner = failureOf(() => request.run());
        return publicOk();
      });

      await expect(request.run()).resolves.toEqual(PUBLIC_RESULT);
      expect(await inner).toBeInstanceOf(Error);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // run() re-entered from the retry decision: the inner call rejects, the retry still happens and succeeds.
    it('run() called from inside the decision rejects; the decision still drives the retry', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const request = scripted();
      let inner: Promise<unknown> | undefined;
      request.script = () => {
        inner = failureOf(() => request.run());
        return 10;
      };

      const out = request.run();
      await vi.advanceTimersByTimeAsync(10);

      await expect(out).resolves.toEqual(PUBLIC_RESULT);
      expect(await inner).toBeInstanceOf(Error);
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });

    // run() re-entered from the retry warn: the inner call rejects, the outer run resolves.
    it('run() called from inside the retry warn rejects; the outer run completes', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      let inner: Promise<unknown> | undefined;
      let request: ScriptedRequest | undefined;
      const logger: Logger = {
        debug: undefined,
        warn: () => {
          inner ??= failureOf(() => request!.run());
        },
        error: vi.fn(),
      };
      request = scripted({}, logger);
      request.script = () => 10;

      const out = request.run();
      await vi.advanceTimersByTimeAsync(10);

      await expect(out).resolves.toEqual(PUBLIC_RESULT);
      expect(await inner).toBeInstanceOf(Error);
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });

    // A retry decision that runs a different instance: both instances resolve independently.
    it('a decision that runs another instance gets that instance result independently', async () => {
      const fetchMock = stubFetch(relayerError, publicOk, publicOk);
      const other = scripted();
      let otherResult: Promise<unknown> | undefined;
      const request = scripted();
      request.script = () => {
        otherResult = other.run();
        return 10;
      };

      const out = request.run();
      await vi.advanceTimersByTimeAsync(10);

      await expect(out).resolves.toEqual(PUBLIC_RESULT);
      await expect(otherResult).resolves.toEqual(PUBLIC_RESULT);
      expect(fetchMock).toHaveBeenCalledTimes(3);
    });
  });

  describe('hooks that abort', () => {
    // Retry warn hook aborts the caller: the wait rejects with that reason, no second fetch.
    it('the logger.warn hook aborting the caller on the retry line: the wait rejects with the reason', async () => {
      const controller = new AbortController();
      const fetchMock = stubFetch(relayerError, publicOk);
      const warn = vi.fn<(message: string) => void>();
      const error = vi.fn<(message: string, cause: unknown) => void>();
      warn.mockImplementationOnce(() => controller.abort('from-warn'));
      const request = scripted({ signal: controller.signal, fetchRetries: 0 }, { debug: vi.fn(), warn, error });
      request.script = () => 10;

      const outcome = capture(request.run());
      await vi.runAllTimersAsync();

      expect(rejected(await outcome)).toBe('from-warn');
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).toHaveBeenCalledTimes(2);
      expect(warn.mock.calls[1]?.[0]).toContain(WAIT_PHASE);
      expect(error).not.toHaveBeenCalled();
    });
  });

  describe('caller signal shared across instances', () => {
    // Two pending requests on one signal: one abort rejects both with the same reason; both listeners removed.
    it('one abort rejects both pending instances with the same reason', async () => {
      stubFetch(
        (init) => untilAborted(init?.signal),
        (init) => untilAborted(init?.signal),
      );
      const controller = new AbortController();
      const spies = listenerSpies(controller.signal);
      const a = scripted({ signal: controller.signal });
      const b = scripted({ signal: controller.signal });

      const settled = Promise.allSettled([a.run(), b.run()]);
      await vi.advanceTimersByTimeAsync(10);
      const reason = { shared: true };
      controller.abort(reason);

      expect(await settled).toEqual([
        { status: 'rejected', reason },
        { status: 'rejected', reason },
      ]);
      expect(spies.added()).toBe(2);
      expect(spies.removed()).toBe(2);
    });

    // Shared signal: a finished request ignores a later abort; only the pending one rejects.
    it('a settled instance is not affected by a later abort of the shared signal', async () => {
      stubFetch(publicOk, (init) => untilAborted(init?.signal));
      const controller = new AbortController();
      const spies = listenerSpies(controller.signal);
      const warnA = vi.fn();
      const a = scripted({ signal: controller.signal }, { debug: undefined, warn: warnA, error: vi.fn() });
      const b = scripted({ signal: controller.signal });

      const resultA = await a.run();
      const removedAfterA = spies.removed();
      const outB = Promise.allSettled([b.run()]);
      await vi.advanceTimersByTimeAsync(10);
      controller.abort('only b');

      expect(resultA).toEqual(PUBLIC_RESULT);
      expect(removedAfterA).toBe(1);
      expect(await outB).toEqual([{ status: 'rejected', reason: 'only b' }]);
      expect(warnA).not.toHaveBeenCalled();
      expect(spies.added()).toBe(2);
      expect(spies.removed()).toBe(2);
    });

    // Request A's decision aborts the shared signal: request B, still pending, rejects with A's reason.
    it('an abort from inside one instance decision rejects the other pending instance', async () => {
      const fetchMock = stubFetch(relayerError, (init) => untilAborted(init?.signal));
      const controller = new AbortController();
      const a = scripted({ signal: controller.signal });
      const b = scripted({ signal: controller.signal });
      a.script = () => (controller.abort('from a'), undefined);

      const outA = Promise.allSettled([a.run()]);
      const outB = Promise.allSettled([b.run()]);
      await outA;

      expect(await outB).toEqual([{ status: 'rejected', reason: 'from a' }]);
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });

    // Shared signal, one done and one in flight: the abort rejects only the in-flight one; the done fetch signal stays live.
    it('a completed request no longer reacts to the shared signal; the in-flight one does', async () => {
      const controller = new AbortController();
      const add = vi.spyOn(controller.signal, 'addEventListener');
      const remove = vi.spyOn(controller.signal, 'removeEventListener');
      const abortCallbacks = (spy: typeof add | typeof remove): unknown[] =>
        spy.mock.calls.filter(([type]) => type === 'abort').map(([, listener]) => listener);
      const fetchMock = stubFetch(publicOk, (init) => untilAborted(init?.signal));
      const warn = vi.fn();
      const logger: Logger = { debug: vi.fn(), warn, error: vi.fn() };
      const done = scripted({ signal: controller.signal, fetchRetries: 0 }, logger);
      const pending = scripted({ signal: controller.signal, fetchRetries: 0 }, logger);

      const doneOutcome = await capture(done.run());
      expect(abortCallbacks(add)).toHaveLength(1);
      expect(abortCallbacks(remove)).toEqual(abortCallbacks(add));
      const doneSignal = fetchMock.mock.calls[0]?.[1]?.signal;
      const pendingOutcome = capture(pending.run());
      await vi.advanceTimersByTimeAsync(0);
      controller.abort('shared');

      expect(doneOutcome).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(rejected(await pendingOutcome)).toBe('shared');
      expect(doneSignal?.aborted).toBe(false);
      expect(fetchMock.mock.calls[1]?.[1]?.signal?.aborted).toBe(true);
      expect(abortCallbacks(add)).toHaveLength(2);
      expect(abortCallbacks(remove)).toHaveLength(2);
      expect(abortCallbacks(remove)[1]).toBe(abortCallbacks(add)[1]);
      expect(warn).toHaveBeenCalledTimes(1);
    });

    // New request on a signal already aborted by another request: rejects with that reason, no fetch.
    it('a request created on a signal aborted by another request lifetime rejects without fetching', async () => {
      const controller = new AbortController();
      const fetchMock = stubFetch((init) => untilAborted(init?.signal));
      const first = scripted({ signal: controller.signal, fetchRetries: 0 });
      const firstOutcome = capture(first.run());
      await vi.advanceTimersByTimeAsync(0);
      controller.abort('shared');
      rejected(await firstOutcome);

      const late = scripted({ signal: controller.signal, fetchRetries: 0 });
      expect(rejected(await capture(late.run()))).toBe('shared');
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });
  });
});

describe('retry', () => {
  const USER_RESULT = [{ payload: 'cd'.repeat(32), signature: 'ef'.repeat(65), extraData: '0x00' }];

  function userOk(): Response {
    const body = { status: 'succeeded', requestId: rid(0), result: { result: USER_RESULT } };
    return new TextResponse(JSON.stringify(body), 200);
  }

  // Retry variant: every field of the error body can be chosen.
  function relayerError(status = 503, code = 'overloaded', requestId = rid(1), message = 'busy'): Response {
    return new TextResponse(JSON.stringify({ code, message, requestId }), status);
  }

  // Retry variant: a debug hook is present.
  function makeLogger(events: string[] = []) {
    const warn = vi.fn((message: string) => void events.push(`warn:${message}`));
    const error = vi.fn((message: string, _cause: unknown) => void events.push(`error:${message}`));
    const debug = vi.fn();
    const logger: Logger = { debug, warn, error };
    return { logger, warn, error };
  }

  // Retry variant: steps take no argument, so the parameterized relayerError can be passed as is.
  function stubFetch(...steps: Array<() => Response | Promise<Response>>) {
    const fetchMock = vi.fn<FetchFn>(() => {
      const step = steps.shift();
      if (step === undefined) throw new Error('test: unexpected fetch');
      return Promise.resolve(step());
    });
    vi.stubGlobal('fetch', fetchMock);
    return fetchMock;
  }

  // Retry variant: the script comes before the options, and the payload is not empty.
  function scripted(logger: Logger, script: Script, options: RelayerCommonOptions = {}): ScriptedRequest {
    const request = new ScriptedRequest({
      relayerOperation: 'PUBLIC_DECRYPT',
      url: URL_,
      payload: { handles: ['0x01'] },
      options,
      logger,
    });
    request.script = script;
    return request;
  }

  type RecordedScript = Script & { calls: Array<[RelayerHttpResponseError, number]> };

  // A finite script: the delay for attempt n is list[n - 1]; every call is recorded.
  function delays(...list: Array<number | undefined>): RecordedScript {
    const fn = ((error: RelayerHttpResponseError, attempt: number) => {
      fn.calls.push([error, attempt]);
      return list[attempt - 1];
    }) as RecordedScript;
    fn.calls = [];
    return fn;
  }

  function retryLine(status: number, code: string, requestId: string, message: string, delay: number, attempt: number) {
    return (
      `[RelayerHttpRequest] relayer-http error on ${URL_} (status=${status}, code=${code}, ` +
      `requestId=${requestId}): ${message}. Retrying in ${delay} ms (attempt ${attempt}).`
    );
  }

  // One request on a fresh caller signal, with its abort listeners recorded.
  function withCallerSignal(options: RelayerCommonOptions = {}) {
    const controller = new AbortController();
    const remove = vi.spyOn(controller.signal, 'removeEventListener');
    const add = vi.spyOn(controller.signal, 'addEventListener');
    const { logger, warn, error } = makeLogger();
    const decision = vi.fn<Script>();
    const request = scripted(logger, decision, { signal: controller.signal, fetchRetries: 0, ...options });
    return { request, decision, warn, error, controller, remove, add };
  }

  // Zero timers, and the one caller listener added is the one removed.
  function expectClean(context: ReturnType<typeof withCallerSignal>) {
    expect(vi.getTimerCount()).toBe(0);
    const added = context.add.mock.calls.filter(([name]) => name === 'abort');
    const removed = context.remove.mock.calls.filter(([name]) => name === 'abort');
    expect(added).toHaveLength(1);
    expect(removed).toHaveLength(1);
    expect(removed[0]?.[1]).toBe(added[0]?.[1]);
  }

  describe('default decision (base class)', () => {
    // Base class on a relayer 503: one fetch, rejects with the parsed RelayerHttpResponseError, one error log.
    it('makes one fetch and throws the relayer-http error with one error line', async () => {
      const fetchMock = stubFetch(() => relayerError(503, 'overloaded', rid(1), 'busy'));
      const { logger, warn, error } = makeLogger();
      const request = new RelayerHttpRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: URL_, payload: {}, logger });

      const reason = rejected(await capture(request.run()));

      expect(reason).toBeInstanceOf(RelayerHttpResponseError);
      const e = reason as RelayerHttpResponseError;
      expect([e.url, e.status, e.code, e.responseMessage, e.requestId]).toEqual([
        URL_,
        503,
        'overloaded',
        'busy',
        rid(1),
      ]);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).not.toHaveBeenCalled();
      expect(error).toHaveBeenCalledTimes(1);
      expect(error.mock.calls[0]?.[0]).toMatch(/^\[RelayerHttpRequest\] relayer-http error on /);
      expect(error.mock.calls[0]?.[0]).toContain(URL_);
      expect(error.mock.calls[0]?.[1]).toBeUndefined();
    });

    const statuses = [400, 401, 403, 404, 408, 422, 429, 500, 502, 503, 504];
    const codes = [...KNOWN_RELAYER_HTTP_ERROR_CODES, 'some_future_code'];
    const pairs = statuses.flatMap((status) => codes.map((code) => [status, code] as const));

    // Default decision never retries: undefined for every status, code and attempt 1..5.
    it.each(pairs)('returns undefined for status %i, code %s and attempts 1..5', async (status, code) => {
      const fetchMock = stubFetch(() => relayerError(status, code));
      const seen: Array<number | undefined> = [];
      class DefaultProbe extends RelayerHttpRequest<'PUBLIC_DECRYPT'> {
        protected override _retryDecision(error: RelayerHttpResponseError, attempt: number): number | undefined {
          for (let k = 1; k <= 5; k++) seen.push(super._retryDecision(error, k));
          return super._retryDecision(error, attempt);
        }
      }
      const request = new DefaultProbe({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        logger: makeLogger().logger,
      });

      const reason = rejected(await capture(request.run()));

      expect(reason).toBeInstanceOf(RelayerHttpResponseError);
      expect(seen).toEqual([undefined, undefined, undefined, undefined, undefined]);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // First-try success: the public-decrypt result, decision never called, nothing logged.
    it('returns the public-decrypt result on a first-try success without calling the decision', async () => {
      const fetchMock = stubFetch(publicOk);
      const { logger, warn, error } = makeLogger();
      const script = delays();
      const outcome = await capture(scripted(logger, script).run());

      expect(outcome).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(script.calls).toEqual([]);
      expect(warn).not.toHaveBeenCalled();
      expect(error).not.toHaveBeenCalled();
    });

    // User-decrypt success returns the result items.
    it('returns the user-decrypt result items', async () => {
      stubFetch(userOk);
      const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: URL_, payload: {} });

      await expect(request.run()).resolves.toEqual(USER_RESULT);
    });

    // User-decrypt success with a wrong result shape: RelayerHttpResultError caused by InvalidPropertyError.
    it.each([
      ['a public-decrypt result', { result: PUBLIC_RESULT }],
      ['a result that is not an array', { result: { result: USER_RESULT[0] } }],
    ])('a user-decrypt success carrying %s is a RelayerHttpResultError', async (_label, payload) => {
      const fetchMock = stubFetch(
        () => new TextResponse(JSON.stringify({ status: 'succeeded', requestId: rid(0), ...payload }), 200),
      );
      const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: URL_, payload: {} });

      const reason = rejected(await capture(request.run()));

      expect(reason).toBeInstanceOf(RelayerHttpResultError);
      expect((reason as Error).cause).toBeInstanceOf(InvalidPropertyError);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // User-decrypt subclass retrying once: a 503 then a 200 returns the items after 2 fetches.
    it('a user-decrypt override retries a relayer-http error, then returns the items', async () => {
      const fetchMock = stubFetch(() => relayerError(503, 'overloaded', rid(9)), userOk);
      const calls: Array<[string | undefined, number]> = [];
      class UserScripted extends RelayerHttpRequest<'USER_DECRYPT'> {
        protected override _retryDecision(error: RelayerHttpResponseError, attempt: number): number | undefined {
          calls.push([error.requestId, attempt]);
          return 10;
        }
      }
      const request = new UserScripted({ relayerOperation: 'USER_DECRYPT', url: URL_, payload: {} });

      const out = capture(request.run());
      await vi.advanceTimersByTimeAsync(10);

      expect(await out).toMatchObject({ ok: true, value: USER_RESULT });
      expect(calls).toEqual([[rid(9), 1]]);
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });
  });

  describe('override dispatch', () => {
    // The decision receives the exact rejected error instance and attempt 1.
    it('calls the override with the exact error instance and the 1-based attempt', async () => {
      stubFetch(() => relayerError(503, 'overloaded', rid(1)));
      const script = delays(undefined);

      const reason = rejected(await capture(scripted(makeLogger().logger, script).run()));

      expect(script.calls).toHaveLength(1);
      expect(script.calls[0]?.[0]).toBe(reason);
      expect(script.calls[0]?.[1]).toBe(1);
    });

    // Two instances with different scripts: each uses its own decision (one retries and succeeds, one stops).
    it('dispatches per instance: two instances with different scripts', async () => {
      const fetchMock = stubFetch(
        () => relayerError(503, 'overloaded', rid(101)),
        publicOk,
        () => relayerError(503, 'overloaded', rid(102)),
      );
      const scriptA = delays(10);
      const scriptB = delays(undefined);
      const a = scripted(makeLogger().logger, scriptA);
      const b = scripted(makeLogger().logger, scriptB);

      const outA = capture(a.run());
      await vi.advanceTimersByTimeAsync(10);
      const resultA = await outA;
      const resultB = await capture(b.run());

      expect(resultA).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(rejected(resultB)).toMatchObject({ requestId: rid(102) });
      expect(scriptA.calls.map(([e, n]) => [e.requestId, n])).toEqual([[rid(101), 1]]);
      expect(scriptB.calls.map(([e, n]) => [e.requestId, n])).toEqual([[rid(102), 1]]);
      expect(fetchMock).toHaveBeenCalledTimes(3);
    });
    // Two interleaved instances: their decisions and attempt counts stay independent; both clean up.
    it('dispatches independent decisions for two interleaved instances', async () => {
      const left = withCallerSignal();
      const right = withCallerSignal();
      left.decision.mockReturnValueOnce(10).mockReturnValue(undefined);
      right.decision.mockReturnValue(undefined);
      const fetchMock = vi.fn<FetchFn>(() => Promise.resolve(relayerError()));
      vi.stubGlobal('fetch', fetchMock);
      const a = capture(left.request.run());
      const b = capture(right.request.run());
      const rightOutcome = await b;
      expect(rightOutcome.ok).toBe(false);
      if (!rightOutcome.ok) expect(rightOutcome.reason).toBe(right.decision.mock.calls[0]?.[0]);
      expect(right.decision.mock.calls.map(([, attempt]) => attempt)).toEqual([1]);
      await vi.advanceTimersByTimeAsync(10);
      const leftOutcome = await a;
      expect(leftOutcome.ok).toBe(false);
      if (!leftOutcome.ok) expect(leftOutcome.reason).toBe(left.decision.mock.calls[1]?.[0]);
      expect(left.decision.mock.calls.map(([, attempt]) => attempt)).toEqual([1, 2]);
      expect(fetchMock).toHaveBeenCalledTimes(3);
      expectClean(left);
      expectClean(right);
    });
  });

  describe('retry then outcome', () => {
    // A 503 then a 200, with one 250 ms retry: the result after 2 fetches, one warn, no error.
    it('retries a 503 then returns the 200 result, with one warn and no error line', async () => {
      const fetchMock = stubFetch(() => relayerError(503, 'overloaded', rid(1), 'busy'), publicOk);
      const { logger, warn, error } = makeLogger();
      const script = delays(250);

      const out = capture(scripted(logger, script).run());
      await vi.advanceTimersByTimeAsync(250);
      const outcome = await out;

      expect(outcome).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(fetchMock).toHaveBeenCalledTimes(2);
      expect(script.calls.map(([, n]) => n)).toEqual([1]);
      expect(warn.mock.calls).toEqual([[retryLine(503, 'overloaded', rid(1), 'busy', 250, 1)]]);
      expect(error).not.toHaveBeenCalled();
    });

    // The retry warn line has the exact expected format.
    it('writes the exact retry warn line with status, code, requestId, message, delay and attempt', async () => {
      stubFetch(() => relayerError(429, 'rate_limited', rid(7), 'slow down'), publicOk);
      const { logger, warn } = makeLogger();

      const out = capture(scripted(logger, delays(1234)).run());
      await vi.advanceTimersByTimeAsync(1234);
      await out;

      expect(warn.mock.calls).toEqual([[retryLine(429, 'rate_limited', rid(7), 'slow down', 1234, 1)]]);
    });

    // The query is sent in the fetch URL but kept out of the warn and error lines.
    it('keeps the query out of the retry and stop lines', async () => {
      const fetchMock = stubFetch(
        () => relayerError(503, 'overloaded', rid(1), 'busy'),
        () => relayerError(503, 'overloaded', rid(2), 'busy'),
      );
      const { logger, warn, error } = makeLogger();
      const request = scripted(logger, delays(10, undefined), { query: { chain: 'sepolia' } });

      const out = capture(request.run());
      await vi.advanceTimersByTimeAsync(10);
      rejected(await out);

      const fetched = fetchMock.mock.calls[0]?.[0];
      expect(fetched instanceof Request ? fetched.url : `${fetched}`).toContain('chain=sepolia');
      expect(warn.mock.calls).toEqual([[retryLine(503, 'overloaded', rid(1), 'busy', 10, 1)]]);
      expect(error).toHaveBeenCalledTimes(1);
      expect(error.mock.calls[0]?.[0]).toContain(URL_);
      expect(error.mock.calls[0]?.[0]).not.toContain('sepolia');
    });

    // The next fetch starts exactly at the decided delay: not at 99 ms, at 100 ms.
    it('waits exactly the decided delay before the next fetch', async () => {
      const fetchMock = stubFetch(() => relayerError(), publicOk);

      const out = capture(scripted(makeLogger().logger, delays(100)).run());
      await vi.advanceTimersByTimeAsync(99);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      await vi.advanceTimersByTimeAsync(1);
      expect(fetchMock).toHaveBeenCalledTimes(2);
      expect((await out).ok).toBe(true);
    });

    // Script retries 3 times then stops: 4 fetches, rejects with the last error, 3 warns, 1 error.
    it('stops when the script ends after 3 retries and throws the last error', async () => {
      const fetchMock = stubFetch(
        () => relayerError(503, 'overloaded', rid(1)),
        () => relayerError(503, 'overloaded', rid(2)),
        () => relayerError(503, 'overloaded', rid(3)),
        () => relayerError(503, 'overloaded', rid(4)),
      );
      const { logger, warn, error } = makeLogger();
      const script = delays(5, 5, 5, undefined);

      const out = capture(scripted(logger, script).run());
      await vi.advanceTimersByTimeAsync(15);
      const reason = rejected(await out);

      expect(fetchMock).toHaveBeenCalledTimes(4);
      expect(script.calls.map(([e, n]) => [e.requestId, n])).toEqual([
        [rid(1), 1],
        [rid(2), 2],
        [rid(3), 3],
        [rid(4), 4],
      ]);
      expect(reason).toBe(script.calls[3]?.[0]);
      expect(warn).toHaveBeenCalledTimes(3);
      expect(error).toHaveBeenCalledTimes(1);
      expect(error.mock.calls[0]?.[1]).toBeUndefined();
    });

    const otherFailures: Array<[string, () => Response, abstract new (...args: never[]) => Error]> = [
      ['a non-JSON body', () => new TextResponse('<html>bad gateway</html>', 502, 'text/html'), RelayerHttpFetchError],
      ['a non-relayer JSON error body', () => new TextResponse('{"foo":1}', 500), RelayerHttpUnexpectedResponseError],
      [
        'a 200 with a wrong result shape',
        () => new TextResponse(JSON.stringify({ status: 'succeeded', requestId: rid(0), result: {} }), 200),
        RelayerHttpResultError,
      ],
    ];

    // A 503 retried, then a non-relayer failure: that failure's type is kept, no second decision.
    it.each(otherFailures)(
      'retries then keeps the type of %s, without a second decision',
      async (_label, second, type) => {
        const fetchMock = stubFetch(() => relayerError(), second);
        const script = delays(10, 10);

        const out = capture(scripted(makeLogger().logger, script).run());
        await vi.advanceTimersByTimeAsync(10);
        const reason = rejected(await out);

        expect(reason).toBeInstanceOf(type);
        expect(reason).not.toBeInstanceOf(RelayerHttpResponseError);
        expect(fetchMock).toHaveBeenCalledTimes(2);
        expect(script.calls).toHaveLength(1);
      },
    );

    const firstAttemptFailures: Array<
      [string, () => Response | Promise<Response>, abstract new (...args: never[]) => Error]
    > = [
      ['a network failure', () => Promise.reject(new TypeError('fetch failed')), RelayerHttpFetchError],
      ['a non-JSON body', () => new TextResponse('<html>', 200, TEXT_PLAIN), RelayerHttpFetchError],
      [
        'a non-ok body that is not a relayer error',
        () => new TextResponse(JSON.stringify({ oops: true }), 500),
        RelayerHttpUnexpectedResponseError,
      ],
      [
        'an ok body with the wrong shape',
        () => new TextResponse(JSON.stringify({ status: 'succeeded', requestId: rid(9), result: {} }), 200),
        RelayerHttpResultError,
      ],
    ];

    // A non-relayer failure on the first attempt: its type is kept, decision never called, one error log.
    it.each(firstAttemptFailures)(
      '%s on the first attempt keeps its type and never reaches the decision',
      async (_name, respond, type) => {
        const fetchMock = stubFetch(respond);
        const { logger, warn, error } = makeLogger();
        const script = vi.fn<Script>(() => 1);

        const reason = rejected(await capture(scripted(logger, script, { fetchRetries: 0 }).run()));

        expect(reason).toBeInstanceOf(type);
        expect(script).not.toHaveBeenCalled();
        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(warn).not.toHaveBeenCalled();
        expect(error).toHaveBeenCalledTimes(1);
      },
    );

    const afterRetryFailures = [
      {
        label: 'body read',
        response: () => new TextResponse('{}', 200, TEXT_PLAIN, () => Promise.reject(new Error('read failed'))),
        error: RelayerHttpFetchError,
      },
      {
        label: 'oversized body',
        response: () => new TextResponse(padded(OK_TEXT, MAX_BODY_TEXT_LENGTH + 1), 200, TEXT_PLAIN),
        error: RelayerHttpFetchError,
      },
    ];

    for (const testCase of afterRetryFailures) {
      // After one retry, a failed or oversized body read: RelayerHttpFetchError, no second decision, cleanup done.
      it(`never consults the decision for ${testCase.label}, after retry=true`, async () => {
        const controller = new AbortController();
        const add = vi.spyOn(controller.signal, 'addEventListener');
        const remove = vi.spyOn(controller.signal, 'removeEventListener');
        const fetchMock = stubFetch(() => relayerError(), testCase.response);
        const decision = vi.fn<Script>(() => 1);
        const request = scripted(makeLogger().logger, decision, {
          signal: controller.signal,
          timeout: 500,
          fetchRetries: 0,
        });
        const pending = capture(request.run()).then((outcome) => {
          expect(vi.getTimerCount()).toBe(0);
          expect(remove.mock.calls.filter(([name]) => name === 'abort')).toHaveLength(1);
          expect(remove.mock.calls[0]?.[1]).toBe(add.mock.calls[0]?.[1]);
          return outcome;
        });
        await vi.runAllTimersAsync();
        const outcome = await pending;
        expect(outcome.ok).toBe(false);
        if (!outcome.ok) expect(outcome.reason).toBeInstanceOf(testCase.error);
        expect(decision).toHaveBeenCalledTimes(1);
        expect(fetchMock).toHaveBeenCalledTimes(2);
      });
    }

    // A 200 with a wrong result shape: RelayerHttpResultError with an InvalidPropertyError cause.
    it('wraps a result schema mismatch with an InvalidPropertyError cause', async () => {
      stubFetch(() => new TextResponse(JSON.stringify({ status: 'succeeded', requestId: rid(0), result: {} }), 200));

      const reason = rejected(await capture(scripted(makeLogger().logger, delays()).run()));

      expect(reason).toBeInstanceOf(RelayerHttpResultError);
      expect((reason as Error).cause).toBeInstanceOf(InvalidPropertyError);
    });

    // Decision throws a value: run() rejects with that exact value after one fetch.
    it('propagates a value thrown by the decision unchanged, after one fetch', async () => {
      const fetchMock = stubFetch(() => relayerError());
      const thrown = { marker: 'decide-throws' };

      const reason = rejected(
        await capture(
          scripted(makeLogger().logger, () => {
            throw thrown;
          }).run(),
        ),
      );

      expect(reason).toBe(thrown);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // Decision throws any value (null, string, Error): rejects with it unchanged, cleanup done.
    it.each([null, 'hook failure', new Error('hook failure')])(
      'propagates a throwing decision unchanged: %j',
      async (reason) => {
        const fetchMock = vi.fn<FetchFn>(() => Promise.resolve(relayerError()));
        vi.stubGlobal('fetch', fetchMock);
        const context = withCallerSignal({ timeout: 500 });
        context.decision.mockImplementation(() => {
          throw reason;
        });
        const outcome = await capture(context.request.run());
        expect(outcome.ok).toBe(false);
        if (!outcome.ok) expect(outcome.reason).toBe(reason);
        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(context.decision).toHaveBeenCalledTimes(1);
        expectClean(context);
      },
    );

    // Network retries run inside one HTTP attempt: network error, 503, 200 gives success after 3 fetches.
    it('runs network retries inside an HTTP attempt', async () => {
      const fetchMock = stubFetch(
        () => Promise.reject(new TypeError('network down')),
        () => relayerError(),
        publicOk,
      );
      const request = scripted(makeLogger().logger, delays(5), { fetchRetries: 1, fetchRetryDelayInMilliseconds: 1 });

      const out = capture(request.run());
      await vi.advanceTimersByTimeAsync(60_000);

      expect(await out).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(fetchMock).toHaveBeenCalledTimes(3);
    });
    // The network retry allowance restarts in each HTTP attempt; every fetch shares one signal.
    //   offline ──► net retry ──► 503 ──► decide 20 ms ──► offline ──► net retry ──► 200
    it('restarts the network retry allowance inside each HTTP attempt', async () => {
      const context = withCallerSignal({ fetchRetries: 1, fetchRetryDelayInMilliseconds: 10 });
      context.decision.mockReturnValueOnce(20);
      const events: string[] = [];
      const signals: Array<AbortSignal | null | undefined> = [];
      vi.stubGlobal(
        'fetch',
        vi.fn<FetchFn>(async (_input, init) => {
          signals.push(init?.signal);
          events.push(`fetch-${signals.length}`);
          if (signals.length === 1 || signals.length === 3) throw new Error('offline');
          return signals.length === 2 ? relayerError() : publicOk();
        }),
      );
      const pending = capture(context.request.run());
      await vi.runAllTimersAsync();
      expect(await pending).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(events).toEqual(['fetch-1', 'fetch-2', 'fetch-3', 'fetch-4']);
      expect(context.decision).toHaveBeenCalledTimes(1);
      expect(context.decision.mock.calls[0]?.[1]).toBe(1);
      for (const signal of signals) expect(signal).toBe(signals[0]);
      expectClean(context);
    });
  });

  describe('attempt ceiling', () => {
    // Decision always retries: stops after 5 attempts and rejects with the 5th error.
    it('never starts a 6th attempt, whatever the decision says', async () => {
      const fetchMock = stubFetch(...[1, 2, 3, 4, 5, 6].map((k) => () => relayerError(503, 'overloaded', rid(k))));
      const { logger, warn, error } = makeLogger();
      const script = delays(1, 1, 1, 1, 1, 1);

      const out = capture(scripted(logger, script).run());
      await vi.advanceTimersByTimeAsync(100);
      const reason = rejected(await out);

      expect(fetchMock).toHaveBeenCalledTimes(5);
      expect(script.calls.map(([, n]) => n)).toEqual([1, 2, 3, 4]);
      expect(reason).toBeInstanceOf(RelayerHttpResponseError);
      expect((reason as RelayerHttpResponseError).requestId).toBe(rid(5));
      expect(warn).toHaveBeenCalledTimes(4);
      expect(error).toHaveBeenCalledTimes(1);
    });

    // A success on the 5th attempt is still returned.
    it('still returns a success reached at the 5th attempt', async () => {
      const fetchMock = stubFetch(relayerError, relayerError, relayerError, relayerError, publicOk);

      const out = capture(scripted(makeLogger().logger, () => 1).run());
      await vi.advanceTimersByTimeAsync(10);

      expect(await out).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(fetchMock).toHaveBeenCalledTimes(5);
    });
  });

  describe('invalid delays fail closed', () => {
    const invalid: unknown[] = [
      0,
      -1,
      1.5,
      NaN,
      Infinity,
      -Infinity,
      MAX_TIMEOUT_MS + 1,
      2 ** 53,
      '1000',
      null,
      true,
      {},
    ];

    // Invalid delay from the decision: stops after one fetch with the same error, no wait, one error log.
    it.each(invalid)('stops on %s: one fetch, same error, one error line, no wait', async (value) => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const { logger, warn, error } = makeLogger();
      const script = delays();
      const request = scripted(logger, (e, n) => {
        script(e, n);
        return value as number;
      });

      const outcome = await capture(request.run());

      expect(rejected(outcome)).toBe(script.calls[0]?.[0]);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).not.toHaveBeenCalled();
      expect(error).toHaveBeenCalledTimes(1);
    });

    // A 1 ms delay is valid.
    it('accepts a delay of 1 ms', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);

      const out = capture(scripted(makeLogger().logger, delays(1)).run());
      await vi.advanceTimersByTimeAsync(1);

      expect((await out).ok).toBe(true);
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });

    // The max timer delay is valid without a timeout: the second fetch happens exactly after it.
    it('accepts a delay of MAX_TIMEOUT_MS when no timeout is set', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const { logger, warn } = makeLogger();

      const out = capture(scripted(logger, delays(MAX_TIMEOUT_MS)).run());
      await vi.advanceTimersByTimeAsync(MAX_TIMEOUT_MS - 1);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).toHaveBeenCalledTimes(1);
      await vi.advanceTimersByTimeAsync(1);

      expect((await out).ok).toBe(true);
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });
  });

  describe('remaining time', () => {
    // Delay longer than the remaining time: stops at once with the error, no wait, no warn.
    it('does not wait when the delay exceeds the remaining time', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const { logger, warn, error } = makeLogger();
      const script = delays(1000);

      const outcome = await capture(scripted(logger, script, { timeout: 500 }).run());

      expect(rejected(outcome)).toBe(script.calls[0]?.[0]);
      expect(outcome.timers).toBe(0);
      expect(fetchMock).toHaveBeenCalledTimes(1);
      expect(warn).not.toHaveBeenCalled();
      expect(error).toHaveBeenCalledTimes(1);
    });

    // Delay equal to the remaining time: stops at once.
    it('does not wait when the delay equals the remaining time', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);
      const script = delays(1000);

      const outcome = await capture(scripted(makeLogger().logger, script, { timeout: 1000 }).run());

      expect(rejected(outcome)).toBe(script.calls[0]?.[0]);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // Delay 1 ms under the remaining time: waits and succeeds.
    it('waits when the delay is just below the remaining time', async () => {
      const fetchMock = stubFetch(relayerError, publicOk);

      const out = capture(scripted(makeLogger().logger, delays(999), { timeout: 1000 }).run());
      await vi.advanceTimersByTimeAsync(999);

      expect(await out).toMatchObject({ ok: true, value: PUBLIC_RESULT, timers: 0 });
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });

    const slowFirstFetch = () => new Promise<Response>((resolve) => setTimeout(() => resolve(relayerError()), 300));

    // First fetch took 300 ms: a 700 ms delay no longer fits a 1000 ms timeout, so it stops.
    it('counts the time already spent: 300 ms elapsed, a 700 ms delay on a 1000 ms timeout stops', async () => {
      const fetchMock = stubFetch(slowFirstFetch, publicOk);
      const script = delays(700);

      const out = capture(scripted(makeLogger().logger, script, { timeout: 1000 }).run());
      await vi.advanceTimersByTimeAsync(300);
      const outcome = await out;

      expect(rejected(outcome)).toBe(script.calls[0]?.[0]);
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });

    // First fetch took 300 ms: a 699 ms delay still fits a 1000 ms timeout, so it waits and succeeds.
    it('counts the time already spent: 300 ms elapsed, a 699 ms delay on a 1000 ms timeout waits', async () => {
      const fetchMock = stubFetch(slowFirstFetch, publicOk);

      const out = capture(scripted(makeLogger().logger, delays(699), { timeout: 1000 }).run());
      await vi.advanceTimersByTimeAsync(999);

      expect(await out).toMatchObject({ ok: true, value: PUBLIC_RESULT });
      expect(fetchMock).toHaveBeenCalledTimes(2);
    });
    // Decision spends 40 ms then asks 60 ms on a 100 ms budget: no longer fits, stops without a warn.
    it('recomputes the remaining budget after time spent in the decision', async () => {
      vi.stubGlobal(
        'fetch',
        vi.fn<FetchFn>(() => Promise.resolve(relayerError())),
      );
      const context = withCallerSignal({ timeout: 100 });
      context.decision.mockImplementation(() => {
        vi.advanceTimersByTime(40);
        return 60;
      });
      const outcome = await capture(context.request.run());
      expect(outcome.ok).toBe(false);
      if (!outcome.ok) expect(outcome.reason).toBe(context.decision.mock.calls[0]?.[0]);
      expect(context.warn).not.toHaveBeenCalled();
      expectClean(context);
    });
  });

  describe('one signal and one deadline across attempts', () => {
    // Every attempt receives the same fetch signal.
    it('every attempt receives the same signal', async () => {
      const fetchMock = stubFetch(
        () => relayerError(),
        () => relayerError(),
        publicOk,
      );

      const out = capture(scripted(makeLogger().logger, delays(10, 10), { timeout: 5000 }).run());
      await vi.advanceTimersByTimeAsync(20);

      expect((await out).ok).toBe(true);
      const [first, second, third] = fetchMock.mock.calls.map(([, init]) => init?.signal);
      expect(first).toBeInstanceOf(AbortSignal);
      expect(second).toBe(first);
      expect(third).toBe(first);
    });

    // Second attempt hangs: it times out at the original 1000 ms deadline, not later.
    it('a pending second attempt times out at the original deadline, not before', async () => {
      const steps = [() => relayerError(), (init?: RequestInit) => untilAborted(init?.signal)];
      const fetchMock = vi.fn<FetchFn>((_input, init) => Promise.resolve(steps.shift()!(init)));
      vi.stubGlobal('fetch', fetchMock);
      const events: string[] = [];

      const out = capture(scripted(makeLogger().logger, delays(300), { timeout: 1000 }).run());
      void out.then(() => events.push('settled'));
      await vi.advanceTimersByTimeAsync(300);
      expect(fetchMock).toHaveBeenCalledTimes(2);
      await vi.advanceTimersByTimeAsync(699);
      expect(events).toEqual([]);
      await vi.advanceTimersByTimeAsync(1);
      const outcome = await out;

      const reason = rejected(outcome);
      expect(reason).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((reason as RelayerHttpTimeoutError).timeoutMs).toBe(1000);
      expect(outcome.timers).toBe(0);
      expect(fetchMock.mock.calls[1]?.[1]?.signal?.aborted).toBe(true);
    });

    const slowFirst = () =>
      new Promise<Response>((resolve) => setTimeout(() => resolve(relayerError(503, 'overloaded', rid(1))), 300));

    // Budget carries over: 300 ms fetch + 200 ms wait leaves 500 ms, so a 500 ms second delay stops.
    it('the second decision sees the budget left after the first retry: 500 ms left, a 500 ms delay stops', async () => {
      const fetchMock = stubFetch(slowFirst, () => relayerError(503, 'overloaded', rid(2)), publicOk);
      const { logger, warn, error } = makeLogger();
      const script = delays(200, 500);

      const out = capture(scripted(logger, script, { timeout: 1000 }).run());
      await vi.advanceTimersByTimeAsync(500);
      const outcome = await out;

      expect(rejected(outcome)).toBe(script.calls[1]?.[0]);
      expect(script.calls.map(([e, n]) => [e.requestId, n])).toEqual([
        [rid(1), 1],
        [rid(2), 2],
      ]);
      expect(fetchMock).toHaveBeenCalledTimes(2);
      expect(warn).toHaveBeenCalledTimes(1);
      expect(error).toHaveBeenCalledTimes(1);
      expect(outcome.timers).toBe(0);
    });

    // Budget carries over: 500 ms left, so a 499 ms second delay waits and succeeds.
    it('the second decision sees the budget left after the first retry: 500 ms left, a 499 ms delay waits', async () => {
      const fetchMock = stubFetch(slowFirst, () => relayerError(503, 'overloaded', rid(2)), publicOk);

      const out = capture(scripted(makeLogger().logger, delays(200, 499), { timeout: 1000 }).run());
      await vi.advanceTimersByTimeAsync(999);

      expect(await out).toMatchObject({ ok: true, value: PUBLIC_RESULT, timers: 0 });
      expect(fetchMock).toHaveBeenCalledTimes(3);
    });

    // A 1 h wall-clock jump during the decision does not eat the remaining time: the retry succeeds.
    it('a wall-clock jump does not consume the remaining time', async () => {
      const fetchMock = stubFetch(() => relayerError(), publicOk);
      const { logger, warn } = makeLogger();
      const script: Script = () => {
        vi.setSystemTime(Date.now() + 3_600_000);
        return 900;
      };

      const out = capture(scripted(logger, script, { timeout: 1000 }).run());
      await vi.advanceTimersByTimeAsync(900);

      expect(await out).toMatchObject({ ok: true, value: PUBLIC_RESULT, timers: 0 });
      expect(fetchMock).toHaveBeenCalledTimes(2);
      expect(warn).toHaveBeenCalledTimes(1);
    });
    // Payload, headers and auth mutated in the decision: the retry sends the original body and headers, same signal.
    it('serializes once and retains the same fetch signal and headers after mutation', async () => {
      const payload = { value: 'original' };
      const headers = { 'x-test': 'original' };
      const auth = { type: 'BearerToken' as const, token: 'original-token' };
      const calls: RequestInit[] = [];
      vi.stubGlobal(
        'fetch',
        vi.fn<FetchFn>(async (_input, init) => {
          calls.push({ ...init, headers: new Headers(init?.headers) });
          return calls.length === 1 ? relayerError() : publicOk();
        }),
      );
      const request = new ScriptedRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload,
        options: { headers, auth, fetchRetries: 0 },
      });
      request.script = () => {
        payload.value = 'changed';
        headers['x-test'] = 'changed';
        auth.token = 'changed';
        return 1;
      };
      const pending = capture(request.run());
      await vi.runAllTimersAsync();
      expect((await pending).ok).toBe(true);
      expect(calls).toHaveLength(2);
      expect(calls[0]?.body).toBe(JSON.stringify({ value: 'original' }));
      expect(calls[1]?.body).toBe(calls[0]?.body);
      expect(calls[1]?.headers).toEqual(calls[0]?.headers);
      expect(calls[1]?.signal).toBe(calls[0]?.signal);
      expect(calls[0]?.signal).toBeInstanceOf(AbortSignal);
      expect(vi.getTimerCount()).toBe(0);
    });
  });
});

describe('security', () => {
  function urlOf(input: string | URL | Request): string {
    if (typeof input === 'string') return input;
    return input instanceof URL ? input.href : input.url;
  }

  function headerPairs(init: RequestInit | undefined): Array<[string, string]> {
    return [...new Headers(init?.headers).entries()];
  }

  function recordingLogger() {
    const lines: string[] = [];
    const logger: Logger = {
      debug: (m) => void lines.push(m),
      warn: (m) => void lines.push(m),
      error: (m, cause) => void lines.push(m, cause === undefined ? '' : (JSON.stringify(cause) ?? '')),
    };
    return { logger, lines };
  }

  // Security variant: the payload is chosen by the test.
  function scripted(options: RelayerCommonOptions, payload: Record<string, unknown>, logger?: Logger) {
    return new ScriptedRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: URL_, payload, options, logger });
  }

  async function failureOf(promise: Promise<unknown>): Promise<unknown> {
    try {
      await promise;
    } catch (e) {
      return e;
    }
    throw new Error('test: expected a rejection');
  }

  function texts(error: unknown): string[] {
    if (!(error instanceof Error)) return [JSON.stringify(error) ?? ''];
    const e = error as Error & { details?: string; url?: string };
    return [e.message, e.details ?? '', e.url ?? ''];
  }

  describe('constructor validation', () => {
    const badOperations: unknown[] = [
      'INPUT_PROOF',
      'DELEGATED_USER_DECRYPT',
      '',
      'public_decrypt',
      undefined,
      null,
      42,
    ];

    // Unsupported operation: the constructor throws InvalidPropertyError, no fetch.
    it.each(badOperations)('rejects the operation %s at new, without fetching', (operation) => {
      const fetchMock = stubFetch(ok);

      expect(
        () =>
          new RelayerHttpRequest({
            relayerOperation: operation as RelayerHttpOperation,
            url: URL_,
            payload: {},
          }),
      ).toThrow(InvalidPropertyError);
      expect(fetchMock).not.toHaveBeenCalled();
    });

    // Supported operations construct fine.
    it.each(['USER_DECRYPT', 'PUBLIC_DECRYPT'] as const)('accepts the operation %s', (operation) => {
      expect(() => new RelayerHttpRequest({ relayerOperation: operation, url: URL_, payload: {} })).not.toThrow();
    });

    const badTimeouts: unknown[] = [0, -1, 1.5, NaN, Infinity, -Infinity, MAX_TIMEOUT_MS + 1, '1000', null];

    // Invalid timeout: the constructor throws InvalidPropertyError, no fetch.
    it.each(badTimeouts)('rejects the timeout %s at new', (timeout) => {
      const fetchMock = stubFetch(ok);

      expect(() => scripted({ timeout: timeout as number }, {})).toThrow(InvalidPropertyError);
      expect(fetchMock).not.toHaveBeenCalled();
    });

    // Valid timeouts (1, max, unset) construct fine.
    it.each([1, MAX_TIMEOUT_MS, undefined])('accepts the timeout %s', (timeout) => {
      const options: RelayerCommonOptions = timeout === undefined ? {} : { timeout };

      expect(() => scripted(options, {})).not.toThrow();
    });
    // Constructor failure: no fetch, no timer.
    it('a constructor failure performs no fetch and arms no timer', () => {
      const fetchMock = stubFetch(ok);

      expect(
        () =>
          new RelayerHttpRequest({
            relayerOperation: 'PUBLIC_DECRYPT',
            url: URL_,
            payload: {},
            options: { timeout: 0, signal: new AbortController().signal, fetchRetries: 0 },
          }),
      ).toThrow();
      expect(fetchMock).not.toHaveBeenCalled();
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('request built once', () => {
    // Body is JSON.stringify(payload); fetch gets its own signal, not the caller's.
    it('sends JSON.stringify(payload) as the body, with a signal that is not the caller signal', async () => {
      const fetchMock = stubFetch(ok);
      const controller = new AbortController();
      const payload = { handles: ['0x01', '0x02'], contractAddress: '0xabc', nested: { a: 1 } };

      await scripted({ signal: controller.signal }, payload).run();

      const init = fetchMock.mock.calls[0]?.[1];
      expect(init?.body).toBe(JSON.stringify(payload));
      expect(init?.signal).toBeInstanceOf(AbortSignal);
      expect(init?.signal).not.toBe(controller.signal);
    });

    // Payload, headers and auth mutated in the retry warn: both attempts send the original body and headers.
    it('sends identical bodies and headers on every attempt, despite mutations during the wait', async () => {
      const payload: Record<string, unknown> = { handles: ['0x01'] };
      const headers: Record<string, string> = { 'x-custom': 'one' };
      const auth = { type: 'BearerToken', token: 'token-one' } as { type: 'BearerToken'; token: string };
      const expectedBody = JSON.stringify(payload);
      const fetchMock = stubFetch(relayerError, ok);
      const logger: Logger = {
        debug: undefined,
        warn: () => {
          payload.handles = ['0xff'];
          payload.extra = 'added';
          headers['x-custom'] = 'two';
          headers['x-added'] = 'three';
          auth.token = 'token-two';
        },
        error: undefined,
      };

      const out = scripted({ headers, auth }, payload, logger);
      out.script = () => 10;
      const run = out.run();
      await vi.advanceTimersByTimeAsync(10);
      await run;

      const [first, second] = fetchMock.mock.calls.map(([, init]) => init);
      expect(fetchMock).toHaveBeenCalledTimes(2);
      expect(first?.body).toBe(expectedBody);
      expect(second?.body).toBe(expectedBody);
      expect(headerPairs(second)).toEqual(headerPairs(first));
      expect(headerPairs(first).some(([, v]) => v.includes('token-one'))).toBe(true);
      expect(headerPairs(second).some(([, v]) => v.includes('token-two'))).toBe(false);
    });

    // Unserializable payload (BigInt): rejects with the TypeError, no fetch.
    it('a payload that cannot be serialized rejects with the TypeError, without fetching', async () => {
      const fetchMock = stubFetch(ok);

      await expect(scripted({}, { big: 1n }).run()).rejects.toBeInstanceOf(TypeError);
      expect(fetchMock).not.toHaveBeenCalled();
    });

    // Query appended to the fetched URL, but the error url stays the bare URL.
    it('appends the query to the fetched URL but keeps it out of error urls', async () => {
      const fetchMock = stubFetch(relayerError);

      const error = await failureOf(scripted({ query: { chain: 'sepolia' } }, {}).run());

      const fetched = new URL(urlOf(fetchMock.mock.calls[0]![0]));
      expect(fetched.searchParams.get('chain')).toBe('sepolia');
      expect(`${fetched.origin}${fetched.pathname}`).toBe(URL_);
      expect((error as RelayerHttpResponseError).url).toBe(URL_);
    });
    // Payload toJSON throws: rejects with that error, no fetch, no decision, listener removed, no timer.
    it('cleans up when payload serialization throws before fetch', async () => {
      const reason = new Error('serialize');
      const controller = new AbortController();
      const remove = vi.spyOn(controller.signal, 'removeEventListener');
      const fetchMock = stubFetch(ok);
      const decide = vi.fn<Script>(() => 1);
      const request = scripted(
        { signal: controller.signal, timeout: 500 },
        {
          toJSON() {
            throw reason;
          },
        },
      );
      request.script = decide;

      expect(await failureOf(request.run())).toBe(reason);
      expect(fetchMock).not.toHaveBeenCalled();
      expect(decide).not.toHaveBeenCalled();
      expect(remove).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('no secret leaks into logs or errors', () => {
    const SECRET = 'S3CR3T-value-5f2a';
    const auths: Array<[string, Auth]> = [
      ['BearerToken', { type: 'BearerToken', token: SECRET }],
      ['ApiKeyHeader', { type: 'ApiKeyHeader', value: SECRET }],
      ['ApiKeyCookie', { type: 'ApiKeyCookie', value: SECRET }],
    ];
    const paths: Array<[string, Step[], Script, (() => Promise<void>) | undefined]> = [
      ['retried then stopped', [relayerError, relayerError], (_e, n) => (n === 1 ? 10 : undefined), () => step(10)],
      ['timeout during fetch', [(init) => untilAborted(init?.signal)], () => undefined, () => step(1000)],
      ['non-JSON body', [() => new TextResponse('<html/>', 502, 'text/html')], () => undefined, undefined],
      ['non-relayer JSON error', [() => new TextResponse('{"x":1}', 500)], () => undefined, undefined],
      ['network error', [() => Promise.reject(new TypeError('net'))], () => undefined, undefined],
    ];
    const cases = auths.flatMap(([authName, auth]) =>
      paths.map(([pathName, steps, script, drive]) => ({ authName, auth, pathName, steps, script, drive })),
    );

    async function step(ms: number) {
      await vi.advanceTimersByTimeAsync(ms);
    }

    // Every auth type and failure path: the secret is sent in headers but never appears in logs or errors.
    it.each(cases)('$authName, $pathName: the secret is sent but never logged or thrown', async (c) => {
      const fetchMock = stubFetch(...c.steps);
      const { logger, lines } = recordingLogger();
      const request = scripted(
        { auth: c.auth, query: { key: `q-${SECRET}` }, timeout: 1000, fetchRetries: 0 },
        { secretField: `p-${SECRET}` },
        logger,
      );
      request.script = c.script;

      const failure = failureOf(request.run());
      if (c.drive) await c.drive();
      const error = await failure;

      const init = fetchMock.mock.calls[0]?.[1];
      expect(headerPairs(init).some(([, v]) => v.includes(SECRET))).toBe(true);
      const seen = [...lines, ...texts(error)];
      expect(seen.length).toBeGreaterThan(0);
      for (const text of seen) expect(text).not.toContain(SECRET);
    });
    const namedAuths: Array<[string, Auth]> = [
      ['a bearer token', { type: 'BearerToken', token: SECRET }],
      ['an api-key header', { type: 'ApiKeyHeader', header: 'x-api-key', value: SECRET }],
      ['an api-key cookie', { type: 'ApiKeyCookie', cookie: 'api_key', value: SECRET }],
    ];

    // The retry warn line carries the server message but neither the secret nor the payload.
    it.each(namedAuths)(
      '%s: the retry warn line carries the server message but neither the secret nor the payload',
      async (_authName, auth) => {
        const serverMessage = JSON.stringify({ code: 'overloaded', message: 'please slow down', requestId: rid(1) });
        stubFetch(() => new TextResponse(serverMessage, 503), ok);
        const { logger, warn } = makeLogger();
        const request = scripted({ auth, fetchRetries: 0 }, { secretField: `p-${SECRET}` }, logger);
        request.script = () => 10;

        const outcome = capture(request.run());
        await vi.runAllTimersAsync();
        await outcome;

        const line = warn.mock.calls[0]?.[0] ?? '';
        expect(line).toContain('please slow down');
        expect(line).not.toContain(SECRET);
        expect(line).not.toContain(`p-${SECRET}`);
      },
    );
  });

  describe('relayer-http error body recognition', () => {
    const notRelayerBodies: Array<[string, string]> = [
      ['missing requestId', JSON.stringify({ code: 'overloaded', message: 'busy' })],
      ['missing message', JSON.stringify({ code: 'overloaded', requestId: '01890a5d-ac96-7000-8000-000000000001' })],
      ['missing code', JSON.stringify({ message: 'busy', requestId: '01890a5d-ac96-7000-8000-000000000001' })],
      [
        'non-string code',
        JSON.stringify({ code: 503, message: 'busy', requestId: '01890a5d-ac96-7000-8000-000000000001' }),
      ],
      [
        'an array',
        JSON.stringify([{ code: 'overloaded', message: 'busy', requestId: '01890a5d-ac96-7000-8000-000000000001' }]),
      ],
      ['null', 'null'],
    ];

    // A 503 whose body is not a valid relayer error: RelayerHttpUnexpectedResponseError, never retried.
    it.each(notRelayerBodies)(
      'a 503 with %s is not a relayer-http error and is never retried',
      async (_label, body) => {
        const fetchMock = stubFetch(() => new TextResponse(body, 503), ok);
        const decide = vi.fn<Script>(() => 10);
        const request = scripted({}, {});
        request.script = decide;

        const error = await failureOf(request.run());

        expect(error).toBeInstanceOf(RelayerHttpUnexpectedResponseError);
        expect(decide).not.toHaveBeenCalled();
        expect(fetchMock).toHaveBeenCalledTimes(1);
      },
    );

    // An unknown error code is kept as is.
    it('an unknown error code is kept as is', async () => {
      stubFetch(
        () =>
          new TextResponse(
            JSON.stringify({ code: 'brand_new', message: 'm', requestId: '01890a5d-ac96-7000-8000-000000000001' }),
            503,
          ),
      );

      const error = await failureOf(scripted({}, {}).run());

      expect(error).toBeInstanceOf(RelayerHttpResponseError);
      expect((error as RelayerHttpResponseError).code).toBe('brand_new');
    });
    // Non-ok body with a null message or a bare string: unexpected response, no decision, one error log.
    it.each([
      ['a null message', { code: 'overloaded', message: null, requestId: rid(9) }],
      ['a string', 'overloaded'],
    ])('a non-ok body with %s is an unexpected response and never reaches the decision', async (_name, body) => {
      stubFetch(() => new TextResponse(JSON.stringify(body), 503));
      const { logger, error } = makeLogger();
      const decide = vi.fn<Script>(() => 1);
      const request = scripted({ fetchRetries: 0 }, {}, logger);
      request.script = decide;

      const reason = await failureOf(request.run());

      expect(reason).toBeInstanceOf(RelayerHttpUnexpectedResponseError);
      expect((reason as RelayerHttpUnexpectedResponseError).status).toBe(503);
      expect(decide).not.toHaveBeenCalled();
      expect(error).toHaveBeenCalledTimes(1);
    });

    // Error envelope with a numeric requestId: unexpected response, no decision.
    it.each([{ code: 'overloaded', message: 'busy', requestId: 42 }])(
      'rejects near-miss error envelope %j without a decision',
      async (body) => {
        const fetchMock = stubFetch(() => new TextResponse(JSON.stringify(body), 503, TEXT_PLAIN));
        const decide = vi.fn<Script>(() => 1);
        const request = scripted({ fetchRetries: 0 }, {});
        request.script = decide;

        expect(await failureOf(request.run())).toBeInstanceOf(RelayerHttpUnexpectedResponseError);
        expect(decide).not.toHaveBeenCalled();
        expect(fetchMock).toHaveBeenCalledTimes(1);
        expect(vi.getTimerCount()).toBe(0);
      },
    );

    // Any non-ok status with a relayer error body: RelayerHttpResponseError with that status and code.
    it.each([400, 401, 404, 418, 429, 500, 502, 599])(
      'a %d response with a relayer error body is a response error carrying that status',
      async (status) => {
        stubFetch(
          () => new TextResponse(JSON.stringify({ code: 'some_code', message: 'm', requestId: rid(9) }), status),
        );
        const request = new RelayerHttpRequest({
          relayerOperation: 'PUBLIC_DECRYPT',
          url: URL_,
          payload: {},
          options: { fetchRetries: 0 },
          logger: makeLogger().logger,
        });

        const reason = (await failureOf(request.run())) as RelayerHttpResponseError;

        expect(reason).toBeInstanceOf(RelayerHttpResponseError);
        expect(reason.status).toBe(status);
        expect(reason.code).toBe('some_code');
      },
    );

    // Non-ok non-JSON body: RelayerHttpFetchError, no decision.
    it('a non-ok non-JSON body is a fetch error, whatever the status', async () => {
      stubFetch(() => new TextResponse('<html>Service Unavailable</html>', 503, TEXT_PLAIN));
      const decide = vi.fn<Script>(() => 1);
      const request = scripted({ fetchRetries: 0 }, {}, makeLogger().logger);
      request.script = decide;

      expect(await failureOf(request.run())).toBeInstanceOf(RelayerHttpFetchError);
      expect(decide).not.toHaveBeenCalled();
    });

    // A 200 with an error-shaped body: RelayerHttpResultError, no decision.
    it('an ok response with an error-shaped body is a result error, not a response error', async () => {
      stubFetch(() => new TextResponse(JSON.stringify({ code: 'overloaded', message: 'x', requestId: rid(9) }), 200));
      const decide = vi.fn<Script>(() => 1);
      const request = scripted({ fetchRetries: 0 }, {}, makeLogger().logger);
      request.script = decide;

      expect(await failureOf(request.run())).toBeInstanceOf(RelayerHttpResultError);
      expect(decide).not.toHaveBeenCalled();
    });

    // A 200 whose status is not 'succeeded': RelayerHttpResultError.
    it('an ok response whose status field is not succeeded is a result error', async () => {
      stubFetch(
        () => new TextResponse(JSON.stringify({ status: 'failed', requestId: rid(0), result: PUBLIC_RESULT }), 200),
      );
      const request = new RelayerHttpRequest({
        relayerOperation: 'PUBLIC_DECRYPT',
        url: URL_,
        payload: {},
        options: { fetchRetries: 0 },
        logger: makeLogger().logger,
      });

      expect(await failureOf(request.run())).toBeInstanceOf(RelayerHttpResultError);
    });
  });

  describe('body size limit', () => {
    // A body of exactly the size limit is parsed.
    it('parses a body of exactly the limit', async () => {
      stubFetch(() => new TextResponse(padded(OK_TEXT, MAX_BODY_TEXT_LENGTH), 200));

      await expect(scripted({}, {}).run()).resolves.toEqual(PUBLIC_RESULT);
    });

    // The size limit counts UTF-16 code units, not bytes.
    it('counts UTF-16 code units, not bytes', async () => {
      const wide = JSON.stringify({
        status: 'succeeded',
        requestId: '01890a5d-ac96-7000-8000-000000000000',
        note: 'é'.repeat(1000),
        result: PUBLIC_RESULT,
      });
      stubFetch(() => new TextResponse(padded(wide, MAX_BODY_TEXT_LENGTH), 200));

      await expect(scripted({}, {}).run()).resolves.toEqual(PUBLIC_RESULT);
    });

    // A body one unit over the limit: RelayerHttpFetchError.
    it('rejects a body one unit over the limit with RelayerHttpFetchError', async () => {
      stubFetch(() => new TextResponse(padded(OK_TEXT, MAX_BODY_TEXT_LENGTH + 1), 200));

      await expect(scripted({}, {}).run()).rejects.toBeInstanceOf(RelayerHttpFetchError);
    });

    // An oversized relayer error body: RelayerHttpFetchError, never retried.
    it('an oversized relayer-http error body is a fetch error and is never retried', async () => {
      const fetchMock = stubFetch(() => new TextResponse(padded(ERROR_TEXT, MAX_BODY_TEXT_LENGTH + 1), 503), ok);
      const decide = vi.fn<Script>(() => 10);
      const request = scripted({}, {});
      request.script = decide;

      const error = await failureOf(request.run());

      expect(error).toBeInstanceOf(RelayerHttpFetchError);
      expect(decide).not.toHaveBeenCalled();
      expect(fetchMock).toHaveBeenCalledTimes(1);
    });
  });
});
