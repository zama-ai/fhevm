// Unit cover for the historical-access proof client. It forks the vault module's public-proof
// client (which has its own tests) into historical mode, and the property worth pinning is the
// retry CLASSIFICATION: only a lagging store is transient. Every other 503 — a corrupt cache, an
// integrity failure — is terminal, and misclassifying one as transient would retry an integrity
// failure ten times and then report it as a timeout.

import { afterEach, describe, expect, test } from "bun:test";
import { address } from "@solana/kit";

import { decodeMmrProofTransportBlob, MAX_MMR_SIBLINGS, MMR_PROOF_MODE_HISTORICAL } from "@sdk-src/solana/proof.js";

import { fetchHistoricalAccessProof, type FheVerticalConfig } from "./fhe-vertical";

const CONFIG: FheVerticalConfig = {
  relayerUrl: "http://relayer:3000",
  proofServiceUrl: "http://proof:8080/",
  gatewayRpcUrl: "http://gateway:8546",
  chainId: 1n,
  publicDecryptContextId: `0x${"00".repeat(32)}`,
  userDecryptContextId: "0",
};

const ENCRYPTED_VALUE = address("11111111111111111111111111111111");
const SUBJECT = address("SysvarC1ock11111111111111111111111111111111");
const OLD_HANDLE = new Uint8Array(32).fill(0x92);

const params = { encryptedValue: ENCRYPTED_VALUE, oldHandle: OLD_HANDLE, subject: SUBJECT };

const jsonResponse = (status: number, body: unknown): Response =>
  new Response(JSON.stringify(body), { status, headers: { "content-type": "application/json" } });

const verifiedBody = (siblings: string[] = [], leafIndex = 0, leafCount = 1) => ({
  mmr_proof: { leaf_index: leafIndex, siblings },
  leaf_count: leafCount,
  verified: true,
  status: "verified",
});

const originalFetch = globalThis.fetch;
afterEach(() => {
  globalThis.fetch = originalFetch;
});

/** Installs a fetch stub returning `responses` in order, and reports how many calls were made. */
const stubFetch = (responses: Response[]): { calls: () => number; urls: string[] } => {
  const urls: string[] = [];
  let index = 0;
  globalThis.fetch = (async (input: RequestInfo | URL) => {
    urls.push(String(input));
    const response = responses[Math.min(index, responses.length - 1)]!;
    index += 1;
    return response;
  }) as typeof globalThis.fetch;
  return { calls: () => index, urls };
};

describe("fetchHistoricalAccessProof", () => {
  test("normalizes a verified proof and builds a 0x01 transport blob the SDK decoder accepts", async () => {
    const sibling = "ab".repeat(32);
    const { urls } = stubFetch([jsonResponse(200, verifiedBody([sibling], 3, 5))]);

    const proof = await fetchHistoricalAccessProof(CONFIG, params);

    expect(proof.leafIndex).toBe(3n);
    expect(proof.leafCount).toBe(5n);
    expect(proof.siblings).toHaveLength(1);
    const decoded = decodeMmrProofTransportBlob(proof.mmrProofBytes);
    expect(decoded.mode).toBe(MMR_PROOF_MODE_HISTORICAL);
    expect(decoded.proof.leafIndex).toBe(3n);
    expect(decoded.proof.siblings).toHaveLength(1);
    // The request keys on (encrypted_value, handle, subject) — the client never supplies a leaf index.
    expect(urls[0]).toContain(`encrypted_value=${ENCRYPTED_VALUE}`);
    expect(urls[0]).toContain(`subject=${SUBJECT}`);
    expect(urls[0]).not.toContain("leaf_index");
  });

  test("retries a lagging store and succeeds once it catches up", async () => {
    const { calls } = stubFetch([
      jsonResponse(503, { status: "lagging" }),
      jsonResponse(200, verifiedBody()),
    ]);

    const proof = await fetchHistoricalAccessProof(CONFIG, params);

    expect(proof.leafCount).toBe(1n);
    expect(calls()).toBe(2);
  });

  // The mutation this file exists for: dropping the `lagging` check makes every 503 retryable, so
  // an integrity failure is swallowed by the retry loop and reported as a timeout much later.
  test("treats a non-lagging 503 as terminal and does not retry it", async () => {
    const { calls } = stubFetch([jsonResponse(503, { status: "corrupt_cache" })]);

    await expect(fetchHistoricalAccessProof(CONFIG, params)).rejects.toThrow(/corrupt_cache/);
    expect(calls()).toBe(1);
  });

  test("does not retry a 404 or a 500", async () => {
    const notFound = stubFetch([jsonResponse(404, { status: "leaf_not_found" })]);
    await expect(fetchHistoricalAccessProof(CONFIG, params)).rejects.toThrow(/HTTP 404/);
    expect(notFound.calls()).toBe(1);

    const serverError = stubFetch([jsonResponse(500, { status: "integrity" })]);
    await expect(fetchHistoricalAccessProof(CONFIG, params)).rejects.toThrow(/HTTP 500/);
    expect(serverError.calls()).toBe(1);
  });

  test("rejects an unverified proof even on a 200", async () => {
    stubFetch([jsonResponse(200, { mmr_proof: null, leaf_count: 1, verified: false, status: "unverified" })]);
    await expect(fetchHistoricalAccessProof(CONFIG, params)).rejects.toThrow(/unverified/);
  });

  test("rejects a sibling that is not 32 bytes", async () => {
    stubFetch([jsonResponse(200, verifiedBody(["ab".repeat(31)]))]);
    await expect(fetchHistoricalAccessProof(CONFIG, params)).rejects.toThrow(/must be 32 bytes/);
  });

  // The cap the vault client enforces and the pre-fix fork omitted; the on-chain verifier rejects
  // past it, so a client that forwards an over-long path fails opaquely on-chain instead.
  test("enforces the MMR sibling cap", async () => {
    const siblings = Array.from({ length: MAX_MMR_SIBLINGS + 1 }, () => "cd".repeat(32));
    stubFetch([jsonResponse(200, verifiedBody(siblings))]);
    await expect(fetchHistoricalAccessProof(CONFIG, params)).rejects.toThrow(/exceeding the cap/);
  });
});
