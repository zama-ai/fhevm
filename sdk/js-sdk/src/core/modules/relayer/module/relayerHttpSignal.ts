import type { Logger } from '../../../types/logger.js';
import { RelayerHttpTimeoutError } from '../../../errors/RelayerHttpTimeoutError.js';

// setTimeout's largest delay (2^31 - 1 ms); bounds options.timeout and any retry delay.
// Node turns a larger delay into 1 ms. https://nodejs.org/api/timers.html#settimeoutcallback-delay-args
export const MAX_TIMEOUT_MS = 2_147_483_647;

/** @internal The run's combined signal; created and cleaned up only by RelayerHttpRequest. */
export type RelayerHttpSignal = {
  // The only internal signal of the run.
  readonly signal: AbortSignal;
  // Defined only when OUR timer fired.
  readonly timedOutAfterMs: () => number | undefined;
  // Monotonic time left before our timer fires; undefined without timeout; never < 0.
  readonly remainingMs: () => number | undefined;
  // Clears the timer and the caller listener; idempotent.
  readonly cleanup: () => void;
};

/**
 * @internal Combines the caller's signal (if any) with `timeoutMs` into one internal signal.
 *
 * @param callerSignal - The caller's own signal, forwarded (or its prior abort copied).
 * @param timeoutMs - The validated `options.timeout`, or undefined for no timer.
 * @returns The combined signal, its timeout marker, the remaining time and `cleanup`.
 *
 * @example
 * ```typescript
 * const combined = createRelayerHttpSignal(options.signal, 5000);
 * try {
 *   await fetch(url, { signal: combined.signal });
 * } finally {
 *   combined.cleanup();
 * }
 * ```
 */
export function createRelayerHttpSignal(
  callerSignal: AbortSignal | undefined,
  timeoutMs: number | undefined,
): RelayerHttpSignal {
  const controller = new AbortController();

  // -------------------------------------------------------------------------
  // 1. Forward the caller's abort
  // -------------------------------------------------------------------------

  const onCallerAbort = (): void => {
    controller.abort(callerSignal?.reason);
    release();
  };

  const removeCallerListener =
    callerSignal === undefined || callerSignal.aborted ? undefined : _addAbortListener(callerSignal, onCallerAbort);

  if (callerSignal?.aborted === true) {
    controller.abort(callerSignal.reason);
  }

  // -------------------------------------------------------------------------
  // 2. Arm our timer
  // -------------------------------------------------------------------------

  // Set only by our own timer, to tell our timeout apart from the caller's abort.
  let firedTimeoutMs: number | undefined;

  // performance.now() is monotonic: unaffected by system clock changes.
  const startedAtMs = performance.now();

  const onTimeout = (): void => {
    // Already aborted (by the caller): the first reason stands.
    if (controller.signal.aborted) {
      return;
    }
    firedTimeoutMs = timeoutMs;
    // Same reason AbortSignal.timeout() uses; a clearable timer, unlike that one.
    controller.abort(new DOMException('Timed out', 'TimeoutError'));
    release();
  };

  // Already aborted (by the caller): our signal is dead from the start, so no timer is armed.
  const timeoutId = timeoutMs === undefined || controller.signal.aborted ? undefined : setTimeout(onTimeout, timeoutMs);

  // Once aborted, the timer and the caller listener are useless: release them now,
  // so a fetch that ignores the signal and never settles cannot keep them alive. Idempotent.
  let released = false;
  function release(): void {
    if (released) {
      return;
    }
    released = true;
    if (timeoutId !== undefined) {
      clearTimeout(timeoutId);
    }
    removeCallerListener?.();
  }

  // -------------------------------------------------------------------------
  // 3. Expose the signal, timeout marker, remaining time and cleanup
  // -------------------------------------------------------------------------

  return {
    signal: controller.signal,
    timedOutAfterMs: () => firedTimeoutMs,
    remainingMs: () => _remainingMs(timeoutMs, startedAtMs),
    cleanup: release,
  };
}

/**
 * Adds a one-shot 'abort' listener and returns the closure that removes it,
 * like undici's addAbortListener (lib/core/util.js).
 *
 * @param signal - The signal to listen to.
 * @param listener - Called once when `signal` aborts.
 * @returns A function that removes `listener` from `signal`.
 *
 * @example
 * ```typescript
 * const remove = _addAbortListener(signal, onAbort);
 * remove();
 * ```
 */
function _addAbortListener(signal: AbortSignal, listener: () => void): () => void {
  signal.addEventListener('abort', listener, { once: true });
  return () => {
    signal.removeEventListener('abort', listener);
  };
}

/**
 * Time left before a timer armed at `startedAtMs` fires, clamped at 0.
 *
 * @param timeoutMs - The timer's delay, or undefined when no timer is armed.
 * @param startedAtMs - `performance.now()` when the timer was armed.
 * @returns The remaining milliseconds (>= 0), or undefined without a timer.
 *
 * @example
 * ```typescript
 * _remainingMs(1000, performance.now()); // ~1000
 * _remainingMs(undefined, 0); // undefined
 * ```
 */
function _remainingMs(timeoutMs: number | undefined, startedAtMs: number): number | undefined {
  if (timeoutMs === undefined) {
    return undefined;
  }

  return Math.max(0, timeoutMs - (performance.now() - startedAtMs));
}

/**
 * @internal Throws if `signal` is aborted, classified by who aborted it; returns otherwise.
 * Our timer gives RelayerHttpTimeoutError; anything else rethrows `signal.reason` unchanged
 * (any value, even null).
 *
 * @param args - The signal, its timeout marker, the url and logger, and a `during` suffix
 * for the log line (`''`, `' while reading the response body'`, `' while waiting to retry'`).
 * @throws {RelayerHttpTimeoutError} If our own timer aborted the signal.
 *
 * @example
 * ```typescript
 * throwIfRelayerHttpAborted({ signal, timedOutAfterMs, url, logger, during: '' });
 * ```
 */
export function throwIfRelayerHttpAborted(args: {
  readonly signal: AbortSignal;
  readonly timedOutAfterMs: () => number | undefined;
  readonly url: string;
  readonly logger: Logger | undefined;
  readonly during: string;
}): void {
  const { signal, url, logger, during } = args;

  if (!signal.aborted) {
    return;
  }

  const timeoutMs = args.timedOutAfterMs();
  if (timeoutMs !== undefined) {
    try {
      logger?.warn?.(`[RelayerHttpRequest] Request to ${url} timed out${during}.`);
    } catch {
      // A throwing logger must never change the outcome.
    }
    throw new RelayerHttpTimeoutError({ url, timeoutMs });
  }

  try {
    logger?.warn?.(`[RelayerHttpRequest] Request aborted on ${url}${during}.`);
  } catch {
    // A throwing logger must never change the outcome.
  }

  throw signal.reason;
}
