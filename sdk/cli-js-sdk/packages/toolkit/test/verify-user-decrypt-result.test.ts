import type { Hex } from "viem";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  createDecryptClientContext: vi.fn(),
  decryptSavedUserDecryptResult: vi.fn(),
  parseSignedDecryptionPermit: vi.fn(),
  parseSavedTransportKeyPair: vi.fn(),
}));

vi.mock("../src/config", () => ({
  createDecryptClientContext: mocks.createDecryptClientContext,
}));

vi.mock("@fhevm/sdk/actions/chain", () => ({
  parseSignedDecryptionPermit: mocks.parseSignedDecryptionPermit,
}));

vi.mock("../src/sdk-saved-user-decrypt-adapter", () => ({
  decryptSavedUserDecryptResult: mocks.decryptSavedUserDecryptResult,
  parseSavedTransportKeyPair: mocks.parseSavedTransportKeyPair,
}));

import type { UserDecryptValidationArtifact } from "../src/types";
import {
  verifyUserDecryptResult,
  verifyUserDecryptShares,
} from "../src/flows/relayer-result/user-decrypt";

const contractAddress = "0x0000000000000000000000000000000000000001";
const ownerAddress = "0x0000000000000000000000000000000000000002";
const handle = `0x${"00".repeat(32)}` as Hex;
const signature = `0x${"11".repeat(65)}` as Hex;

const artifact = (): UserDecryptValidationArtifact => ({
  schemaVersion: 2,
  flow: "user-decrypt",
  network: "testnet",
  relayer: { jobId: "job-1" },
  contractAddress,
  ownerAddress,
  signerAddress: ownerAddress,
  isDelegated: false,
  encryptedValues: [handle],
  handleContractPairs: [{ handle, contractAddress }],
  transportKeyPair: {
    publicKey: "0x1234",
    privateKey: "0x5678",
    tkmsVersion: "0.13.20-0",
  },
  serializedPermit: {
    version: 2,
    eip712: {
      domain: { chainId: "11155111" },
      message: { extraData: "0x0102" },
    },
    signature,
    signerAddress: ownerAddress,
  },
  permit: {
    version: 2,
    isDelegated: false,
    signerAddress: ownerAddress,
    encryptedDataOwnerAddress: ownerAddress,
    transportPublicKey: "0x1234",
    signature,
    contractAddresses: [contractAddress],
    startTimestamp: 1_700_000_000,
    durationSeconds: 86_400,
  },
  expectedClearValues: [{ type: "uint64", value: "42" }],
});

const share = { payload: "abcd", signature: "ef01" } as const;

