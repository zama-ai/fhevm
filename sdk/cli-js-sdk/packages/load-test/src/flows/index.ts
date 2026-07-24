import type { LoadTestEnv } from "../env";
import type { RelayerClient } from "../relayer/client";
import type { FlowKind } from "../relayer/types";
import type { FlowExecutor } from "./types";

export type { FlowExecutor, RequestOutcome, RequestRecord } from "./types";
export { PoolExhaustedError } from "./types";

export const createFlowExecutor = async (options: {
  flow: FlowKind;
  env: LoadTestEnv;
  client: RelayerClient;
  clientB?: RelayerClient;
  requestTimeoutMs: number;
  handlesPerRequest: number;
}): Promise<FlowExecutor> => {
  switch (options.flow) {
    case "input-proof": {
      const { InputProofExecutor } = await import("./input-proof");
      return new InputProofExecutor(
        options.env,
        options.client,
        options.clientB,
        options.requestTimeoutMs,
      );
    }
    case "public-decrypt": {
      const { PublicDecryptExecutor } = await import("./public-decrypt");
      return new PublicDecryptExecutor(
        options.env,
        options.client,
        options.clientB,
        options.requestTimeoutMs,
        options.handlesPerRequest,
      );
    }
    case "user-decrypt": {
      const { UserDecryptExecutor } = await import("./user-decrypt");
      return new UserDecryptExecutor(
        options.env,
        options.client,
        options.clientB,
        options.requestTimeoutMs,
        false,
      );
    }
    case "delegated-user-decrypt": {
      const { UserDecryptExecutor } = await import("./user-decrypt");
      return new UserDecryptExecutor(
        options.env,
        options.client,
        options.clientB,
        options.requestTimeoutMs,
        true,
      );
    }
  }
};
