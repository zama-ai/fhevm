import type { Auth } from '../../../types/auth.js';
import type { Logger } from '../../../types/logger.js';
import type {
  FetchUserDecryptResult,
  RelayerPublicDecryptOptions,
  RelayerUserDecryptOptions,
} from '../../../types/relayer.js';
import { sdkName, version } from '../../../_version.js';
import { setAuth } from '../../../base/auth.js';
import { fetchWithRetry, normalizeHeaders } from '../../../base/fetch.js';
import { isNonEmptyString, assertRecordStringProperty } from '../../../base/string.js';
import { isUuidV7 } from '../../../base/uuidv7.js';
import { assertRecordBytesHexNo0xProperty } from '../../../base/bytes.js';
import { assertRecordArrayProperty } from '../../../base/record.js';
import { InvalidPropertyError } from '../../../base/errors/InvalidPropertyError.js';
import { assertExtraDataFieldOnResultItems } from './guards/RelayerUserDecryptSucceeded.js';
import { appendRelayerQuery, normalizeRelayerQuery } from './relayerUrl.js';
import { RelayerHttpFetchError } from '../../../errors/RelayerHttpFetchError.js';
import { RelayerHttpResponseError } from '../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../errors/RelayerHttpUnexpectedResponseError.js';
import { RelayerHttpResultError } from '../../../errors/RelayerHttpResultError.js';
import { RelayerHttpTimeoutError } from '../../../errors/RelayerHttpTimeoutError.js';

// One-shot request/response client for relayer-http's synchronous routes (e.g. POST
// /v4/user-decrypt): a single POST, a single JSON response, no job id, no polling —
// unlike RelayerAsyncRequest, which targets the existing relayer's async job-queue API.

// relayer-http's success envelope is flat: `result` is directly the array of shares
// (`{ status, requestId, result: [{ payload, signature, extraData }, ...] }`), unlike
// the existing relayer's `RelayerUserDecryptSucceeded`, which nests a second `result`
// one level deeper. So the existing guard can't be reused as-is — reusing its
// lower-level field assertions instead.

type RelayerHttpRequestParams = {
  url: string;
  payload: Record<string, unknown>;
  // onProgress is ignored: this is a single-shot request, not a polled job, so
  // there's no intermediate progress to report.
  options?:
    | Omit<RelayerUserDecryptOptions, 'onProgress'>
    | Omit<RelayerPublicDecryptOptions, 'onProgress'>
    | undefined;
  logger?: Logger | undefined;
};

export class RelayerHttpRequest {
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
  private _hasRun: boolean;

  constructor(params: RelayerHttpRequestParams) {
    this._url = params.url;
    this._payload = params.payload;
    this._auth = params.options?.auth;
    this._headers = normalizeHeaders(params.options?.headers);
    this._query = normalizeRelayerQuery(params.options?.query);
    this._signal = params.options?.signal;
    this._timeoutMs = params.options?.timeout;
    this._fetchRetries = params.options?.fetchRetries;
    this._fetchRetryDelayInMilliseconds = params.options?.fetchRetryDelayInMilliseconds;
    this._logger = params.logger;
    this._hasRun = false;
  }

  public async run(): Promise<FetchUserDecryptResult> {
    if (this._hasRun) {
      throw new Error(
        'RelayerHttpRequest.run() was already called on this instance. Create a new instance per request.',
      );
    }
    this._hasRun = true;

    // -------------------------------------------------------------------------
    // 1. Build the request init (headers, body, auth, abort signal)
    // -------------------------------------------------------------------------

    const { signal, cleanup } = this._createCombinedSignal();

    // The build stays inside the try: JSON.stringify (e.g. a BigInt payload) and
    // setAuth (an invalid header) can throw, and cleanup() must still run.
    try {
      const init = setAuth(
        {
          method: 'POST',
          headers: {
            ...this._headers,
            'content-type': 'application/json',
            'zama-sdk-version': version,
            'zama-sdk-name': sdkName,
          },
          body: JSON.stringify(this._payload),
          signal,
        } satisfies RequestInit,
        this._auth,
      );

      return await this._runAfterInit(init);
    } finally {
      cleanup();
    }
  }

  /**
   * Combines the caller's own signal (if any) with `options.timeout` into one
   * internal signal, the same pattern RelayerAsyncRequest uses: never pass the
   * caller's signal straight to fetch, so a timeout can abort independently of
   * whether the caller passed a signal at all.
   *
   * @returns `signal` to pass as `init.signal`, and `cleanup` to call once the
   * request settles (success or failure) so neither the timer nor the listener
   * on the caller's own signal lingers.
   */
  private _createCombinedSignal(): { signal: AbortSignal; cleanup: () => void } {
    const internalController = new AbortController();
    const externalSignal = this._signal;
    const onExternalAbort = (): void => {
      internalController.abort(externalSignal?.reason);
    };
    if (externalSignal?.aborted === true) {
      internalController.abort(externalSignal.reason);
    } else {
      externalSignal?.addEventListener('abort', onExternalAbort, { once: true });
    }
    const timeoutId =
      this._timeoutMs !== undefined
        ? setTimeout(() => {
            internalController.abort(new DOMException('Timed out', 'TimeoutError'));
          }, this._timeoutMs)
        : undefined;
    const cleanup = (): void => {
      if (timeoutId !== undefined) {
        clearTimeout(timeoutId);
      }
      externalSignal?.removeEventListener('abort', onExternalAbort);
    };

    return { signal: internalController.signal, cleanup };
  }

