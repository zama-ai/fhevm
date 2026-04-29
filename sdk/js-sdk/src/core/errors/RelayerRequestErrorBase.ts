import type { RelayerErrorBaseParams } from './RelayerErrorBase.js';
import type { Prettify } from '../types/utils.js';
import type { RelayerOperation } from '../types/relayer-p.js';
import { RelayerErrorBase } from './RelayerErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// RelayerRequestErrorBase
////////////////////////////////////////////////////////////////////////////////

export type RelayerRequestErrorBaseType = RelayerRequestErrorBase & {
  name: 'RelayerRequestErrorBase';
};

export type RelayerRequestErrorBaseParams = Prettify<
  RelayerErrorBaseParams & {
    readonly url: string;
    readonly operation: RelayerOperation;
    readonly jobId?: string | undefined;
  }
>;

export abstract class RelayerRequestErrorBase extends RelayerErrorBase {
  readonly #url: string;
  readonly #operation: RelayerOperation;
  readonly #jobId?: string | undefined;

  constructor(params: RelayerRequestErrorBaseParams) {
    const metaMessages = [
      ...(params.metaMessages ?? []),
      `url: ${params.url}`,
      `operation: ${params.operation}`,
      ...(params.jobId !== undefined ? [`jobId: ${params.jobId}`] : []),
    ];

    super({
      ...params,
      metaMessages,
    });

    this.#url = params.url;
    this.#operation = params.operation;
    this.#jobId = params.jobId;
  }

  public get url(): string {
    return this.#url;
  }

  public get jobId(): string | undefined {
    return this.#jobId;
  }

  public get operation(): RelayerOperation {
    return this.#operation;
  }
}
