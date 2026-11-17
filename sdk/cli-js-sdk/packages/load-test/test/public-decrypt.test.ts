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

  it.each([
    {
      label: "accepts rotating HTTP request identities for one job",
      terminalRequestId: "terminal-request",
      terminalJobId: "post-job",
      outcome: "succeeded",
      errorLabel: undefined,
      verified: true,
    },
    {
      label: "rejects a mismatched terminal job identity",
      terminalRequestId: "post-request",
      terminalJobId: "wrong-job",
      outcome: "protocol_error",
      errorLabel: "client_response_identity_mismatch",
      verified: false,
    },
  ] as const)("keeps POST telemetry and $label", async ({
    terminalRequestId,
    terminalJobId,
    outcome,
    errorLabel,
    verified,
  }) => {
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
          requestId: terminalRequestId, jobId: terminalJobId, elapsed: 3, retryCount: 2,
        });
        return { clearValues: [{ type: "uint64", value: 42n }] };
      },
    );
    mocks.createClientContext.mockReturnValue({
      fhevm: {
        decryptPublicValuesWithSignatures,
        generateTransportKeyPair: vi.fn().mockResolvedValue({ key: Symbol("key") }),
        signLegacyDecryptionPermit: vi.fn().mockResolvedValue({ permit: Symbol("permit") }),
      },
    });
    mocks.openIfExists.mockResolvedValue({
      meta: { contractAddress: `0x${"33".repeat(20)}` },
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
      outcome,
      ...(errorLabel ? { errorLabel } : {}),
      verified,
    });
  });

  it("fails prepare before any relayer submission when the SDK preflight fails", async () => {
    const protocolError = new Error(
      "ProtocolConfig.getCurrentKmsContextAndEpoch() requires ProtocolConfig >= v0.2.0",
    );
    const decryptPublicValuesWithSignatures = vi.fn();
    mocks.createClientContext.mockReturnValue({
      fhevm: {
        decryptPublicValuesWithSignatures,
        generateTransportKeyPair: vi.fn().mockResolvedValue({ key: Symbol("key") }),
        signLegacyDecryptionPermit: vi.fn().mockRejectedValue(protocolError),
      },
    });
    mocks.openIfExists.mockResolvedValue({
      meta: { contractAddress: `0x${"33".repeat(20)}` },
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
        dataDir: "/tmp/load-test-public-preflight",
      },
      {} as never,
      undefined,
      5_000,
      1,
    );

    await expect(executor.prepare(1)).rejects.toMatchObject({
      message: expect.stringContaining(
        "public-decrypt SDK preflight failed for target A",
      ),
      cause: protocolError,
    });
    await expect(
      executor.execute(0, new AbortController().signal),
    ).rejects.toThrow("Executor not prepared");
    expect(decryptPublicValuesWithSignatures).not.toHaveBeenCalled();
  });
});

describe("PublicDecryptExecutor over relayer-http", () => {
  const prepared = async (
    decryptPublicValuesWithSignaturesNoGateway: ReturnType<typeof vi.fn>,
  ) => {
    const fhevm = {
      decryptPublicValuesWithSignatures: vi.fn(),
      decryptPublicValuesWithSignaturesNoGateway,
      generateTransportKeyPair: vi.fn().mockResolvedValue({ key: Symbol("key") }),
      signLegacyDecryptionPermit: vi.fn().mockResolvedValue({ permit: Symbol("legacy") }),
      signUnifiedDecryptionPermit: vi.fn().mockResolvedValue({ permit: Symbol("unified") }),
    };
    mocks.createClientContext.mockReturnValue({ fhevm });
    mocks.openIfExists.mockResolvedValue({
      meta: { contractAddress: `0x${"33".repeat(20)}` },
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
        dataDir: "/tmp/load-test-public-relayer-http",
        useRelayerHttp: true,
      },
      {} as never,
      undefined,
      5_000,
      1,
    );
    await executor.prepare(1);
    return { executor, fhevm };
  };

  it("decrypts in one request, preflights a unified permit, and records no job telemetry", async () => {
    const { executor, fhevm } = await prepared(
      vi.fn().mockResolvedValue({ clearValues: [{ type: "uint64", value: 42n }] }),
    );
    const controller = new AbortController();

    const result = await executor.execute(0, controller.signal);

    expect(fhevm.signUnifiedDecryptionPermit).toHaveBeenCalledTimes(1);
    expect(fhevm.signLegacyDecryptionPermit).not.toHaveBeenCalled();
    expect(fhevm.decryptPublicValuesWithSignatures).not.toHaveBeenCalled();
    expect(fhevm.decryptPublicValuesWithSignaturesNoGateway).toHaveBeenCalledWith({
      encryptedValues: [`0x${"11".repeat(32)}`],
      options: {
        timeout: 5_000,
        signal: controller.signal,
        headers: { "x-request-id": result.sentRequestId },
      },
    });
    expect(result).toMatchObject({ outcome: "succeeded", verified: true, pollCount: 0 });
    expect(result.jobId).toBeUndefined();
    expect(result.submitLatencyMs).toBeUndefined();
  });

  it.each([
    {
      label: "a relayer-http error response",
      error: Object.assign(new Error("relayer-http request failed"), {
        name: "RelayerHttpResponseError",
        status: 404,
        code: "ciphertext_not_found",
        responseMessage: "unknown handle",
        requestId: "server-request",
      }),
      expected: {
        outcome: "failed",
        errorLabel: "ciphertext_not_found",
        submitHttpStatus: 404,
        echoedRequestId: "server-request",
      },
    },
    {
      label: "an SDK request timeout",
      error: Object.assign(new Error("timed out"), { name: "RelayerHttpTimeoutError" }),
      expected: { outcome: "timed_out", errorLabel: "client_request_deadline_exceeded" },
    },
    {
      label: "a KMS signature verification failure",
      error: new Error("Invalid decrypted result."),
      expected: { outcome: "failed", errorLabel: "sdk_public_decrypt_error" },
    },
  ])("classifies $label", async ({ error, expected }) => {
    const { executor } = await prepared(vi.fn().mockRejectedValue(error));

    await expect(executor.execute(0, new AbortController().signal)).resolves.toMatchObject({
      ...expected,
      verified: false,
    });
  });

  it("flags wrong clear values as verify_failed", async () => {
    const { executor } = await prepared(
      vi.fn().mockResolvedValue({ clearValues: [{ type: "uint64", value: 41n }] }),
    );

    await expect(executor.execute(0, new AbortController().signal)).resolves.toMatchObject({
      outcome: "verify_failed",
      errorLabel: "values_mismatch",
    });
  });
});
