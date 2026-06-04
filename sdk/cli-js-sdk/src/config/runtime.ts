import {
  setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem";

import type { NetworkConfig } from "./types";

export const configureFhevmRuntime = (networkConfig: NetworkConfig): void => {
  const apiKey = process.env.ZAMA_FHEVM_API_KEY;
  setFhevmRuntimeConfig({
    singleThread: true,
    moduleVersions: networkConfig.runtime?.moduleVersions,
    ...(apiKey
      ? {
          auth: {
            type: "ApiKeyHeader",
            value: apiKey,
          },
        }
      : {}),
  });
};
