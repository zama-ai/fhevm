import type { Mock } from 'vitest';
import type { Logger } from '../../../../types/logger.js';
import type { RelayerHttpRetryLoopParams } from '../relayerHttpRetryLoop.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { runRelayerHttpRetryLoop, waitBeforeHttpRetry } from '../relayerHttpRetryLoop.js';
import { MAX_TIMEOUT_MS } from '../relayerHttpSignal.js';
import { abortableSleep } from '../../../../base/timeout.js';
import { RelayerHttpFetchError } from '../../../../errors/RelayerHttpFetchError.js';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpResultError } from '../../../../errors/RelayerHttpResultError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../../errors/RelayerHttpUnexpectedResponseError.js';
import { InvalidPropertyError } from '../../../../base/errors/InvalidPropertyError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.retryLoop.test.ts
////////////////////////////////////////////////////////////////////////////////

// Pass-through by default; a test can make one call reject with mockImplementationOnce.
vi.mock('../../../../base/timeout.js', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../../../../base/timeout.js')>();
  return { ...actual, abortableSleep: vi.fn(actual.abortableSleep) };
});

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';

type TestLogger = Logger & { debug: Mock; warn: Mock; error: Mock };
function makeLogger(): TestLogger {
  return { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
}

function responseError(message = 'busy'): RelayerHttpResponseError {
  return new RelayerHttpResponseError({
    url: RELAYER_URL,
    status: 503,
    code: 'overloaded',
    responseMessage: message,
    requestId: VALID_REQUEST_ID,
  });
}

function retryLine(message: string, delayMs: number, attempt: number): string {
  return `[RelayerHttpRequest] relayer-http error on ${RELAYER_URL} (status=503, code=overloaded, requestId=${VALID_REQUEST_ID}): ${message}. Retrying in ${delayMs} ms (attempt ${attempt}).`;
}

// Today's exact relayer-http error line.
function stopLine(message: string): string {
  return `[RelayerHttpRequest] relayer-http error on ${RELAYER_URL} (status=503, code=overloaded, requestId=${VALID_REQUEST_ID}): ${message}`;
}

type Harness = {
  params: RelayerHttpRetryLoopParams<string>;
  runAttempt: Mock<() => Promise<string>>;
  decide: Mock<(error: RelayerHttpResponseError, attempt: number) => number | undefined>;
  logger: TestLogger;
  controller: AbortController;
};

function makeHarness(overrides: Partial<RelayerHttpRetryLoopParams<string>> = {}): Harness {
  const runAttempt = vi.fn<() => Promise<string>>();
  const decide = vi.fn<(error: RelayerHttpResponseError, attempt: number) => number | undefined>();
  const logger = makeLogger();
  const controller = new AbortController();
  const params: RelayerHttpRetryLoopParams<string> = {
    runAttempt,
    decide,
    signal: controller.signal,
    timedOutAfterMs: () => undefined,
    remainingMs: () => undefined,
    url: RELAYER_URL,
    logger,
    ...overrides,
  };
  return { params, runAttempt, decide, logger, controller };
}

// A decision that returns a value the type system forbids (runtime type check, S17).
function forced(value: unknown): number | undefined {
  return value as number | undefined;
}

describe('runRelayerHttpRetryLoop', () => {
  afterEach(() => {
    vi.restoreAllMocks();
    vi.mocked(abortableSleep).mockReset();
    vi.useRealTimers();
  });

  // First attempt succeeds: its result, with no decision, wait or log line.
  it('returns the first attempt result without deciding, waiting or logging', async () => {
    const h = makeHarness();
    h.runAttempt.mockResolvedValueOnce('ok');

    await expect(runRelayerHttpRetryLoop(h.params)).resolves.toBe('ok');
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    expect(h.decide).not.toHaveBeenCalled();
    expect(h.logger.warn).not.toHaveBeenCalled();
    expect(h.logger.error).not.toHaveBeenCalled();
  });

  // A failure, then success after a 1000 ms decided delay: second run only after the full delay, one retry warn.
  it('retries after the decided delay, then returns: 2 runs, 1 warn, decide(sameError, 1)', async () => {
    vi.useFakeTimers();
    const h = makeHarness();
    const error = responseError();
    h.runAttempt.mockRejectedValueOnce(error).mockResolvedValueOnce('ok');
    h.decide.mockReturnValueOnce(1000);

    const run = runRelayerHttpRetryLoop(h.params);
    await vi.advanceTimersByTimeAsync(999);
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    await vi.advanceTimersByTimeAsync(1);

    await expect(run).resolves.toBe('ok');
    expect(h.runAttempt).toHaveBeenCalledTimes(2);
    expect(h.decide).toHaveBeenCalledTimes(1);
    expect(h.decide.mock.calls[0]![0]).toBe(error);
    expect(h.decide.mock.calls[0]![1]).toBe(1);
    expect(h.logger.warn).toHaveBeenCalledTimes(1);
    expect(h.logger.warn).toHaveBeenCalledWith(retryLine('busy', 1000, 1));
    expect(h.logger.error).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });

  // Decision returns undefined: stops and rethrows the same error, one error line, no wait.
  it('stops on undefined: 1 run, the same instance, one error line with cause undefined, no wait', async () => {
    vi.useFakeTimers();
    const h = makeHarness();
    const error = responseError();
    h.runAttempt.mockRejectedValueOnce(error);
    h.decide.mockReturnValueOnce(undefined);

    const thrown = await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);

    expect(thrown).toBe(error);
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    expect(h.logger.error).toHaveBeenCalledTimes(1);
    expect(h.logger.error).toHaveBeenCalledWith(stopLine('busy'), undefined);
    expect(h.logger.warn).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });

  // Three failures: each decision gets that attempt's error and its 1-based attempt number.
  it('passes a fresh error and the 1-based attempt number to each decision', async () => {
    vi.useFakeTimers();
    const h = makeHarness();
    const errors = [responseError('e1'), responseError('e2'), responseError('e3')];
    h.runAttempt.mockRejectedValueOnce(errors[0]).mockRejectedValueOnce(errors[1]).mockRejectedValueOnce(errors[2]);
    h.decide.mockReturnValueOnce(10).mockReturnValueOnce(20).mockReturnValueOnce(undefined);

    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(30);

    expect(await run).toBe(errors[2]);
    expect(h.decide.mock.calls).toEqual([
      [errors[0], 1],
      [errors[1], 2],
      [errors[2], 3],
    ]);
    expect(h.logger.warn.mock.calls).toEqual([[retryLine('e1', 10, 1)], [retryLine('e2', 20, 2)]]);
    expect(h.logger.error).toHaveBeenCalledTimes(1);
    expect(h.logger.error).toHaveBeenCalledWith(stopLine('e3'), undefined);
  });

  // A retry, then a non-relayer error on attempt 2: that error rethrown as is, no second decision.
  it.each([
    ['RelayerHttpFetchError', (): unknown => new RelayerHttpFetchError({ url: RELAYER_URL, message: 'net' })],
    [
      'RelayerHttpUnexpectedResponseError',
      (): unknown => new RelayerHttpUnexpectedResponseError({ url: RELAYER_URL, status: 502 }),
    ],
    [
      'RelayerHttpResultError',
      (): unknown =>
        new RelayerHttpResultError({
          url: RELAYER_URL,
          cause: new InvalidPropertyError({ subject: 'body', property: 'x', expectedType: 'string' }, {}),
          bodyJson: '{}',
        }),
    ],
  ])('retry then %s: that instance unchanged, decide called once only', async (_label, makeError) => {
    vi.useFakeTimers();
    const h = makeHarness();
    const other = makeError();
    h.runAttempt.mockRejectedValueOnce(responseError()).mockRejectedValueOnce(other);
    h.decide.mockReturnValue(10);

    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(10);

    expect(await run).toBe(other);
    expect(h.runAttempt).toHaveBeenCalledTimes(2);
    expect(h.decide).toHaveBeenCalledTimes(1);
    expect(h.logger.error).not.toHaveBeenCalled();
  });

  // Anything but a real RelayerHttpResponseError (look-alike included): rethrown at once, no decision, no log.
  it.each([
    ['a TypeError', new TypeError('x')],
    ['a RelayerHttpTimeoutError', new RelayerHttpTimeoutError({ url: RELAYER_URL, timeoutMs: 1 })],
    ['a string', 'boom'],
    ['null', null],
    ['undefined', undefined],
    ['a look-alike object', { name: 'RelayerHttpResponseError', status: 503, code: 'overloaded' }],
    ['a look-alike Error', Object.assign(new Error('x'), { name: 'RelayerHttpResponseError', status: 503 })],
  ])('rethrows %s unchanged on the first attempt, decide not called', async (_label, value) => {
    const h = makeHarness();
    h.runAttempt.mockRejectedValueOnce(value);
    h.decide.mockReturnValue(10);

    const thrown = await runRelayerHttpRetryLoop(h.params).then(
      () => 'resolved',
      (e: unknown) => e,
    );

    expect(thrown).toBe(value);
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    expect(h.decide).not.toHaveBeenCalled();
    expect(h.logger.error).not.toHaveBeenCalled();
    expect(h.logger.warn).not.toHaveBeenCalled();
  });

  // Decision always retries: stops at 5 attempts, rethrowing the 5th error with one error line.
  it('stops an always-retry decision at 5 runs with the 5th error, decide called for attempts 1..4 only', async () => {
    vi.useFakeTimers();
    const h = makeHarness();
    const errors = [1, 2, 3, 4, 5, 6].map((n) => responseError(`e${n}`));
    for (const e of errors) {
      h.runAttempt.mockRejectedValueOnce(e);
    }
    h.decide.mockReturnValue(1);

    // Four 1 ms waits: advance exactly that, then inspect.
    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(4);

    expect(await run).toBe(errors[4]);
    expect(h.runAttempt).toHaveBeenCalledTimes(5);
    expect(h.decide.mock.calls.map(([, attempt]) => attempt)).toEqual([1, 2, 3, 4]);
    expect(h.logger.warn).toHaveBeenCalledTimes(4);
    expect(h.logger.error).toHaveBeenCalledTimes(1);
    expect(h.logger.error).toHaveBeenCalledWith(stopLine('e5'), undefined);
    expect(vi.getTimerCount()).toBe(0);
  });

  // Retry delay at its maximum: the 5-attempt ceiling still holds.
  it('keeps the ceiling with the largest valid delay (timers not run past attempt 5)', async () => {
    vi.useFakeTimers();
    const h = makeHarness();
    h.runAttempt.mockImplementation(() => Promise.reject(responseError()));
    h.decide.mockReturnValue(MAX_TIMEOUT_MS);

    // Exactly the four waits the ceiling allows; nothing past the stop.
    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    for (let i = 0; i < 4; i++) {
      await vi.advanceTimersByTimeAsync(MAX_TIMEOUT_MS);
    }

    expect(await run).toBeInstanceOf(RelayerHttpResponseError);
    expect(h.runAttempt).toHaveBeenCalledTimes(5);
    expect(vi.getTimerCount()).toBe(0);
  });

  const valueOfSpy = vi.fn(() => 1000);
  // Decision returns an invalid delay (out of range, non-integer, non-number): stop at once,
  // no wait, no coercion, one error line.
  it.each([
    ['0', 0],
    ['-1', -1],
    ['1.5', 1.5],
    ['NaN', Number.NaN],
    ['Infinity', Number.POSITIVE_INFINITY],
    ['-Infinity', Number.NEGATIVE_INFINITY],
    ['MAX_TIMEOUT_MS + 1', MAX_TIMEOUT_MS + 1],
    ['Number.MAX_SAFE_INTEGER', Number.MAX_SAFE_INTEGER],
    ['-0', -0],
    ["'1000'", '1000'],
    ["''", ''],
    ['null', null],
    ['true', true],
    ['1000n', 1000n],
    ['{}', {}],
    ['[1000]', [1000]],
    ['new Number(1000)', new Number(1000)],
    ['an object with valueOf', { valueOf: valueOfSpy }],
    ['a Promise of 1000', Promise.resolve(1000)],
    ['a function', (): number => 1000],
  ])('stops without waiting when decide returns %s', async (_label, value) => {
    vi.useFakeTimers();
    const h = makeHarness();
    const error = responseError();
    h.runAttempt.mockRejectedValueOnce(error).mockResolvedValueOnce('should not run');
    h.decide.mockReturnValueOnce(forced(value));

    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(0);
    expect(vi.getTimerCount()).toBe(0);
    h.controller.abort('backstop: a wait was accepted');

    expect(await run).toBe(error);
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    expect(h.logger.error).toHaveBeenCalledTimes(1);
    expect(h.logger.warn).not.toHaveBeenCalled();
    expect(valueOfSpy).not.toHaveBeenCalled();
  });

  // Smallest and largest valid delays are accepted: a retry warn and one armed wait.
  it.each([1, MAX_TIMEOUT_MS])('accepts a %d ms delay without a timeout (one wait armed)', async (delay) => {
    vi.useFakeTimers();
    const h = makeHarness();
    h.runAttempt.mockRejectedValueOnce(responseError());
    h.decide.mockReturnValueOnce(delay);

    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(0);

    expect(h.logger.warn).toHaveBeenCalledWith(retryLine('busy', delay, 1));
    expect(h.logger.error).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(1);
    expect(h.runAttempt).toHaveBeenCalledTimes(1);

    h.controller.abort('done');
    expect(await run).toBe('done');
    expect(vi.getTimerCount()).toBe(0);
  });

  // Retry only when the 1000 ms delay is strictly below the remaining time; otherwise stop with the error.
  it.each([
    ['remaining > delay', 1001, true],
    ['remaining === delay', 1000, false],
    ['remaining < delay', 999, false],
    ['remaining 0', 0, false],
  ])('%s → wait: %s', async (_label, remaining, waits) => {
    vi.useFakeTimers();
    const h = makeHarness({ remainingMs: () => remaining });
    const error = responseError();
    h.runAttempt.mockRejectedValueOnce(error).mockResolvedValueOnce('ok');
    h.decide.mockReturnValueOnce(1000);

    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    // A stop settles without time passing; only the accepted wait needs its 1000 ms.
    await vi.advanceTimersByTimeAsync(waits ? 1000 : 0);

    if (waits) {
      expect(await run).toBe('ok');
      expect(h.runAttempt).toHaveBeenCalledTimes(2);
      expect(h.logger.warn).toHaveBeenCalledTimes(1);
    } else {
      expect(await run).toBe(error);
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
      expect(h.logger.warn).not.toHaveBeenCalled();
      expect(h.logger.error).toHaveBeenCalledTimes(1);
    }
    expect(vi.getTimerCount()).toBe(0);
  });

  // Remaining time is re-read at each decision: once it drops to 1000 ms, the second failure stops.
  it('reads the remaining time when deciding to wait, not once per run', async () => {
    vi.useFakeTimers();
    let remaining = 5000;
    const h = makeHarness({ remainingMs: () => remaining });
    const errors = [responseError('e1'), responseError('e2')];
    h.runAttempt.mockRejectedValueOnce(errors[0]).mockRejectedValueOnce(errors[1]);
    h.decide.mockReturnValue(1000);

    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(0);
    remaining = 1000;
    await vi.advanceTimersByTimeAsync(1000);
    h.controller.abort('backstop: a second wait was accepted');

    expect(await run).toBe(errors[1]);
    expect(h.runAttempt).toHaveBeenCalledTimes(2);
  });

  // Signal already aborted with a RelayerHttpResponseError reason: rethrown, never retried, no decision.
  it('aborted signal whose reason is a RelayerHttpResponseError: rethrows it, decide not called', async () => {
    const reason = responseError('caller reason');
    const h = makeHarness();
    h.controller.abort(reason);
    h.runAttempt.mockRejectedValueOnce(reason);
    h.decide.mockReturnValue(10);

    expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe(reason);
    expect(h.decide).not.toHaveBeenCalled();
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    expect(h.logger.warn).not.toHaveBeenCalled();
    expect(h.logger.error).not.toHaveBeenCalled();
  });

  // Signal already aborted: the attempt's own error is rethrown (not the abort reason), no decision.
  it('aborted signal: rethrows the caught value itself (not the reason), decide not called', async () => {
    const caught = responseError('from the attempt');
    const h = makeHarness();
    h.controller.abort('user left');
    h.runAttempt.mockRejectedValueOnce(caught);
    h.decide.mockReturnValue(10);

    expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe(caught);
    expect(h.decide).not.toHaveBeenCalled();
    expect(h.logger.error).not.toHaveBeenCalled();
  });

  // 5th attempt fails with an abort or a non-relayer error: that error rethrown, without the ceiling's error line.
  it.each([
    ['an aborted signal', true, (): unknown => responseError('fifth')],
    ['a non-response error', false, (): unknown => new TypeError('fifth')],
  ])('on the 5th attempt, %s beats the ceiling stop: rethrown, no stop line', async (_label, aborts, makeFifth) => {
    vi.useFakeTimers();
    const h = makeHarness();
    const fifth = makeFifth();
    for (let i = 1; i <= 4; i++) {
      h.runAttempt.mockRejectedValueOnce(responseError(`e${i}`));
    }
    h.runAttempt.mockImplementationOnce(() => {
      if (aborts) {
        h.controller.abort('aborted on the fifth');
      }
      return Promise.reject(fifth);
    });
    h.decide.mockReturnValue(1);

    const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(10);

    expect(await run).toBe(fifth);
    expect(h.runAttempt).toHaveBeenCalledTimes(5);
    expect(h.decide).toHaveBeenCalledTimes(4);
    expect(h.logger.error).not.toHaveBeenCalled();
  });

  // Decision throws: that error propagates as is, no wait, no log.
  it('a decide throw propagates unchanged: 1 run, no wait', async () => {
    vi.useFakeTimers();
    const h = makeHarness();
    const thrownByDecide = new Error('decide failed');
    h.runAttempt.mockRejectedValueOnce(responseError());
    h.decide.mockImplementationOnce(() => {
      throw thrownByDecide;
    });

    expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe(thrownByDecide);
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    expect(h.logger.warn).not.toHaveBeenCalled();
    expect(h.logger.error).not.toHaveBeenCalled();
    expect(vi.getTimerCount()).toBe(0);
  });

  // Decision throws a RelayerHttpResponseError: it propagates and is never retried.
  it('a decide throwing a RelayerHttpResponseError: that instance propagates, never retried', async () => {
    const h = makeHarness();
    const fromDecide = responseError('from decide');
    h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('should not run');
    h.decide.mockImplementationOnce(() => {
      throw fromDecide;
    });

    expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe(fromDecide);
    expect(h.runAttempt).toHaveBeenCalledTimes(1);
    expect(h.decide).toHaveBeenCalledTimes(1);
    expect(h.logger.error).not.toHaveBeenCalled();
  });

  // Decision throws null: null propagates as is.
  it('a decide throw of a non-Error value (null) propagates unchanged', async () => {
    const h = makeHarness();
    h.runAttempt.mockRejectedValueOnce(responseError());
    h.decide.mockImplementationOnce(() => {
      throw null;
    });

    expect(
      await runRelayerHttpRetryLoop(h.params).then(
        () => 'resolved',
        (e: unknown) => e,
      ),
    ).toBeNull();
    expect(h.logger.error).not.toHaveBeenCalled();
  });

  // Guard order: the fit check reads the remaining time last, so every earlier exit skips it.
  describe('guard order before the fit check', () => {
    const REMAINING_MS = 60_000;

    // Abort, non-relayer error or throwing decision: exits before reading the remaining time, no error line.
    it.each([
      ['an aborted signal (guard 1)', 'abort'],
      ['a non-response error (guard 2)', 'non-response'],
      ['a decide throw (guard 4)', 'decide-throw'],
    ] as const)('%s: no decision past it, remaining time not read, no stop line', async (_label, kind) => {
      vi.useFakeTimers();
      const remainingMs = vi.fn(() => REMAINING_MS);
      const h = makeHarness({ remainingMs });
      const caught = kind === 'non-response' ? new TypeError('network') : responseError();
      const thrownByDecide = new Error('decide failed');
      if (kind === 'abort') {
        h.controller.abort('user left');
      }
      h.runAttempt.mockRejectedValueOnce(caught).mockResolvedValueOnce('should not run');
      h.decide.mockImplementation(() => {
        if (kind === 'decide-throw') {
          throw thrownByDecide;
        }
        return 10;
      });

      const thrown = await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);

      expect(thrown).toBe(kind === 'decide-throw' ? thrownByDecide : caught);
      expect(h.decide).toHaveBeenCalledTimes(kind === 'decide-throw' ? 1 : 0);
      expect(remainingMs).not.toHaveBeenCalled();
      expect(h.logger.error).not.toHaveBeenCalled();
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
    });

    // Decision stops or returns an invalid delay: stops before reading the remaining time, though plenty is left.
    it.each([
      ['undefined (guard 5)', undefined],
      ['0 (guard 6)', 0],
      ['1.5 (guard 6)', 1.5],
      ['NaN (guard 6)', Number.NaN],
      ["'1000' (guard 6)", '1000'],
      ['MAX_TIMEOUT_MS + 1 (guard 6)', MAX_TIMEOUT_MS + 1],
    ])('decide returning %s stops before the remaining time is read', async (_label, value) => {
      vi.useFakeTimers();
      const remainingMs = vi.fn(() => REMAINING_MS);
      const h = makeHarness({ remainingMs });
      const error = responseError();
      h.runAttempt.mockRejectedValueOnce(error).mockResolvedValueOnce('should not run');
      h.decide.mockReturnValueOnce(forced(value));

      expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe(error);
      expect(h.decide).toHaveBeenCalledTimes(1);
      expect(remainingMs).not.toHaveBeenCalled();
      expect(h.logger.error).toHaveBeenCalledTimes(1);
      expect(h.logger.warn).not.toHaveBeenCalled();
      expect(vi.getTimerCount()).toBe(0);
    });

    // 5th failure: the ceiling stops the loop before the decision or the remaining-time check runs.
    it('the 5th failure consults neither decide nor the remaining time', async () => {
      vi.useFakeTimers();
      const remainingMs = vi.fn(() => REMAINING_MS);
      const h = makeHarness({ remainingMs });
      const errors = [1, 2, 3, 4, 5].map((n) => responseError(`e${n}`));
      for (const e of errors) {
        h.runAttempt.mockRejectedValueOnce(e);
      }
      h.runAttempt.mockImplementation(() => Promise.reject(new Error('forbidden sixth attempt')));
      h.decide.mockReturnValue(1);

      const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(4);

      expect(await run).toBe(errors[4]);
      expect(vi.getTimerCount()).toBe(0);
      expect(h.runAttempt).toHaveBeenCalledTimes(5);
      expect(h.decide.mock.calls).toEqual(errors.slice(0, 4).map((e, i) => [e, i + 1]));
      expect(remainingMs).toHaveBeenCalledTimes(4);
      expect(h.logger.error).toHaveBeenCalledTimes(1);
      expect(h.logger.error).toHaveBeenCalledWith(stopLine('e5'), undefined);
    });
  });

  // Two failures then success: runAttempt called afresh each time, 3 calls.
  it('calls runAttempt again for each attempt (fresh attempt per call)', async () => {
    vi.useFakeTimers();
    const h = makeHarness();
    h.runAttempt
      .mockRejectedValueOnce(responseError())
      .mockRejectedValueOnce(responseError())
      .mockResolvedValueOnce('ok');
    h.decide.mockReturnValue(5);

    const run = runRelayerHttpRetryLoop(h.params);
    await vi.advanceTimersByTimeAsync(10);

    await expect(run).resolves.toBe('ok');
    expect(h.runAttempt).toHaveBeenCalledTimes(3);
  });

  // No logger: a retry still works.
  it('works without a logger', async () => {
    vi.useFakeTimers();
    const h = makeHarness({ logger: undefined });
    h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('ok');
    h.decide.mockReturnValueOnce(5);

    const run = runRelayerHttpRetryLoop(h.params);
    await vi.advanceTimersByTimeAsync(5);

    await expect(run).resolves.toBe('ok');
  });

  describe('aborts and hooks around the wait', () => {
    // Caller aborts during the retry wait: the caller's reason, no further attempt, an abort warn.
    it('caller abort during the wait → the reason, no further run, no timer', async () => {
      vi.useFakeTimers();
      const h = makeHarness();
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('should not run');
      h.decide.mockReturnValueOnce(1000);

      const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(500);
      h.controller.abort('user left');

      expect(await run).toBe('user left');
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
      expect(h.logger.warn).toHaveBeenLastCalledWith(
        `[RelayerHttpRequest] Request aborted on ${RELAYER_URL} while waiting to retry.`,
      );
    });

    // Our timer fires during the retry wait: RelayerHttpTimeoutError carrying the timeout.
    it('our-timer abort during the wait (stub timedOutAfterMs) → RelayerHttpTimeoutError', async () => {
      vi.useFakeTimers();
      let fired: number | undefined;
      const h = makeHarness({ timedOutAfterMs: () => fired });
      h.runAttempt.mockRejectedValueOnce(responseError());
      h.decide.mockReturnValueOnce(1000);

      const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(500);
      fired = 700;
      h.controller.abort(new DOMException('Timed out', 'TimeoutError'));

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(700);
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
    });

    // Decision aborts but returns a delay: the wait rejects with the abort reason at once.
    it('a decide that aborts the signal and returns a delay: the wait sees the abort', async () => {
      vi.useFakeTimers();
      const h = makeHarness();
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('should not run');
      h.decide.mockImplementationOnce(() => {
        h.controller.abort('aborted in decide');
        return 1000;
      });

      expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe('aborted in decide');
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Retry warn hook aborts: the wait rejects with the abort reason at once.
    it('a warn hook that aborts the signal: the wait sees the abort', async () => {
      vi.useFakeTimers();
      const h = makeHarness();
      h.logger.warn.mockImplementationOnce(() => h.controller.abort('aborted in warn'));
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('should not run');
      h.decide.mockReturnValueOnce(1000);

      expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe('aborted in warn');
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Retry warn hook throws: swallowed, the retry goes on as if the hook had not thrown.
    it('a warn hook that throws: swallowed, the wait and the next run still happen', async () => {
      vi.useFakeTimers();
      const h = makeHarness();
      h.logger.warn.mockImplementationOnce(() => {
        throw new Error('warn hook failed');
      });
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('ok');
      h.decide.mockReturnValue(1000);

      const out = runRelayerHttpRetryLoop(h.params);
      await vi.advanceTimersByTimeAsync(1000);

      expect(await out).toBe('ok');
      expect(h.logger.warn).toHaveBeenCalledTimes(1);
      expect(h.runAttempt).toHaveBeenCalledTimes(2);
      expect(h.decide).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Retry warn hook throws a RelayerHttpResponseError: swallowed, never mistaken for an attempt failure.
    it('a warn hook throwing a RelayerHttpResponseError: swallowed, not decided on, the retry goes on', async () => {
      vi.useFakeTimers();
      const h = makeHarness();
      h.logger.warn.mockImplementationOnce(() => {
        throw responseError('from the warn hook');
      });
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('ok');
      h.decide.mockReturnValue(10);

      const out = runRelayerHttpRetryLoop(h.params);
      await vi.advanceTimersByTimeAsync(10);

      expect(await out).toBe('ok');
      expect(h.runAttempt).toHaveBeenCalledTimes(2);
      expect(h.decide).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Our timer already fired when the wait starts: RelayerHttpTimeoutError, no timer armed.
    it('our timer already fired when the wait starts (stub timedOutAfterMs) → RelayerHttpTimeoutError', async () => {
      vi.useFakeTimers();
      let fired: number | undefined;
      const h = makeHarness({ timedOutAfterMs: () => fired });
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('should not run');
      h.decide.mockImplementationOnce(() => {
        fired = 700;
        h.controller.abort(new DOMException('Timed out', 'TimeoutError'));
        return 100;
      });

      const error = await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(700);
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Stop error hook throws: swallowed, the relayer-http error still propagates.
    it('an error hook that throws at stop: swallowed, the response error propagates', async () => {
      const h = makeHarness();
      h.logger.error.mockImplementationOnce(() => {
        throw new Error('error hook failed');
      });
      const stopError = responseError();
      h.runAttempt.mockRejectedValueOnce(stopError);
      h.decide.mockReturnValueOnce(undefined);

      expect(await runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e)).toBe(stopError);
      expect(h.logger.error).toHaveBeenCalledTimes(1);
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
    });

    // Warn hook advances the clock 400 ms: the retry still waits the full 1000 ms from the sleep start.
    it('a warn hook that advances time: the full delay still elapses from the start of the sleep', async () => {
      vi.useFakeTimers();
      const h = makeHarness();
      h.logger.warn.mockImplementationOnce(() => {
        vi.advanceTimersByTime(400);
      });
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('ok');
      h.decide.mockReturnValueOnce(1000);

      const run = runRelayerHttpRetryLoop(h.params);
      await vi.advanceTimersByTimeAsync(999);
      expect(h.runAttempt).toHaveBeenCalledTimes(1);
      await vi.advanceTimersByTimeAsync(1);

      await expect(run).resolves.toBe('ok');
      expect(h.runAttempt).toHaveBeenCalledTimes(2);
    });

    // Abort after the wait ends, before the next attempt: that attempt runs, sees the abort, and its throw is rethrown.
    //   attempt 1 fails ──► wait ends ──► caller aborts ──► attempt 2 throws ──► rejects 'between'
    it('abort between the end of the wait and the next attempt: next attempt runs, its throw is rethrown', async () => {
      vi.useFakeTimers();
      const h = makeHarness();
      const actual = await vi.importActual<typeof import('../../../../base/timeout.js')>('../../../../base/timeout.js');
      vi.mocked(abortableSleep).mockImplementationOnce(async (ms, signal) => {
        await actual.abortableSleep(ms, signal);
        h.controller.abort('between');
      });
      h.runAttempt.mockRejectedValueOnce(responseError()).mockImplementationOnce(() => {
        h.params.signal.throwIfAborted();
        return Promise.resolve('should not resolve');
      });
      h.decide.mockReturnValue(10);

      const run = runRelayerHttpRetryLoop(h.params).catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);

      expect(await run).toBe('between');
      expect(h.runAttempt).toHaveBeenCalledTimes(2);
      expect(h.decide).toHaveBeenCalledTimes(1);
    });
  });

  // 50 retry loops in a row: no timer left behind.
  it('runs many loops back to back without leaving a timer', async () => {
    vi.useFakeTimers();
    for (let i = 0; i < 50; i++) {
      const h = makeHarness();
      h.runAttempt.mockRejectedValueOnce(responseError()).mockResolvedValueOnce('ok');
      h.decide.mockReturnValueOnce(1 + i);
      const run = runRelayerHttpRetryLoop(h.params);
      await vi.advanceTimersByTimeAsync(1 + i);
      await expect(run).resolves.toBe('ok');
    }
    expect(vi.getTimerCount()).toBe(0);
  });
});

describe('waitBeforeHttpRetry', () => {
  afterEach(() => {
    vi.restoreAllMocks();
    vi.mocked(abortableSleep).mockReset();
    vi.useRealTimers();
  });

  function wait(
    controller: AbortController,
    overrides: { delayMs?: number; timedOutAfterMs?: () => number | undefined; logger?: Logger | undefined } = {},
  ): Promise<void> {
    return waitBeforeHttpRetry({
      delayMs: overrides.delayMs ?? 1000,
      signal: controller.signal,
      timedOutAfterMs: overrides.timedOutAfterMs ?? ((): undefined => undefined),
      url: RELAYER_URL,
      logger: 'logger' in overrides ? overrides.logger : makeLogger(),
    });
  }

  // Resolves only once the full delay has passed, leaving no timer or abort listener.
  it('resolves after the delay, not before, and leaves no timer nor listener', async () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const addSpy = vi.spyOn(controller.signal, 'addEventListener');
    const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
    let settled = false;

    const done = wait(controller).then(() => {
      settled = true;
    });
    await vi.advanceTimersByTimeAsync(999);
    expect(settled).toBe(false);
    await vi.advanceTimersByTimeAsync(1);
    await done;

    expect(settled).toBe(true);
    expect(vi.getTimerCount()).toBe(0);
    expect(removeSpy).toHaveBeenCalledTimes(addSpy.mock.calls.length);
  });

  // Sleeps for the given delay on the given signal.
  it('sleeps on the given signal with the given delay', async () => {
    vi.useFakeTimers();
    const controller = new AbortController();

    const done = wait(controller, { delayMs: 42 });
    await vi.advanceTimersByTimeAsync(42);
    await done;

    expect(abortableSleep).toHaveBeenLastCalledWith(42, controller.signal);
  });

  // Caller aborts during the sleep, whatever the reason: that exact reason, timer released, an abort warn.
  it.each([
    ['a string', 'user left'],
    ['null', null],
    ['a caller DOMException TimeoutError', new DOMException('Caller timed out', 'TimeoutError')],
    ['an Error', new Error('cancelled')],
    ['a RelayerHttpResponseError', responseError('as reason')],
  ])('caller abort with %s during the sleep → that reason unchanged, timer released', async (_label, reason) => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const logger = makeLogger();

    const done = wait(controller, { logger }).then(
      () => 'resolved',
      (e: unknown) => e,
    );
    await vi.advanceTimersByTimeAsync(10);
    controller.abort(reason);

    const error = await done;
    expect(error).toBe(reason);
    expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
    expect(vi.getTimerCount()).toBe(0);
    expect(logger.warn).toHaveBeenCalledWith(
      `[RelayerHttpRequest] Request aborted on ${RELAYER_URL} while waiting to retry.`,
    );
  });

  // Our timer fires during the sleep: RelayerHttpTimeoutError and a timeout warn.
  it('our-timer abort (stub timedOutAfterMs) during the sleep → RelayerHttpTimeoutError', async () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const logger = makeLogger();
    let fired: number | undefined;

    const done = wait(controller, { logger, timedOutAfterMs: () => fired }).catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(10);
    fired = 500;
    controller.abort(new DOMException('Timed out', 'TimeoutError'));

    const error = await done;
    expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
    expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(500);
    expect((error as RelayerHttpTimeoutError).url).toBe(RELAYER_URL);
    expect(logger.warn).toHaveBeenCalledWith(
      `[RelayerHttpRequest] Request to ${RELAYER_URL} timed out while waiting to retry.`,
    );
    expect(vi.getTimerCount()).toBe(0);
  });

  // Signal already aborted: rejects at once with its reason, no timer.
  it('an already-aborted signal rejects at once with the reason and arms no timer', async () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    controller.abort('gone');

    expect(await wait(controller).catch((e: unknown) => e)).toBe('gone');
    expect(vi.getTimerCount()).toBe(0);
  });

  // No logger: an abort still rejects with the reason.
  it('works without a logger', async () => {
    const controller = new AbortController();
    controller.abort('gone');

    expect(await wait(controller, { logger: undefined }).catch((e: unknown) => e)).toBe('gone');
  });

  // Sleep rejects while the signal is still live: that value rethrown as is, no warn.
  it.each([
    ['an Error', new Error('sleep broke')],
    ['undefined', undefined],
    ['a RelayerHttpResponseError', responseError('odd')],
  ])('rethrows %s unchanged when abortableSleep rejects on a live signal', async (_label, value) => {
    const controller = new AbortController();
    vi.mocked(abortableSleep).mockImplementationOnce(() => Promise.reject(value));
    const logger = makeLogger();

    const result = await wait(controller, { logger }).then(
      () => 'resolved',
      (e: unknown) => e,
    );

    expect(result).toBe(value);
    expect(logger.warn).not.toHaveBeenCalled();
  });

  // Sleep throws synchronously while the signal is live: that error rethrown as is.
  it('rethrows unchanged when abortableSleep throws synchronously on a live signal', async () => {
    const controller = new AbortController();
    const syncError = new Error('sync throw');
    vi.mocked(abortableSleep).mockImplementationOnce(() => {
      throw syncError;
    });

    expect(await wait(controller).catch((e: unknown) => e)).toBe(syncError);
  });
});
