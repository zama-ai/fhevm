import type { Logger } from '../../../types/logger.js';
import type {
  FetchPublicDecryptResult,
  FetchResultOf,
  FetchUserDecryptResult,
  RelayerPostOperation,
} from '../../../types/relayer.js';
import { fetchWithRetry } from '../../../base/fetch.js';
import { isNonEmptyString, isRecordStringProperty, assertRecordStringProperty } from '../../../base/string.js';
import { isUuidV7 } from '../../../base/uuidv7.js';
import {
  assertRecordBytesHexNo0xArrayProperty,
  assertRecordBytesHexNo0xProperty,
  assertRecordBytesHexProperty,
} from '../../../base/bytes.js';
import { assertRecordArrayProperty, assertRecordNonNullableProperty } from '../../../base/record.js';
import { InvalidPropertyError } from '../../../base/errors/InvalidPropertyError.js';
import { assertExtraDataFieldOnResultItems } from './guards/RelayerUserDecryptSucceeded.js';
import { appendRelayerQuery } from './relayerUrl.js';
import { throwIfRelayerHttpAborted } from './relayerHttpSignal.js';
import { RelayerHttpFetchError } from '../../../errors/RelayerHttpFetchError.js';
import { RelayerHttpResponseError } from '../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../errors/RelayerHttpUnexpectedResponseError.js';
import { RelayerHttpResultError } from '../../../errors/RelayerHttpResultError.js';

// One HTTP attempt against relayer-http: fetch (with network retries) → read → size check
// → parse → status/shape → validate. Stateless: the owner (RelayerHttpRequest) holds the
// signal, timer and cleanup; this class only uses the signal it is given.

// relayer-http nests user-decrypt shares like the existing relayer (`result.result`),
// but `RelayerUserDecryptSucceeded` tolerates v11's missing extraData and skips the
// UUIDv7/65-byte/non-empty checks — so only its lower-level field assertions are reused.

// The relayer-http routes this client targets.
export type RelayerHttpOperation = Extract<RelayerPostOperation, 'USER_DECRYPT' | 'PUBLIC_DECRYPT'>;

// Largest response body parsed, in characters (UTF-16 code units, not bytes), so
// roughly 10 MB for relayer-http's ASCII JSON. Checked after reading, before JSON.parse.
const MAX_BODY_TEXT_LENGTH = 10 * 1024 * 1024;

/** @internal Already-validated values from the owner; no raw options, payload or auth. */
export type RelayerHttpAttemptParams<O extends RelayerHttpOperation> = {
  readonly relayerOperation: O;
  // Query-less: used in errors and logs.
  readonly url: string;
  // Appended inside the fetch try, so an invalid query is a RelayerHttpFetchError.
  readonly query: Readonly<Record<string, string>> | undefined;
  // Built once by the owner; string body.
  readonly init: Readonly<Omit<RequestInit, 'signal'>>;
  // Attached after spreading init: one signal by construction.
  readonly signal: AbortSignal;
  readonly timedOutAfterMs: () => number | undefined;
  readonly fetchRetries: number | undefined;
  readonly fetchRetryDelayInMilliseconds: number | undefined;
  readonly logger: Logger | undefined;
};

/** @internal One HTTP attempt. No timer, listener, cleanup or one-shot guard. */
export class RelayerHttpAttempt<O extends RelayerHttpOperation> {
  private readonly _params: RelayerHttpAttemptParams<O>;

  constructor(params: RelayerHttpAttemptParams<O>) {
    this._params = params;
  }

  // A relayer-http error body is thrown unlogged: the retry loop logs the retry or the stop.
  public async run(): Promise<FetchResultOf<O>> {
    // -------------------------------------------------------------------------
    // 1. Fetch with network-level retry
    // -------------------------------------------------------------------------

    const response = await this._fetch();

    // -------------------------------------------------------------------------
    // 2. Read and parse the response body as JSON
    // -------------------------------------------------------------------------

    const bodyText = await this._readBody(response);
    const bodyJson = this._parseBody(response, bodyText);

    // -------------------------------------------------------------------------
    // 3. Validate the status and shape, then return the result
    // -------------------------------------------------------------------------

    if (!response.ok) {
      this._throwNonOk(response, bodyText, bodyJson);
    }
    return this._validateResult(bodyJson, bodyText);
  }

