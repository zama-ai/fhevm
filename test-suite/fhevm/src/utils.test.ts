import { describe, expect, test } from "bun:test";

import { hostReachableMaterialUrl, hostReachableRpcUrl, mergeArgs } from "./utils/fs";
import { run } from "./utils/process";

describe("utils/fs", () => {
  test("rewrites container rpc urls to localhost while preserving explicit ports", () => {
    expect(hostReachableRpcUrl("http://gateway-node:8546")).toBe("http://localhost:8546");
    expect(hostReachableRpcUrl("ws://host-node-chain-b:8547")).toBe("ws://localhost:8547");
    expect(hostReachableRpcUrl("https://example.com:8545")).toBe("https://example.com:8545");
  });

  test("rewrites minio urls to the external host endpoint", () => {
    expect(hostReachableMaterialUrl("http://minio:9000/kms-public/foo")).toBe("http://localhost:9000/kms-public/foo");
    expect(hostReachableMaterialUrl("http://10.0.0.5:9000/kms-public/foo")).toBe("http://localhost:9000/kms-public/foo");
    expect(hostReachableMaterialUrl("https://example.com:9001/kms-public/foo")).toBe("https://example.com:9001/kms-public/foo");
  });

  test("times out bounded process execution", async () => {
    await expect(run(["bun", "-e", "await new Promise((resolve) => setTimeout(resolve, 1000))"], { timeoutMs: 10 }))
      .rejects.toThrow(/timed out after 10ms/);
  });

  test("replaces split-form flags without leaving orphaned values", () => {
    expect(mergeArgs(["cmd", "--log-level", "info"], ["--log-level", "debug"])).toEqual([
      "cmd",
      "--log-level",
      "debug",
    ]);
  });
});
