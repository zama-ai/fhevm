import type { Mock } from 'vitest';
import type { Logger } from '../../../../types/logger.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { MAX_TIMEOUT_MS, createRelayerHttpSignal, throwIfRelayerHttpAborted } from '../relayerHttpSignal.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.combinedSignal.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '9b2c1f4e-3a5d-4e6f-8a7b-1c2d3e4f5a6b';

type TestLogger = Logger & { debug: Mock; warn: Mock; error: Mock };
function makeLogger(): TestLogger {
  return { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
}

function responseError(): RelayerHttpResponseError {
  return new RelayerHttpResponseError({
    url: RELAYER_URL,
    status: 503,
    code: 'overloaded',
    responseMessage: 'busy',
    requestId: VALID_REQUEST_ID,
  });
}

// Returns what `fn` threw (any value, even null or undefined), or a marker when it returned.
const RETURNED = Symbol('returned');
function thrownBy(fn: () => void): unknown {
  try {
    fn();
  } catch (e) {
    return e;
  }
  return RETURNED;
}

// Every caller-side abort reason shape the classifier must hand back unchanged.
function callerReasons(): Array<[string, unknown]> {
  return [
    ['a string', 'user left'],
    ['null', null],
    ['an Error', new Error('cancelled')],
    ['a caller DOMException TimeoutError', new DOMException('Caller timed out', 'TimeoutError')],
    ['a RelayerHttpResponseError', responseError()],
    ['a number', 0],
  ];
}

// AbortSignal.timeout() on the fake clock: same caller-owned TimeoutError reason, no wall-clock wait.
function fakeAbortSignalTimeout(ms: number): AbortSignal {
  const controller = new AbortController();
  setTimeout(() => controller.abort(new DOMException('The operation timed out.', 'TimeoutError')), ms);
  return controller.signal;
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

function nextTick(): Promise<void> {
  return new Promise((resolve) => process.nextTick(resolve));
}

describe('relayerHttpSignal MAX_TIMEOUT_MS', () => {
  // The cap equals setTimeout's largest allowed delay.
  it("is setTimeout's largest delay (2^31 - 1 ms)", () => {
    expect(MAX_TIMEOUT_MS).toBe(2_147_483_647);
  });
});

describe('createRelayerHttpSignal', () => {
  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  // No caller signal, no timeout: a live signal, no timer, no remaining time.
  it('without a caller signal nor a timeout: live signal, no timer, no remaining time', () => {
    vi.useFakeTimers();
    const combined = createRelayerHttpSignal(undefined, undefined);

    expect(combined.signal.aborted).toBe(false);
    expect(combined.timedOutAfterMs()).toBeUndefined();
    expect(combined.remainingMs()).toBeUndefined();
    expect(vi.getTimerCount()).toBe(0);

    combined.cleanup();
    expect(combined.signal.aborted).toBe(false);
  });

  // The combined signal is always a new signal, never the caller's.
  it('never returns the caller signal itself', () => {
    const controller = new AbortController();
    const combined = createRelayerHttpSignal(controller.signal, undefined);

    expect(combined.signal).not.toBe(controller.signal);
    combined.cleanup();
  });

  // Caller signal already aborted: the combined signal starts aborted with the same reason, nothing left after cleanup.
  it.each(callerReasons())('forwards a caller signal already aborted with %s, unchanged', (_label, reason) => {
    vi.useFakeTimers();
    const controller = new AbortController();
    controller.abort(reason);
    const addSpy = vi.spyOn(controller.signal, 'addEventListener');
    const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

    const combined = createRelayerHttpSignal(controller.signal, 60_000);

    expect(combined.signal.aborted).toBe(true);
    expect(combined.signal.reason).toBe(reason);
    expect(combined.timedOutAfterMs()).toBeUndefined();

    combined.cleanup();
    expect(vi.getTimerCount()).toBe(0);
    expect(removeSpy.mock.calls.length).toBeGreaterThanOrEqual(addSpy.mock.calls.length);
    for (const [type, handler] of addSpy.mock.calls) {
      expect(removeSpy).toHaveBeenCalledWith(type, handler);
    }
  });

  // Caller aborted with no reason: the platform's AbortError is forwarded as-is.
  it('forwards a caller signal already aborted without a reason (platform AbortError)', () => {
    const controller = new AbortController();
    controller.abort();

    const combined = createRelayerHttpSignal(controller.signal, undefined);

    expect(combined.signal.reason).toBe(controller.signal.reason);
    expect((combined.signal.reason as DOMException).name).toBe('AbortError');
    combined.cleanup();
  });

  // A signal from AbortSignal.abort(reason) is forwarded already aborted with that reason.
  it('forwards a caller signal created by AbortSignal.abort(reason)', () => {
    const callerSignal = AbortSignal.abort('pre-aborted');

    const combined = createRelayerHttpSignal(callerSignal, undefined);

    expect(combined.signal.aborted).toBe(true);
    expect(combined.signal.reason).toBe('pre-aborted');
    combined.cleanup();
  });

  // Caller aborts later: the combined signal aborts with the same reason, not counted as our timeout.
  it.each(callerReasons())('forwards a later caller abort with %s, unchanged', (_label, reason) => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const combined = createRelayerHttpSignal(controller.signal, 60_000);

    expect(combined.signal.aborted).toBe(false);
    controller.abort(reason);

    expect(combined.signal.aborted).toBe(true);
    expect(combined.signal.reason).toBe(reason);
    expect(combined.timedOutAfterMs()).toBeUndefined();
    combined.cleanup();
  });

  // Caller's own AbortSignal.timeout() fires: forwarded as the caller's reason, not our timeout.
  it("forwards the caller's own AbortSignal.timeout() reason, not as our timeout", () => {
    vi.useFakeTimers();
    vi.spyOn(AbortSignal, 'timeout').mockImplementation(fakeAbortSignalTimeout);
    const callerSignal = AbortSignal.timeout(5);
    const combined = createRelayerHttpSignal(callerSignal, 60_000);

    vi.advanceTimersByTime(5);

    expect(combined.signal.reason).toBe(callerSignal.reason);
    expect((combined.signal.reason as DOMException).name).toBe('TimeoutError');
    expect(combined.timedOutAfterMs()).toBeUndefined();
    combined.cleanup();
  });

  // A live caller signal gets exactly one abort listener.
  it('adds exactly one listener on a live caller signal', () => {
    const controller = new AbortController();
    const addSpy = vi.spyOn(controller.signal, 'addEventListener');

    const combined = createRelayerHttpSignal(controller.signal, undefined);

    expect(addSpy).toHaveBeenCalledTimes(1);
    expect(addSpy.mock.calls[0]![0]).toBe('abort');
    combined.cleanup();
  });

  // Our timer: still live at 499 ms, TimeoutError and timedOutAfterMs at 500 ms.
  it('aborts with a TimeoutError and reports timedOutAfterMs once OUR timer fires, not before', () => {
    vi.useFakeTimers();
    const combined = createRelayerHttpSignal(undefined, 500);
    expect(vi.getTimerCount()).toBe(1);

    vi.advanceTimersByTime(499);
    expect(combined.signal.aborted).toBe(false);
    expect(combined.timedOutAfterMs()).toBeUndefined();

    vi.advanceTimersByTime(1);
    expect(combined.signal.aborted).toBe(true);
    expect((combined.signal.reason as DOMException).name).toBe('TimeoutError');
    expect(combined.timedOutAfterMs()).toBe(500);
    combined.cleanup();
  });

  // A 1 ms timeout fires after 1 ms.
  it('honors the smallest timeout (1 ms)', () => {
    vi.useFakeTimers();
    const combined = createRelayerHttpSignal(undefined, 1);

    expect(combined.signal.aborted).toBe(false);
    vi.advanceTimersByTime(1);

    expect(combined.timedOutAfterMs()).toBe(1);
    combined.cleanup();
  });

  // The maximum timeout counts down correctly and fires only at the very end.
  it('honors the largest timeout (MAX_TIMEOUT_MS) without firing early', () => {
    vi.useFakeTimers();
    const combined = createRelayerHttpSignal(undefined, MAX_TIMEOUT_MS);

    expect(combined.remainingMs()).toBe(MAX_TIMEOUT_MS);
    vi.advanceTimersByTime(MAX_TIMEOUT_MS - 1);
    expect(combined.signal.aborted).toBe(false);
    expect(combined.remainingMs()).toBe(1);

    vi.advanceTimersByTime(1);
    expect(combined.timedOutAfterMs()).toBe(MAX_TIMEOUT_MS);
    combined.cleanup();
  });

  // Caller aborts before our timer: caller reason kept, our later timer changes nothing.
  it('keeps the caller reason when the caller aborts first (our timer fires later)', () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const combined = createRelayerHttpSignal(controller.signal, 500);

    vi.advanceTimersByTime(100);
    controller.abort('user left');
    vi.advanceTimersByTime(1000);

    expect(combined.signal.reason).toBe('user left');
    expect(combined.timedOutAfterMs()).toBeUndefined();
    combined.cleanup();
  });

  // Our timer fires before the caller aborts: our timeout kept.
  it('keeps our timeout when our timer fires first (the caller aborts later)', () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const combined = createRelayerHttpSignal(controller.signal, 500);

    vi.advanceTimersByTime(500);
    controller.abort('user left');

    expect((combined.signal.reason as DOMException).name).toBe('TimeoutError');
    expect(combined.timedOutAfterMs()).toBe(500);
    combined.cleanup();
  });

  // Caller timer and ours due at the same instant, caller's armed first: caller reason wins (fake-timer FIFO).
  //   caller setTimeout(500) ──► our timer(500) ──► +500: caller fires first ──► 'caller first'
  it('keeps the caller reason when a caller timer due at the same instant was armed before ours', () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    setTimeout(() => controller.abort('caller first'), 500);
    const combined = createRelayerHttpSignal(controller.signal, 500);

    vi.advanceTimersByTime(500);

    expect(combined.signal.reason).toBe('caller first');
    expect(combined.timedOutAfterMs()).toBeUndefined();
    combined.cleanup();
  });

  // Same instant, but ours armed first: our timeout wins.
  it('keeps our timeout when a caller timer due at the same instant was armed after ours', () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    const combined = createRelayerHttpSignal(controller.signal, 500);
    setTimeout(() => controller.abort('caller second'), 500);

    vi.advanceTimersByTime(500);

    expect(combined.timedOutAfterMs()).toBe(500);
    expect((combined.signal.reason as DOMException).name).toBe('TimeoutError');
    combined.cleanup();
  });

  // Caller signal already aborted, then our timer comes due: still the caller's abort, not a timeout.
  it('a pre-aborted caller signal is never reclassified as our timeout when the timer is due', () => {
    vi.useFakeTimers();
    const controller = new AbortController();
    controller.abort('gone');
    const combined = createRelayerHttpSignal(controller.signal, 10);

    vi.advanceTimersByTime(10);

    expect(combined.signal.reason).toBe('gone');
    expect(combined.timedOutAfterMs()).toBeUndefined();
    combined.cleanup();
  });

  describe('remainingMs', () => {
    // No timeout: remainingMs stays undefined.
    it('is undefined without a timeout, whatever time elapses', () => {
      vi.useFakeTimers();
      const combined = createRelayerHttpSignal(undefined, undefined);

      vi.advanceTimersByTime(10_000);

      expect(combined.remainingMs()).toBeUndefined();
      combined.cleanup();
    });

    // remainingMs counts down from the timeout as time passes.
    it('starts at the timeout and decreases as monotonic time elapses', () => {
      vi.useFakeTimers();
      const combined = createRelayerHttpSignal(undefined, 1000);

      expect(combined.remainingMs()).toBe(1000);
      vi.advanceTimersByTime(300);
      expect(combined.remainingMs()).toBe(700);
      vi.advanceTimersByTime(699);
      expect(combined.remainingMs()).toBe(1);
      combined.cleanup();
    });

    // remainingMs never goes up between reads.
    it('never increases between two reads', () => {
      vi.useFakeTimers();
      const combined = createRelayerHttpSignal(undefined, 1000);

      let previous = combined.remainingMs()!;
      for (const step of [0, 1, 7, 100, 0, 250, 642, 1, 500]) {
        vi.advanceTimersByTime(step);
        const current = combined.remainingMs()!;
        expect(current).toBeLessThanOrEqual(previous);
        previous = current;
      }
      combined.cleanup();
    });

    // remainingMs bottoms out at 0: at and past the deadline, and after cleanup.
    it('never goes below 0, at and past the deadline, and after cleanup', () => {
      vi.useFakeTimers();
      const combined = createRelayerHttpSignal(undefined, 1000);

      vi.advanceTimersByTime(1000);
      expect(combined.remainingMs()).toBe(0);
      vi.advanceTimersByTime(5000);
      expect(combined.remainingMs()).toBe(0);
      combined.cleanup();
      vi.advanceTimersByTime(5000);
      expect(combined.remainingMs()).toBe(0);
    });

    // A wall-clock jump changes neither remainingMs nor the timer.
    it.each([
      ['forward', 1_000_000_000],
      ['backward', -1_000_000_000],
    ])('is unaffected by a system-clock jump %s (vi.setSystemTime)', (_label, jump) => {
      vi.useFakeTimers();
      const combined = createRelayerHttpSignal(undefined, 1000);
      vi.advanceTimersByTime(200);

      vi.setSystemTime(Date.now() + jump);

      expect(combined.remainingMs()).toBe(800);
      expect(combined.signal.aborted).toBe(false);
      combined.cleanup();
    });
  });

  describe('cleanup', () => {
    // cleanup removes the caller listener and clears our timer.
    it('removes the listener it added on the caller signal and clears our timer', () => {
      vi.useFakeTimers();
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
      const combined = createRelayerHttpSignal(controller.signal, 500);

      combined.cleanup();

      expect(vi.getTimerCount()).toBe(0);
      const [type, handler] = addSpy.mock.calls[0]!;
      expect(removeSpy).toHaveBeenCalledWith(type, handler);
    });

    // After cleanup, neither a caller abort nor the old deadline aborts the combined signal.
    it('stops forwarding: neither a later caller abort nor the old deadline aborts the signal', () => {
      vi.useFakeTimers();
      const controller = new AbortController();
      const combined = createRelayerHttpSignal(controller.signal, 500);

      combined.cleanup();
      controller.abort('too late');
      vi.advanceTimersByTime(1000);

      expect(combined.signal.aborted).toBe(false);
      expect(combined.timedOutAfterMs()).toBeUndefined();
    });

    // Calling cleanup twice is harmless and arms nothing.
    it('is idempotent: a second call neither throws nor re-arms anything', () => {
      vi.useFakeTimers();
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const combined = createRelayerHttpSignal(controller.signal, 500);

      combined.cleanup();
      expect(() => combined.cleanup()).not.toThrow();

      expect(vi.getTimerCount()).toBe(0);
      expect(addSpy).toHaveBeenCalledTimes(1);
      controller.abort('after');
      expect(combined.signal.aborted).toBe(false);
    });

    // cleanup after our timer already fired is harmless.
    it('is safe after our timer already fired', () => {
      vi.useFakeTimers();
      const combined = createRelayerHttpSignal(undefined, 10);
      vi.advanceTimersByTime(10);

      expect(() => combined.cleanup()).not.toThrow();
      expect(vi.getTimerCount()).toBe(0);
      expect(combined.timedOutAfterMs()).toBe(10);
    });

    // 100 signals created and cleaned up on one caller signal: listeners balanced, no timer left.
    it('leaves nothing behind across many signals sharing one long-lived caller signal', () => {
      vi.useFakeTimers();
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

      for (let i = 0; i < 100; i++) {
        createRelayerHttpSignal(controller.signal, 1 + i).cleanup();
      }

      expect(vi.getTimerCount()).toBe(0);
      expect(addSpy).toHaveBeenCalledTimes(100);
      expect(removeSpy).toHaveBeenCalledTimes(100);
      for (const [type, handler] of addSpy.mock.calls) {
        expect(removeSpy).toHaveBeenCalledWith(type, handler);
      }
    });
  });

  describe('release on abort', () => {
    // Our timer aborts only the combined signal, never the caller's.
    it('our timer never aborts the caller signal', () => {
      vi.useFakeTimers();
      const controller = new AbortController();
      const combined = createRelayerHttpSignal(controller.signal, 500);

      vi.advanceTimersByTime(500);

      expect(controller.signal.aborted).toBe(false);
      expect(combined.signal.aborted).toBe(true);
      expect((combined.signal.reason as DOMException).name).toBe('TimeoutError');
      expect(combined.timedOutAfterMs()).toBe(500);
      combined.cleanup();
    });

    // Caller signal already aborted: our signal is dead from the start, so no timer is armed.
    it('a pre-aborted caller signal arms no timer', () => {
      vi.useFakeTimers();
      const controller = new AbortController();
      controller.abort('early');
      const combined = createRelayerHttpSignal(controller.signal, 500);

      expect(combined.signal.reason).toBe('early');
      expect(vi.getTimerCount()).toBe(0);
      combined.cleanup();
      expect(vi.getTimerCount()).toBe(0);
    });

    // Another caller listener throws on abort: forwarding and release still happen; the throw surfaces as uncaught.
    it('a throwing listener on the caller signal blocks neither forwarding nor release', async () => {
      vi.useFakeTimers();
      const trap = trapUncaughtExceptions();
      try {
        const controller = new AbortController();
        const boom = new Error('caller listener failed');
        controller.signal.addEventListener('abort', () => {
          throw boom;
        });
        const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
        const combined = createRelayerHttpSignal(controller.signal, 500);

        controller.abort('user left');
        await nextTick();

        expect(combined.signal.reason).toBe('user left');
        expect(combined.timedOutAfterMs()).toBeUndefined();
        expect(vi.getTimerCount()).toBe(0);
        expect(removeSpy).toHaveBeenCalledTimes(1);
        expect(trap.caught).toEqual([boom]);
      } finally {
        trap.restore();
      }
    });

    // A listener on the combined signal throws: release still happens; the throw surfaces as uncaught.
    it('a throwing listener on the combined signal does not block release', async () => {
      vi.useFakeTimers();
      const trap = trapUncaughtExceptions();
      try {
        const controller = new AbortController();
        const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
        const combined = createRelayerHttpSignal(controller.signal, 500);
        const boom = new Error('combined listener failed');
        combined.signal.addEventListener('abort', () => {
          throw boom;
        });

        controller.abort('user left');
        await nextTick();

        expect(combined.signal.reason).toBe('user left');
        expect(vi.getTimerCount()).toBe(0);
        expect(removeSpy).toHaveBeenCalledTimes(1);
        expect(trap.caught).toEqual([boom]);
      } finally {
        trap.restore();
      }
    });
  });
});