  // One fetchWithRetry call on the shared init plus the run's signal.
  private async _fetch(): Promise<Response> {
    const params = this._params;
    try {
      // undefined falls back to fetchWithRetry's own defaults (3 retries, 1000ms delay),
      // matching RelayerAsyncRequest's FETCH_RETRY/FETCH_RETRY_AFTER_MS.
      return await fetchWithRetry({
        url: appendRelayerQuery(params.url, params.query),
        init: { ...params.init, signal: params.signal },
        retries: params.fetchRetries,
        retryDelayMs: params.fetchRetryDelayInMilliseconds,
        logger: params.logger,
      });
    } catch (cause) {
      // An aborted request is our timeout or the caller's cancellation, never a network failure.
      this._throwIfAborted('');

      params.logger?.error?.(`[RelayerHttpRequest] Fetch failed on ${params.url}.`, cause);

      throw new RelayerHttpFetchError({
        url: params.url,
        cause,
        message: 'relayer-http request failed (network error).',
      });
    }
  }

  // Reads the body as text (not response.json()) so a non-JSON body can still be reported.
  private async _readBody(response: Response): Promise<string> {
    const params = this._params;
    try {
      // TODO(SECURITY): NO SIZE LIMIT WHILE READING THE RESPONSE BODY. response.text()
      // buffers the whole body in memory; _parseBody only keeps an oversized body
      // from being parsed, and options.timeout bounds time, not bytes.
      // Fix: read response.body as a stream, reject a content-length above a cap before
      // reading, and stop with RelayerHttpFetchError once the cap is exceeded.
      return await response.text();
    } catch (cause) {
      // Headers arrived but the body can still fail: dropped connection, timeout or abort.
      this._throwIfAborted(' while reading the response body');

      params.logger?.error?.(`[RelayerHttpRequest] Failed to read the response body on ${params.url}.`, cause);

      throw new RelayerHttpFetchError({
        url: params.url,
        cause,
        status: response.status,
        message: 'relayer-http request failed (could not read response body).',
      });
    }
  }

  // By design, a non-JSON body is a RelayerHttpFetchError whatever the status (e.g. a
  // proxy's 502 HTML page): RelayerHttpUnexpectedResponseError is only for non-ok JSON.
  private _parseBody(response: Response, bodyText: string): unknown {
    const params = this._params;

    // 2.1. Reject an oversized body first: the parsed value can take several times its memory.
    if (bodyText.length > MAX_BODY_TEXT_LENGTH) {
      params.logger?.error?.(
        `[RelayerHttpRequest] Response body on ${params.url} is too large to parse (${bodyText.length} characters, status=${response.status}).`,
        undefined,
      );

      throw new RelayerHttpFetchError({
        url: params.url,
        status: response.status,
        contentType: response.headers.get('content-type') ?? undefined,
        body: bodyText.slice(0, 500),
        message: `relayer-http response body too large (${bodyText.length} characters, limit ${MAX_BODY_TEXT_LENGTH}).`,
      });
    }

    // 2.2. Parse the text as JSON (HTML error pages, empty or truncated bodies are not JSON).
    try {
      return JSON.parse(bodyText) as unknown;
    } catch (cause) {
      params.logger?.error?.(
        `[RelayerHttpRequest] Response body on ${params.url} is not valid JSON (status=${response.status}).`,
        cause,
      );

      throw new RelayerHttpFetchError({
        url: params.url,
        cause,
        status: response.status,
        contentType: response.headers.get('content-type') ?? undefined,
        body: bodyText.slice(0, 500),
        message: 'Failed to parse relayer-http response body as JSON.',
      });
    }
  }

  // Non-ok: a relayer-http error body throws RelayerHttpResponseError unlogged;
  // any other body is logged and thrown as RelayerHttpUnexpectedResponseError.
  private _throwNonOk(response: Response, bodyText: string, bodyJson: unknown): never {
    const params = this._params;

    // relayer-http's own ErrorBody always carries all three fields:
    //
    //   { "code": "acl_denied", "message": "...", "requestId": "<uuidv7>" }
    //
    // Thrown unlogged: the retry loop logs either the retry or the final error.
    if (_isRelayerHttpErrorBody(bodyJson)) {
      throw new RelayerHttpResponseError({
        url: params.url,
        status: response.status,
        code: bodyJson.code,
        responseMessage: bodyJson.message,
        requestId: bodyJson.requestId,
      });
    }

    // Not relayer-http's error contract: most likely an intermediary (gateway, proxy, load balancer).
    params.logger?.error?.(
      `[RelayerHttpRequest] Unexpected non-relayer-http error response on ${params.url} (status=${response.status}).`,
      undefined,
    );

    throw new RelayerHttpUnexpectedResponseError({
      url: params.url,
      status: response.status,
      contentType: response.headers.get('content-type') ?? undefined,
      body: bodyText.slice(0, 500),
    });
  }

