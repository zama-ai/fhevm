import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  createFhevmClient: vi.fn(),
  createFhevmDecryptClient: vi.fn(),
  createPublicClient: vi.fn(),
  http: vi.fn(),
  setFhevmRuntimeConfig: vi.fn(),
}));

vi.mock("@fhevm/sdk/viem", () => ({
  createFhevmClient: mocks.createFhevmClient,
  createFhevmDecryptClient: mocks.createFhevmDecryptClient,
  setFhevmRuntimeConfig: mocks.setFhevmRuntimeConfig,
}));

vi.mock("viem", async (importOriginal) => ({
  ...(await importOriginal<typeof import("viem")>()),
  createPublicClient: mocks.createPublicClient,
  createWalletClient: vi.fn(),
  http: mocks.http,
}));

import { createDecryptClientContext } from "../src/config/clients";

describe("createDecryptClientContext", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.http.mockReturnValue({ transport: true });
    mocks.createPublicClient.mockReturnValue({ publicClient: true });
    mocks.createFhevmDecryptClient.mockReturnValue({ decryptClient: true });
  });

  it.each(["0.13.10", "0.13.20-0"] as const)(
    "constructs a decrypt-only client pinned to TKMS %s",
    (tkmsVersion) => {
      const context = createDecryptClientContext(
        { network: "testnet" },
        tkmsVersion,
      );

      expect(mocks.createFhevmDecryptClient).toHaveBeenCalledWith(
        expect.objectContaining({
          publicClient: { publicClient: true },
          options: { moduleVersions: { kms: tkmsVersion } },
        }),
      );
      expect(mocks.createFhevmClient).not.toHaveBeenCalled();
      expect(context.fhevm).toEqual({ decryptClient: true });
    },
  );
});