describe('throwIfRelayerHttpAborted', () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  function check(
    signal: AbortSignal,
    timedOutAfterMs: number | undefined,
    during = '',
    logger: Logger | undefined = makeLogger(),
  ): unknown {
    return thrownBy(() =>
      throwIfRelayerHttpAborted({ signal, timedOutAfterMs: () => timedOutAfterMs, url: RELAYER_URL, logger, during }),
    );
  }

  // Live signal: returns, logs nothing.
  it('returns without logging when the signal is live', () => {
    const logger = makeLogger();

    expect(check(new AbortController().signal, undefined, '', logger)).toBe(RETURNED);
    expect(logger.warn).not.toHaveBeenCalled();
    expect(logger.error).not.toHaveBeenCalled();
  });

  // Live signal: returns even if timedOutAfterMs reports a value.
  it('returns when the signal is live even if timedOutAfterMs reports a value', () => {
    expect(check(new AbortController().signal, 500)).toBe(RETURNED);
  });

  // Aborted by our timer: throws RelayerHttpTimeoutError and warns once, naming the phase.
  it.each(['', ' while reading the response body', ' while waiting to retry'])(
    'throws RelayerHttpTimeoutError when OUR timer aborted (during=%j) and warns once',
    (during) => {
      const logger = makeLogger();
      const signal = AbortSignal.abort(new DOMException('Timed out', 'TimeoutError'));

      const error = check(signal, 500, during, logger);

      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(500);
      expect((error as RelayerHttpTimeoutError).url).toBe(RELAYER_URL);
      expect(logger.warn).toHaveBeenCalledTimes(1);
      expect(logger.warn).toHaveBeenCalledWith(`[RelayerHttpRequest] Request to ${RELAYER_URL} timed out${during}.`);
      expect(logger.error).not.toHaveBeenCalled();
    },
  );

  // Aborted by the caller: rethrows the caller's reason as-is and warns once.
  it.each(callerReasons())('rethrows a caller reason that is %s, unchanged, and warns once', (_label, reason) => {
    const logger = makeLogger();
    const controller = new AbortController();
    controller.abort(reason);

    const error = check(controller.signal, undefined, ' while waiting to retry', logger);

    expect(error).toBe(reason);
    expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
    expect(logger.warn).toHaveBeenCalledTimes(1);
    expect(logger.warn).toHaveBeenCalledWith(
      `[RelayerHttpRequest] Request aborted on ${RELAYER_URL} while waiting to retry.`,
    );
  });

  // timedOutAfterMs decides: our timeout wins even when the reason is a response error.
  it('classifies by timedOutAfterMs, not by the reason: our timer wins over a RelayerHttpResponseError reason', () => {
    const signal = AbortSignal.abort(responseError());

    expect(check(signal, 250)).toBeInstanceOf(RelayerHttpTimeoutError);
  });

  // Works with no logger, or with a logger whose warn is undefined.
  it('works without a logger, and with a logger whose warn is undefined', () => {
    const signal = AbortSignal.abort('gone');

    expect(check(signal, undefined, '', undefined)).toBe('gone');
    expect(check(signal, undefined, '', { debug: undefined, warn: undefined, error: undefined })).toBe('gone');
  });

  // A throwing warn hook is swallowed: the caller reason and our timeout are still thrown as-is.
  it('a throwing warn hook does not replace the classified abort', () => {
    const throwingLogger: Logger = {
      debug: undefined,
      warn: () => {
        throw new Error('warn hook failed');
      },
      error: undefined,
    };

    expect(check(AbortSignal.abort('gone'), undefined, '', throwingLogger)).toBe('gone');
    expect(check(AbortSignal.abort('gone'), 250, '', throwingLogger)).toBeInstanceOf(RelayerHttpTimeoutError);
  });
});