  // Each assert is wrapped so a schema mismatch surfaces as a typed RelayerHttpResultError,
  // not the guard helpers' raw InvalidPropertyError.
  private _validateResult(bodyJson: unknown, bodyText: string): FetchResultOf<O> {
    const relayerOperation: RelayerHttpOperation = this._params.relayerOperation;
    switch (relayerOperation) {
      case 'USER_DECRYPT': {
        try {
          _assertIsRelayerHttpUserDecryptResult(bodyJson, 'body');
        } catch (cause) {
          throw this._resultError(cause, bodyText);
        }
        return bodyJson.result.result as FetchResultOf<O>;
      }
      case 'PUBLIC_DECRYPT': {
        try {
          _assertIsRelayerHttpPublicDecryptResult(bodyJson, 'body');
        } catch (cause) {
          throw this._resultError(cause, bodyText);
        }
        return bodyJson.result as FetchResultOf<O>;
      }
      default:
        // Compile-time exhaustiveness, plus a defensive runtime throw.
        relayerOperation satisfies never;
        throw new Error(`Unknown relayer operation: ${String(relayerOperation)}`);
    }
  }

  // Logs a success body that failed its schema check and wraps the guard's error.
  private _resultError(cause: unknown, bodyText: string): RelayerHttpResultError {
    const params = this._params;
    params.logger?.error?.(
      `[RelayerHttpRequest] Response body on ${params.url} does not match the expected schema.`,
      cause,
    );
    return new RelayerHttpResultError({
      url: params.url,
      cause: cause as InvalidPropertyError,
      // By design, the raw text, not the parsed JSON, despite the field name.
      bodyJson: bodyText.slice(0, 500),
    });
  }

  // Classifies an abort of this attempt's signal: our timer or the caller's.
  private _throwIfAborted(during: string): void {
    const params = this._params;
    throwIfRelayerHttpAborted({
      signal: params.signal,
      timedOutAfterMs: params.timedOutAfterMs,
      url: params.url,
      logger: params.logger,
      during,
    });
  }
}

/**
 * Whether `value` matches relayer-http's own error body:
 *
 * ```json
 * { "code": "acl_denied", "message": "...", "requestId": "<uuidv7>" }
 * ```
 *
 * relayer-http always sets all three fields, so a body missing any of them (or
 * not an object at all, e.g. JSON `null`) did not come from relayer-http itself.
 *
 * @param value - The parsed response body; may be any JSON value.
 * @returns `true` when `code` and `message` are non-empty strings and
 * `requestId` is a UUIDv7.
 *
 * @example
 * ```typescript
 * if (_isRelayerHttpErrorBody(bodyJson)) {
 *   console.log(bodyJson.code);
 * }
 * ```
 */
function _isRelayerHttpErrorBody(value: unknown): value is { code: string; message: string; requestId: string } {
  return (
    isRecordStringProperty(value, 'code') &&
    isNonEmptyString(value.code) &&
    isRecordStringProperty(value, 'message') &&
    isNonEmptyString(value.message) &&
    isRecordStringProperty(value, 'requestId') &&
    isUuidV7(value.requestId)
  );
}

/**
 * Asserts that `value` carries relayer-http's success envelope, shared by every
 * route: `status` is `"succeeded"` and `requestId` is a UUIDv7.
 *
 * @param value - The parsed response body to validate.
 * @param name - The subject name used in a thrown error, e.g. `'body'`.
 * @returns Nothing; narrows `value`'s type on success.
 * @throws If `status` isn't `"succeeded"` or `requestId` isn't a UUIDv7.
 *
 * @example
 * ```typescript
 * _assertIsRelayerHttpSucceeded(bodyJson, 'body');
 * console.log(bodyJson.requestId);
 * ```
 */
