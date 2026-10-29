import { describe, expect, test } from "bun:test";
import path from "node:path";

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
    ).toBe(false);
    expect(
      isDockerRegistryTransient(
        'failed to resolve reference "ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker:abc1234": not found',
      ),
    ).toBe(false);
  });

  test("keeps the zama-ai/kms token out of the environment that child processes inherit", () => {
    // A fresh process, because the token is read when the module loads.
    const module = JSON.stringify(path.join(import.meta.dir, "flow/runtime-compose.ts"));
    const child = Bun.spawnSync(
      ["bun", "--eval", `await import(${module}); console.log(process.env.BLOCKCHAIN_ACTIONS_TOKEN ?? "unset");`],
      { env: { ...process.env, BLOCKCHAIN_ACTIONS_TOKEN: "test-token" } },
    );
    expect(child.stdout.toString().trim()).toBe("unset");
  });
});
