import sdkPackage from "@fhevm/sdk/package.json" with { type: "json" };
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";

/**
 * Compatibility seam for verifying a saved relayer response with SDK alpha.8.
 *
 * Alpha.8 exposes permit/key parsing publicly, but not construction and
 * decryption of already-fetched KMS shares. These internals must be resolved
 * from the installed package so their private brands match the public alpha.8
 * objects. Do not import the adjacent js-sdk source tree here: it creates a
 * second set of branded classes and makes the toolkit depend on repository
 * layout rather than its declared dependency.
 *
 * This adapter is intentionally Node ESM and unbundled-only. A bundler or CJS
 * facade may duplicate SDK modules and invalidate their private brands.
 */
const SUPPORTED_SDK_VERSION = "1.1.0-alpha.8";

export type SavedUserDecryptShare = Readonly<{
  payload: string;
  signature: string;
  extraData: string;
}>;

type KmsSignersContext = Readonly<{
  id: bigint;
  epochId: bigint;
  threshold: number;
  signers: readonly string[];
}>;

type Alpha8Internals = Readonly<{
  fromKmsExtraDataBytesHex: (extraData: string) => unknown;
  readKmsSignersContextFromExtraData: (
    context: unknown,
    parameters: Readonly<{
      kmsVerifierAddress: string;
      protocolConfigAddress: string | undefined;
      extraData: unknown;
    }>,
  ) => Promise<KmsSignersContext>;
  toFhevmHandle: (value: unknown) => unknown;
  createKmsSigncryptedShares: (
    context: unknown,
    parameters: Readonly<{
      metadata: Readonly<{
        kmsSignersContext: KmsSignersContext;
        eip712Domain: unknown;
        eip712Signature: string;
        eip712SignerAddress: string;
        handles: readonly unknown[];
        tkmsVersion: unknown;
      }>;
      shares: readonly SavedUserDecryptShare[];
    }>,
  ) => Promise<unknown>;
  decryptKmsSigncryptedShares: (
    context: unknown,
    parameters: Readonly<{
      kmsSigncryptedShares: unknown;
      transportKeyPair: unknown;
    }>,
  ) => Promise<readonly unknown[]>;
}>;

type AdapterDependencies = Readonly<{
  sdkVersion: string;
  loadInternals: () => Promise<Alpha8Internals>;
}>;

export type DecryptSavedUserDecryptParameters = Readonly<{
  fhevm: {
    readonly ready?: Promise<void>;
    readonly chain: {
      readonly fhevm: {
        readonly contracts: {
          readonly kmsVerifier: { readonly address: string };
          readonly protocolConfig?: { readonly address: string } | undefined;
        };
        readonly gateway: {
          readonly id: number;
          readonly contracts: {
            readonly decryption: { readonly address: string };
          };
        };
      };
    };
  };
  encryptedValues: readonly unknown[];
  signedPermit: {
    readonly signature: string;
    readonly signerAddress: string;
  };
  transportKeyPair: {
    readonly tkmsVersion: unknown;
  };
  shares: readonly SavedUserDecryptShare[];
}>;

export type DecryptSavedUserDecryptResult = Readonly<{
  clearValues: readonly unknown[];
  verification: Readonly<{
    shareCount: number;
    kmsContextId: bigint;
    kmsEpochId: bigint;
    kmsThreshold: number;
    kmsSignerCount: number;
  }>;
}>;

const remove0x = (value: string): string => value.replace(/^0x/i, "");
const ensure0x = (value: string): string =>
  value.startsWith("0x") || value.startsWith("0X") ? value : `0x${value}`;

const assertSupportedSdkVersion = (sdkVersion: string): void => {
  if (sdkVersion !== SUPPORTED_SDK_VERSION) {
    throw new Error(
      `Saved user-decrypt verification supports @fhevm/sdk ${SUPPORTED_SDK_VERSION} only; resolved ${sdkVersion}.`,
    );
  }
};