function _assertIsRelayerHttpSucceeded(
  value: unknown,
  name: string,
): asserts value is { status: 'succeeded'; requestId: string } {
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
}

/**
 * Asserts that `value` matches relayer-http's `POST /v4/user-decrypt` success body:
 *
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "<uuidv7>",
 *   "result": {
 *     "result": [
 *       {
 *          "payload": "<hex, no 0x>",
 *          "signature": "<65-byte hex, no 0x>",
 *          "extraData": "<hex, with 0x>"
 *       }
 *     ]
 *   }
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
 * is missing or null, `value.result.result` is missing, not an array, empty, or any
 * item's `payload`, `signature`, or `extraData` does not match the expected hex shape.
 *
 * @example
 * ```typescript
 * _assertIsRelayerHttpUserDecryptResult(bodyJson, 'body');
 * const shares = bodyJson.result.result;
 * ```
 */
function _assertIsRelayerHttpUserDecryptResult(
  value: unknown,
  name: string,
): asserts value is { result: { result: FetchUserDecryptResult } } {
  _assertIsRelayerHttpSucceeded(value, name);

  assertRecordNonNullableProperty(value, 'result', name, {});
  const result: unknown = value.result;
  const resultName = `${name}.result`;
  assertRecordArrayProperty(result, 'result', resultName, {});
  const shares = result.result;
  // A 200 success with zero shares is anomalous: the aggregator only returns
  // success after reaching its threshold, so at least one share is expected.
  if (shares.length === 0) {
    throw new InvalidPropertyError(
      {
        subject: resultName,
        property: 'result',
        expectedType: 'non-empty array',
        type: 'array',
        value: '[]',
      },
      {},
    );
  }
  for (let i = 0; i < shares.length; ++i) {
    const item: unknown = shares[i];
    const itemName = `${resultName}.result[${i}]`;
    assertRecordBytesHexNo0xProperty(item, 'payload', itemName, {});
    assertRecordBytesHexNo0xProperty(item, 'signature', itemName, { byteLength: 65 });
  }
  // relayer-http always includes extraData on every share (UserDecryptShare.extra_data
  // is a required field, not Option<...>) — no v11-style legacy omission to tolerate here.
  assertExtraDataFieldOnResultItems(shares, resultName, /* required */ true, {});
}

/**
 * Asserts that `value` matches relayer-http's `POST /v4/public-decrypt` success body:
 *
 * ```json
 * {
 *   "status": "succeeded",
 *   "requestId": "<uuidv7>",
 *   "result": {
 *     "decryptedValue": "<hex, no 0x>",
 *     "signatures": ["<65-byte hex, no 0x>", ...],
 *     "extraData": "<hex, with 0x>"
 *   }
 * }
 * ```
 *
 * @param value - The parsed response body to validate.
 * @param name - The subject name used in a thrown error, e.g. `'body'`.
 * @returns Nothing; narrows `value`'s type on success.
 * @throws If the envelope is invalid, `value.result` is missing or null, or its
 * `decryptedValue`, `signatures` (non-empty), or `extraData` does not match the
 * expected hex shape.
 *
 * @example
 * ```typescript
 * _assertIsRelayerHttpPublicDecryptResult(bodyJson, 'body');
 * const { decryptedValue } = bodyJson.result;
 * ```
 */
function _assertIsRelayerHttpPublicDecryptResult(
  value: unknown,
  name: string,
): asserts value is { result: FetchPublicDecryptResult } {
  _assertIsRelayerHttpSucceeded(value, name);

  assertRecordNonNullableProperty(value, 'result', name, {});
  const result: unknown = value.result;
  const resultName = `${name}.result`;
  assertRecordBytesHexNo0xProperty(result, 'decryptedValue', resultName, {});
  assertRecordBytesHexNo0xArrayProperty(result, 'signatures', resultName, { byteLength: 65 });
  // Success is only returned once the KMS threshold is reached, so at least one signature.
  if (result.signatures.length === 0) {
    throw new InvalidPropertyError(
      {
        subject: resultName,
        property: 'signatures',
        expectedType: 'non-empty array',
        type: 'array',
        value: '[]',
      },
      {},
    );
  }
  // relayer-http always includes extraData (PublicDecryptOutput.extra_data is a String, not Option<...>).
  assertRecordBytesHexProperty(result, 'extraData', resultName, {});
}
