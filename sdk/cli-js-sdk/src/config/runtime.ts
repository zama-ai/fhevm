import {
  hasFhevmRuntimeConfig,
  setFhevmRuntimeConfig,
} from "@fhevm/sdk/viem";

export const configureFhevmRuntime = (): void => {
  if (hasFhevmRuntimeConfig()) return;

  const apiKey = process.env.ZAMA_FHEVM_API_KEY;
  setFhevmRuntimeConfig({
    singleThread: true,
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
