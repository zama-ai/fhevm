import type { Logger } from '../../../types/logger.js';
import { abortableSleep } from '../../../base/timeout.js';
import { RelayerHttpResponseError } from '../../../errors/RelayerHttpResponseError.js';
import { MAX_TIMEOUT_MS, throwIfRelayerHttpAborted } from './relayerHttpSignal.js';

// Safety ceiling, not a policy cap: the loop never starts attempt 6, whatever `decide` says.
// A later policy's cap must be <= this value, or raise it deliberately.
const MAX_HTTP_ATTEMPTS_CEILING = 5;

/** @internal Everything the retry loop needs; no fetch, no class. */
export type RelayerHttpRetryLoopParams<T> = {
  // One fresh attempt per call.
  readonly runAttempt: () => Promise<T>;
  // The owner's retry decision: a delay in ms, or undefined to stop.
  readonly decide: (error: RelayerHttpResponseError, attempt: number) => number | undefined;
  readonly signal: AbortSignal;
  readonly timedOutAfterMs: () => number | undefined;
  readonly remainingMs: () => number | undefined;
  readonly url: string;
  readonly logger: Logger | undefined;
};

/**
 * @internal Runs attempts until one returns or a throw is final.
 *
 * ```
 * attempt ──► ok ───────────────────────────────────────────► return
 *    │
 *    └─► throw ──► aborted, or not a RelayerHttpResponseError ──► rethrow
 *                     │
 *                     └─► retry delay? ──► no ──► log error ──► rethrow
 *                            │
 *                            └─► yes ──► log warn ──► wait ──► attempt
 * ```
 *
 * @param params - The attempt runner, the decision, the combined signal, url and logger.
 * @returns The first successful attempt's result.
 * @throws The final attempt's error, the same instance; or the abort/timeout error.
 *
 * @example
 * ```typescript
 * const result = await runRelayerHttpRetryLoop({
 *   runAttempt: () => attempt.run(),
 *   decide: () => undefined,
 *   ...combined,
 *   url,
 *   logger,
 * });
 * ```
 */
export async function runRelayerHttpRetryLoop<T>(params: RelayerHttpRetryLoopParams<T>): Promise<T> {
  // 1-based number of HTTP attempts made so far; drives the ceiling and the decision.
  for (let attempt = 1; ; ++attempt) {
    // -------------------------------------------------------------------------
    // 1. Run one attempt
    // -------------------------------------------------------------------------

    let error: unknown;
    try {
      return await params.runAttempt();
    } catch (e) {
      error = e;
    }

    // -------------------------------------------------------------------------
    // 2. Rethrow an abort or a non-retryable error type unchanged
    // -------------------------------------------------------------------------

    if (params.signal.aborted || !(error instanceof RelayerHttpResponseError)) {
      throw error;
    }

    // -------------------------------------------------------------------------
    // 3. Stop, or wait before the next attempt
    // -------------------------------------------------------------------------

    const delayMs = _retryDelay(params, error, attempt);
    if (delayMs === undefined) {
      _logResponseError(params, error);
      throw error;
    }

    _logRetry(params, error, attempt, delayMs);

    await waitBeforeHttpRetry({
      delayMs,
      signal: params.signal,
      timedOutAfterMs: params.timedOutAfterMs,
      url: params.url,
      logger: params.logger,
    });
  }
}

/**
 * Asks `decide` for a delay, then vetoes it unless every loop guard accepts it.
 * Guards only veto: a `decide` throw propagates unchanged.
 *
 * @param params - The loop params (`decide` and `remainingMs` are used).
 * @param error - The attempt's relayer-http error.
 * @param attempt - The 1-based number of attempts already made.
 * @returns The accepted delay in ms, or undefined to stop.
 *
 * @example
 * ```typescript
 * const delayMs = _retryDelay(params, error, 1); // undefined or e.g. 1000
 * ```
 */
function _retryDelay<T>(
  params: RelayerHttpRetryLoopParams<T>,
  error: RelayerHttpResponseError,
  attempt: number,
): number | undefined {
  if (attempt >= MAX_HTTP_ATTEMPTS_CEILING) {
    return undefined;
  }
  // unknown: an override may return any runtime type; _isValidRetryDelay fails it closed.
  const delayMs: unknown = params.decide(error, attempt);
  if (!_isValidRetryDelay(delayMs)) {
    return undefined;
  }

  if (!_fitsRemaining(delayMs, params.remainingMs())) {
    return undefined;
  }

  return delayMs;
}