const loadAlpha8Internals = async (): Promise<Alpha8Internals> => {
  const packageJsonUrl = pathToFileURL(
    createRequire(import.meta.url).resolve("@fhevm/sdk/package.json"),
  );
  const load = async <T>(path: string): Promise<T> =>
    (await import(new URL(path, packageJsonUrl).href)) as T;

  const [extraData, signers, handles, shares, decrypt] = await Promise.all([
    load<Pick<Alpha8Internals, "fromKmsExtraDataBytesHex">>(
      "_esm/core/kms/kmsExtraData-p.js",
    ),
    load<Pick<Alpha8Internals, "readKmsSignersContextFromExtraData">>(
      "_esm/core/host-contracts/readKmsSignersContext-p.js",
    ),
    load<Pick<Alpha8Internals, "toFhevmHandle">>(
      "_esm/core/handle/FhevmHandle.js",
    ),
    load<Pick<Alpha8Internals, "createKmsSigncryptedShares">>(
      "_esm/core/kms/KmsSigncryptedShares-p.js",
    ),
    load<Pick<Alpha8Internals, "decryptKmsSigncryptedShares">>(
      "_esm/core/kms/decryptKmsSigncryptedShares-p.js",
    ),
  ]);

  const internals = {
    fromKmsExtraDataBytesHex: extraData.fromKmsExtraDataBytesHex,
    readKmsSignersContextFromExtraData:
      signers.readKmsSignersContextFromExtraData,
    toFhevmHandle: handles.toFhevmHandle,
    createKmsSigncryptedShares: shares.createKmsSigncryptedShares,
    decryptKmsSigncryptedShares: decrypt.decryptKmsSigncryptedShares,
  } satisfies Alpha8Internals;
  for (const [name, value] of Object.entries(internals)) {
    if (typeof value !== "function") {
      throw new Error(`@fhevm/sdk alpha.8 internal ${name} is unavailable`);
    }
  }
  return internals;
};

/** Resolves the installed alpha.8 internal seam without making network calls. */
export const assertAlpha8SavedUserDecryptAdapterAvailable = async (): Promise<void> => {
  assertSupportedSdkVersion(sdkPackage.version);
  await loadAlpha8Internals();
};

export const createAlpha8SavedUserDecryptAdapter = (
  dependencies: AdapterDependencies,
) =>
  async (
    parameters: DecryptSavedUserDecryptParameters,
  ): Promise<DecryptSavedUserDecryptResult> => {
    assertSupportedSdkVersion(dependencies.sdkVersion);
    if (parameters.shares.length === 0) {
      throw new Error("Expected at least one user-decrypt share");
    }

    await parameters.fhevm.ready;

    const normalizedShares = parameters.shares.map((share) => ({
      payload: remove0x(share.payload),
      signature: remove0x(share.signature),
      extraData: remove0x(share.extraData),
    }));
    const firstExtraData = normalizedShares[0]?.extraData;
    if (firstExtraData === undefined) {
      throw new Error("Expected at least one user-decrypt share");
    }
    if (normalizedShares.some((share) => share.extraData !== firstExtraData)) {
      throw new Error("User-decrypt shares contain mismatched KMS extraData");
    }

    const internals = await dependencies.loadInternals();
    const kmsExtraData = internals.fromKmsExtraDataBytesHex(
      ensure0x(firstExtraData),
    );
    const contracts = parameters.fhevm.chain.fhevm.contracts;
    const kmsSignersContext =
      await internals.readKmsSignersContextFromExtraData(parameters.fhevm, {
        kmsVerifierAddress: contracts.kmsVerifier.address,
        protocolConfigAddress: contracts.protocolConfig?.address,
        extraData: kmsExtraData,
      });
    const kmsSigncryptedShares =
      await internals.createKmsSigncryptedShares(parameters.fhevm, {
        metadata: {
          kmsSignersContext,
          // KMS response shares are signed on the gateway domain. The saved
          // permit itself is signed on the host-chain domain and cannot be reused.
          eip712Domain: {
            name: "Decryption",
            version: "1",
            chainId: BigInt(parameters.fhevm.chain.fhevm.gateway.id),
            verifyingContract:
              parameters.fhevm.chain.fhevm.gateway.contracts.decryption.address,
          },
          eip712Signature: parameters.signedPermit.signature,
          eip712SignerAddress: parameters.signedPermit.signerAddress,
          handles: parameters.encryptedValues.map(internals.toFhevmHandle),
          tkmsVersion: parameters.transportKeyPair.tkmsVersion,
        },
        shares: normalizedShares,
      });
    const clearValues = await internals.decryptKmsSigncryptedShares(
      parameters.fhevm,
      { kmsSigncryptedShares, transportKeyPair: parameters.transportKeyPair },
    );

    return {
      clearValues,
      verification: {
        shareCount: normalizedShares.length,
        kmsContextId: kmsSignersContext.id,
        kmsEpochId: kmsSignersContext.epochId,
        kmsThreshold: kmsSignersContext.threshold,
        kmsSignerCount: kmsSignersContext.signers.length,
      },
    };
  };

export const decryptSavedUserDecryptResult =
  createAlpha8SavedUserDecryptAdapter({
    sdkVersion: sdkPackage.version,
    loadInternals: loadAlpha8Internals,
  });
