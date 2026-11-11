import type { RelayerErrorBaseParams } from './RelayerErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { RelayerErrorBase } from './RelayerErrorBase.js';
import { ensureError } from '../base/errors/utils.js';
import { formatFetchErrorMetaMessages } from '../base/fetch.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerHttpFetchError
////////////////////////////////////////////////////////////////////////////////

export type RelayerHttpFetchErrorType = RelayerHttpFetchError & {
  name: 'RelayerHttpFetchError';
};

export type RelayerHttpFetchErrorParams = Prettify<
  Omit<RelayerErrorBaseParams, 'cause' | 'name'> & {
    readonly url: string;
    readonly cause?: unknown;
    readonly message: string;
    /** The response's HTTP status, when a response was actually received. */
    readonly status?: number | undefined;
    /** The response's `content-type` header, when a response was actually received. */
    readonly contentType?: string | undefined;
    /**
     * A snippet of the raw response body, when the body could not be parsed as JSON
     * (e.g. an edge proxy's HTML error page). Callers should already have truncated
     * this to a reasonable length before passing it in.
     */
    readonly body?: string | undefined;
  }
>;

/**
 * A relayer-http request failed: a network error occurred, or the response body
 * could not be parsed as JSON. Unlike {@link RelayerFetchError}, this carries no
 * job-queue state (no retryCount/elapsed/jobId) — relayer-http is a one-shot
 * synchronous request, not a polled async job.
 */
export class RelayerHttpFetchError extends RelayerErrorBase {
  readonly #url: string;
  readonly #status: number | undefined;
  readonly #contentType: string | undefined;
  readonly #body: string | undefined;

  constructor({ cause, url, status, contentType, body, ...params }: RelayerHttpFetchErrorParams) {
    const metaMessages = [
      ...(params.metaMessages ?? []),
      `url: ${url}`,
      ...(status !== undefined ? [`status: ${status}`] : []),
      ...(contentType !== undefined ? [`contentType: ${contentType}`] : []),
      ...(body !== undefined ? [`body: ${body}`] : []),
      ...(cause !== undefined ? formatFetchErrorMetaMessages(cause) : []),
    ];
    super({
      ...params,
      name: 'RelayerHttpFetchError',
      message: params.message,
      metaMessages,
      ...(cause !== undefined ? { cause: ensureError(cause) } : {}),
    });
    this.#url = url;
    this.#status = status;
    this.#contentType = contentType;
    this.#body = body;
  }

  public get url(): string {
    return this.#url;
  }

  public get status(): number | undefined {
    return this.#status;
  }

  public get contentType(): string | undefined {
    return this.#contentType;
  }

  public get body(): string | undefined {
    return this.#body;
  }
}