/**
 * @internal Sleeps `delayMs` on the combined signal. On rejection, throws the classified
 * abort error, or rethrows the caught value unchanged if the signal is not aborted.
 *
 * @param args - The delay, the combined signal, its timeout marker, url and logger.
 * @returns A promise that resolves after `delayMs`.
 * @throws {RelayerHttpTimeoutError} If our own timer aborted the signal during the wait.
 *
 * @example
 * ```typescript
 * await waitBeforeHttpRetry({ delayMs: 1000, signal, timedOutAfterMs, url, logger });
 * ```
 */
export async function waitBeforeHttpRetry(args: {
  readonly delayMs: number;
  readonly signal: AbortSignal;
  readonly timedOutAfterMs: () => number | undefined;
  readonly url: string;
  readonly logger: Logger | undefined;
}): Promise<void> {
  try {
    await abortableSleep(args.delayMs, args.signal);
  } catch (e) {
    // Classify from the signal, not the rejection: our own timeout becomes RelayerHttpTimeoutError.
    throwIfRelayerHttpAborted({
      signal: args.signal,
      timedOutAfterMs: args.timedOutAfterMs,
      url: args.url,
      logger: args.logger,
      during: ' while waiting to retry',
    });
    throw e;
  }
}

/**
 * Whether `value` is a usable retry delay: an integer in [1, MAX_TIMEOUT_MS].
 * Anything else, including a non-number, fails closed.
 *
 * @param value - The decision's return value.
 * @returns `true` when `value` is an integer from 1 to MAX_TIMEOUT_MS.
 *
 * @example
 * ```typescript
 * _isValidRetryDelay(1000); // true
 * _isValidRetryDelay(1.5); // false
 * ```
 */
function _isValidRetryDelay(value: unknown): value is number {
  return typeof value === 'number' && Number.isInteger(value) && value >= 1 && value <= MAX_TIMEOUT_MS;
}

/**
 * Whether a wait of `delayMs` ends before the run's timeout fires.
 *
 * @param delayMs - The retry delay in ms.
 * @param remainingMs - Time left before the timeout, or undefined without timeout.
 * @returns `true` when no timeout is set or `delayMs < remainingMs`.
 *
 * @example
 * ```typescript
 * _fitsRemaining(1000, 500); // false
 * _fitsRemaining(1000, undefined); // true
 * ```
 */
function _fitsRemaining(delayMs: number, remainingMs: number | undefined): boolean {
  return remainingMs === undefined || delayMs < remainingMs;
}

/**
 * Warns once per accepted retry wait.
 *
 * @param params - The loop params (`url` and `logger` are used).
 * @param error - The attempt's relayer-http error.
 * @param attempt - The 1-based number of attempts already made.
 * @param delayMs - The accepted delay in ms.
 *
 * @example
 * ```typescript
 * _logRetry(params, error, 1, 1000);
 * ```
 */
function _logRetry<T>(
  params: RelayerHttpRetryLoopParams<T>,
  error: RelayerHttpResponseError,
  attempt: number,
  delayMs: number,
): void {
  try {
    params.logger?.warn?.(
      `[RelayerHttpRequest] relayer-http error on ${params.url} (status=${error.status}, code=${error.code}, requestId=${error.requestId}): ${error.responseMessage}. Retrying in ${delayMs} ms (attempt ${attempt}).`,
    );
  } catch {
    // A throwing logger must never change the outcome.
  }
}

/**
 * Logs the final relayer-http error once, when the loop stops.
 *
 * @param params - The loop params (`url` and `logger` are used).
 * @param error - The relayer-http error about to be rethrown.
 *
 * @example
 * ```typescript
 * _logResponseError(params, error);
 * ```
 */
function _logResponseError<T>(params: RelayerHttpRetryLoopParams<T>, error: RelayerHttpResponseError): void {
  try {
    params.logger?.error?.(
      `[RelayerHttpRequest] relayer-http error on ${params.url} (status=${error.status}, code=${error.code}, requestId=${error.requestId}): ${error.responseMessage}`,
      undefined,
    );
  } catch {
    // A throwing logger must never change the outcome.
  }
}
