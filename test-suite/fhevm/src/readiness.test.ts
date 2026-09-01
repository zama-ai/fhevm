import { describe, expect, test } from "bun:test";

import { ensureOneMaterial, waitForRpc } from "./flow/readiness";

describe("waitForRpc", () => {
  test("retries until eth_chainId returns a JSON-RPC result", async () => {
    const originalFetch = globalThis.fetch;
    let calls = 0;
    globalThis.fetch = (async () => {
      calls += 1;
      return new Response(
        JSON.stringify(
          calls === 1
            ? { jsonrpc: "2.0", id: 1, error: { code: -32000, message: "not ready" } }
            : { jsonrpc: "2.0", id: 1, result: "0x3039" },
        ),
        {
          status: 200,
          headers: { "content-type": "application/json" },
        },
      );
    }) as unknown as typeof fetch;
    try {
      await waitForRpc("http://localhost:8545");
      expect(calls).toBe(2);
    } finally {
      globalThis.fetch = originalFetch;
    }
  });
});

describe("ensureOneMaterial", () => {
  test("accepts a published compressed keyset without waiting for the legacy key path", async () => {
    const originalFetch = globalThis.fetch;
    const requested: string[] = [];
    globalThis.fetch = (async (input: string | URL | Request) => {
      const url = String(input);
      requested.push(url);
      return new Response(null, { status: url.includes("CompressedXofKeySet") ? 200 : 404 });
    }) as unknown as typeof fetch;
    try {
      await ensureOneMaterial([
        "http://minio:9000/kms-public/PUB/CompressedXofKeySet/key-id",
        "http://minio:9000/kms-public/PUB/ServerKey/key-id",
      ]);
      expect(requested).toHaveLength(2);
      expect(requested[0]).toContain("CompressedXofKeySet");
    } finally {
      globalThis.fetch = originalFetch;
    }
  });
});
