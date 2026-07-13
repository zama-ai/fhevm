import {
  setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem";

/**
 * Configures global FHEVM SDK runtime knobs before client creation.
 *
 * `ZAMA_FHEVM_API_KEY` is applied as SDK auth when present.
 */
export const configureFhevmRuntime = (): void => {
  const apiKey = process.env.ZAMA_FHEVM_API_KEY;
  setFhevmRuntimeConfig({
    singleThread: true,
    moduleVersions: "auto",
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
