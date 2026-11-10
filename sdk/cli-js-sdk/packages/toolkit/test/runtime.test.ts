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
  it("runs the SDK single-threaded", () => {
    configureFhevmRuntime();

    expect(mocks.setFhevmRuntimeConfig).toHaveBeenCalledWith({
      singleThread: true,
    });
  });

  it("passes the API key as SDK auth", () => {
    process.env.ZAMA_FHEVM_API_KEY = "secret";
    configureFhevmRuntime();

    expect(mocks.setFhevmRuntimeConfig).toHaveBeenCalledWith({
      singleThread: true,
      auth: { type: "ApiKeyHeader", value: "secret" },
    });
  });

  it("accepts repeated identical runtime configuration", async () => {
    const script = `
      import { setFhevmRuntimeConfig } from "@fhevm/sdk/viem";
      for (let index = 0; index < 4; index += 1) {
        setFhevmRuntimeConfig({ singleThread: true });
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
