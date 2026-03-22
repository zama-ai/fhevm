import type { RelayerFetchErrorBaseParams } from "./RelayerFetchErrorBase.js";
import type { Prettify } from "../types/utils.js";
import { RelayerFetchErrorBase } from "./RelayerFetchErrorBase.js";
import { humanReadableOperation } from "./RelayerErrorBase.js";

////////////////////////////////////////////////////////////////////////////////
// RelayerMaxRetryError
////////////////////////////////////////////////////////////////////////////////

export type RelayerMaxRetryErrorType = RelayerMaxRetryError & {
  name: "RelayerMaxRetryError";
};

export type RelayerMaxRetryErrorParams = Prettify<
  Omit<RelayerFetchErrorBaseParams, "name" | "message" | "details">
>;

/**
 * The maximum number of retries is exceeded.
 */
export class RelayerMaxRetryError extends RelayerFetchErrorBase {
  constructor(params: RelayerMaxRetryErrorParams) {
    super({
      ...params,
      name: "RelayerMaxRetryError",
      message: `${humanReadableOperation(params.operation, true)}: Maximum polling retry limit exceeded (${params.retryCount} attempts)`,
      details: `After ${params.retryCount} polling attempts, the retry limit was exceeded. The operation may still complete on the server - consider checking the result later.`,
    });
  }
}
