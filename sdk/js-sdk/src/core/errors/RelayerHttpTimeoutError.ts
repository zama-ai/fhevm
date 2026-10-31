import type { RelayerErrorBaseParams } from './RelayerErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { RelayerErrorBase } from './RelayerErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerHttpTimeoutError
////////////////////////////////////////////////////////////////////////////////

export type RelayerHttpTimeoutErrorType = RelayerHttpTimeoutError & {
  name: 'RelayerHttpTimeoutError';
};

export type RelayerHttpTimeoutErrorParams = Prettify<
  Omit<RelayerErrorBaseParams, 'cause' | 'name' | 'message'> & {
    readonly url: string;
    readonly timeoutMs: number;
  }
>;

/**
 * `options.timeout` elapsed before the request completed. Distinct from a user
 * cancellation ({@link RelayerHttpFetchError}'s plain `AbortError` propagation) —
 * this is a policy decision this client made, not something the caller requested.
 */
export class RelayerHttpTimeoutError extends RelayerErrorBase {
  readonly #url: string;
  readonly #timeoutMs: number;

  constructor({ url, timeoutMs, ...params }: RelayerHttpTimeoutErrorParams) {
    super({
      ...params,
      name: 'RelayerHttpTimeoutError',
      message: `relayer-http request timed out after ${timeoutMs}ms.`,
      metaMessages: [...(params.metaMessages ?? []), `url: ${url}`, `timeoutMs: ${timeoutMs}`],
    });
    this.#url = url;
    this.#timeoutMs = timeoutMs;
  }

  public get url(): string {
    return this.#url;
  }

  public get timeoutMs(): number {
    return this.#timeoutMs;
  }
}
