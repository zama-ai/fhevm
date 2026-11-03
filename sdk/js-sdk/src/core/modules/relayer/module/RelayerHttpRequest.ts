import type { Auth } from '../../../types/auth.js';
import type { Logger } from '../../../types/logger.js';
import type { FetchResultOf, RelayerPublicDecryptOptions, RelayerUserDecryptOptions } from '../../../types/relayer.js';
import type { RelayerHttpOperation } from './RelayerHttpAttempt.js';
import type { RelayerHttpSignal } from './relayerHttpSignal.js';
import type { RelayerHttpResponseError } from '../../../errors/RelayerHttpResponseError.js';
import { sdkName, version } from '../../../_version.js';
import { setAuth } from '../../../base/auth.js';
import { normalizeHeaders } from '../../../base/fetch.js';
import { isUintNumber } from '../../../base/uint.js';
import { InvalidPropertyError } from '../../../base/errors/InvalidPropertyError.js';
import { RelayerHttpTimeoutError } from '../../../errors/RelayerHttpTimeoutError.js';
import { normalizeRelayerQuery } from './relayerUrl.js';
import { RelayerHttpAttempt } from './RelayerHttpAttempt.js';
import { runRelayerHttpRetryLoop } from './relayerHttpRetryLoop.js';
import { MAX_TIMEOUT_MS, createRelayerHttpSignal, throwIfRelayerHttpAborted } from './relayerHttpSignal.js';

// One-shot client for relayer-http's synchronous routes (e.g. POST /v4/user-decrypt,
// /v4/public-decrypt): one POST, one JSON response, no job id, no polling
// (unlike RelayerAsyncRequest).
//
// Flow:
//
//   RelayerHttpRequest.run()
//     │  combine caller signal + timeout ──► relayerHttpSignal
//     ▼
//   runRelayerHttpRetryLoop ◄──── _retryDecision: retry after a delay, or stop
//     │  one fresh attempt per try
//     ▼
//   RelayerHttpAttempt.run()
//     fetch ──► read body ──► parse JSON ──► check status ──► validate result
//
// Files:
//   RelayerHttpRequest.ts    Entry point. Owns the run: signal, timeout, cleanup, retry policy.
//   relayerHttpSignal.ts     Merges the caller's abort signal and the timeout into one signal.
//   relayerHttpRetryLoop.ts  Runs attempts, waits between them, stops on success or a final error.
//   RelayerHttpAttempt.ts    One HTTP call: sends the request, checks and returns the response.

type RelayerHttpRequestParams<O extends RelayerHttpOperation> = {
  relayerOperation: O;
  url: string;
  payload: Record<string, unknown>;
  // onProgress is ignored: this is a single-shot request, not a polled job, so
  // there's no intermediate progress to report.
  options?: Omit<RelayerUserDecryptOptions, 'onProgress'> | Omit<RelayerPublicDecryptOptions, 'onProgress'> | undefined;
  logger?: Logger | undefined;
};

export class RelayerHttpRequest<O extends RelayerHttpOperation> {
  private readonly _relayerOperation: O;
  private readonly _url: string;
  private readonly _payload: Record<string, unknown>;
  private readonly _auth: Auth | undefined;
  private readonly _headers: Readonly<Record<string, string>>;
  private readonly _query: Readonly<Record<string, string>> | undefined;
  private readonly _signal: AbortSignal | undefined;
  private readonly _timeoutMs: number | undefined;
  private readonly _fetchRetries: number | undefined;
  private readonly _fetchRetryDelayInMilliseconds: number | undefined;
  private readonly _logger: Logger | undefined;
  // One-shot guard: run() may be called once per instance.
  private _hasRun: boolean;

  constructor(params: RelayerHttpRequestParams<O>) {
    _assertRelayerHttpOperation(params.relayerOperation);

    this._relayerOperation = params.relayerOperation;
    this._url = params.url;
    this._payload = params.payload;
    this._auth = params.options?.auth;
    this._headers = normalizeHeaders(params.options?.headers);
    this._query = normalizeRelayerQuery(params.options?.query);
    this._signal = params.options?.signal ?? undefined;
    this._timeoutMs = _validateTimeoutMs(params.options?.timeout);
    this._fetchRetries = params.options?.fetchRetries;
    this._fetchRetryDelayInMilliseconds = params.options?.fetchRetryDelayInMilliseconds;
    this._logger = params.logger;
    this._hasRun = false;
  }

