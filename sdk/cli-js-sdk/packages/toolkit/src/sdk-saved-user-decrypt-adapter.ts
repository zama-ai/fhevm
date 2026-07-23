import sdkPackage from "@fhevm/sdk/package.json" with { type: "json" };
import { parseTransportKeyPair } from "@fhevm/sdk/actions/chain";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";

/**
 * Compatibility seam for verifying a saved relayer user-decrypt response offline.
 *
 * The SDK exposes permit/key parsing publicly, but not the construction and
 * decryption of already-fetched KMS shares: the on-chain fetch and the local
 * reconstruction are welded together in the public `decryptValues*` actions, and
 * there is no public entry point that reconstructs from shares captured earlier.
 * These steps must therefore be resolved from the installed package's private
 * `_esm/core/**` modules so their private brands match the public objects the
 * toolkit already holds (the parsed permit, transport key, and FhevmHandle).
 * Importing the adjacent js-sdk source tree instead would create a second set of
 * branded classes and make the toolkit depend on repository layout rather than
 * its declared dependency.
 *
 * The seam mirrors the SDK's own `fetchKmsSigncryptedShares*` +
 * `decryptKmsSigncryptedShares` flow with the fetch replaced by saved shares. It
 * is intentionally Node ESM and unbundled-only: a bundler or CJS facade may
 * duplicate SDK modules and invalidate their private brands.
 */
const SUPPORTED_SDK_VERSION = "0.13.2";

export type SavedTkmsVersion = "0.13.10" | "0.13.20-0";

type SerializedTransportKeyPair = Readonly<{
  publicKey: string;
  privateKey: string;
  tkmsVersion: SavedTkmsVersion;
}>;

/** Forwards to the public parser; kept as the toolkit's stable, versioned seam. */
export const parseSavedTransportKeyPair = (
  fhevm: Parameters<typeof parseTransportKeyPair>[0],
  parameters: SerializedTransportKeyPair,
): ReturnType<typeof parseTransportKeyPair> =>
  parseTransportKeyPair(fhevm, parameters);

/** Signcrypted share as sent by the relayer (`payload`/`signature`/`extraData`). */
export type SavedUserDecryptShare = Readonly<{
  payload: string;
  signature: string;
  extraData: string;
}>;

/**
 * The `extraData`-indexed KMS signer snapshot returned by
 * `readKmsSignersContextFromPermitExtraData`. Only the fields this adapter
 * reports on are modeled; `.kmsVerifierAddress` and `.mpcThreshold` exist but
 * are unused here.
 */
type KmsSignersContext = Readonly<{
  id: bigint;
  epochId: bigint;
  threshold: number;
  signers: readonly string[];
}>;

/**
 * The immutable version basis a single FHEVM operation resolves against. The
 * adapter only reads `tkmsVersion` — the same value the SDK's fetch path stamps
 * into the shares metadata, and the value `decryptKmsSigncryptedShares` checks
 * the reconstructed shares against.
 */
type FhevmClientFrozenContext = Readonly<{ tkmsVersion: string }>;

type SdkInternals = Readonly<{
  createKmsExtraDataFromBytesHex: (extraDataBytesHex: string) => unknown;
  readKmsSignersContextFromPermitExtraData: (
    context: unknown,
    parameters: Readonly<{
      kmsVerifierAddress: string;
      protocolConfigAddress: string | undefined;
      fhevmContext: FhevmClientFrozenContext;
      extraData: unknown;
      forceRefresh?: boolean;
    }>,
  ) => Promise<KmsSignersContext>;
  toFhevmHandle: (value: unknown) => unknown;
  createKmsEip712Domain: (
    parameters: Readonly<{
      chainId: number | bigint;
      verifyingContractAddressDecryption: string;
    }>,
  ) => unknown;
  createKmsSigncryptedShares: (
    parameters: Readonly<{
      metadata: Readonly<{
        kmsSignersContext: KmsSignersContext;
        eip712ExtraData: string;
        eip712Domain: unknown;
        eip712Signature: string;
        eip712SignerAddress: string;
        handles: readonly unknown[];
        tkmsVersion: string;
      }>;
      shares: readonly SavedUserDecryptShare[];
    }>,
  ) => unknown;
  decryptKmsSigncryptedShares: (
    context: unknown,
    parameters: Readonly<{
      kmsSigncryptedShares: unknown;
      transportKeyPair: unknown;
      fhevmContext: FhevmClientFrozenContext;
    }>,
  ) => Promise<readonly unknown[]>;
  initPublicAction: (fhevm: unknown) => Promise<FhevmClientFrozenContext>;
}>;

