import type { Hex } from "viem";
import { beforeEach, describe, expect, it, vi } from "vitest";

const sdk = vi.hoisted(() => {
  const client = {
    ready: Promise.resolve(),
    decryptValues: vi.fn(),
    generateTransportKeyPair: vi.fn(),
    signLegacyDecryptionPermit: vi.fn(),
  };
  return {
    client,
    createFhevmDecryptClient: vi.fn(() => client),
    serializeSignedDecryptionPermit: vi.fn(),
    serializeTransportKeyPair: vi.fn(),
  };
});

vi.mock("@fhevm/sdk/viem", () => ({
  createFhevmDecryptClient: sdk.createFhevmDecryptClient,
}));

vi.mock("@fhevm/sdk/actions/chain", () => ({
  serializeSignedDecryptionPermit: sdk.serializeSignedDecryptionPermit,
  serializeTransportKeyPair: sdk.serializeTransportKeyPair,
}));

import { decryptUserValues } from "../src/fhevm/user-decrypt";

const contractAddress = "0x0000000000000000000000000000000000000001";
const ownerAddress = "0x0000000000000000000000000000000000000002";
const delegatorAddress = "0x0000000000000000000000000000000000000003";
const handle = `0x${"00".repeat(32)}` as Hex;
const signature = `0x${"11".repeat(65)}` as Hex;

describe("decryptUserValues", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    sdk.client.ready = Promise.resolve();
    sdk.client.generateTransportKeyPair.mockResolvedValue({
      key: "transport",
      tkmsVersion: "0.13.20-0",
    });
    sdk.client.signLegacyDecryptionPermit.mockResolvedValue({
      version: 2,
      isDelegated: false,
      signerAddress: ownerAddress,
      encryptedDataOwnerAddress: ownerAddress,
      transportPublicKey: "0x1234",
      signature,
    });
    sdk.client.decryptValues.mockResolvedValue([{ type: "uint64", value: 42n }]);
    sdk.serializeTransportKeyPair.mockResolvedValue({
      publicKey: "0x1234",
      privateKey: "0x5678",
    });
    sdk.serializeSignedDecryptionPermit.mockResolvedValue({
      version: 2,
      eip712: {},
      signature,
      signerAddress: ownerAddress,
    });
  });

  it("passes canonical seconds to the SDK and reports the permit version", async () => {
    const result = await decryptUserValues(
      {
        chain: {} as never,
        contractAddress,
        publicClient: {} as never,
      },
      {
        encryptedValues: [handle],
        signer: { address: ownerAddress } as never,
        ownerAddress,
        durationSeconds: 604_800,
        network: "testnet",
        includeValidationArtifact: true,
      },
    );

    expect(sdk.client.signLegacyDecryptionPermit).toHaveBeenCalledWith(
      expect.objectContaining({
        durationSeconds: 604_800,
        signerAddress: ownerAddress,
      }),
    );
    expect(sdk.client.signLegacyDecryptionPermit.mock.calls[0]?.[0]).not.toHaveProperty(
      "durationDays",
    );
    expect(result.permit).toMatchObject({
      version: 2,
      durationSeconds: 604_800,
      isDelegated: false,
    });
    expect(result.validationArtifact).toMatchObject({
      schemaVersion: 2,
      transportKeyPair: {
        publicKey: "0x1234",
        privateKey: "0x5678",
        tkmsVersion: "0.13.20-0",
      },
      permit: { version: 2, durationSeconds: 604_800 },
    });
  });

  it.each(["0.13.10", "0.13.20-0"] as const)(
    "preserves TKMS version %s in the validation artifact JSON",
    async (tkmsVersion) => {
      sdk.client.generateTransportKeyPair.mockResolvedValue({
        key: "transport",
        tkmsVersion,
      });

      const result = await decryptUserValues(
        {
          chain: {} as never,
          contractAddress,
          publicClient: {} as never,
        },
        {
          encryptedValues: [handle],
          signer: { address: ownerAddress } as never,
          ownerAddress,
          durationSeconds: 86_400,
          network: "testnet",
          includeValidationArtifact: true,
        },
      );

      expect(
        JSON.parse(JSON.stringify(result.validationArtifact)),
      ).toHaveProperty("transportKeyPair.tkmsVersion", tkmsVersion);
    },
  );

  it("rejects invalid second durations before creating an SDK client", async () => {
    await expect(
      decryptUserValues(
        {
          chain: {} as never,
          contractAddress,
          publicClient: {} as never,
        },
        {
          encryptedValues: [handle],
          signer: { address: ownerAddress } as never,
          ownerAddress,
          durationSeconds: 0,
          network: "testnet",
        },
      ),
    ).rejects.toThrow("Permit duration must be a positive safe integer");
    expect(sdk.createFhevmDecryptClient).not.toHaveBeenCalled();
  });

  it("passes the encrypted-data owner for delegated permits", async () => {
    sdk.client.signLegacyDecryptionPermit.mockResolvedValue({
      version: 2,
      isDelegated: true,
      signerAddress: ownerAddress,
      encryptedDataOwnerAddress: delegatorAddress,
      transportPublicKey: "0x1234",
      signature,
    });

    const result = await decryptUserValues(
      {
        chain: {} as never,
        contractAddress,
        publicClient: {} as never,
      },
      {
        encryptedValues: [handle],
        signer: { address: ownerAddress } as never,
        ownerAddress: delegatorAddress,
        durationSeconds: 86_400,
        network: "devnet",
      },
    );

    expect(sdk.client.signLegacyDecryptionPermit).toHaveBeenCalledWith(
      expect.objectContaining({
        delegatorAddress,
        durationSeconds: 86_400,
      }),
    );
    expect(result).toMatchObject({
      isDelegated: true,
      ownerAddress: delegatorAddress,
      permit: { version: 2, durationSeconds: 86_400 },
    });
  });

  it("does not start decrypt actions before the SDK client is ready", async () => {
    let resolveReady!: () => void;
    sdk.client.ready = new Promise<void>((resolve) => {
      resolveReady = resolve;
    });

    const decrypting = decryptUserValues(
      {
        chain: {} as never,
        contractAddress,
        publicClient: {} as never,
      },
      {
        encryptedValues: [handle],
        signer: { address: ownerAddress } as never,
        ownerAddress,
        durationSeconds: 86_400,
        network: "testnet",
      },
    );
    await vi.waitFor(() => expect(sdk.createFhevmDecryptClient).toHaveBeenCalled());
    expect(sdk.client.generateTransportKeyPair).not.toHaveBeenCalled();
    expect(sdk.client.signLegacyDecryptionPermit).not.toHaveBeenCalled();
    expect(sdk.client.decryptValues).not.toHaveBeenCalled();

    resolveReady();
    await decrypting;
    expect(sdk.client.generateTransportKeyPair).toHaveBeenCalledTimes(1);
  });

  it("propagates readiness failure before starting decrypt actions", async () => {
    const readinessError = new Error("decrypt runtime unavailable");
    sdk.client.ready = Promise.reject(readinessError);

    await expect(
      decryptUserValues(
        {
          chain: {} as never,
          contractAddress,
          publicClient: {} as never,
        },
        {
          encryptedValues: [handle],
          signer: { address: ownerAddress } as never,
          ownerAddress,
          durationSeconds: 86_400,
          network: "testnet",
        },
      ),
    ).rejects.toBe(readinessError);
    expect(sdk.client.generateTransportKeyPair).not.toHaveBeenCalled();
    expect(sdk.client.signLegacyDecryptionPermit).not.toHaveBeenCalled();
    expect(sdk.client.decryptValues).not.toHaveBeenCalled();
  });
});
