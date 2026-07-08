import {
  setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem";

import type { NetworkConfig } from "./types";

/**
 * Configures global FHEVM SDK runtime knobs before client creation.
 *
 * `ZAMA_FHEVM_API_KEY` is applied as SDK auth when present.
 */
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
