import type { RelayerErrorBaseParams } from './RelayerErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { RelayerErrorBase } from './RelayerErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerHttpResponseError
////////////////////////////////////////////////////////////////////////////////

/**
 * Error `code` values relayer-http is known to return today (its own route-level
 * codes, plus codes forwarded verbatim from a KMS connector node). This list is
 * NOT exhaustive and never will be: the connector protocol is explicitly designed
 * to grow it over time (it has its own `unknown` fallback for codes a given
 * connector-api version doesn't recognize), so a new code must never be a type
 * error here — only ever missing from autocomplete until this list is refreshed.
 */
export const KNOWN_RELAYER_HTTP_ERROR_CODES = [
  'malformed',
  'not_found',
  'method_not_allowed',
  'unsupported_attestation_type',
  'sender_authentication_failed',
  'auth_expired',
  'acl_denied',
  'user_signature_rejected',
  'user_decryption_expired',
  'ciphertext_not_found',
  'kms_context_destroyed',
  'kms_context_invalid',
  'unprocessable',
  'rate_limited',
  'copro_consensus_failed',
  'upstream_transient',
  'overloaded',
  'timeout',
  'internal',
  'shutting_down',
] as const;

export type KnownRelayerHttpErrorCode = (typeof KNOWN_RELAYER_HTTP_ERROR_CODES)[number];

/**
 * `code` is deliberately loose: `KnownRelayerHttpErrorCode` gives autocomplete and
 * docs for the codes above, but `(string & {})` keeps the type open so a code
 * relayer-http introduces later is still assignable without a type error — the
 * standard TypeScript trick for "documented, not closed" unions.
 */
// eslint-disable-next-line @typescript-eslint/ban-types
export type RelayerHttpErrorCode = KnownRelayerHttpErrorCode | (string & {});

const RETRYABLE_STATUSES = new Set([429, 502, 503, 504]);

export type RelayerHttpResponseErrorType = RelayerHttpResponseError & {
  name: 'RelayerHttpResponseError';
};

export type RelayerHttpResponseErrorParams = Prettify<
  Omit<RelayerErrorBaseParams, 'cause' | 'name' | 'message'> & {
    readonly url: string;
    readonly status: number;
    /**
     * The server's own `code`, forwarded verbatim. `undefined` means the response
     * body had no `code` at all (a malformed/unexpected error body) — distinct
     * from the server explicitly returning its own `'unknown'` code.
     */
    readonly code?: RelayerHttpErrorCode | undefined;
    /** The server's own `message`, forwarded verbatim; `undefined` if absent. */
    readonly responseMessage?: string | undefined;
    /** From the body's `requestId`, falling back to the `x-request-id` header. */
    readonly requestId?: string | undefined;
  }
>;

/**
 * A relayer-http request answered with a non-2xx status and a well-formed
 * `{ code, message, requestId }` error body. For a response that isn't even
 * JSON (e.g. an edge proxy's HTML error page), see {@link RelayerHttpFetchError}
 * instead — this class assumes the body parsed.
 *
 * `code` is an open, server-defined set (see {@link KnownRelayerHttpErrorCode}),
 * not a small fixed enum: branch on `code` for specific cases, or on
 * {@link RelayerHttpResponseError.isRetryable} for a stable retry/no-retry split
 * that doesn't need updating every time a new `code` is introduced.
 */
export class RelayerHttpResponseError extends RelayerErrorBase {
  readonly #url: string;
  readonly #status: number;
  readonly #code: RelayerHttpErrorCode | undefined;
  readonly #responseMessage: string | undefined;
  readonly #requestId: string | undefined;

  constructor({ url, status, code, responseMessage, requestId, ...params }: RelayerHttpResponseErrorParams) {
    const message = [
      `relayer-http request failed (status=${status}${code !== undefined ? `, code=${code}` : ''}): ${responseMessage ?? 'no message in response body'}`,
      ...(requestId !== undefined ? [`requestId: ${requestId}`] : []),
    ].join(' — ');

    super({
      ...params,
      name: 'RelayerHttpResponseError',
      message,
      metaMessages: [
        ...(params.metaMessages ?? []),
        `url: ${url}`,
        `status: ${status}`,
        ...(code !== undefined ? [`code: ${code}`] : []),
        ...(requestId !== undefined ? [`requestId: ${requestId}`] : []),
      ],
    });
    this.#url = url;
    this.#status = status;
    this.#code = code;
    this.#responseMessage = responseMessage;
    this.#requestId = requestId;
  }

  public get url(): string {
    return this.#url;
  }

  public get status(): number {
    return this.#status;
  }

  public get code(): RelayerHttpErrorCode | undefined {
    return this.#code;
  }

  public get responseMessage(): string | undefined {
    return this.#responseMessage;
  }

  public get requestId(): string | undefined {
    return this.#requestId;
  }

  /**
   * Whether retrying the same request might succeed — derived from `status`
   * alone (429, 502, 503, 504), since `status` is a small set the SDK already
   * models, unlike the open-ended `code`. Stays correct as new codes appear.
   */
  public get isRetryable(): boolean {
    return RETRYABLE_STATUSES.has(this.#status);
  }
}
