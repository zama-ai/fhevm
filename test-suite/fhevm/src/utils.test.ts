import { describe, expect, test } from "bun:test";

import { hostReachableMaterialUrl, hostReachableRpcUrl } from "./utils/fs";

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
});
