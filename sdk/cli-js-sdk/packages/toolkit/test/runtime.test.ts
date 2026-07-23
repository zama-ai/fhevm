import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  setFhevmRuntimeConfig: vi.fn(),
}));

vi.mock("@fhevm/sdk/viem", () => ({
  setFhevmRuntimeConfig: mocks.setFhevmRuntimeConfig,
}));

import { configureFhevmRuntime } from "../src/config/runtime";

const execFileAsync = promisify(execFile);
let previousApiKey: string | undefined;

beforeEach(() => {
  previousApiKey = process.env.ZAMA_FHEVM_API_KEY;
  delete process.env.ZAMA_FHEVM_API_KEY;
  vi.clearAllMocks();
});

afterEach(() => {
  if (previousApiKey === undefined) delete process.env.ZAMA_FHEVM_API_KEY;
  else process.env.ZAMA_FHEVM_API_KEY = previousApiKey;
});

describe("configureFhevmRuntime", () => {
  it("uses one explicit auto-version policy", () => {
    configureFhevmRuntime();

    expect(mocks.setFhevmRuntimeConfig).toHaveBeenCalledWith({
      singleThread: true,
      moduleVersions: "auto",
    });
  });

  it("preserves API-key auth with the global auto-version policy", () => {
    process.env.ZAMA_FHEVM_API_KEY = "secret";
    configureFhevmRuntime();

    expect(mocks.setFhevmRuntimeConfig).toHaveBeenCalledWith({
      singleThread: true,
      moduleVersions: "auto",
      auth: { type: "ApiKeyHeader", value: "secret" },
    });
  });

  it("accepts the repeated global auto policy despite immutable SDK runtime configuration", async () => {
    const script = `
      import { setFhevmRuntimeConfig } from "@fhevm/sdk/viem";
      for (let index = 0; index < 4; index += 1) {
        setFhevmRuntimeConfig({ singleThread: true, moduleVersions: "auto" });
      }
    `;

    await expect(
      execFileAsync(
        process.execPath,
        ["--input-type=module", "--eval", script],
        {
          cwd: new URL("..", import.meta.url),
          env: { ...process.env, ZAMA_FHEVM_API_KEY: undefined },
        },
      ),
    ).resolves.toMatchObject({ stderr: "" });
  });
});