type AdapterDependencies = Readonly<{
  sdkVersion: string;
  loadInternals: () => Promise<SdkInternals>;
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
  /**
   * The parsed, signature-verified permit. Its `eip712.message.extraData` is the
   * value the user signed and the KMS signers context is indexed on — passed
   * through verbatim, never re-encoded, exactly as the SDK's fetch path treats it.
   */
  signedPermit: {
    readonly signature: string;
    readonly signerAddress: string;
    readonly eip712: {
      readonly message: { readonly extraData: string };
    };
  };
  transportKeyPair: unknown;
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

const loadSdkInternals = async (): Promise<SdkInternals> => {
  const packageJsonUrl = pathToFileURL(
    createRequire(import.meta.url).resolve("@fhevm/sdk/package.json"),
  );
  const load = async <T>(path: string): Promise<T> =>
    (await import(new URL(path, packageJsonUrl).href)) as T;

  const [extraData, signers, handles, domain, shares, decrypt, runtime] =
    await Promise.all([
      load<Pick<SdkInternals, "createKmsExtraDataFromBytesHex">>(
        "_esm/core/kms/kmsExtraData-p.js",
      ),
      load<Pick<SdkInternals, "readKmsSignersContextFromPermitExtraData">>(
        "_esm/core/host-contracts/readKmsSignersContext-p.js",
      ),
      load<Pick<SdkInternals, "toFhevmHandle">>(
        "_esm/core/handle/FhevmHandle.js",
      ),
      load<Pick<SdkInternals, "createKmsEip712Domain">>(
        "_esm/core/kms/createKmsEip712Domain.js",
      ),
      load<Pick<SdkInternals, "createKmsSigncryptedShares">>(
        "_esm/core/kms/KmsSigncryptedShares-p.js",
      ),
      load<Pick<SdkInternals, "decryptKmsSigncryptedShares">>(
        "_esm/core/kms/decryptKmsSigncryptedShares-p.js",
      ),
      load<Pick<SdkInternals, "initPublicAction">>(
        "_esm/core/runtime/CoreFhevm-p.js",
      ),
    ]);

  const internals = {
    createKmsExtraDataFromBytesHex: extraData.createKmsExtraDataFromBytesHex,
    readKmsSignersContextFromPermitExtraData:
      signers.readKmsSignersContextFromPermitExtraData,
    toFhevmHandle: handles.toFhevmHandle,
    createKmsEip712Domain: domain.createKmsEip712Domain,
    createKmsSigncryptedShares: shares.createKmsSigncryptedShares,
    decryptKmsSigncryptedShares: decrypt.decryptKmsSigncryptedShares,
    initPublicAction: runtime.initPublicAction,
  } satisfies SdkInternals;
  for (const [name, value] of Object.entries(internals)) {
    if (typeof value !== "function") {
      throw new Error(`@fhevm/sdk internal ${name} is unavailable`);
    }
  }
  return internals;
};

/** Resolves the guarded internal seam without making network calls. */
export const assertSavedUserDecryptAdapterAvailable = async (): Promise<void> => {
  assertSupportedSdkVersion(sdkPackage.version);
  await loadSdkInternals();
};

export const createSavedUserDecryptAdapter = (
  dependencies: AdapterDependencies,
) =>
  async (
    parameters: DecryptSavedUserDecryptParameters,
  ): Promise<DecryptSavedUserDecryptResult> => {
    assertSupportedSdkVersion(dependencies.sdkVersion);
    if (parameters.shares.length === 0) {
      throw new Error("Expected at least one user-decrypt share");
    }

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

    await parameters.fhevm.ready;

    const internals = await dependencies.loadInternals();

    // The version basis the reconstruction resolves against. initPublicAction
    // re-awaits readiness (idempotent), reads the context resolved during init,
    // and throws if it is absent — so a client not built through the SDK, or one
    // whose init never completed, fails clearly here.
    const fhevmContext = await internals.initPublicAction(parameters.fhevm);

    // The exact extraData the user signed. Passed through verbatim so the
    // resolved signer set is the one the permit committed to.
    const permitExtraData = ensure0x(
      parameters.signedPermit.eip712.message.extraData,
    );
    const kmsExtraData =
      internals.createKmsExtraDataFromBytesHex(permitExtraData);

    const contracts = parameters.fhevm.chain.fhevm.contracts;
    const kmsSignersContext =
      await internals.readKmsSignersContextFromPermitExtraData(
        parameters.fhevm,
        {
          kmsVerifierAddress: contracts.kmsVerifier.address,
          protocolConfigAddress: contracts.protocolConfig?.address,
          extraData: kmsExtraData,
          fhevmContext,
        },
      );

    const gateway = parameters.fhevm.chain.fhevm.gateway;
    const kmsSigncryptedShares = internals.createKmsSigncryptedShares({
      metadata: {
        kmsSignersContext,
        // KMS response shares are signed on the gateway domain. The saved permit
        // itself is signed on the host-chain domain and cannot be reused.
        eip712Domain: internals.createKmsEip712Domain({
          chainId: gateway.id,
          verifyingContractAddressDecryption:
            gateway.contracts.decryption.address,
        }),
        eip712ExtraData: permitExtraData,
        eip712Signature: parameters.signedPermit.signature,
        eip712SignerAddress: parameters.signedPermit.signerAddress,
        handles: parameters.encryptedValues.map(internals.toFhevmHandle),
        // Version-locked to the resolved client, not the saved key:
        // decryptKmsSigncryptedShares asserts these agree.
        tkmsVersion: fhevmContext.tkmsVersion,
      },
      shares: normalizedShares,
    });

    const clearValues = await internals.decryptKmsSigncryptedShares(
      parameters.fhevm,
      {
        kmsSigncryptedShares,
        transportKeyPair: parameters.transportKeyPair,
        fhevmContext,
      },
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

export const decryptSavedUserDecryptResult = createSavedUserDecryptAdapter({
  sdkVersion: sdkPackage.version,
  loadInternals: loadSdkInternals,
});
