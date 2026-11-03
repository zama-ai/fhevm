import type { Logger } from '../../../../types/logger.js';
import type { RelayerCommonOptions } from '../../../../types/relayer.js';

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';

const URL_ = 'https://relayer.test/v2/public-decrypt';
const LATE_PHASE = ' after the response was received';
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

function deferred<T>() {
  let resolve: (value: T) => void = () => undefined;
  const promise = new Promise<T>((r) => (resolve = r));
  return { promise, resolve };
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

// Node reports a throwing EventTarget listener as an uncaughtException on the next tick.
// Swaps the runner's handlers for a trap until `restore()`, so the test can assert on it.
function trapUncaughtExceptions(): { caught: unknown[]; restore: () => void } {
  const saved = process.listeners('uncaughtException');
  process.removeAllListeners('uncaughtException');
  const caught: unknown[] = [];
  const trap = (error: unknown): void => void caught.push(error);
  process.on('uncaughtException', trap);
  return {
    caught,
    restore: () => {
      process.off('uncaughtException', trap);
      for (const listener of saved) process.on('uncaughtException', listener);
    },
  };
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

  describe('abort during fetch', () => {
    // Our timer fires while fetch ignores the signal: RelayerHttpTimeoutError, not the late 200.
    it('our timer wins over a fetch that ignores the signal and succeeds later', async () => {
      stubFetch(() => new Promise<Response>((resolve) => setTimeout(() => resolve(publicOk()), 800)));

      const out = capture(scripted(makeLogger().logger, { timeout: 500 }).run());
      await vi.advanceTimersByTimeAsync(800);

      expectTimeout(rejected(await out), 500);
    });

    // Caller aborts while fetch ignores the signal: rejects with the caller's reason, not the late 200.
    it('a caller abort wins over a fetch that ignores the signal and succeeds later', async () => {
      stubFetch(() => new Promise<Response>((resolve) => setTimeout(() => resolve(publicOk()), 800)));
      const controller = new AbortController();

      const out = capture(scripted(makeLogger().logger, { signal: controller.signal }).run());
      await vi.advanceTimersByTimeAsync(100);
      controller.abort('late caller');
      await vi.advanceTimersByTimeAsync(700);

      expect(rejected(await out)).toBe('late caller');
    });

    // Caller abort and our timer both fire while fetch hangs: the first one decides the error, one warn.
    //   fetch ──► caller aborts ──► timer due ──► 200 ──► rejects 'caller'
    //   fetch ──► timer due ──► caller aborts ──► 200 ──► rejects RelayerHttpTimeoutError
    it.each(['caller-first', 'timer-first'])('retains first abort with an uncooperative fetch: %s', async (order) => {
      const context = setup();
      const gate = deferred<Response>();
      const entered = deferred<void>();
      const events: string[] = [];
      stubFetch(() => {
        events.push('fetch');
        entered.resolve();
        return gate.promise;
      });
      const pending = context.run();
      await entered.promise;
      if (order === 'caller-first') {
        events.push('caller');
        context.controller.abort('caller');
      }
      await vi.advanceTimersByTimeAsync(100);
      events.push('timer-due');
      if (order === 'timer-first') {
        events.push('caller');
        context.controller.abort('caller');
      }
      events.push('response');
      gate.resolve(publicOk());
      const outcome = await pending;
      events.push('settled');
      expect(outcome.ok).toBe(false);
      if (!outcome.ok) {
        if (order === 'caller-first') expect(outcome.error).toBe('caller');
        else expect(outcome.error).toBeInstanceOf(RelayerHttpTimeoutError);
      }
      expect(events).toEqual(
        order === 'caller-first'
          ? ['fetch', 'caller', 'timer-due', 'response', 'settled']
          : ['fetch', 'timer-due', 'caller', 'response', 'settled'],
      );
      expect(context.log.warn).toHaveBeenCalledTimes(1);
    });

    // Fetch ignores the signal and never settles: after an abort, run() stays pending by design,
    // but its timers and caller listener are released.
    it.each(['a caller abort', 'our timer'])(
      'a global fetch that ignores the signal and never settles: run() stays pending after %s, resources released',
      async (trigger) => {
        stubFetch(() => new Promise<Response>(() => undefined));
        const controller = new AbortController();
        const add = vi.spyOn(controller.signal, 'addEventListener');
        const remove = vi.spyOn(controller.signal, 'removeEventListener');
        let settled = false;

        void scripted(makeLogger().logger, { signal: controller.signal, timeout: 500 })
          .run()
          .then(
            () => (settled = true),
            () => (settled = true),
          );
        await vi.advanceTimersByTimeAsync(100);
        if (trigger === 'a caller abort') controller.abort('gone');
        await vi.advanceTimersByTimeAsync(500);

        expect(settled).toBe(false);
        expect(vi.getTimerCount()).toBe(0);
        expect(add).toHaveBeenCalledTimes(1);
        expect(remove).toHaveBeenCalledTimes(1);
        expect(remove.mock.calls[0]?.[1]).toBe(add.mock.calls[0]?.[1]);
      },
    );

    // A listener fetch added throws on abort: run() still rejects with the caller's reason;
    // the throw surfaces as one uncaught exception.
    it('a fetch whose own abort listener throws: run() still rejects with the caller reason, nothing left behind', async () => {
      const trap = trapUncaughtExceptions();
      try {
        const context = setup(5000);
        const boom = new Error('fetch listener failed');
        stubFetch(
          (init) =>
            new Promise<Response>((_resolve, reject) => {
              const signal = init!.signal!;
              signal.addEventListener('abort', () => {
                throw boom;
              });
              signal.addEventListener('abort', () => reject(signal.reason));
            }),
        );

        const pending = context.run();
        await vi.advanceTimersByTimeAsync(100);
        context.controller.abort('caller');
        const outcome = await pending;
        await new Promise<void>((resolve) => process.nextTick(resolve));

        expect(outcome).toEqual({ ok: false, error: 'caller' });
        expect(trap.caught).toEqual([boom]);
      } finally {
        trap.restore();
      }
    });
  });

  describe('abort during the body read', () => {
    // Caller aborts during a slow body read that ignores the signal: rejects with the caller's reason.
    it('an abort wins over a body read that ignores the signal and completes later', async () => {
      const slowIgnoring = () =>
        new TextResponse(
          OK_TEXT,
          200,
          undefined,
          () => new Promise((resolve) => setTimeout(() => resolve(OK_TEXT), 400)),
        );
      stubFetch(slowIgnoring);
      const controller = new AbortController();

      const out = capture(scripted(makeLogger().logger, { signal: controller.signal }).run());
      await vi.advanceTimersByTimeAsync(100);
      controller.abort('stop reading');
      await vi.advanceTimersByTimeAsync(300);

      expect(rejected(await out)).toBe('stop reading');
    });
  });

  describe('abort after the response arrived, before settlement', () => {
    function abortingBody(text: string, status: number, abort: () => void) {
      return () => new TextResponse(text, status, undefined, () => (abort(), Promise.resolve(text)));
    }

    // Caller aborts during the body read of a 200: rejects with the caller's reason, one warn, no error line.
    it('a successful result is replaced by the caller reason, with one warn', async () => {
      const controller = new AbortController();
      stubFetch(abortingBody(OK_TEXT, 200, () => controller.abort('after body')));
      const { logger, warn, error } = makeLogger();

      const outcome = await capture(scripted(logger, { signal: controller.signal }).run());

      expect(rejected(outcome)).toBe('after body');
      expect(outcome.timers).toBe(0);
      expect(warn).toHaveBeenCalledTimes(1);
      expect(warn.mock.calls[0]?.[0]).toMatch(/^\[RelayerHttpRequest\]/);
      expect(warn.mock.calls[0]?.[0]).toContain(LATE_PHASE);
      expect(error).not.toHaveBeenCalled();
    });

    // Caller aborts during the body read of a 503: the caller's reason wins, no retry decision, no error line.
    it('a relayer-http error is replaced by the caller reason; no decision, no error line', async () => {
      const controller = new AbortController();
      stubFetch(abortingBody(ERROR_TEXT, 503, () => controller.abort('after error body')));
      const { logger, warn, error } = makeLogger();
      const decide = vi.fn<Script>(() => 10);

      const outcome = await capture(scripted(logger, { signal: controller.signal }, decide).run());

      expect(rejected(outcome)).toBe('after error body');
      expect(decide).not.toHaveBeenCalled();
      expect(warn).toHaveBeenCalledTimes(1);
      expect(warn.mock.calls[0]?.[0]).toContain(LATE_PHASE);
      expect(error).not.toHaveBeenCalled();
    });

    // Our timer fires during the body read of a 200: RelayerHttpTimeoutError and one warn.
    it('a successful result is replaced by RelayerHttpTimeoutError when our timer fires during the read', async () => {
      stubFetch(abortingBody(OK_TEXT, 200, () => vi.advanceTimersByTime(700)));
      const { logger, warn } = makeLogger();

      const outcome = await capture(scripted(logger, { timeout: 700 }).run());

      expectTimeout(rejected(outcome), 700);
      expect(outcome.timers).toBe(0);
      expect(warn).toHaveBeenCalledTimes(1);
      expect(warn.mock.calls[0]?.[0]).toContain(LATE_PHASE);
    });

    // The caller's abort reason is itself a RelayerHttpResponseError: rejects with that exact object, no decision.
    it('a caller reason that is itself a RelayerHttpResponseError is returned as is', async () => {
      const first = new RelayerHttpRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: URL_, payload: {} });
      stubFetch(relayerError);
      const reasonError = rejected(await capture(first.run()));
      expect(reasonError).toBeInstanceOf(RelayerHttpResponseError);

      const controller = new AbortController();
      stubFetch(abortingBody(ERROR_TEXT, 503, () => controller.abort(reasonError)));
      const decide = vi.fn<Script>(() => 10);

      const outcome = await capture(scripted(makeLogger().logger, { signal: controller.signal }, decide).run());

      expect(rejected(outcome)).toBe(reasonError);
      expect(decide).not.toHaveBeenCalled();
    });
  });

  describe('abort from hooks at the stop', () => {
    // logger.error aborts at the stop: rejects with the caller's reason; the error line comes before the abort warn.
    it('abort inside logger.error at the stop: the abort wins, after the error line', async () => {
      stubFetch(relayerError);
      const controller = new AbortController();
      const events: string[] = [];
      const warn = vi.fn((message: string) => void events.push(`warn:${message}`));
      const error = vi.fn((_message: string, _cause: unknown) => {
        events.push('error');
        controller.abort('abort in error hook');
      });
      const logger: Logger = { debug: undefined, warn, error };

      const outcome = await capture(scripted(logger, { signal: controller.signal }).run());

      expect(rejected(outcome)).toBe('abort in error hook');
      expect(events.map((e) => e.split(':')[0])).toEqual(['error', 'warn']);
    });

    // The decision aborts then stops: decide, error line, abort warn, then rejects with the caller's reason.
    it('abort inside the decision returning undefined: decision, error line, then the abort', async () => {
      stubFetch(relayerError);
      const controller = new AbortController();
      const events: string[] = [];
      const { logger } = makeLogger(events);
      const decide: Script = () => {
        events.push('decide');
        controller.abort('abort in decide');
        return undefined;
      };

      const outcome = await capture(scripted(logger, { signal: controller.signal }, decide).run());

      expect(rejected(outcome)).toBe('abort in decide');
      expect(events.map((e) => e.split(':')[0])).toEqual(['decide', 'error', 'warn']);
    });

    // The decision aborts then throws: rejects with the caller's reason, not the thrown error.
    it('abort inside a throwing decision: the abort replaces the thrown value', async () => {
      stubFetch(relayerError);
      const controller = new AbortController();
      const decide: Script = () => {
        controller.abort('abort then throw');
        throw new Error('decide failed');
      };

      const outcome = await capture(scripted(makeLogger().logger, { signal: controller.signal }, decide).run());

      expect(rejected(outcome)).toBe('abort then throw');
    });

    // Our timer fires while the decision runs (stop or retry): RelayerHttpTimeoutError, no second fetch.
    it.each([undefined, 10])(
      'our timer fires while the decision runs (returns %s): RelayerHttpTimeoutError',
      async (delay) => {
        const fetchMock = stubFetch(relayerError, publicOk);
        const decide: Script = () => {
          vi.advanceTimersByTime(1000);
          return delay;
        };

        const out = capture(scripted(makeLogger().logger, { timeout: 1000 }, decide).run());
        await vi.advanceTimersByTimeAsync(50);
        const outcome = await out;

        expectTimeout(rejected(outcome), 1000);
        expect(outcome.timers).toBe(0);
        expect(fetchMock).toHaveBeenCalledTimes(1);
      },
    );

    // The stop or retry log hook aborts then throws: rejects with the caller's reason, not the hook's error.
    it.each(['stop-log', 'retry-log'])('abort from %s replaces a hook exception', async (hook) => {
      const context = setup();
      const events: string[] = [];
      vi.stubGlobal(
        'fetch',
        vi.fn<FetchFn>(() => Promise.resolve(relayerError())),
      );
      const abortAndThrow = () => {
        events.push(hook);
        context.controller.abort('caller');
        events.push('aborted');
        throw new Error('hook');
      };
      if (hook === 'stop-log') {
        context.decision.mockReturnValue(undefined);
        context.log.error.mockImplementationOnce(abortAndThrow);
      }
      if (hook === 'retry-log') context.log.warn.mockImplementationOnce(abortAndThrow);
      const outcome = await context.run();
      events.push('settled');
      expect(outcome.ok).toBe(false);
      if (!outcome.ok) expect(outcome.error).toBe('caller');
      expect(events).toEqual([hook, 'aborted', 'settled']);
    });
  });
});