  public async run(): Promise<FetchResultOf<O>> {
    // -------------------------------------------------------------------------
    // 1. Allow one run per instance
    // -------------------------------------------------------------------------

    if (this._hasRun) {
      throw new Error(
        'RelayerHttpRequest.run() was already called on this instance. Create a new instance per request.',
      );
    }
    this._hasRun = true;

    // A caller signal already aborted: stop now, before arming a timer or building the request.
    if (this._signal !== undefined) {
      throwIfRelayerHttpAborted({
        signal: this._signal,
        timedOutAfterMs: () => undefined,
        url: this._url,
        logger: this._logger,
        during: ' before the request was sent',
      });
    }

    // -------------------------------------------------------------------------
    // 2. Combine the caller's signal with options.timeout
    // -------------------------------------------------------------------------

    const combined = createRelayerHttpSignal(this._signal, this._timeoutMs);

    // -------------------------------------------------------------------------
    // 3. Build the request init once, then run the HTTP attempts
    // -------------------------------------------------------------------------

    // An abort before settlement wins over the loop's outcome:
    //
    //   loop ──► result ──► aborted? ──► yes ──► throw the abort ──┐
    //    │                     └──► no ──► return result           │
    //    └──► error ◄──────────────────────────────────────────────┘
    //           └──► already the abort? ──► yes ──► rethrow it
    //                     └──► no ──► aborted? ──► yes ──► throw the abort
    //                                    └──► no ──► rethrow the error
    //
    // The build stays inside the try: JSON.stringify (e.g. a BigInt payload) and
    // setAuth (an invalid header) can throw, and cleanup() must still run.
    // Until the init is built, nothing was sent: the abort warn says so.
    let during = ' before the request was sent';
    try {
      // 3.1. Build the request init once (headers, body, auth)
      const init = this._buildInit();
      during = ' after the response was received';

      // 3.2. Run HTTP attempts until one succeeds or the error is final
      // BY DESIGN, not raced against the signal: a global fetch that ignores the abort leaves
      // run() pending (native fetch never does; the timer and listener are still released).
      const result = await runRelayerHttpRetryLoop({
        runAttempt: () => this._createAttempt(init, combined).run(),
        // Called through `this`, so a subclass override is the one used.
        decide: (error, attempt) => this._retryDecision(error, attempt),
        signal: combined.signal,
        timedOutAfterMs: combined.timedOutAfterMs,
        remainingMs: combined.remainingMs,
        url: this._url,
        logger: this._logger,
      });

      // -----------------------------------------------------------------------
      // 4. Let an abort before settlement win over the result
      // -----------------------------------------------------------------------

      this._throwIfAbortedAtExit(combined, during);
      return result;
    } catch (error) {
      // -----------------------------------------------------------------------
      // 5. Let an abort before settlement win over the error
      // -----------------------------------------------------------------------

      // The abort itself (e.g. thrown by step 4) passes through: same instance, no second warn.
      if (!_isAbortOutcome(error, combined.signal)) {
        this._throwIfAbortedAtExit(combined, during);
      }
      throw error;
    } finally {
      combined.cleanup();
    }
  }

  /**
   * The single retry decision, called after each relayer-http error response.
   * Override in a subclass; the default never retries.
   *
   * @param _error - The attempt's relayer-http error.
   * @param _attempt - The 1-based number of HTTP attempts already made.
   * @returns The delay in ms before the next attempt (an integer in [1, MAX_TIMEOUT_MS]),
   * or undefined to stop.
   *
   * @example
   * ```typescript
   * class RetryOnce<O extends RelayerHttpOperation> extends RelayerHttpRequest<O> {
   *   protected override _retryDecision(error: RelayerHttpResponseError, attempt: number): number | undefined {
   *     return attempt === 1 ? 1000 : undefined;
   *   }
   * }
   * ```
   */
  protected _retryDecision(_error: RelayerHttpResponseError, _attempt: number): number | undefined {
    return undefined;
  }

  /**
   * Throws the abort if the combined signal was aborted before run() settled; returns otherwise.
   *
   * @param combined - The run's combined signal.
   * @param during - The warn suffix: ' before the request was sent' or ' after the response was received'.
   *
   * @example
   * ```typescript
   * this._throwIfAbortedAtExit(combined, during); // throws RelayerHttpTimeoutError if our timer fired
   * ```
   */
  private _throwIfAbortedAtExit(combined: RelayerHttpSignal, during: string): void {
    throwIfRelayerHttpAborted({
      signal: combined.signal,
      timedOutAfterMs: combined.timedOutAfterMs,
      url: this._url,
      logger: this._logger,
      during,
    });
  }

