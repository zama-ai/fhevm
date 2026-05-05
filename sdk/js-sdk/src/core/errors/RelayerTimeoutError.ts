import type { RelayerRequestErrorBaseParams } from './RelayerRequestErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { RelayerRequestErrorBase } from './RelayerRequestErrorBase.js';
import { humanReadableOperation } from './RelayerErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerTimeoutError
////////////////////////////////////////////////////////////////////////////////

export type RelayerTimeoutErrorType = RelayerTimeoutError & {
  name: 'RelayerTimeoutError';
};

export type RelayerTimeoutErrorParams = Prettify<
  Omit<RelayerRequestErrorBaseParams, 'message' | 'name'> & {
    readonly timeoutMs: number;
  }
>;

/**
 * The request timed out. (Global)
 */
export class RelayerTimeoutError extends RelayerRequestErrorBase {
  constructor(params: RelayerTimeoutErrorParams) {
    super({
      ...params,
      name: 'RelayerTimeoutError',
      message: `${humanReadableOperation(params.operation, true)}: Request timed out after ${params.timeoutMs}ms`,
    });
  }
}
