import type { RelayerErrorBaseParams } from "./RelayerErrorBase.js";
import type { Prettify } from "../types/utils.js";
import { RelayerErrorBase } from "./RelayerErrorBase.js";
import type {
  RelayerAsyncRequestState,
  RelayerOperation,
} from "../types/relayer-p.js";

////////////////////////////////////////////////////////////////////////////////
// RelayerFetchErrorBase
////////////////////////////////////////////////////////////////////////////////

export type RelayerFetchErrorBaseType = RelayerFetchErrorBase & {
  name: "RelayerFetchErrorBase";
};

export type RelayerFetchErrorBaseParams = Prettify<
  RelayerErrorBaseParams & {
    readonly fetchMethod: "GET" | "POST";
    readonly url: string;
    readonly operation: RelayerOperation;
    readonly retryCount: number;
    readonly elapsed: number;
    readonly state?: RelayerAsyncRequestState;
    readonly jobId?: string | undefined;
  }
>;

export abstract class RelayerFetchErrorBase extends RelayerErrorBase {
  readonly #fetchMethod: "POST" | "GET";
  readonly #url: string;
  readonly #jobId: string | undefined;
  readonly #operation: RelayerOperation;
  readonly #retryCount: number;
  readonly #elapsed: number;
  readonly #state: RelayerAsyncRequestState | undefined;

  constructor(params: RelayerFetchErrorBaseParams) {
    const metaMessages = [
      ...(params.metaMessages ?? []),
      `url: ${params.url}`,
      `method: ${params.fetchMethod}`,
      `operation: ${params.operation}`,
      `retryCount: ${params.retryCount}`,
      ...(params.jobId !== undefined ? [`jobId: ${params.jobId}`] : []),
    ];
    super({
      ...params,
      metaMessages,
      name: params.name,
    });
    this.#fetchMethod = params.fetchMethod;
    this.#url = params.url;
    this.#operation = params.operation;
    this.#elapsed = params.elapsed;
    this.#retryCount = params.retryCount;
    this.#state = params.state;
    this.#jobId = params.jobId;
  }

  public get url(): string {
    return this.#url;
  }

  public get operation(): RelayerOperation {
    return this.#operation;
  }

  public get fetchMethod(): "POST" | "GET" {
    return this.#fetchMethod;
  }

  public get jobId(): string | undefined {
    return this.#jobId;
  }

  public get retryCount(): number {
    return this.#retryCount;
  }

  public get elapsed(): number {
    return this.#elapsed;
  }

  public get state(): RelayerAsyncRequestState | undefined {
    return this.#state;
  }

  public get isAbort(): boolean {
    // AbortError is not an instance of Error!
    return this.cause !== undefined
      ? (this.cause as { name: string }).name === "AbortError"
      : false;
  }
}
