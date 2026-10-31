import type { RelayerErrorBaseParams } from './RelayerErrorBase.js';
import type { Prettify } from '../types/utils.js';
import type { InvalidPropertyError } from '../base/errors/InvalidPropertyError.js';
import { RelayerErrorBase } from './RelayerErrorBase.js';
import { ensureError } from '../base/errors/utils.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerHttpResultError
////////////////////////////////////////////////////////////////////////////////

export type RelayerHttpResultErrorType = RelayerHttpResultError & {
  name: 'RelayerHttpResultError';
};

export type RelayerHttpResultErrorParams = Prettify<
  Omit<RelayerErrorBaseParams, 'cause' | 'name' | 'message'> & {
    readonly url: string;
    readonly cause: InvalidPropertyError;
    /** A snippet of the raw (successful, status-ok) body that failed validation. */
    readonly bodyJson?: string | undefined;
  }
>;

/**
 * relayer-http answered `200` with a well-formed JSON body, but its `result`
 * doesn't match the shape this client expects (missing/wrong-typed field, wrong
 * hex length, empty array, etc.) — a protocol/version mismatch or a relayer-http
 * bug, not a network, parsing, or relayer-http-reported error. The underlying
 * {@link InvalidPropertyError} (from the field-level guards) is always the `cause`.
 */
export class RelayerHttpResultError extends RelayerErrorBase {
  readonly #url: string;
  readonly #bodyJson: string | undefined;

  constructor({ url, cause, bodyJson, ...params }: RelayerHttpResultErrorParams) {
    super({
      ...params,
      name: 'RelayerHttpResultError',
      message: 'relayer-http response body does not match the expected schema.',
      cause: ensureError(cause),
      metaMessages: [
        ...(params.metaMessages ?? []),
        `url: ${url}`,
        ...(bodyJson !== undefined ? [`bodyJson: ${bodyJson}`] : []),
      ],
    });
    this.#url = url;
    this.#bodyJson = bodyJson;
  }

  public get url(): string {
    return this.#url;
  }

  public get bodyJson(): string | undefined {
    return this.#bodyJson;
  }
}
