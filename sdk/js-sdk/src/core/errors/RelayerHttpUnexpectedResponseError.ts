import type { RelayerErrorBaseParams } from './RelayerErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { RelayerErrorBase } from './RelayerErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerHttpUnexpectedResponseError
////////////////////////////////////////////////////////////////////////////////

export type RelayerHttpUnexpectedResponseErrorType = RelayerHttpUnexpectedResponseError & {
  name: 'RelayerHttpUnexpectedResponseError';
};

export type RelayerHttpUnexpectedResponseErrorParams = Prettify<
  Omit<RelayerErrorBaseParams, 'cause' | 'name' | 'message'> & {
    readonly url: string;
    readonly status: number;
    /** The response's `content-type` header, when present. */
    readonly contentType?: string | undefined;
    /** A snippet of the raw response body (callers should already have truncated it). */
    readonly body?: string | undefined;
  }
>;

/**
 * A relayer-http request answered with a non-2xx status, and the (valid JSON)
 * response body does NOT match relayer-http's own `{ code, message, requestId }`
 * error contract — all three fields are always present when relayer-http (or a
 * KMS connector error it forwards) produces the response, so a body missing any
 * of them did not come from relayer-http itself. It most likely came from
 * something in front of it on the network path (an edge gateway/API proxy
 * rejecting the request, a load balancer returning its own error for an
 * unreachable backend, etc.) — see {@link RelayerHttpResponseError} for the
 * genuine relayer-http/KMS error case.
 */
export class RelayerHttpUnexpectedResponseError extends RelayerErrorBase {
  readonly #url: string;
  readonly #status: number;
  readonly #contentType: string | undefined;
  readonly #body: string | undefined;

  constructor({ url, status, contentType, body, ...params }: RelayerHttpUnexpectedResponseErrorParams) {
    super({
      ...params,
      name: 'RelayerHttpUnexpectedResponseError',
      message: `Unexpected non-relayer-http error response (status=${status}) — likely an intermediary (proxy, gateway, load balancer) rather than relayer-http itself.`,
      metaMessages: [
        ...(params.metaMessages ?? []),
        `url: ${url}`,
        `status: ${status}`,
        ...(contentType !== undefined ? [`contentType: ${contentType}`] : []),
        ...(body !== undefined ? [`body: ${body}`] : []),
      ],
    });
    this.#url = url;
    this.#status = status;
    this.#contentType = contentType;
    this.#body = body;
  }

  public get url(): string {
    return this.#url;
  }

  public get status(): number {
    return this.#status;
  }

  public get contentType(): string | undefined {
    return this.#contentType;
  }

  public get body(): string | undefined {
    return this.#body;
  }
}
