import { describe, expect, it, vi } from "vitest";
import { execFileSync } from "node:child_process";

import {
  assertSavedUserDecryptAdapterAvailable,
  createSavedUserDecryptAdapter,
} from "../src/sdk-saved-user-decrypt-adapter";

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
    eip712: { message: { extraData: "0x00" } },
  },
  transportKeyPair: { parsed: "transport-key" },
  shares: [{ payload: "0xaabb", signature: "0xccdd", extraData: "0x00" }],
});

// Private SDK modules the adapter resolves, paired with the export it needs from
// each. Kept in lockstep with `loadSdkInternals` in the adapter.
const SEAM_PATHS = [
  "_esm/core/kms/kmsExtraData-p.js",
  "_esm/core/host-contracts/readKmsSignersContext-p.js",
  "_esm/core/handle/FhevmHandle.js",
  "_esm/core/kms/createKmsEip712Domain.js",
  "_esm/core/kms/KmsSigncryptedShares-p.js",
  "_esm/core/kms/decryptKmsSigncryptedShares-p.js",
  "_esm/core/runtime/CoreFhevm-p.js",
];
const SEAM_NAMES = [
  "createKmsExtraDataFromBytesHex",
  "readKmsSignersContextFromPermitExtraData",
  "toFhevmHandle",
  "createKmsEip712Domain",
  "createKmsSigncryptedShares",
  "decryptKmsSigncryptedShares",
  "initPublicAction",
];

describe("saved user-decrypt adapter", () => {
  it("resolves the guarded internal seam from the installed package", async () => {
    await expect(
      assertSavedUserDecryptAdapterAvailable(),
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
          `const paths = ${JSON.stringify(SEAM_PATHS)};`,
          `const names = ${JSON.stringify(SEAM_NAMES)};`,
          `const modules = await Promise.all(paths.map((path) => import(new URL(path, root).href)));`,
          `if (!modules.every((module, index) => typeof module[names[index]] === "function")) process.exit(1);`,
        ].join("\n"),
      ],
      { stdio: "pipe" },
    );
  });

  it("composes the internals and exposes verified context metadata", async () => {
    const kmsSignersContext = {
      id: 7n,
      epochId: 9n,
      threshold: 2,
      signers: ["one", "two", "three"],
    };
    const internals = {
      createKmsExtraDataFromBytesHex: vi.fn().mockReturnValue({ parsed: true }),
      readKmsSignersContextFromPermitExtraData: vi
        .fn()
        .mockResolvedValue(kmsSignersContext),
      toFhevmHandle: vi.fn().mockReturnValue({ handle: true }),
      createKmsEip712Domain: vi.fn().mockReturnValue({ domain: true }),
      createKmsSigncryptedShares: vi.fn().mockReturnValue({ sealed: true }),
      decryptKmsSigncryptedShares: vi
        .fn()
        .mockResolvedValue([{ type: "uint64", value: 42n }]),
      initPublicAction: vi
        .fn()
        .mockResolvedValue({ tkmsVersion: "0.13.20-0" }),
    };
    const decrypt = createSavedUserDecryptAdapter({
      sdkVersion: "0.13.2",
      loadInternals: vi.fn().mockResolvedValue(internals),
    });

    const input = parameters();
    const result = await decrypt(input);

    expect(internals.initPublicAction).toHaveBeenCalledWith(input.fhevm);
    // The permit's signed extraData drives context resolution — not the shares.
    expect(internals.createKmsExtraDataFromBytesHex).toHaveBeenCalledWith("0x00");
    expect(
      internals.readKmsSignersContextFromPermitExtraData,
    ).toHaveBeenCalledWith(
      input.fhevm,
      expect.objectContaining({
        kmsVerifierAddress: "0x0000000000000000000000000000000000000001",
        protocolConfigAddress: "0x0000000000000000000000000000000000000002",
        extraData: { parsed: true },
        fhevmContext: { tkmsVersion: "0.13.20-0" },
      }),
    );
    expect(internals.createKmsEip712Domain).toHaveBeenCalledWith({
      chainId: 5_432,
      verifyingContractAddressDecryption:
        "0x0000000000000000000000000000000000000004",
    });
    expect(internals.createKmsSigncryptedShares).toHaveBeenCalledWith(
      expect.objectContaining({
        metadata: expect.objectContaining({
          kmsSignersContext,
          // Locked to the resolved client's frozen context, not the saved key.
          tkmsVersion: "0.13.20-0",
          eip712ExtraData: "0x00",
          eip712Domain: { domain: true },
          eip712Signature: `0x${"22".repeat(65)}`,
          eip712SignerAddress:
            "0x0000000000000000000000000000000000000003",
          handles: [{ handle: true }],
        }),
        shares: [{ payload: "aabb", signature: "ccdd", extraData: "00" }],
      }),
    );
    expect(internals.decryptKmsSigncryptedShares).toHaveBeenCalledWith(
      input.fhevm,
      expect.objectContaining({
        kmsSigncryptedShares: { sealed: true },
        transportKeyPair: { parsed: "transport-key" },
        fhevmContext: { tkmsVersion: "0.13.20-0" },
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
    const decrypt = createSavedUserDecryptAdapter({
      sdkVersion: "0.13.2",
      loadInternals,
    });

    const decrypting = decrypt(input);
    await Promise.resolve();
    expect(loadInternals).not.toHaveBeenCalled();

    resolveReady();
    await expect(decrypting).rejects.toThrow();
    expect(loadInternals).toHaveBeenCalledTimes(1);
  });

  it("fails closed for an unsupported SDK version before loading internals", async () => {
    const loadInternals = vi.fn();
    const decrypt = createSavedUserDecryptAdapter({
      sdkVersion: "1.1.0-alpha.9",
      loadInternals,
    });

    await expect(decrypt(parameters())).rejects.toThrow(
      "supports @fhevm/sdk 0.13.2 only",
    );
    expect(loadInternals).not.toHaveBeenCalled();
  });

  it("fails closed for the prior alpha.8 SDK version", async () => {
    const loadInternals = vi.fn();
    const decrypt = createSavedUserDecryptAdapter({
      sdkVersion: "1.1.0-alpha.8",
      loadInternals,
    });

    await expect(decrypt(parameters())).rejects.toThrow(
      "supports @fhevm/sdk 0.13.2 only",
    );
    expect(loadInternals).not.toHaveBeenCalled();
  });

  it("fails closed for the 0.13.2-1 prerelease", async () => {
    const loadInternals = vi.fn();
    const decrypt = createSavedUserDecryptAdapter({
      sdkVersion: "0.13.2-1",
      loadInternals,
    });

    await expect(decrypt(parameters())).rejects.toThrow(
      "supports @fhevm/sdk 0.13.2 only",
    );
    expect(loadInternals).not.toHaveBeenCalled();
  });

  it("rejects mismatched KMS shares before loading private modules", async () => {
    const loadInternals = vi.fn();
    const decrypt = createSavedUserDecryptAdapter({
      sdkVersion: "0.13.2",
      loadInternals,
    });
    const input = parameters();
    const firstShare = input.shares[0]!;

    await expect(
      decrypt({
        ...input,
        shares: [firstShare, { ...firstShare, extraData: "0x01" }],
      }),
    ).rejects.toThrow("mismatched KMS extraData");
    expect(loadInternals).not.toHaveBeenCalled();
  });
});
