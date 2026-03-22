import type { RelayerRequestErrorBaseParams } from "./RelayerRequestErrorBase.js";
import type { Prettify } from "../types/utils.js";
import { RelayerRequestErrorBase } from "./RelayerRequestErrorBase.js";
import { humanReadableOperation } from "./RelayerErrorBase.js";

////////////////////////////////////////////////////////////////////////////////
// RelayerRequestInternalError
////////////////////////////////////////////////////////////////////////////////

export type RelayerInternalRequestErrorType = RelayerRequestInternalError & {
  name: "RelayerRequestInternalError";
};

export type RelayerInternalRequestErrorParams = Prettify<
  Omit<RelayerRequestErrorBaseParams, "name"> & {
    readonly status?: number;
    readonly state?: string;
  }
>;

/**
 * Internal error
 */
export class RelayerRequestInternalError extends RelayerRequestErrorBase {
  readonly #status?: number | undefined;
  readonly #state?: string | undefined;

  constructor(params: RelayerInternalRequestErrorParams) {
    const metaMessages = [
      ...(params.metaMessages ?? []),
      ...(params.status !== undefined ? [`status: ${params.status}`] : []),
      ...(params.state !== undefined ? [`state: ${params.state}`] : []),
    ];

    super({
      ...params,
      metaMessages,
      name: "RelayerRequestInternalError",
      message:
        params.message ??
        `${humanReadableOperation(params.operation, true)}: Relayer SDK internal error`,
    });
    this.#status = params.status;
    this.#state = params.state;
  }

  public get status(): number | undefined {
    return this.#status;
  }

  public get state(): string | undefined {
    return this.#state;
  }
}
