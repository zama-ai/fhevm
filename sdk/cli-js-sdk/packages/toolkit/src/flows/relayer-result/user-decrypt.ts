import { readKmsSignersContext } from "@fhevm/sdk/actions/base";
import {
  parseSignedDecryptionPermit,
  parseTransportKeyPair,
} from "@fhevm/sdk/actions/chain";
import type { Hex } from "viem";

import { createClientContext, type ClientOptions } from "../../config";
import type {
  DecryptedValue,
  UserDecryptValidationArtifact,
} from "../../types";

type RelayerUserDecryptShare = Readonly<{
  payload: string;
  signature: string;
  extraData?: string;
}>;

type CreateKmsSigncryptedShares = (
  context: unknown,
  parameters: {
    metadata: unknown;
    shares: readonly RelayerUserDecryptShare[];
  },
) => Promise<unknown>;

type ToFhevmHandle = (value: unknown) => unknown;

type DecryptKmsSignedcryptedShares = (
  context: unknown,
  parameters: {
    kmsSigncryptedShares: unknown;
    transportKeyPair: unknown;
  },
) => Promise<readonly unknown[]>;

export type VerifyUserDecryptResultOptions = Pick<
  ClientOptions,
  "rpcUrl"
> &
  Readonly<{
    url: string;
    artifact: UserDecryptValidationArtifact;
  }>;

type VerifyUserDecryptSharesOptions = Pick<
  ClientOptions,
  "rpcUrl"
> &
  Readonly<{
    artifact: UserDecryptValidationArtifact;
    shares: readonly RelayerUserDecryptShare[];
  }>;

export type VerifyUserDecryptSharesResult = Readonly<{
  flow: UserDecryptValidationArtifact["flow"];
  encryptedValues: readonly Hex[];
  clearValues: readonly DecryptedValue[];
  expectedClearValues?: readonly DecryptedValue[];
  valuesMatch?: boolean;
  shareCount: number;
  kmsThreshold: number;
  kmsSignerCount: number;
}>;

export type VerifyUserDecryptResult = Readonly<{
  url: string;
  httpStatus: number;
  requestId?: string;
  status?: string;
}> &
  VerifyUserDecryptSharesResult;

const strip0x = (value: string): string => value.replace(/^0x/i, "");

