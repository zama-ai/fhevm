import type { RelayerFetchErrorBaseParams } from './RelayerFetchErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { ensureError } from '../base/errors/utils.js';
import { RelayerFetchErrorBase } from './RelayerFetchErrorBase.js';
import { formatFetchErrorMetaMessages } from '../base/fetch.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerFetchError
////////////////////////////////////////////////////////////////////////////////

export type RelayerFetchErrorType = RelayerFetchError & {
  name: 'RelayerFetchError';
};

export type RelayerFetchErrorParams = Prettify<
  Omit<RelayerFetchErrorBaseParams, 'cause' | 'name'> & {
    readonly cause?: unknown;
    readonly message: string;
  }
>;

/**
 * If a network error occurs or JSON parsing fails.
 */
export class RelayerFetchError extends RelayerFetchErrorBase {
  constructor({ cause, ...params }: RelayerFetchErrorParams) {
    super({
      ...params,
      name: 'RelayerFetchError',
      message: params.message,
      ...(cause !== undefined ? { cause: ensureError(cause) } : {}),
      ...(cause !== undefined
        ? { metaMessages: formatFetchErrorMetaMessages(cause) }
        : {}),
    });
  }
}
