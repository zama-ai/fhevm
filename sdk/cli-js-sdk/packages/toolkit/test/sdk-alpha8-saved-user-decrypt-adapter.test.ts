import { describe, expect, it, vi } from "vitest";
import { execFileSync } from "node:child_process";

import {
  assertAlpha8SavedUserDecryptAdapterAvailable,
  createAlpha8SavedUserDecryptAdapter,
} from "../src/sdk-alpha8-saved-user-decrypt-adapter";

const parameters = () => ({
  fhevm: {
    ready: Promise.resolve(),
    chain: {
      fhevm: {
        contracts: {
          kmsVerifier: { address: "0x0000000000000000000000000000000000000001" },
          protocolConfig: { address: "0x0000000000000000000000000000000000000002" },
        },
        gateway: {
          id: 5_432,
          contracts: {
            decryption: {
              address: "0x0000000000000000000000000000000000000004",
            },
          },
        },
      },
    },
  },
  encryptedValues: [`0x${"11".repeat(32)}`],
  signedPermit: {
    signature: `0x${"22".repeat(65)}`,
    signerAddress: "0x0000000000000000000000000000000000000003",
  },
  transportKeyPair: { tkmsVersion: "0.13.20-0" },
  shares: [{ payload: "0xaabb", signature: "0xccdd", extraData: "0x00" }],
});

describe("alpha.8 saved user-decrypt adapter", () => {
  it("resolves the guarded internal seam from the installed alpha.8 package", async () => {
    await expect(
      assertAlpha8SavedUserDecryptAdapterAvailable(),
    ).resolves.toBeUndefined();
  });

  it("loads the exact private seam in an unbundled Node ESM process", () => {
    execFileSync(
      process.execPath,
      [
        "--input-type=module",
        "--eval",
        [
          `import { createRequire } from "node:module";`,
          `import { pathToFileURL } from "node:url";`,
          `const root = pathToFileURL(createRequire(import.meta.url).resolve("@fhevm/sdk/package.json"));`,
          `const paths = ["_esm/core/kms/kmsExtraData-p.js", "_esm/core/host-contracts/readKmsSignersContext-p.js", "_esm/core/handle/FhevmHandle.js", "_esm/core/kms/KmsSigncryptedShares-p.js", "_esm/core/kms/decryptKmsSigncryptedShares-p.js"];`,
          `const names = ["fromKmsExtraDataBytesHex", "readKmsSignersContextFromExtraData", "toFhevmHandle", "createKmsSigncryptedShares", "decryptKmsSigncryptedShares"];`,
          `const modules = await Promise.all(paths.map((path) => import(new URL(path, root).href)));`,
          `if (!modules.every((module, index) => typeof module[names[index]] === "function")) process.exit(1);`,
        ].join("\n"),
      ],
      { stdio: "pipe" },
    );
  });

  it("composes the alpha.8 internals and exposes verified context metadata", async () => {
    const kmsSignersContext = {
      id: 7n,
      epochId: 9n,
      threshold: 2,
      signers: ["one", "two", "three"],
    };
    const internals = {
      fromKmsExtraDataBytesHex: vi.fn().mockReturnValue({ parsed: true }),
      readKmsSignersContextFromExtraData: vi.fn().mockResolvedValue(kmsSignersContext),
      toFhevmHandle: vi.fn().mockReturnValue({ handle: true }),
      createKmsSigncryptedShares: vi.fn().mockResolvedValue({ sealed: true }),
      decryptKmsSigncryptedShares: vi.fn().mockResolvedValue([
        { type: "uint64", value: 42n },
      ]),
    };
    const decrypt = createAlpha8SavedUserDecryptAdapter({
      sdkVersion: "1.1.0-alpha.8",
      loadInternals: vi.fn().mockResolvedValue(internals),
    });

    const result = await decrypt(parameters());

    expect(internals.fromKmsExtraDataBytesHex).toHaveBeenCalledWith("0x00");
    expect(internals.readKmsSignersContextFromExtraData).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({
        kmsVerifierAddress: "0x0000000000000000000000000000000000000001",
        protocolConfigAddress: "0x0000000000000000000000000000000000000002",
        extraData: { parsed: true },
      }),
    );
    expect(internals.createKmsSigncryptedShares).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({
        metadata: expect.objectContaining({
          kmsSignersContext,
          tkmsVersion: "0.13.20-0",
          eip712Signature: `0x${"22".repeat(65)}`,
          eip712SignerAddress:
            "0x0000000000000000000000000000000000000003",
          handles: [{ handle: true }],
          eip712Domain: {
            name: "Decryption",
            version: "1",
            chainId: 5_432n,
            verifyingContract:
              "0x0000000000000000000000000000000000000004",
          },
        }),
        shares: [{ payload: "aabb", signature: "ccdd", extraData: "00" }],
      }),
    );
    expect(result).toEqual({
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

  it("does not load or invoke private decrypt internals before readiness", async () => {
    let resolveReady!: () => void;
    const input = parameters();
    input.fhevm.ready = new Promise<void>((resolve) => {
      resolveReady = resolve;
    });
    const loadInternals = vi.fn().mockResolvedValue({});
    const decrypt = createAlpha8SavedUserDecryptAdapter({
      sdkVersion: "1.1.0-alpha.8",
      loadInternals,
    });

    const decrypting = decrypt(input);
    await Promise.resolve();
    expect(loadInternals).not.toHaveBeenCalled();

    resolveReady();
    await expect(decrypting).rejects.toThrow();
    expect(loadInternals).toHaveBeenCalledTimes(1);
  });

  it("fails closed for a different SDK version before loading internals", async () => {
    const loadInternals = vi.fn();
    const decrypt = createAlpha8SavedUserDecryptAdapter({
      sdkVersion: "1.1.0-alpha.9",
      loadInternals,
    });

    await expect(decrypt(parameters())).rejects.toThrow(
      "supports @fhevm/sdk 1.1.0-alpha.8 only",
    );
    expect(loadInternals).not.toHaveBeenCalled();
  });

  it("rejects mismatched KMS contexts before loading private modules", async () => {
    const loadInternals = vi.fn();
    const decrypt = createAlpha8SavedUserDecryptAdapter({
      sdkVersion: "1.1.0-alpha.8",
      loadInternals,
    });
    const input = parameters();
    const firstShare = input.shares[0]!;

    await expect(
      decrypt({
        ...input,
        shares: [
          firstShare,
          { ...firstShare, extraData: "0x01" },
        ],
      }),
    ).rejects.toThrow("mismatched KMS extraData");
    expect(loadInternals).not.toHaveBeenCalled();
  });
});