  // Normalized headers + content-type + sdk headers, the JSON body (serialized once) and auth.
  private _buildInit(): Readonly<Omit<RequestInit, 'signal'>> {
    return setAuth(
      {
        method: 'POST',
        headers: {
          ...this._headers,
          'content-type': 'application/json',
          'zama-sdk-version': version,
          'zama-sdk-name': sdkName,
        },
        body: JSON.stringify(this._payload),
      } satisfies RequestInit,
      this._auth,
    );
  }

  // One fresh attempt from the stored values, the shared init and the combined signal.
  private _createAttempt(
    init: Readonly<Omit<RequestInit, 'signal'>>,
    combined: RelayerHttpSignal,
  ): RelayerHttpAttempt<O> {
    return new RelayerHttpAttempt({
      relayerOperation: this._relayerOperation,
      url: this._url,
      query: this._query,
      init,
      signal: combined.signal,
      timedOutAfterMs: combined.timedOutAfterMs,
      fetchRetries: this._fetchRetries,
      fetchRetryDelayInMilliseconds: this._fetchRetryDelayInMilliseconds,
      logger: this._logger,
    });
  }
}

/**
 * Tells whether `error` already is the classified abort of `signal`.
 *
 * @param error - The value the loop rejected with.
 * @param signal - The run's combined signal.
 * @returns True if `error` is the abort reason itself or our RelayerHttpTimeoutError.
 *
 * @example
 * ```typescript
 * _isAbortOutcome(signal.reason, signal); // true
 * _isAbortOutcome(new Error('boom'), signal); // false
 * ```
 */
function _isAbortOutcome(error: unknown, signal: AbortSignal): boolean {
  if (!signal.aborted) {
    return false;
  }
  // Object.is, not ===: a NaN reason must still match itself.
  return Object.is(error, signal.reason) || error instanceof RelayerHttpTimeoutError;
}

/**
 * Asserts that `value` is one of the relayer-http operations this client targets.
 *
 * @param value - The `relayerOperation` passed to the constructor.
 * @returns Nothing; narrows `value` to RelayerHttpOperation on success.
 * @throws {InvalidPropertyError} Unless `value` is `'USER_DECRYPT'` or `'PUBLIC_DECRYPT'`.
 *
 * @example
 * ```typescript
 * _assertRelayerHttpOperation('USER_DECRYPT'); // ok
 * _assertRelayerHttpOperation('INPUT_PROOF'); // throws
 * ```
 */
function _assertRelayerHttpOperation(value: unknown): asserts value is RelayerHttpOperation {
  if (value === 'USER_DECRYPT' || value === 'PUBLIC_DECRYPT') {
    return;
  }
  throw new InvalidPropertyError(
    {
      subject: 'RelayerHttpRequestParams',
      property: 'relayerOperation',
      expectedType: 'string',
      value: value as string,
      expectedValue: 'USER_DECRYPT | PUBLIC_DECRYPT',
    },
    {},
  );
}

/**
 * Returns `timeout` unchanged if it is a whole number of milliseconds that
 * setTimeout honors as-is (1 to {@link MAX_TIMEOUT_MS}), or undefined if unset.
 *
 * @param timeout - `options.timeout`, in milliseconds.
 * @returns The validated timeout, or undefined when no timeout is set.
 * @throws {InvalidPropertyError} If `timeout` is set but out of range or not an integer.
 *
 * @example
 * ```typescript
 * _validateTimeoutMs(5000); // 5000
 * _validateTimeoutMs(0); // throws
 * ```
 */
function _validateTimeoutMs(timeout: number | undefined): number | undefined {
  if (timeout === undefined) {
    return undefined;
  }
  if (!isUintNumber(timeout, MAX_TIMEOUT_MS) || timeout < 1) {
    throw new InvalidPropertyError(
      {
        subject: 'options',
        property: 'timeout',
        expectedType: `integer from 1 to ${MAX_TIMEOUT_MS} (ms)`,
        type: 'number',
        value: String(timeout),
      },
      {},
    );
  }
  return timeout;
}