  // Everything after the request init is built. Split out so `run()` can wrap it
  // in a try/finally that always clears the timeout timer, success or failure.
  private async _runAfterInit(init: RequestInit): Promise<FetchUserDecryptResult> {
    // -------------------------------------------------------------------------
    // 2. Fetch with network-level retry
    // -------------------------------------------------------------------------

    let response: Response;
    try {
      // undefined falls back to fetchWithRetry's own defaults (3 retries, 1000ms delay),
      // matching RelayerAsyncRequest's FETCH_RETRY/FETCH_RETRY_AFTER_MS.
      response = await fetchWithRetry({
        url: appendRelayerQuery(this._url, this._query),
        init,
        retries: this._fetchRetries,
        retryDelayMs: this._fetchRetryDelayInMilliseconds,
        logger: this._logger,
      });
    } catch (cause) {
      // Only ours to claim if we actually set up a timer — a 'TimeoutError' from
      // anywhere else isn't one we can attach a meaningful timeoutMs to.
      if (this._timeoutMs !== undefined && (cause as { name?: string }).name === 'TimeoutError') {
        this._logger?.warn?.(`[RelayerHttpRequest] Request to ${this._url} timed out.`);
        throw new RelayerHttpTimeoutError({ url: this._url, timeoutMs: this._timeoutMs });
      }

      // AbortError is deliberate cancellation, not a network failure — propagate
      // it as-is rather than wrapping it here.
      if ((cause as { name?: string }).name === 'AbortError') {
        this._logger?.warn?.(`[RelayerHttpRequest] Request aborted on ${this._url}.`);
        throw cause;
      }

      this._logger?.error?.(`[RelayerHttpRequest] Fetch failed on ${this._url}.`, cause);

      throw new RelayerHttpFetchError({
        url: this._url,
        cause,
        message: 'relayer-http request failed (network error).',
      });
    }

    // -------------------------------------------------------------------------
    // 3. Parse the response body as JSON
    // -------------------------------------------------------------------------

    let bodyJson: { result?: unknown; code?: string; message?: string; requestId?: string };

    // 3.1. Read the body as text (not response.json()) so a non-JSON body (e.g. an
    // edge proxy's HTML error page) can still be reported in the error below.
    let bodyText: string;
    try {
      bodyText = await response.text();
    } catch (cause) {
      // fetch() resolving only means the headers arrived — reading the body can
      // still fail on its own (connection dropped mid-stream, the timeout firing
      // after headers but before the body finished, or the abort signal firing
      // in that same window).
      if (this._timeoutMs !== undefined && (cause as { name?: string }).name === 'TimeoutError') {
        this._logger?.warn?.(`[RelayerHttpRequest] Request to ${this._url} timed out while reading the response body.`);
        throw new RelayerHttpTimeoutError({ url: this._url, timeoutMs: this._timeoutMs });
      }

      if ((cause as { name?: string }).name === 'AbortError') {
        this._logger?.warn?.(`[RelayerHttpRequest] Request aborted on ${this._url} while reading the response body.`);
        throw cause;
      }

      this._logger?.error?.(`[RelayerHttpRequest] Failed to read the response body on ${this._url}.`, cause);

      throw new RelayerHttpFetchError({
        url: this._url,
        cause,
        status: response.status,
        message: 'relayer-http request failed (could not read response body).',
      });
    }

    // 3.2. Parse the text as JSON (e.g. an edge proxy's HTML error page, an empty
    // body, or a truncated response from a dropped connection are not valid JSON).
    try {
      bodyJson = JSON.parse(bodyText) as { result?: unknown; code?: string; message?: string; requestId?: string };
    } catch (cause) {
      this._logger?.error?.(
        `[RelayerHttpRequest] Response body on ${this._url} is not valid JSON (status=${response.status}).`,
        cause,
      );

      throw new RelayerHttpFetchError({
        url: this._url,
        cause,
        status: response.status,
        contentType: response.headers.get('content-type') ?? undefined,
        body: bodyText.slice(0, 500),
        message: 'Failed to parse relayer-http response body as JSON.',
      });
    }

    // -------------------------------------------------------------------------
    // 4. Validate the status and shape, then return the result
    // -------------------------------------------------------------------------

    if (!response.ok) {
      // relayer-http's own ErrorBody always carries all three fields — code, message
      // and requestId are never optional on its side:
      //
      //   {
      //     "code": "acl_denied",
      //     "message": "...",
      //     "requestId": "<uuidv7>"
      //   }
      //
      // A body missing any of them did not come from relayer-http itself: it's most
      // likely an intermediary (an edge gateway/API proxy, a load balancer) rejecting
      // the request before it ever reached relayer-http.
      if (isNonEmptyString(bodyJson.code) && isNonEmptyString(bodyJson.message) && isUuidV7(bodyJson.requestId)) {
        this._logger?.error?.(
          `[RelayerHttpRequest] relayer-http error on ${this._url} (status=${response.status}, code=${bodyJson.code}, requestId=${bodyJson.requestId}): ${bodyJson.message}`,
          undefined,
        );

        throw new RelayerHttpResponseError({
          url: this._url,
          status: response.status,
          code: bodyJson.code,
          responseMessage: bodyJson.message,
          requestId: bodyJson.requestId,
        });
      }

      // The body didn't match relayer-http's own error contract (checked above), so
      // it most likely came from something other than relayer-http itself.
      this._logger?.error?.(
        `[RelayerHttpRequest] Unexpected non-relayer-http error response on ${this._url} (status=${response.status}).`,
        undefined,
      );

      throw new RelayerHttpUnexpectedResponseError({
        url: this._url,
        status: response.status,
        contentType: response.headers.get('content-type') ?? undefined,
        body: bodyText.slice(0, 500),
      });
    }

    // {
    //   "status": "succeeded",
    //   "requestId": "<uuidv7>",
    //   "result": [
    //     {
    //        "payload": "<hex, no 0x>",
    //        "signature": "<65-byte hex, no 0x>",
    //        "extraData": "<hex, with 0x>"
    //     }
    //   ]
    // }
    //
    // Wrapped in try/catch so a schema mismatch (a well-formed, status-ok body that
    // still doesn't match the expected shape) is logged and surfaced as a typed
    // RelayerHttpResultError, consistent with every other failure mode above,
    // instead of leaking the raw InvalidPropertyError from the guard helpers.
    try {
      _assertIsRelayerHttpUserDecryptResult(bodyJson, 'body');
    } catch (cause) {
      this._logger?.error?.(
        `[RelayerHttpRequest] Response body on ${this._url} does not match the expected schema.`,
        cause,
      );
      throw new RelayerHttpResultError({
        url: this._url,
        cause: cause as InvalidPropertyError,
        bodyJson: bodyText.slice(0, 500),
      });
    }

    // Narrowed to { result: FetchUserDecryptResult } by the assert above.
    return bodyJson.result;
  }
}

