import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  createClientContext: vi.fn(),
  openIfExists: vi.fn(),
}));

vi.mock("@cli-fhevm-sdk/toolkit/config", () => ({
  createClientContext: mocks.createClientContext,
}));
vi.mock("../src/pool/store", () => ({
  PoolStore: { openIfExists: mocks.openIfExists },
}));

import {
  PublicDecryptExecutor,
  publicValuesMatch,
} from "../src/flows/public-decrypt";

beforeEach(() => vi.clearAllMocks());

describe("publicValuesMatch", () => {
  it("matches SDK typed values against serialized pool values", () => {
    expect(
      publicValuesMatch(
        [
          { type: "bool", value: true },
          { type: "uint64", value: 42n },
        ],
        [
          { type: "bool", value: "true" },
          { type: "uint64", value: "42" },
        ],
      ),
    ).toBe(true);
  });

  it("rejects reordered, mistyped, or changed values", () => {
    expect(
      publicValuesMatch(
        [{ type: "uint64", value: 42n }],
        [{ type: "uint32", value: "42" }],
      ),
    ).toBe(false);
    expect(
      publicValuesMatch(
        [{ type: "uint64", value: 41n }],
        [{ type: "uint64", value: "42" }],
      ),
    ).toBe(false);
  });

  it("keeps POST provenance and rejects mismatched terminal SDK identity", async () => {
    const decryptPublicValuesWithSignatures = vi.fn().mockImplementation(
      async (parameters: Record<string, unknown>) => {
        const onProgress = (
          parameters.options as { onProgress: (value: unknown) => void }
        ).onProgress;
        onProgress({
          type: "queued", method: "POST", status: 202,
          requestId: "post-request", jobId: "post-job", retryAfterMs: 10,
          elapsed: 1, retryCount: 0,
        });
        onProgress({
          type: "queued", method: "GET", status: 202,
          requestId: "post-request", jobId: "post-job", retryAfterMs: 20,
          elapsed: 2, retryCount: 1,
        });
        onProgress({
          type: "succeeded", method: "GET", status: 200,
          requestId: "wrong-request", jobId: "post-job", elapsed: 3, retryCount: 2,
        });
        return { clearValues: [{ type: "uint64", value: 42n }] };
      },
    );
    mocks.createClientContext.mockReturnValue({
      fhevm: { decryptPublicValuesWithSignatures },
    });
    mocks.openIfExists.mockResolvedValue({
      loadItems: vi.fn().mockResolvedValue([{
        index: 0,
        type: "uint64",
        value: "42",
        handle: `0x${"11".repeat(32)}`,
        ownerIndex: 0,
        ownerAddress: "0x0000000000000000000000000000000000000001",
        isPublic: true,
        transactionHash: `0x${"22".repeat(32)}`,
      }]),
      cursor: vi.fn().mockReturnValue({ position: 0n, claim: vi.fn().mockReturnValue(0n) }),
    });
    const executor = new PublicDecryptExecutor(
      {
        network: "devnet",
        relayerUrl: "https://relayer.example",
        contractChainId: 9_000,
        dataDir: "/tmp/load-test-public-provenance",
      },
      {} as never,
      undefined,
      5_000,
      1,
    );
    await executor.prepare(1);

    await expect(executor.execute(0, new AbortController().signal)).resolves.toMatchObject({
      echoedRequestId: "post-request",
      jobId: "post-job",
      submitLatencyMs: 1,
      outcome: "protocol_error",
      errorLabel: "client_response_identity_mismatch",
      verified: false,
    });
  });
});