describe("user-decrypt result verification", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.createDecryptClientContext.mockImplementation(
      (_options, tkmsVersion) => ({
        fhevm: { client: true, ready: Promise.resolve(), tkmsVersion },
      }),
    );
    mocks.parseSavedTransportKeyPair.mockResolvedValue({
      publicKey: "0x1234",
    });
    mocks.parseSignedDecryptionPermit.mockResolvedValue({
      version: 2,
      signature,
      signerAddress: ownerAddress,
      encryptedDataOwnerAddress: ownerAddress,
      isDelegated: false,
      transportPublicKey: "0x1234",
      eip712: {
        message: {
          extraData: "0x0102",
          allowedContracts: [contractAddress],
          durationSeconds: 86_400n,
          startTimestamp: 1_700_000_000n,
        },
      },
    });
    mocks.decryptSavedUserDecryptResult.mockResolvedValue({
      clearValues: [{ type: "uint64", value: 42n }],
      verification: {
        shareCount: 1,
        kmsContextId: 7n,
        kmsEpochId: 9n,
        kmsThreshold: 2,
        kmsSignerCount: 3,
      },
    });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("uses the isolated saved-user-decrypt adapter and reports verification metadata", async () => {
    const result = await verifyUserDecryptShares({
      artifact: artifact(),
      shares: [share],
    });

    expect(mocks.parseSavedTransportKeyPair).toHaveBeenCalledWith(
      expect.objectContaining({ client: true }),
      {
        publicKey: "0x1234",
        privateKey: "0x5678",
        tkmsVersion: "0.13.20-0",
      },
    );
    expect(mocks.parseSignedDecryptionPermit).toHaveBeenCalledWith(
      expect.objectContaining({ client: true }),
      expect.objectContaining({
        serializedPermit: expect.objectContaining({
          version: 2,
          eip712: expect.objectContaining({
            domain: expect.objectContaining({ chainId: 11_155_111n }),
          }),
        }),
      }),
    );
    expect(mocks.decryptSavedUserDecryptResult).toHaveBeenCalledWith(
      expect.objectContaining({
        fhevm: expect.objectContaining({ client: true }),
        encryptedValues: [handle],
        shares: [{ ...share, extraData: "0x0102" }],
      }),
    );
    expect(result).toEqual({
      flow: "user-decrypt",
      encryptedValues: [handle],
      clearValues: [{ type: "uint64", value: "42" }],
      expectedClearValues: [{ type: "uint64", value: "42" }],
      valuesMatch: true,
      shareCount: 1,
      kmsContextId: "7",
      kmsEpochId: "9",
      kmsThreshold: 2,
      kmsSignerCount: 3,
      provenance: {
        shares: "kms-cryptographically-verified",
        permit: "signature-verified",
        ownerAndDelegation: "artifact-asserted",
        expectedClearValues: "artifact-asserted",
      },
    });
  });

  it.each(["0.13.10", "0.13.20-0"] as const)(
    "preserves TKMS version %s when parsing the saved transport key",
    async (tkmsVersion) => {
      const versionedArtifact = artifact();
      await verifyUserDecryptShares({
        artifact: {
          ...versionedArtifact,
          transportKeyPair: {
            ...versionedArtifact.transportKeyPair,
            tkmsVersion,
          },
        },
        shares: [share],
      });

      expect(mocks.parseSavedTransportKeyPair).toHaveBeenCalledWith(
        expect.objectContaining({ client: true }),
        expect.objectContaining({ tkmsVersion }),
      );
      expect(mocks.createDecryptClientContext).toHaveBeenCalledWith(
        expect.anything(),
        tkmsVersion,
      );
    },
  );

  it("rejects a versionless transport key before creating an SDK context", async () => {
    const versionlessArtifact = artifact() as unknown as {
      transportKeyPair: Record<string, unknown>;
    };
    delete versionlessArtifact.transportKeyPair.tkmsVersion;

    await expect(
      verifyUserDecryptShares({
        artifact: versionlessArtifact as never,
        shares: [share],
      }),
    ).rejects.toThrow(
      "Artifact transportKeyPair.tkmsVersion must be 0.13.10 or 0.13.20-0",
    );
    expect(mocks.createDecryptClientContext).not.toHaveBeenCalled();
  });

  it("rejects legacy artifacts before creating an SDK context", async () => {
    await expect(
      verifyUserDecryptShares({
        artifact: { ...artifact(), schemaVersion: 1 } as never,
        shares: [share],
      }),
    ).rejects.toThrow("Unsupported artifact schemaVersion 1; expected 2");
    expect(mocks.createDecryptClientContext).not.toHaveBeenCalled();
  });

  it("fails before parsing saved decrypt material when SDK readiness fails", async () => {
    const readinessError = new Error("saved decrypt runtime unavailable");
    mocks.createDecryptClientContext.mockReturnValue({
      fhevm: {
        client: true,
        ready: Promise.reject(readinessError),
        tkmsVersion: "0.13.20-0",
      },
    });

    await expect(
      verifyUserDecryptShares({ artifact: artifact(), shares: [share] }),
    ).rejects.toBe(readinessError);
    expect(mocks.parseSavedTransportKeyPair).not.toHaveBeenCalled();
    expect(mocks.parseSignedDecryptionPermit).not.toHaveBeenCalled();
    expect(mocks.decryptSavedUserDecryptResult).not.toHaveBeenCalled();
  });

  it("rejects a resolved client whose TKMS version differs from the artifact", async () => {
    mocks.createDecryptClientContext.mockReturnValue({
      fhevm: {
        client: true,
        ready: Promise.resolve(),
        tkmsVersion: "0.13.20-0",
      },
    });

    const mismatchedArtifact = artifact();
    await expect(
      verifyUserDecryptShares({
        artifact: {
          ...mismatchedArtifact,
          transportKeyPair: {
            ...mismatchedArtifact.transportKeyPair,
            tkmsVersion: "0.13.10",
          },
        },
        shares: [share],
      }),
    ).rejects.toThrow(
      "Saved transport key TKMS version 0.13.10 does not match the resolved decrypt client version 0.13.20-0",
    );
    expect(mocks.parseSavedTransportKeyPair).not.toHaveBeenCalled();
    expect(mocks.decryptSavedUserDecryptResult).not.toHaveBeenCalled();
  });

  it("wraps SDK verification failures with the saved job id", async () => {
    mocks.decryptSavedUserDecryptResult.mockRejectedValue(
      new Error("bad KMS signature"),
    );

    await expect(
      verifyUserDecryptShares({ artifact: artifact(), shares: [share] }),
    ).rejects.toThrow(
      "Could not decrypt relayer response with the artifact transport key and permit. Artifact job id: job-1.",
    );
  });

  it("rejects artifact metadata that disagrees with the signed permit", async () => {
    await expect(
      verifyUserDecryptShares({
        artifact: {
          ...artifact(),
          signerAddress: "0x0000000000000000000000000000000000000009",
        },
        shares: [share],
      }),
    ).rejects.toThrow("Artifact signerAddress does not match");
    expect(mocks.decryptSavedUserDecryptResult).not.toHaveBeenCalled();
  });

  it("rejects non-http URLs before fetching", async () => {
    const fetchMock = vi.fn();
    vi.stubGlobal("fetch", fetchMock);
    await expect(
      verifyUserDecryptResult({ url: "file:///etc/passwd", artifact: artifact() }),
    ).rejects.toThrow("must use HTTP or HTTPS");
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it("bounds untrusted response bodies", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(new Response(JSON.stringify({ result: "x".repeat(100) }))),
    );
    await expect(
      verifyUserDecryptResult({
        url: "https://relayer.example/result",
        artifact: artifact(),
        maxResponseBytes: 16,
      }),
    ).rejects.toThrow("exceeds 16 bytes");
  });

  it("propagates cancellation to the verifier fetch", async () => {
    const controller = new AbortController();
    controller.abort();
    const fetchMock = vi.fn().mockImplementation(
      async (_url: URL, init: RequestInit) => {
        init.signal?.throwIfAborted();
        return new Response("{}");
      },
    );
    vi.stubGlobal("fetch", fetchMock);

    await expect(verifyUserDecryptResult({
      url: "https://relayer.example/result",
      artifact: artifact(),
      signal: controller.signal,
    })).rejects.toMatchObject({ name: "AbortError" });
    expect(fetchMock).toHaveBeenCalledOnce();
  });

  it("redacts and bounds remote error details", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn().mockResolvedValue(new Response(
        JSON.stringify({ error: { message: `authorization: Bearer bearer-secret ${"x".repeat(500)}` } }),
        { status: 500 },
      )),
    );

    let error: Error | undefined;
    try {
      await verifyUserDecryptResult({
        url: "https://relayer.example/result",
        artifact: artifact(),
      });
    } catch (caught) {
      if (caught instanceof Error) error = caught;
    }

    expect(error?.message).toContain("authorization: [redacted]");
    expect(error?.message).not.toContain("bearer-secret");
    expect(error?.message.length).toBeLessThan(300);
  });

  it("structurally redacts JSON secret fields and bearer values", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(new Response(JSON.stringify({
      error: {
        authorization: "Bearer bearer-secret",
        apiKey: "api-secret",
        nested: { private_key: "private-secret", accessToken: "access-secret" },
      },
    }), { status: 500 })));

    await expect(verifyUserDecryptResult({
      url: "https://relayer.example/result",
      artifact: artifact(),
    })).rejects.toSatisfy((error: Error) =>
      error.message.includes("[redacted]") &&
      !["bearer-secret", "api-secret", "private-secret", "access-secret"]
        .some((secret) => error.message.includes(secret)),
    );
  });

  it("finds nested relayer shares and preserves response metadata", async () => {
    const fetchMock =
      vi.fn().mockResolvedValue(
        new Response(
          JSON.stringify({
            requestId: "request-1",
            status: "succeeded",
            result: { result: [share] },
          }),
          { status: 200 },
        ),
      );
    vi.stubGlobal(
      "fetch",
      fetchMock,
    );

    const result = await verifyUserDecryptResult({
      url: "https://relayer.example/v2/user-decrypt/job-1",
      artifact: artifact(),
      authHeaders: () => ({ "x-api-key": "secret" }),
    });

    expect(result).toMatchObject({
      url: "https://relayer.example/v2/user-decrypt/job-1",
      httpStatus: 200,
      requestId: "request-1",
      status: "succeeded",
      valuesMatch: true,
      responseIdentity: { requestId: "unbound", jobId: "url-artifact-matched" },
    });
    expect(fetchMock).toHaveBeenCalledWith(
      new URL("https://relayer.example/v2/user-decrypt/job-1"),
      expect.objectContaining({ headers: { "x-api-key": "secret" } }),
    );
  });

  it("reports response identity as unbound when neither side supplies identifiers", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(new Response(JSON.stringify({
      status: "succeeded",
      result: [share],
    }))));

    const result = await verifyUserDecryptResult({
      url: "https://relayer.example/result",
      artifact: { ...artifact(), relayer: undefined },
    });

    expect(result.responseIdentity).toEqual({ requestId: "unbound", jobId: "unbound" });
  });

  it("reports response identifiers that match the artifact", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(new Response(JSON.stringify({
      requestId: "request-1",
      jobId: "job-1",
      status: "succeeded",
      result: [share],
    }))));

    const result = await verifyUserDecryptResult({
      url: "https://relayer.example/v2/user-decrypt/job-1",
      artifact: { ...artifact(), relayer: { requestId: "request-1", jobId: "job-1" } },
    });

    expect(result.responseIdentity).toEqual({
      requestId: "artifact-matched",
      jobId: "response-artifact-matched",
    });
  });

  it("rejects response and URL identities that disagree with the artifact", async () => {
    const response = (requestId: string, jobId?: string) => new Response(JSON.stringify({
      requestId,
      ...(jobId ? { jobId } : {}),
      status: "succeeded",
      result: [share],
    }));
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(response("wrong-request", "job-1")));
    await expect(verifyUserDecryptResult({
      url: "https://relayer.example/v2/user-decrypt/job-1",
      artifact: { ...artifact(), relayer: { requestId: "request-1", jobId: "job-1" } },
    })).rejects.toThrow("requestId does not match");

    vi.stubGlobal("fetch", vi.fn().mockResolvedValue(response("request-1")));
    await expect(verifyUserDecryptResult({
      url: "https://relayer.example/v2/user-decrypt/wrong-job",
      artifact: { ...artifact(), relayer: { requestId: "request-1", jobId: "job-1" } },
    })).rejects.toThrow("URL job id does not match");
  });
});
