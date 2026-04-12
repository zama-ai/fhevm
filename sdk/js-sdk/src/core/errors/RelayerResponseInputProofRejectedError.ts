import type { Prettify } from '../types/utils.js';
import type { RelayerResponseErrorBaseParams } from './RelayerResponseErrorBase.js';
import { RelayerResponseErrorBase } from './RelayerResponseErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerResponseInputProofRejectedError
////////////////////////////////////////////////////////////////////////////////

export type RelayerResponseInputProofRejectedErrorType =
  RelayerResponseInputProofRejectedError & {
    name: 'RelayerResponseInputProofRejectedError';
  };

export type RelayerResponseInputProofRejectedErrorParams = Prettify<
  Omit<RelayerResponseErrorBaseParams, 'name' | 'message'>
>;

/**
 * The input proof is rejected.
 */
export class RelayerResponseInputProofRejectedError extends RelayerResponseErrorBase {
  constructor(params: RelayerResponseInputProofRejectedErrorParams) {
    super({
      ...params,
      name: 'RelayerResponseInputProofRejectedError',
      message: `Relayer rejected the input proof`,
    });
  }
}
