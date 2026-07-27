import { describe, expect, test } from "bun:test";

import { signerHandleFromListObjects, signerHandleFromLogs, waitForRpc } from "./flow/readiness";

describe("signerHandleFromListObjects", () => {
  test("finds a persisted centralized KMS signing-key handle", () => {
    const handle = "60b7070add74be3827160aa635fb255eeeeb88586c4debf7ab1134ddceb4beee";
    const xml = `<ListBucketResult><Contents><Key>PUB/VerfAddress/${handle}</Key></Contents></ListBucketResult>`;

    expect(signerHandleFromListObjects(xml, "PUB")).toBe(handle);
    expect(signerHandleFromListObjects(xml, "PUB/PUB")).toBeNull();
  });

  test("does not mistake a generated CRS handle for the signing-key handle", () => {
    const crsHandle = "05".padEnd(64, "0");
    const signerHandle = "60b7070add74be3827160aa635fb255eeeeb88586c4debf7ab1134ddceb4beee";
    const logs = [
      `Successfully stored public data element [CRS] under the handle ${crsHandle}`,
      `Checking key PUB/SigningKey/${signerHandle}`,
    ].join("\n");

    expect(signerHandleFromLogs(logs)).toBe(signerHandle);
    expect(signerHandleFromLogs(`stored under the handle ${crsHandle}`)).toBeNull();
  });
});

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
