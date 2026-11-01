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
import { normalizeRelayerQuery } from './relayerUrl.js';
import { RelayerHttpAttempt } from './RelayerHttpAttempt.js';
import { runRelayerHttpRetryLoop } from './relayerHttpRetryLoop.js';
import { MAX_TIMEOUT_MS, createRelayerHttpSignal } from './relayerHttpSignal.js';

// One-shot request/response client for relayer-http's synchronous routes (e.g. POST
// /v4/user-decrypt): a single POST, a single JSON response, no job id, no polling —
// unlike RelayerAsyncRequest, which targets the existing relayer's async job-queue API.
// Owns the run's lifetime (combined signal, timer, cleanup); each HTTP attempt is a
// RelayerHttpAttempt, and _retryDecision alone decides whether another one follows.

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
    this._signal = params.options?.signal;
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

    // -------------------------------------------------------------------------
    // 2. Combine the caller's signal with options.timeout
    // -------------------------------------------------------------------------

    const combined = createRelayerHttpSignal(this._signal, this._timeoutMs);

    // -------------------------------------------------------------------------
    // 3. Build the request init once, then run the HTTP attempts
    // -------------------------------------------------------------------------

    // The build stays inside the try: JSON.stringify (e.g. a BigInt payload) and
    // setAuth (an invalid header) can throw, and cleanup() must still run.
    try {
      // 3.1. Build the request init once (headers, body, auth)
      const init = this._buildInit();

      // 3.2. Run HTTP attempts until one succeeds or the error is final
      return await runRelayerHttpRetryLoop({
        runAttempt: () => this._createAttempt(init, combined).run(),
        // Called through `this`, so a subclass override is the one used.
        decide: (error, attempt) => this._retryDecision(error, attempt),
        signal: combined.signal,
        timedOutAfterMs: combined.timedOutAfterMs,
        remainingMs: combined.remainingMs,
        url: this._url,
        logger: this._logger,
      });
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
