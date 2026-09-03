import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  openIfExists: vi.fn(),
}));

vi.mock("../src/pool/store", () => ({
  PoolStore: { openIfExists: mocks.openIfExists },
}));

import type { LoadTestEnv } from "../src/env";
import { InputProofExecutor } from "../src/flows/input-proof";
import type { RelayerClient } from "../src/relayer/client";

const env: LoadTestEnv = {
  network: "devnet",
  relayerUrl: "https://relayer.example",
  contractChainId: 9_000,
  dataDir: ".load-test",
};

describe("InputProofExecutor", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.openIfExists.mockResolvedValue({
      loadItems: vi.fn().mockResolvedValue([{
        index: 0,
        contractChainId: 9_000,
        contractAddress: "0x0000000000000000000000000000000000000001",
        userAddress: "0x0000000000000000000000000000000000000002",
        ciphertextWithInputVerification: "aabb",
        extraData: "0x00",
        expectedHandles: ["0xab"],
        values: [{ type: "uint64", value: "42" }],
      }]),
      cursor: vi.fn().mockReturnValue({
        position: 0n,
        claim: vi.fn().mockReturnValue(0n),
      }),
    });
  });

  it("uses the accepted POST Retry-After as the first-poll delay", async () => {
    const submitInputProof = vi.fn().mockResolvedValue({
      httpStatus: 202,
      latencyMs: 5,
      retryAfterMs: 1_250,
      accepted: {
        status: "queued",
        requestId: "submit-response-request",
        result: { jobId: "job-1" },
      },
    });
    const pollJob = vi.fn().mockResolvedValue({
      httpStatus: 200,
      pollCount: 1,
      deadlineExceeded: false,
      result: {
        accepted: true,
        extraData: "0x00",
        handles: ["0xab"],
        signatures: [],
      },
    });
    const client = { submitInputProof, pollJob } as unknown as RelayerClient;
    const executor = new InputProofExecutor(env, client, undefined, 10_000);
    await executor.prepare(1);

    await expect(executor.execute(0, new AbortController().signal)).resolves.toMatchObject({
      outcome: "succeeded",
      verified: true,
    });
    expect(pollJob).toHaveBeenCalledWith(
      "input-proof",
      "job-1",
      expect.objectContaining({
        deadlineMs: 10_000,
        initialRetryAfterMs: 1_250,
      }),
    );
  });

  it("leaves a missing POST Retry-After for the client fallback", async () => {
    const submitInputProof = vi.fn().mockResolvedValue({
      httpStatus: 202,
      latencyMs: 5,
      accepted: {
        status: "queued",
        requestId: "submit-response-request",
        result: { jobId: "job-1" },
      },
    });
    const pollJob = vi.fn().mockResolvedValue({
      httpStatus: 200,
      pollCount: 1,
      deadlineExceeded: false,
      result: {
        accepted: true,
        extraData: "0x00",
        handles: ["0xab"],
        signatures: [],
      },
    });
    const client = { submitInputProof, pollJob } as unknown as RelayerClient;
    const executor = new InputProofExecutor(env, client, undefined, 10_000);
    await executor.prepare(1);

    await executor.execute(0, new AbortController().signal);
    expect(pollJob).toHaveBeenCalledWith(
      "input-proof",
      "job-1",
      expect.objectContaining({ initialRetryAfterMs: undefined }),
    );
  });
});
