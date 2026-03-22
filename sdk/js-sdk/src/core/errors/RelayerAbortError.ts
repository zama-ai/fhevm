import type { Prettify } from "../types/utils.js";
import type { RelayerRequestErrorBaseParams } from "./RelayerRequestErrorBase.js";
import { RelayerRequestErrorBase } from "./RelayerRequestErrorBase.js";
import { humanReadableOperation } from "./RelayerErrorBase.js";

////////////////////////////////////////////////////////////////////////////////
// RelayerAbortError
////////////////////////////////////////////////////////////////////////////////

export type RelayerAbortErrorType = RelayerAbortError & {
  name: "RelayerAbortError";
};

export type RelayerAbortErrorParams = Prettify<
  Omit<RelayerRequestErrorBaseParams, "name" | "message">
>;

/**
 * Request was aborted.
 */
export class RelayerAbortError extends RelayerRequestErrorBase {
  constructor(params: RelayerAbortErrorParams) {
    super({
      ...params,
      name: "RelayerAbortError",
      message: `${humanReadableOperation(params.operation, true)}: Request aborted`,
    });
  }
}