const asRecord = (value: unknown, label: string): Record<string, unknown> => {
  if (typeof value === "object" && value !== null && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  throw new Error(`${label} must be an object`);
};

const asString = (value: unknown, label: string): string => {
  if (typeof value === "string" && value.length > 0) return value;
  throw new Error(`${label} must be a non-empty string`);
};

const isShareLike = (value: unknown): value is RelayerUserDecryptShare => {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const record = value as Record<string, unknown>;
  return typeof record.payload === "string" && typeof record.signature === "string";
};

const findShareArray = (body: unknown): readonly RelayerUserDecryptShare[] => {
  const candidates = [
    body,
    asOptionalRecord(body)?.result,
    asOptionalRecord(asOptionalRecord(body)?.result)?.result,
    asOptionalRecord(asOptionalRecord(body)?.result)?.response,
    asOptionalRecord(body)?.response,
  ];

  for (const candidate of candidates) {
    if (Array.isArray(candidate) && candidate.every(isShareLike)) {
      return candidate;
    }
  }
  throw new Error("Could not find user-decrypt shares in relayer response");
};

const getErrorMessage = (error: unknown): string =>
  error instanceof Error ? error.message : String(error);

const asOptionalRecord = (
  value: unknown,
): Record<string, unknown> | undefined =>
  typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : undefined;

const normalizeShares = (
  shares: readonly RelayerUserDecryptShare[],
  fallbackExtraData: string,
): readonly RelayerUserDecryptShare[] =>
  shares.map((share) => ({
    payload: strip0x(share.payload),
    signature: strip0x(share.signature),
    extraData: strip0x(share.extraData ?? fallbackExtraData),
  }));

const serializeTypedValues = (values: readonly unknown[]): readonly DecryptedValue[] =>
  values.map((value, index) => {
    const record = asRecord(value, `clearValues[${index.toString()}]`);
    const rawValue = record.value;
    return {
      type: asString(record.type, `clearValues[${index.toString()}].type`),
      value:
        typeof rawValue === "bigint"
          ? rawValue.toString()
          : String(rawValue),
    };
  });

const valuesEqual = (
  expected: readonly DecryptedValue[],
  actual: readonly DecryptedValue[],
): boolean =>
  expected.length === actual.length &&
  expected.every((expectedValue, index) => {
    const actualValue = actual[index];
    if (!actualValue || actualValue.type !== expectedValue.type) return false;
    if (expectedValue.type === "address") {
      return actualValue.value.toLowerCase() === expectedValue.value.toLowerCase();
    }
    if (expectedValue.type === "bool") {
      const normalizeBool = (value: string): string =>
        value === "true" || value === "1" ? "1" : "0";
      return normalizeBool(actualValue.value) === normalizeBool(expectedValue.value);
    }
    return actualValue.value === expectedValue.value;
  });

const normalizeSerializedPermit = (value: unknown): Record<string, unknown> => {
  const permit = asRecord(value, "artifact.serializedPermit");
  const eip712 = asRecord(permit.eip712, "artifact.serializedPermit.eip712");
  const domain = asRecord(
    eip712.domain,
    "artifact.serializedPermit.eip712.domain",
  );
  const chainId = domain.chainId;
  if (typeof chainId !== "string" && typeof chainId !== "number") {
    return permit;
  }
  return {
    ...permit,
    eip712: {
      ...eip712,
      domain: {
        ...domain,
        chainId: BigInt(chainId),
      },
    },
  };
};

const loadCreateKmsSigncryptedShares = async (): Promise<CreateKmsSigncryptedShares> => {
  const packageJsonUrl = import.meta.resolve("@fhevm/sdk/package.json");
  const moduleUrl = new URL("_esm/core/kms/KmsSigncryptedShares-p.js", packageJsonUrl);
  const module = (await import(moduleUrl.href)) as {
    createKmsSigncryptedShares?: CreateKmsSigncryptedShares;
  };
  if (typeof module.createKmsSigncryptedShares !== "function") {
    throw new Error("Could not load SDK KmsSigncryptedShares builder");
  }
  return module.createKmsSigncryptedShares;
};

const loadToFhevmHandle = async (): Promise<ToFhevmHandle> => {
  const packageJsonUrl = import.meta.resolve("@fhevm/sdk/package.json");
  const moduleUrl = new URL("_esm/core/handle/FhevmHandle.js", packageJsonUrl);
  const module = (await import(moduleUrl.href)) as {
    toFhevmHandle?: ToFhevmHandle;
  };
  if (typeof module.toFhevmHandle !== "function") {
    throw new Error("Could not load SDK handle parser");
  }
  return module.toFhevmHandle;
};

const loadDecryptKmsSignedcryptedShares =
  async (): Promise<DecryptKmsSignedcryptedShares> => {
    const packageJsonUrl = import.meta.resolve("@fhevm/sdk/package.json");
    const moduleUrl = new URL(
      "_esm/core/kms/decryptKmsSignedcryptedShares-p.js",
      packageJsonUrl,
    );
    const module = (await import(moduleUrl.href)) as {
      decryptKmsSignedcryptedShares?: DecryptKmsSignedcryptedShares;
    };
    if (typeof module.decryptKmsSignedcryptedShares !== "function") {
      throw new Error("Could not load SDK KMS decrypt helper");
    }
    return module.decryptKmsSignedcryptedShares;
  };

const verifyUserDecryptShares = async (
  options: VerifyUserDecryptSharesOptions,
): Promise<VerifyUserDecryptSharesResult> => {
  if (options.artifact.schemaVersion !== 1) {
    throw new Error(
      `Unsupported artifact schemaVersion ${String(options.artifact.schemaVersion)}`,
    );
  }
  const context = createClientContext({
    network: options.artifact.network,
    rpcUrl: options.rpcUrl,
    contractAddress: options.artifact.contractAddress,
  });

  const transportKeyPair = await parseTransportKeyPair(context.fhevm, {
    publicKey: options.artifact.transportKeyPair.publicKey,
    privateKey: options.artifact.transportKeyPair.privateKey,
  });
  const signedPermit = await parseSignedDecryptionPermit(context.fhevm, {
    serializedPermit: normalizeSerializedPermit(
      options.artifact.serializedPermit,
    ) as never,
    transportKeyPair,
  });
  const kmsSignersContext = await readKmsSignersContext(context.fhevm);
  const createKmsSigncryptedShares = await loadCreateKmsSigncryptedShares();
  const toFhevmHandle = await loadToFhevmHandle();
  const decryptKmsSignedcryptedShares =
    await loadDecryptKmsSignedcryptedShares();

  const normalizedShares = normalizeShares(
    options.shares,
    signedPermit.eip712.message.extraData,
  );
  const kmsSigncryptedShares = await createKmsSigncryptedShares(context.fhevm, {
    metadata: {
      kmsSignersContext,
      eip712Domain: {
        name: "Decryption",
        version: "1",
        chainId: BigInt(context.chain.fhevm.gateway.id),
        verifyingContract: context.chain.fhevm.gateway.contracts.decryption.address,
      },
      eip712Signature: signedPermit.signature,
      eip712SignerAddress: signedPermit.signerAddress,
      handles: options.artifact.encryptedValues.map(toFhevmHandle),
    },
    shares: normalizedShares,
  });

  let decryptedValues: readonly unknown[];
  try {
    decryptedValues = await decryptKmsSignedcryptedShares(context.fhevm, {
      kmsSigncryptedShares,
      transportKeyPair,
    });
  } catch (error) {
    const artifactJobId = options.artifact.relayer?.jobId;
    const jobHint = artifactJobId
      ? ` Artifact job id: ${artifactJobId}.`
      : "";
    throw new Error(
      `Could not decrypt relayer response with the artifact transport key and permit.${jobHint} ` +
        `Make sure the artifact was captured from the same original user-decrypt request material. ` +
        `SDK error: ${getErrorMessage(error)}`,
      { cause: error },
    );
  }

  const clearValues = serializeTypedValues(decryptedValues);
  const expectedClearValues = options.artifact.expectedClearValues;

  return {
    flow: options.artifact.flow,
    encryptedValues: options.artifact.encryptedValues,
    clearValues,
    expectedClearValues,
    valuesMatch: expectedClearValues
      ? valuesEqual(expectedClearValues, clearValues)
      : undefined,
    shareCount: normalizedShares.length,
    kmsThreshold: Number(kmsSignersContext.threshold),
    kmsSignerCount: kmsSignersContext.signers.length,
  };
};

export const verifyUserDecryptResult = async (
  options: VerifyUserDecryptResultOptions,
): Promise<VerifyUserDecryptResult> => {
  const response = await fetch(options.url);
  const body = (await response.json()) as unknown;
  const envelope = asOptionalRecord(body);
  if (!response.ok) {
    const message = envelope?.error
      ? JSON.stringify(envelope.error)
      : JSON.stringify(body).slice(0, 240);
    throw new Error(`GET returned HTTP ${response.status.toString()}: ${message}`);
  }

  const verification = await verifyUserDecryptShares({
    artifact: options.artifact,
    rpcUrl: options.rpcUrl,
    shares: findShareArray(body),
  });

  return {
    ...verification,
    url: options.url,
    httpStatus: response.status,
    requestId:
      typeof envelope?.requestId === "string" ? envelope.requestId : undefined,
    status: typeof envelope?.status === "string" ? envelope.status : undefined,
  };
};