describe('leaks', () => {
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

  const scenarios: Scenario[] = [
    {
      name: 'our timer with a fetch ignoring the signal',
      steps: [() => new Promise<Response>((resolve) => setTimeout(() => resolve(ok()), 800))],
      timeout: 300,
      drive: advance(800),
      expectOk: false,
    },
    {
      name: 'abort after the response, before settlement',
      steps: (c) => [
        () => new TextResponse(OK_TEXT, 200, undefined, () => (c.abort('late'), Promise.resolve(OK_TEXT))),
      ],
      script: () => undefined,
      timeout: 5000,
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

  describe('resources at settlement', () => {
    // Each scenario settles with zero timers and the caller's abort listener added once and removed once.
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
  });
});

describe('races', () => {
  const MAX_DEPTH = 200;

  type Kind = 'abort' | 'timeout' | 'result' | 'response-error' | 'other';

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

  function afterMicrotasks(depth: number, fn: () => void): void {
    queueMicrotask(depth === 0 ? fn : () => afterMicrotasks(depth - 1, fn));
  }

  function kindOf(settled: PromiseSettledResult<unknown>, callerReason: unknown): Kind {
    if (settled.status === 'fulfilled') return 'result';
    if (settled.reason === callerReason) return 'abort';
    if (settled.reason instanceof RelayerHttpTimeoutError) return 'timeout';
    if (settled.reason instanceof RelayerHttpResponseError) return 'response-error';
    return 'other';
  }

  // Every early kind must come before every late kind: one switch point, never back.
  function expectSingleSwitch(kinds: Kind[], early: Kind, late: Kind) {
    const firstLate = kinds.indexOf(late);
    expect(kinds[0]).toBe(early);
    expect(firstLate).toBeGreaterThan(0);
    expect(kinds.slice(0, firstLate).every((k) => k === early)).toBe(true);
    expect(kinds.slice(firstLate).every((k) => k === late)).toBe(true);
  }

  describe('gated orderings after the response arrived', () => {
    // The body read is held open by the test, so the abort provably lands before the outcome exists.
    async function gatedBody(text: string, status: number, decide?: Script) {
      const events: string[] = [];
      const body = deferred<string>();
      const controller = new AbortController();
      stubFetch(() => new TextResponse(text, status, undefined, () => (events.push('body'), body.promise)));
      const { logger, warn, error } = makeLogger();
      const request = scripted(logger, { signal: controller.signal }, decide);
      const done = recordSettlement(request.run(), events, 'gated');
      await vi.advanceTimersByTimeAsync(1);
      return { events, body, controller, warn, error, done };
    }

    // 200 body read held open, caller aborts, then the body is released: the caller's reason, one warn.
    //   body read ──► caller aborts ──► body released ──► rejects 'gated'
    it('success body: entered, abort, released, then the caller reason', async () => {
      const g = await gatedBody(OK_TEXT, 200);
      g.controller.abort('gated');
      g.events.push('abort');
      g.body.resolve(OK_TEXT);
      g.events.push('released');
      await g.done;

      expect(g.events).toEqual(['body', 'abort', 'released', 'rejected:reason']);
      expect(g.warn).toHaveBeenCalledTimes(1);
      expect(g.warn.mock.calls[0]?.[0]).toContain(LATE_PHASE);
      expect(vi.getTimerCount()).toBe(0);
    });

    // 503 body read held open, caller aborts, then released: the caller's reason, no decision, no error line.
    it('relayer-http error body: entered, abort, released, then the caller reason; no decision', async () => {
      const decide = vi.fn<Script>(() => 10);
      const g = await gatedBody(ERROR_TEXT, 503, decide);
      g.controller.abort('gated');
      g.events.push('abort');
      g.body.resolve(ERROR_TEXT);
      g.events.push('released');
      await g.done;

      expect(g.events).toEqual(['body', 'abort', 'released', 'rejected:reason']);
      expect(decide).not.toHaveBeenCalled();
      expect(g.error).not.toHaveBeenCalled();
      expect(g.warn).toHaveBeenCalledTimes(1);
    });

    // 200 body read held open past our deadline, then released: RelayerHttpTimeoutError.
    //   body read ──► 500 ms deadline ──► body released ──► rejects timeout
    it('success body: our timer fires while the body is held, then the timeout', async () => {
      const events: string[] = [];
      const body = deferred<string>();
      stubFetch(() => new TextResponse(OK_TEXT, 200, undefined, () => (events.push('body'), body.promise)));
      // No caller signal: a sentinel the run cannot produce keeps the classifier unambiguous.
      const noCallerReason = { sentinel: 'no caller reason' };
      const done = recordSettlement(scripted(makeLogger().logger, { timeout: 500 }).run(), events, noCallerReason);
      await vi.advanceTimersByTimeAsync(1);
      vi.advanceTimersByTime(499);
      events.push('deadline');
      body.resolve(OK_TEXT);
      events.push('released');
      await done;

      expect(events).toEqual(['body', 'deadline', 'released', 'rejected:timeout']);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Fetch held open (ignoring the signal), caller aborts, then a 200 arrives: the caller's reason.
    //   fetch ──► caller aborts ──► 200 ──► rejects 'gated'
    it('fetch held open and ignoring the signal: entered, abort, resolved, then the caller reason', async () => {
      const events: string[] = [];
      const response = deferred<Response>();
      const controller = new AbortController();
      stubFetch(() => (events.push('fetch'), response.promise));
      const done = recordSettlement(
        scripted(makeLogger().logger, { signal: controller.signal }).run(),
        events,
        'gated',
      );
      await vi.advanceTimersByTimeAsync(1);
      controller.abort('gated');
      events.push('abort');
      response.resolve(new TextResponse(OK_TEXT, 200));
      events.push('resolved');
      await done;

      expect(events).toEqual(['fetch', 'abort', 'resolved', 'rejected:reason']);
    });

    // Fetch resolves and the caller aborts in the same synchronous block: the caller's reason wins.
    it('fetch resolving and the caller aborting in the same synchronous block: the reason wins', async () => {
      const events: string[] = [];
      const response = deferred<Response>();
      const controller = new AbortController();
      stubFetch(() => response.promise);
      const done = recordSettlement(scripted(makeLogger().logger, { signal: controller.signal }).run(), events, 'tick');
      await vi.advanceTimersByTimeAsync(1);
      response.resolve(new TextResponse(OK_TEXT, 200));
      controller.abort('tick');
      await done;

      expect(events).toEqual(['rejected:reason']);
    });

    // Caller aborts inside fetch, which then returns a 503: the caller's reason, no decision, no error line.
    it('an abort in the same tick as a relayer error resolution wins over the error, and the decision is skipped', async () => {
      const events: string[] = [];
      const controller = new AbortController();
      const decide = vi.fn<Script>(() => 1);
      stubFetch(() => {
        events.push('fetch');
        events.push('abort');
        controller.abort('same-tick');
        return relayerError();
      });
      const { logger, error } = makeLogger();

      const outcome = await capture(scripted(logger, { signal: controller.signal, fetchRetries: 0 }, decide).run());
      events.push(outcome.ok ? 'settled:resolved' : 'settled:rejected');

      expect(rejected(outcome)).toBe('same-tick');
      expect(events).toEqual(['fetch', 'abort', 'settled:rejected']);
      expect(decide).not.toHaveBeenCalled();
      expect(error).not.toHaveBeenCalled();
    });
  });

  describe('microtask sweep after the response (smoke check)', () => {
    // Smoke check only: the gated tests prove the boundary. Five late depths suffice, because the
    // caller listener is removed at settlement and no later abort can reach the run.
    // Runs one request per depth; the hook fires `depth` microtasks after fetch is called.
    async function sweep(text: string, status: number, hook: (controller: AbortController) => void) {
      const kinds: Kind[] = [];
      const warns: number[] = [];
      const errors: number[] = [];
      let lateRuns = 0;
      for (let depth = 0; depth <= MAX_DEPTH && lateRuns < 5; depth++) {
        const controller = new AbortController();
        let markFired: () => void = () => undefined;
        const fired = new Promise<void>((resolve) => (markFired = resolve));
        stubFetch(() => {
          afterMicrotasks(depth, () => (hook(controller), markFired()));
          return new TextResponse(text, status);
        });
        const { logger, warn, error } = makeLogger();
        const request = scripted(logger, { signal: controller.signal, timeout: 60_000 });
        const [settled] = await Promise.allSettled([request.run()]);
        await fired;
        kinds.push(kindOf(settled as PromiseSettledResult<unknown>, controller.signal.reason));
        warns.push(warn.mock.calls.length);
        errors.push(error.mock.calls.length);
        vi.unstubAllGlobals();
        if (kinds.at(-1) !== 'abort' && kinds.at(-1) !== 'timeout') lateRuns++;
      }
      return { kinds, warns, errors };
    }

    // Abort fired N microtasks after fetch, N growing, on a 200: aborts first (one warn each),
    // then results (no warn), switching exactly once.
    it('success response: from the abort to the result at one depth, never back', async () => {
      const { kinds, warns } = await sweep(OK_TEXT, 200, (c) => c.abort('m'));

      expectSingleSwitch(kinds, 'abort', 'result');
      kinds.forEach((kind, i) => expect([kind, warns[i]]).toEqual([kind, kind === 'abort' ? 1 : 0]));
    });

    // Same sweep on a 503 with the default decision: aborts first, then the relayer error with its error line.
    it('relayer-http error, default decision: from the abort to the error with its line, never back', async () => {
      const { kinds, warns, errors } = await sweep(ERROR_TEXT, 503, (c) => c.abort('m'));

      expectSingleSwitch(kinds, 'abort', 'response-error');
      kinds.forEach((kind, i) => {
        if (kind === 'abort') expect(warns[i]).toBe(1);
        else expect([warns[i], errors[i]]).toEqual([0, 1]);
      });
    });

    // Same sweep firing our timer instead of an abort: timeouts first, then the result, switching once.
    it('our timer at each depth: from the timeout to the result at one depth, never back', async () => {
      const { kinds, warns } = await sweep(OK_TEXT, 200, () => vi.advanceTimersByTime(60_000));

      expectSingleSwitch(kinds, 'timeout', 'result');
      kinds.forEach((kind, i) => expect([kind, warns[i]]).toEqual([kind, kind === 'timeout' ? 1 : 0]));
    });
  });

  describe('forced orderings', () => {
    // Caller aborts inside fetch before it returns a 200: rejects with the caller's reason.
    it('abort inside fetch before it resolves: fetch, abort, then the rejection', async () => {
      const events: string[] = [];
      const controller = new AbortController();
      stubFetch(() => {
        events.push('fetch');
        controller.abort('in fetch');
        events.push('abort');
        return new TextResponse(OK_TEXT, 200);
      });

      await scripted(makeLogger().logger, { signal: controller.signal })
        .run()
        .then(
          () => events.push('fulfilled'),
          (reason: unknown) => events.push(`rejected:${reason as string}`),
        );

      expect(events).toEqual(['fetch', 'abort', 'rejected:in fetch']);
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

  describe('caller signal shared across instances', () => {
    // The decision aborts the caller signal and stops: rejects with that abort's reason.
    it('the instance whose decision aborted the shared signal rejects with that reason', async () => {
      stubFetch(relayerError);
      const controller = new AbortController();
      const a = scripted({ signal: controller.signal });
      a.script = () => (controller.abort('from a'), undefined);

      expect(await Promise.allSettled([a.run()])).toEqual([{ status: 'rejected', reason: 'from a' }]);
    });
  });
});
