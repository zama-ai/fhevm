import { describe, expect, test } from "bun:test";

import {
  backendSplit,
  ensureOneMaterial,
  parseSquashBackend,
  strayWorkerPids,
  waitForRpc,
} from "./flow/readiness";

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

describe("strayWorkerPids", () => {
  test("ignores the container processes this stack owns", () => {
    expect(strayWorkerPids(new Set([11, 22]), [11, 22])).toEqual([]);
  });

  test("reports a worker that no container of this stack accounts for", () => {
    // The GPU units' case: containers up, plus a host-run binary on the same queue.
    expect(strayWorkerPids(new Set([11, 22]), [11, 22, 953222])).toEqual([953222]);
  });

  test("reports every stray, not just the first", () => {
    expect(strayWorkerPids(new Set([11]), [953222, 11, 953223])).toEqual([953222, 953223]);
  });
});

describe("parseSquashBackend", () => {
  test("reads gpu_enabled out of the worker's JSON startup line", () => {
    const line = '{"timestamp":"2026-09-02T08:29:31Z","level":"INFO","fields":{"gpu_enabled":true}}';
    expect(parseSquashBackend(line)).toBe("true");
  });

  test("reads the CPU case", () => {
    expect(parseSquashBackend('"gpu_enabled":false,"other":1')).toBe("false");
  });

  test("tolerates a bare key=value form so the guard does not go quiet", () => {
    expect(parseSquashBackend("gpu_enabled=TRUE")).toBe("true");
  });

  test("returns undefined when the line is absent", () => {
    expect(parseSquashBackend("starting sns worker")).toBeUndefined();
  });
});

describe("backendSplit", () => {
  test("a homogeneous fleet is one group", () => {
    expect([...backendSplit(["false", "false", "false"]).keys()]).toEqual(["false"]);
  });

  test("groups operators by backend so the split can be named", () => {
    const split = backendSplit(["true", "false", "false"]);
    expect(split.get("true")).toEqual([0]);
    expect(split.get("false")).toEqual([1, 2]);
  });

  test("an unreadable backend is not evidence of a split", () => {
    // Older image without the startup line: one group, so no failure.
    expect(backendSplit(["false", undefined, "false"]).size).toBe(1);
  });
});
