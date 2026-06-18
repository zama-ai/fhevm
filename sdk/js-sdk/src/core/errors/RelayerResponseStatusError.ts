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
    super({
      ...params,
      name: 'RelayerResponseStatusError',
      message: `${humanReadableOperation(params.operation, true)}: Relayer returned unexpected response status: ${params.status}`,
      // Surface the relayer/edge message (e.g. a Cloudflare/Kong 403 block) via
      // the `Details:` line when present; otherwise keep the generic
      // explanation. The primary message is unchanged for backward
      // compatibility.
      details:
        responseMessage ??
        `The Relayer server returned an unexpected response status (${params.status}). This status ${params.status} is not part of the expected API contract and may indicate a server configuration issue.`,
    });
  }
}
