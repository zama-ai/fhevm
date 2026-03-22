import type { Prettify } from "../types/utils.js";
import type { RelayerAsyncRequestState } from "../types/relayer-p.js";
import type { RelayerResponseErrorBaseParams } from "./RelayerResponseErrorBase.js";
import { humanReadableOperation } from "./RelayerErrorBase.js";
import { RelayerResponseErrorBase } from "./RelayerResponseErrorBase.js";

////////////////////////////////////////////////////////////////////////////////
// RelayerResponseStatusError
////////////////////////////////////////////////////////////////////////////////

export type RelayerResponseStatusErrorType = RelayerResponseStatusError & {
  name: "RelayerResponseStatusError";
};

export type RelayerResponseStatusErrorParams = Prettify<
  Omit<RelayerResponseErrorBaseParams, "message" | "name" | "details"> & {
    readonly state: RelayerAsyncRequestState;
  }
>;

/**
 * The response status is unexpected.
 */
export class RelayerResponseStatusError extends RelayerResponseErrorBase {
  constructor(params: RelayerResponseStatusErrorParams) {
    super({
      ...params,
      name: "RelayerResponseStatusError",
      message: `${humanReadableOperation(params.operation, true)}: Relayer returned unexpected response status: ${params.status}`,
      details: `The Relayer server returned an unexpected response status (${params.status}). This status ${params.status} is not part of the expected API contract and may indicate a server configuration issue.`,
    });
  }
}
