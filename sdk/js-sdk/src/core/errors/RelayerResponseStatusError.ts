import type { Prettify } from '../types/utils.js';
import type { RelayerAsyncRequestState } from '../types/relayer-p.js';
import type { RelayerResponseErrorBaseParams } from './RelayerResponseErrorBase.js';
import { humanReadableOperation } from './RelayerErrorBase.js';
import { RelayerResponseErrorBase } from './RelayerResponseErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerResponseStatusError
////////////////////////////////////////////////////////////////////////////////

export type RelayerResponseStatusErrorType = RelayerResponseStatusError & {
  name: 'RelayerResponseStatusError';
};

export type RelayerResponseStatusErrorParams = Prettify<
  Omit<RelayerResponseErrorBaseParams, 'message' | 'name' | 'details'> & {
    readonly state: RelayerAsyncRequestState;
    /**
     * Optional message surfaced from the response body — e.g. the JSON error a
     * Cloudflare/Kong edge proxy returns on a 403. Appended to the error
     * message and details when present.
     */
    readonly responseMessage?: string | undefined;
  }
>;

/**
 * The response status is unexpected.
 */
export class RelayerResponseStatusError extends RelayerResponseErrorBase {
  constructor({ responseMessage, ...params }: RelayerResponseStatusErrorParams) {
    const baseMessage = `${humanReadableOperation(params.operation, true)}: Relayer returned unexpected response status: ${params.status}`;
    super({
      ...params,
      name: 'RelayerResponseStatusError',
      message: responseMessage !== undefined ? `${baseMessage}: ${responseMessage}` : baseMessage,
      details:
        responseMessage !== undefined
          ? `The Relayer server returned an unexpected response status (${params.status}): ${responseMessage}`
          : `The Relayer server returned an unexpected response status (${params.status}). This status ${params.status} is not part of the expected API contract and may indicate a server configuration issue.`,
    });
  }
}