/**
 * Asserts that `value` matches relayer-http's `POST /v4/user-decrypt` success body:
 *
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "<uuidv7>",
 *   "result": [
 *     {
 *        "payload": "<hex, no 0x>",
 *        "signature": "<65-byte hex, no 0x>",
 *        "extraData": "<hex, with 0x>"
 *     }
 *   ]
 * }
 * ```
 *
 * `status` and `requestId` are checked for minimal consistency with the same-origin
 * check already done on the error path ({@link RelayerHttpResponseError}) — not full
 * validation of the envelope.
 *
 * @param value - The parsed response body to validate.
 * @param name - The subject name used in a thrown error, e.g. `'body'`.
 * @returns Nothing; narrows `value`'s type on success.
 * @throws If `status` isn't `"succeeded"`, `requestId` isn't a UUIDv7, `value.result`
 * is missing, not an array, empty, or any item's `payload`, `signature`, or
 * `extraData` does not match the expected hex shape.
 */
function _assertIsRelayerHttpUserDecryptResult(
  value: unknown,
  name: string,
): asserts value is { result: FetchUserDecryptResult } {
  assertRecordStringProperty(value, 'status', name, { expectedValue: 'succeeded' });
  assertRecordStringProperty(value, 'requestId', name, {});
  if (!isUuidV7(value.requestId)) {
    throw new InvalidPropertyError(
      {
        subject: name,
        property: 'requestId',
        expectedType: 'UUIDv7',
        type: 'string',
        value: value.requestId,
      },
      {},
    );
  }

  assertRecordArrayProperty(value, 'result', name, {});
  // A 200 success with zero shares is anomalous: the aggregator only returns
  // success after reaching its threshold, so at least one share is expected.
  if (value.result.length === 0) {
    throw new InvalidPropertyError(
      {
        subject: name,
        property: 'result',
        expectedType: 'non-empty array',
        type: 'array',
        value: '[]',
      },
      {},
    );
  }
  for (let i = 0; i < value.result.length; ++i) {
    const item: unknown = value.result[i];
    const itemName = `${name}.result[${i}]`;
    assertRecordBytesHexNo0xProperty(item, 'payload', itemName, {});
    assertRecordBytesHexNo0xProperty(item, 'signature', itemName, { byteLength: 65 });
  }
  // relayer-http always includes extraData on every share (UserDecryptShare.extra_data
  // is a required field, not Option<...>) — no v11-style legacy omission to tolerate here.
  assertExtraDataFieldOnResultItems(value.result, name, /* required */ true, {});
}
