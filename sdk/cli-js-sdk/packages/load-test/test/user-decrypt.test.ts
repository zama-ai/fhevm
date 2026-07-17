import { beforeEach, describe, expect, it, vi } from "vitest";

const OWNER = "0x0000000000000000000000000000000000000001";
const DELEGATE = "0x0000000000000000000000000000000000000002";
const CONTRACT = "0x0000000000000000000000000000000000000003";

const mocks = vi.hoisted(() => ({
  createClientContext: vi.fn(),
  getDelegationExpiration: vi.fn(),
  laneAccount: vi.fn(),
  openIfExists: vi.fn(),
}));

vi.mock("@cli-fhevm-sdk/toolkit/config", () => ({
  createClientContext: mocks.createClientContext,
}));

vi.mock("@cli-fhevm-sdk/toolkit/acl/delegation", () => ({
  getUserDecryptionDelegationExpirationDate: mocks.getDelegationExpiration,
}));

vi.mock("../src/env", () => ({
  laneAccount: mocks.laneAccount,
  poolDir: vi.fn().mockReturnValue("/pool"),
}));

vi.mock("../src/pool/store", () => ({
  PoolStore: { openIfExists: mocks.openIfExists },
}));

import type { LoadTestEnv } from "../src/env";
import {
  UserDecryptExecutor,
  type UserDecryptSdkClient,
  validateUserDecryptPool,
} from "../src/flows/user-decrypt";
import type { FheHandlePoolItem, PoolMeta } from "../src/pool/types";

const env = (candidate = false): LoadTestEnv => ({
  network: "devnet",
  relayerUrl: "https://a.example",
  relayerBUrl: candidate ? "https://b.example" : undefined,
  contractChainId: 9_000,
  contractAddress: CONTRACT,
  dataDir: ".load-test",
});

const item = (): FheHandlePoolItem => ({
  index: 0,
  type: "uint64",
  value: "42",
  handle: `0x${"11".repeat(32)}`,
  ownerIndex: 0,
  ownerAddress: OWNER,
  isPublic: false,
  transactionHash: `0x${"22".repeat(32)}`,
});

const meta = (delegated = false): PoolMeta => ({
  kind: "fhe-handles",
  flow: delegated ? "delegated-user-decrypt" : "user-decrypt",
  network: "devnet",
  contractChainId: 9_000,
  contractAddress: CONTRACT,
  createdAt: "2026-01-01T00:00:00.000Z",
  count: 1,
  ownerIndices: [0],
  ...(delegated
    ? {
        delegateIndex: 99,
        delegateAddress: DELEGATE,
        delegationExpiration: "4102444800",
        delegationExpirations: { "0": "4102444800" },
      }
    : {}),
});

const progress = {
  queued: {
    type: "queued",
    method: "POST",
    operation: "USER_DECRYPT",
    status: 202,
    requestId: "echoed",
    jobId: "job",
    retryAfterMs: 250,
    elapsed: 12,
    retryCount: 0,
    totalSteps: 1,
    step: 0,
    url: "https://relayer.example/v2/user-decrypt",
  },
  succeeded: {
    type: "succeeded",
    method: "GET",
    operation: "USER_DECRYPT",
    status: 200,
    requestId: "terminal-http-request",
    jobId: "job",
    elapsed: 100,
    retryCount: 2,
    totalSteps: 1,
    step: 1,
    result: [],
    url: "https://relayer.example/v2/user-decrypt/job",
  },
} as const;

const sdkClient = (value = 42n): UserDecryptSdkClient => ({
  ready: Promise.resolve(),
  generateTransportKeyPair: vi.fn().mockResolvedValue({ key: Symbol("key") }),
  signDecryptionPermit: vi.fn().mockResolvedValue({ permit: Symbol("permit") }),
  decryptValues: vi.fn().mockImplementation(async (parameters) => {
    const onProgress = (
      parameters.options as { onProgress: (value: unknown) => void }
    ).onProgress;
    onProgress(progress.queued);
    onProgress(progress.succeeded);
    return [{ type: "uint64", value }];
  }),
});

const client = (baseUrl: string) =>
  ({ baseUrl, apiPrefix: "/v2" }) as never;

const deferred = <T = void>() => {
  let resolve!: (value: T | PromiseLike<T>) => void;
  let reject!: (reason?: unknown) => void;
  const promise = new Promise<T>((resolvePromise, rejectPromise) => {
    resolve = resolvePromise;
    reject = rejectPromise;
  });
  return { promise, reject, resolve };
};

