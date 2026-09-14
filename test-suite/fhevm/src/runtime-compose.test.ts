import { describe, expect, test } from "bun:test";

import { isDockerRegistryTransient } from "./flow/runtime-compose";

describe("runtime compose", () => {
  test("detects transient registry pull failures", () => {
    expect(
      isDockerRegistryTransient(
        'Error response from daemon: Get "https://ghcr.io/v2/": net/http: request canceled (Client.Timeout exceeded while awaiting headers)',
      ),
    ).toBe(true);
    expect(isDockerRegistryTransient("transaction reverted: InvalidNullThreshold")).toBe(false);
    expect(
      isDockerRegistryTransient(
        "Error response from daemon: unknown: ghcr.io/zama-ai/fhevm/relayer:abc1234: manifest unknown",
      ),
    ).toBe(true);
    expect(
      isDockerRegistryTransient(
        'failed to resolve reference "ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker:abc1234": not found',
      ),
    ).toBe(true);
  });
});