describe("UserDecryptExecutor", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.createClientContext.mockImplementation(({ relayerUrl }) => ({
      contractAddress: CONTRACT,
      chain: { relayerUrl },
      publicClient: {},
    }));
    mocks.getDelegationExpiration.mockResolvedValue(4_102_444_800n);
    mocks.laneAccount.mockImplementation((index: number) => ({
      address: index === 99 ? DELEGATE : OWNER,
    }));
  });

  it("runs independent SDK-native A/B user-decrypt legs with the run signal", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const targetA = sdkClient();
    const targetB = sdkClient();
    const createSdkClient = vi
      .fn()
      .mockReturnValueOnce(targetA)
      .mockReturnValueOnce(targetB);
    const executor = new UserDecryptExecutor(
      env(true),
      client("https://a.example"),
      client("https://b.example"),
      5_000,
      false,
      { createSdkClient },
    );
    await executor.prepare(1);
    const controller = new AbortController();

    const result = await executor.execute(3, controller.signal);

    expect(createSdkClient).toHaveBeenCalledTimes(2);
    expect(mocks.createClientContext).toHaveBeenNthCalledWith(
      1,
      expect.objectContaining({ relayerUrl: "https://a.example" }),
    );
    expect(mocks.createClientContext).toHaveBeenNthCalledWith(
      2,
      expect.objectContaining({ relayerUrl: "https://b.example" }),
    );
    expect(targetA.signDecryptionPermit).toHaveBeenCalledWith(
      expect.objectContaining({ durationSeconds: 86_400 }),
    );
    expect(targetA.signDecryptionPermit).toHaveBeenCalledWith(
      expect.not.objectContaining({ delegatorAddress: expect.anything() }),
    );
    // One preflight call at prepare() plus one per executed request.
    expect(targetB.signDecryptionPermit).toHaveBeenCalledTimes(2);
    expect(targetA.decryptValues).toHaveBeenCalledWith(
      expect.objectContaining({
        options: expect.objectContaining({ signal: controller.signal }),
      }),
    );
    expect(result).toMatchObject({
      flow: "user-decrypt",
      outcome: "succeeded",
      pollCount: 2,
      verified: true,
      outcomeB: "succeeded",
      pollCountB: 2,
      verifiedB: true,
    });
    const targetAKey = (
      targetA.signDecryptionPermit as ReturnType<typeof vi.fn>
    ).mock.calls[0]?.[0].transportKeyPair;
    const targetBKey = (
      targetB.signDecryptionPermit as ReturnType<typeof vi.fn>
    ).mock.calls[0]?.[0].transportKeyPair;
    expect(targetAKey).not.toBe(targetBKey);
  });

  it("waits for both SDK clients before permitting A/B user-decrypt actions", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const readyA = deferred();
    const readyB = deferred();
    const targetA = { ...sdkClient(), ready: readyA.promise };
    const targetB = { ...sdkClient(), ready: readyB.promise };
    const executor = new UserDecryptExecutor(
      env(true),
      client("https://a.example"),
      client("https://b.example"),
      5_000,
      false,
      {
        createSdkClient: vi.fn()
          .mockReturnValueOnce(targetA)
          .mockReturnValueOnce(targetB),
      },
    );

    const preparing = executor.prepare(1);
    await vi.waitFor(() => expect(mocks.createClientContext).toHaveBeenCalledTimes(2));
    readyA.resolve();
    await Promise.resolve();
    await expect(
      executor.execute(0, new AbortController().signal),
    ).rejects.toThrow("Executor not prepared");
    expect(targetA.generateTransportKeyPair).not.toHaveBeenCalled();
    expect(targetB.generateTransportKeyPair).not.toHaveBeenCalled();
    readyB.resolve();
    await preparing;

    await executor.execute(0, new AbortController().signal);
    // One preflight call at prepare() plus one per executed request.
    expect(targetA.generateTransportKeyPair).toHaveBeenCalledTimes(2);
    expect(targetB.generateTransportKeyPair).toHaveBeenCalledTimes(2);
  });

  it("fails prepare before any relayer submission when the SDK preflight fails", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const protocolError = new Error(
      "ProtocolConfig.getCurrentKmsContextAndEpoch() requires ProtocolConfig >= v0.2.0",
    );
    const target = {
      ...sdkClient(),
      signDecryptionPermit: vi.fn().mockRejectedValue(protocolError),
    };
    const executor = new UserDecryptExecutor(
      env(),
      client("https://a.example"),
      undefined,
      5_000,
      false,
      { createSdkClient: vi.fn().mockReturnValue(target) },
    );

    await expect(executor.prepare(1)).rejects.toMatchObject({
      message: expect.stringContaining("SDK preflight failed for target A"),
      cause: protocolError,
    });
    await expect(
      executor.execute(0, new AbortController().signal),
    ).rejects.toThrow("Executor not prepared");
    expect(target.decryptValues).not.toHaveBeenCalled();
  });

  it("fails prepare without starting user-decrypt actions when SDK readiness fails", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const readinessError = new Error("SDK decrypt client failed to initialize");
    const target = { ...sdkClient(), ready: Promise.reject(readinessError) };
    const executor = new UserDecryptExecutor(
      env(),
      client("https://a.example"),
      undefined,
      5_000,
      false,
      { createSdkClient: vi.fn().mockReturnValue(target) },
    );

    await expect(executor.prepare(1)).rejects.toBe(readinessError);
    await expect(
      executor.execute(0, new AbortController().signal),
    ).rejects.toThrow("Executor not prepared");
    expect(target.generateTransportKeyPair).not.toHaveBeenCalled();
    expect(target.signDecryptionPermit).not.toHaveBeenCalled();
    expect(target.decryptValues).not.toHaveBeenCalled();
  });

  it("runs delegated decrypt as the recorded delegate for the pooled owner", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(true),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const target = sdkClient();
    const executor = new UserDecryptExecutor(
      env(),
      client("https://a.example"),
      undefined,
      5_000,
      true,
      { createSdkClient: vi.fn().mockReturnValue(target) },
    );
    await executor.prepare(1);

    const result = await executor.execute(0, new AbortController().signal);

    expect(target.signDecryptionPermit).toHaveBeenCalledWith(
      expect.objectContaining({
        delegatorAddress: OWNER,
        signerAddress: DELEGATE,
        signer: expect.objectContaining({ address: DELEGATE }),
      }),
    );
    expect(result).toMatchObject({
      flow: "delegated-user-decrypt",
      outcome: "succeeded",
      verified: true,
    });
    expect(mocks.getDelegationExpiration).toHaveBeenCalledWith(
      expect.anything(),
      { delegatorAddress: OWNER, delegateAddress: DELEGATE },
    );
  });

  it.each([
    ["user-decrypt", false],
    ["delegated-user-decrypt", true],
  ] as const)("rejects mismatched SDK terminal job provenance for %s", async (_flow, delegated) => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(delegated),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const target = sdkClient();
    (target.decryptValues as ReturnType<typeof vi.fn>).mockImplementation(
      async (parameters: Record<string, unknown>) => {
        const onProgress = (
          parameters.options as { onProgress: (value: unknown) => void }
        ).onProgress;
        onProgress(progress.queued);
        onProgress({ ...progress.succeeded, jobId: "different-job" });
        return [{ type: "uint64", value: 42n }];
      },
    );
    const executor = new UserDecryptExecutor(
      env(), client("https://a.example"), undefined, 5_000, delegated,
      { createSdkClient: vi.fn().mockReturnValue(target) },
    );
    await executor.prepare(1);

    await expect(executor.execute(0, new AbortController().signal)).resolves.toMatchObject({
      outcome: "protocol_error",
      errorLabel: "client_response_identity_mismatch",
      verified: false,
    });
  });

  it("classifies an in-flight abort consistently and passes the exact signal", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const target = sdkClient();
    const controller = new AbortController();
    (target.decryptValues as ReturnType<typeof vi.fn>).mockImplementation(
      async (parameters: Record<string, unknown>) => {
        expect(
          (parameters.options as { signal: AbortSignal }).signal,
        ).toBe(controller.signal);
        controller.abort();
        throw new DOMException("aborted", "AbortError");
      },
    );
    const executor = new UserDecryptExecutor(
      env(),
      client("https://a.example"),
      undefined,
      5_000,
      false,
      { createSdkClient: vi.fn().mockReturnValue(target) },
    );
    await executor.prepare(1);

    const result = await executor.execute(0, controller.signal);

    expect(result).toMatchObject({
      outcome: "aborted",
      errorLabel: "client_aborted",
      pollCount: 0,
      verified: false,
    });
  });

  it("maps queued terminal SDK failures to the relayer outcome and poll taxonomy", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const target = sdkClient();
    (target.decryptValues as ReturnType<typeof vi.fn>).mockImplementation(
      async (parameters: Record<string, unknown>) => {
        const onProgress = (
          parameters.options as { onProgress: (value: unknown) => void }
        ).onProgress;
        onProgress(progress.queued);
        onProgress({
          type: "failed",
          method: "GET",
          operation: "USER_DECRYPT",
          status: 500,
          elapsed: 200,
          retryCount: 4,
          totalSteps: 1,
          step: 1,
          url: "https://relayer.example/v2/user-decrypt/job",
          relayerApiError: { label: "kms_failed", message: "KMS failed" },
        });
        throw new Error("SDK wrapper");
      },
    );
    const executor = new UserDecryptExecutor(
      env(),
      client("https://a.example"),
      undefined,
      5_000,
      false,
      { createSdkClient: vi.fn().mockReturnValue(target) },
    );
    await executor.prepare(1);

    const result = await executor.execute(0, new AbortController().signal);

    expect(result).toMatchObject({
      outcome: "failed",
      errorLabel: "kms_failed",
      errorMessage: "KMS failed",
      pollCount: 4,
      verified: false,
    });
  });

  it("rejects mismatched network, chain, contract, and expired delegation metadata", () => {
    const validate = (poolMeta: PoolMeta, runEnv = env()) =>
      validateUserDecryptPool({
        env: runEnv,
        flow: poolMeta.flow as "user-decrypt" | "delegated-user-decrypt",
        meta: poolMeta,
        items: [item()],
        resolvedContractAddress: CONTRACT,
        nowSeconds: 2_000n,
      });

    expect(() => validate({ ...meta(), network: "testnet" })).toThrow(
      "Pool network testnet does not match run network devnet",
    );
    expect(() => validate({ ...meta(), contractChainId: 1 })).toThrow(
      "Pool chain 1 does not match run chain 9000",
    );
    expect(() =>
      validate({
        ...meta(),
        contractAddress: "0x0000000000000000000000000000000000000009",
      }),
    ).toThrow("does not match run contract");
    expect(() =>
      validate({
        ...meta(true),
        delegationExpiration: "1999",
        delegationExpirations: { "0": "1999" },
      }),
    ).toThrow("expires at 1999");
  });

  it("rejects public handles and invalid typed expected values during preflight", () => {
    const validate = (poolItem: FheHandlePoolItem) =>
      validateUserDecryptPool({
        env: env(),
        flow: "user-decrypt",
        meta: meta(),
        items: [poolItem],
        resolvedContractAddress: CONTRACT,
      });

    expect(() => validate({ ...item(), isPublic: true })).toThrow(
      "cannot serve private user-decrypt load",
    );
    expect(() => validate({ ...item(), type: "bool", value: "not-a-bool" })).toThrow(
      "invalid bool expected value",
    );
  });

  it("classifies SDK verification failures after a successful GET separately", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    const target = sdkClient();
    (target.decryptValues as ReturnType<typeof vi.fn>).mockImplementation(
      async (parameters: Record<string, unknown>) => {
        const onProgress = (
          parameters.options as { onProgress: (value: unknown) => void }
        ).onProgress;
        onProgress(progress.queued);
        onProgress(progress.succeeded);
        throw new Error("invalid KMS signature");
      },
    );
    const executor = new UserDecryptExecutor(
      env(), client("https://a.example"), undefined, 5_000, false,
      { createSdkClient: vi.fn().mockReturnValue(target) },
    );
    await executor.prepare(1);

    const result = await executor.execute(0, new AbortController().signal);

    expect(result).toMatchObject({
      outcome: "verify_failed",
      errorLabel: "kms_verification_or_reconstruction_failed",
      verified: false,
    });
  });

  it("rejects identical primary and candidate origins", () => {
    expect(() => new UserDecryptExecutor(
      { ...env(true), relayerBUrl: "https://a.example" },
      client("https://a.example"),
      client("https://a.example/v2"),
      5_000,
      false,
    )).toThrow("must be different origins");
  });

  it("rejects pool owner-account mismatches during preflight", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    mocks.laneAccount.mockReturnValue({
      address: "0x0000000000000000000000000000000000000009",
    });
    const executor = new UserDecryptExecutor(
      env(),
      client("https://a.example"),
      undefined,
      5_000,
      false,
      { createSdkClient: vi.fn() },
    );

    await expect(executor.prepare(1)).rejects.toThrow(
      "does not match lane 0 account",
    );
  });

  it("rejects stale on-chain delegation even when pool metadata is current", async () => {
    mocks.openIfExists.mockResolvedValue({
      meta: meta(true),
      loadItems: vi.fn().mockResolvedValue([item()]),
    });
    mocks.getDelegationExpiration.mockResolvedValue(1n);
    const executor = new UserDecryptExecutor(
      env(),
      client("https://a.example"),
      undefined,
      5_000,
      true,
      { createSdkClient: vi.fn() },
    );

    await expect(executor.prepare(1)).rejects.toThrow(
      "On-chain ACL delegation for owner lane 0 expires at 1",
    );
  });
});
