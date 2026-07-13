import { parseSignedDecryptionPermit } from "@fhevm/sdk/actions/chain";
import type { Hex } from "viem";

import {
  createDecryptClientContext,
  resolveChain,
  type ClientOptions,
} from "../../config";
import {
  decryptSavedUserDecryptResult,
  parseAlpha8TransportKeyPair,
  type SavedUserDecryptShare,
} from "../../sdk-alpha8-saved-user-decrypt-adapter";
import type {
  DecryptedValue,
  UserDecryptValidationArtifact,
} from "../../types";

type RelayerUserDecryptShare = Readonly<{
  payload: string;
  signature: string;
  extraData?: string;
}>;

/** Overrides for deriving the relayer GET result URL from an artifact. */
export type BuildUserDecryptResultUrlOptions = Readonly<{
  /** Full URL override; wins over every derived component. */
  url?: string;
  /** Relayer base URL override; falls back to the artifact network config. */
  relayerUrl?: string;
  /** Job id override; falls back to the artifact's relayer.jobId. */
  jobId?: string;
}>;

const FLOW_RESULT_PATHS = {
  "user-decrypt": "v2/user-decrypt",
  "delegated-user-decrypt": "v2/delegated-user-decrypt",
} as const satisfies Record<UserDecryptValidationArtifact["flow"], string>;

/**
 * Derives the relayer GET result URL for a user-decrypt artifact.
 *
 * `url` short-circuits everything for exotic relayers. Otherwise the base comes
 * from `relayerUrl` or the artifact network config, the path segment from the
 * artifact flow, and the job id from `jobId` or the artifact. The job id is the
 * only binding identifier for a relayer job.
 */
export const buildUserDecryptResultUrl = (
  artifact: UserDecryptValidationArtifact,
  options: BuildUserDecryptResultUrlOptions = {},
): string => {
  if (options.url) return options.url;

  const jobId = options.jobId ?? artifact.relayer?.jobId;
  if (!jobId) {
    throw new Error(
      "Artifact has no relayer.jobId; pass --job-id <id> or --url <full-url> to target the relayer result.",
    );
  }

  const base = resolveChain({
    network: artifact.network,
    relayerUrl: options.relayerUrl,
  }).fhevm.relayerUrl;
  return `${base}/${FLOW_RESULT_PATHS[artifact.flow]}/${encodeURIComponent(jobId)}`;
};

export type VerifyUserDecryptResultOptions = Pick<
  ClientOptions,
  "rpcUrl"
> &
  BuildUserDecryptResultUrlOptions &
  Readonly<{
    artifact: UserDecryptValidationArtifact;
    signal?: AbortSignal;
    timeoutMs?: number;
    maxResponseBytes?: number;
    /** Supplies authorization without persisting it in the validation artifact. */
    authHeaders?: (url: URL) =>
      | Readonly<Record<string, string>>
      | Promise<Readonly<Record<string, string>>>;
  }>;

export type VerifyUserDecryptSharesOptions = Pick<
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
  kmsContextId: string;
  kmsEpochId: string;
  kmsThreshold: number;
  kmsSignerCount: number;
  provenance: Readonly<{
    shares: "kms-cryptographically-verified";
    permit: "signature-verified";
    ownerAndDelegation: "permit-verified" | "artifact-asserted";
    expectedClearValues: "artifact-asserted" | "not-provided";
  }>;
}>;

export type VerifyUserDecryptResult = Readonly<{
  url: string;
  httpStatus: number;
  /** Binding relayer job identity the result was fetched for. */
  jobId?: string;
  /** Request-scoped provenance echoed by the relayer; informational only. */
  requestId?: string;
  status?: string;
  responseIdentity: Readonly<{
    requestId: "artifact-matched" | "unbound";
    jobId: "response-artifact-matched" | "url-artifact-matched" | "unbound";
  }>;
}> &
  VerifyUserDecryptSharesResult;

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

const isSensitiveKey = (key: string): boolean =>
  ["apikey", "authorization", "privatekey", "accesstoken"].includes(
    key.toLowerCase().replace(/[-_ ]/g, ""),
  );

const structurallyRedact = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.map(structurallyRedact);
  if (typeof value !== "object" || value === null) return value;
  return Object.fromEntries(Object.entries(value as Record<string, unknown>).map(
    ([key, nested]) => [key, isSensitiveKey(key) ? "[redacted]" : structurallyRedact(nested)],
  ));
};

const boundedRedactedMessage = (value: unknown, maxLength = 320): string => {
  const serialized = typeof value === "string"
    ? value
    : JSON.stringify(structurallyRedact(value)) ?? String(value);
  return serialized
    .replace(
      /((?:api[-_ ]?key|authorization|private[-_ ]?key|access[-_ ]?token)"?\s*[:=]\s*"?)[^",;}\r\n]*/gi,
      "$1[redacted]",
    )
    .replace(/\bBearer\s+[^",;}\s]+/gi, "Bearer [redacted]")
    .replace(/[\r\n\t]+/g, " ")
    .slice(0, maxLength);
};

const assertSame = (actual: string, expected: string, label: string): void => {
  if (actual.toLowerCase() !== expected.toLowerCase()) {
    throw new Error(`${label} does not match the signed permit.`);
  }
};

const asOptionalRecord = (
  value: unknown,
): Record<string, unknown> | undefined =>
  typeof value === "object" && value !== null && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : undefined;

const normalizeShares = (
  shares: readonly RelayerUserDecryptShare[],
  fallbackExtraData: string,
): readonly SavedUserDecryptShare[] =>
  shares.map((share) => ({
    payload: share.payload,
    signature: share.signature,
    extraData: share.extraData ?? fallbackExtraData,
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
      const normalizeBool = (value: string): string | undefined =>
        value === "true" || value === "1"
          ? "1"
          : value === "false" || value === "0"
            ? "0"
            : undefined;
      const expectedBool = normalizeBool(expectedValue.value);
      return expectedBool !== undefined && normalizeBool(actualValue.value) === expectedBool;
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

const crossCheckArtifact = (
  artifact: UserDecryptValidationArtifact,
  signedPermit: Awaited<ReturnType<typeof parseSignedDecryptionPermit>>,
): "permit-verified" | "artifact-asserted" => {
  if ((artifact.flow === "delegated-user-decrypt") !== artifact.isDelegated) {
    throw new Error("Artifact flow and isDelegated fields disagree.");
  }
  assertSame(artifact.signerAddress, signedPermit.signerAddress, "Artifact signerAddress");
  assertSame(artifact.permit.signerAddress, signedPermit.signerAddress, "Permit summary signerAddress");
  assertSame(artifact.permit.signature, signedPermit.signature, "Permit summary signature");
  assertSame(
    artifact.permit.transportPublicKey,
    signedPermit.transportPublicKey,
    "Permit summary transportPublicKey",
  );
  if (artifact.permit.version !== signedPermit.version) {
    throw new Error("Permit summary version does not match the signed permit.");
  }
  if (artifact.permit.isDelegated !== artifact.isDelegated) {
    throw new Error("Permit summary and artifact delegation fields disagree.");
  }
  assertSame(
    artifact.permit.encryptedDataOwnerAddress,
    artifact.ownerAddress,
    "Permit summary encryptedDataOwnerAddress",
  );
  if (artifact.handleContractPairs.length !== artifact.encryptedValues.length) {
    throw new Error("Artifact handleContractPairs length does not match encryptedValues.");
  }
  for (const [index, handle] of artifact.encryptedValues.entries()) {
    const pair = artifact.handleContractPairs[index];
    if (!pair || pair.handle.toLowerCase() !== handle.toLowerCase()) {
      throw new Error(`Artifact handleContractPairs[${index.toString()}] does not match encryptedValues.`);
    }
    assertSame(pair.contractAddress, artifact.contractAddress, `Artifact contract pair ${index.toString()}`);
  }

  const message = asRecord(signedPermit.eip712.message, "signedPermit.eip712.message");
  const signedContracts = message.allowedContracts ?? message.contractAddresses;
  if (Array.isArray(signedContracts)) {
    const normalized = signedContracts.map((value) => asString(value, "signed permit contract" ).toLowerCase());
    if (
      normalized.length > 0 &&
      !normalized.includes(artifact.contractAddress.toLowerCase())
    ) {
      throw new Error("Artifact contractAddress is not authorized by the signed permit.");
    }
    const summary = artifact.permit.contractAddresses.map((value) => value.toLowerCase());
    if (summary.length !== normalized.length || summary.some((value, index) => value !== normalized[index])) {
      throw new Error("Permit summary contractAddresses do not match the signed permit.");
    }
  }
  const durationSeconds = message.durationSeconds ??
    (message.durationDays === undefined
      ? undefined
      : BigInt(message.durationDays as bigint | number | string) * 86_400n);
  if (
    durationSeconds !== undefined &&
    BigInt(artifact.permit.durationSeconds) !== BigInt(durationSeconds as bigint | number | string)
  ) {
    throw new Error("Permit summary durationSeconds does not match the signed permit.");
  }
  const startTimestamp = message.startTimestamp;
  if (
    startTimestamp !== undefined &&
    BigInt(artifact.permit.startTimestamp) !== BigInt(startTimestamp as bigint | number | string)
  ) {
    throw new Error("Permit summary startTimestamp does not match the signed permit.");
  }

  if (signedPermit.version === 1) {
    assertSame(artifact.ownerAddress, signedPermit.encryptedDataOwnerAddress, "Artifact ownerAddress");
    if (artifact.isDelegated !== signedPermit.isDelegated) {
      throw new Error("Artifact delegation does not match the signed V1 permit.");
    }
    return "permit-verified";
  }
  if (!artifact.isDelegated) {
    assertSame(artifact.ownerAddress, signedPermit.signerAddress, "Artifact ownerAddress");
  }
  return "artifact-asserted";
};

export const verifyUserDecryptShares = async (
  options: VerifyUserDecryptSharesOptions,
): Promise<VerifyUserDecryptSharesResult> => {
  const schemaVersion = (
    options.artifact as { readonly schemaVersion?: unknown }
  ).schemaVersion;
  if (schemaVersion !== 2) {
    throw new Error(
      `Unsupported artifact schemaVersion ${String(schemaVersion)}; expected 2`,
    );
  }

  const tkmsVersion = (
    options.artifact.transportKeyPair as { readonly tkmsVersion?: unknown }
  ).tkmsVersion;
  if (tkmsVersion !== "0.13.10" && tkmsVersion !== "0.13.20-0") {
    throw new Error(
      "Artifact transportKeyPair.tkmsVersion must be 0.13.10 or 0.13.20-0",
    );
  }

  const context = createDecryptClientContext(
    {
      network: options.artifact.network,
      rpcUrl: options.rpcUrl,
      contractAddress: options.artifact.contractAddress,
    },
    tkmsVersion,
  );
  await context.fhevm.ready;
  if (context.fhevm.tkmsVersion !== tkmsVersion) {
    throw new Error(
      `Saved transport key TKMS version ${tkmsVersion} does not match the resolved decrypt client version ${context.fhevm.tkmsVersion}.`,
    );
  }

  const transportKeyPair = await parseAlpha8TransportKeyPair(context.fhevm, {
    publicKey: options.artifact.transportKeyPair.publicKey,
    privateKey: options.artifact.transportKeyPair.privateKey,
    tkmsVersion,
  });
  const signedPermit = await parseSignedDecryptionPermit(context.fhevm, {
    serializedPermit: normalizeSerializedPermit(
      options.artifact.serializedPermit,
    ) as never,
    transportKeyPair,
  });
  const ownerAndDelegationProvenance = crossCheckArtifact(
    options.artifact,
    signedPermit,
  );
  const normalizedShares = normalizeShares(
    options.shares,
    signedPermit.eip712.message.extraData,
  );
  let verification: Awaited<
    ReturnType<typeof decryptSavedUserDecryptResult>
  >;
  try {
    verification = await decryptSavedUserDecryptResult({
      fhevm: context.fhevm,
      encryptedValues: options.artifact.encryptedValues,
      signedPermit,
      transportKeyPair,
      shares: normalizedShares,
    });
  } catch (error) {
    const artifactJobId = options.artifact.relayer?.jobId;
    const jobHint = artifactJobId
      ? ` Artifact job id: ${artifactJobId}.`
      : "";
    throw new Error(
      `Could not decrypt relayer response with the artifact transport key and permit.${jobHint} ` +
        `Make sure the artifact was captured from the same original user-decrypt request material. ` +
        `SDK error: ${boundedRedactedMessage(getErrorMessage(error))}`,
      { cause: error },
    );
  }

  const clearValues = serializeTypedValues(verification.clearValues);
  const expectedClearValues = options.artifact.expectedClearValues;

  return {
    flow: options.artifact.flow,
    encryptedValues: options.artifact.encryptedValues,
    clearValues,
    expectedClearValues,
    valuesMatch: expectedClearValues
      ? valuesEqual(expectedClearValues, clearValues)
      : undefined,
    shareCount: verification.verification.shareCount,
    kmsContextId: verification.verification.kmsContextId.toString(),
    kmsEpochId: verification.verification.kmsEpochId.toString(),
    kmsThreshold: verification.verification.kmsThreshold,
    kmsSignerCount: verification.verification.kmsSignerCount,
    provenance: {
      shares: "kms-cryptographically-verified",
      permit: "signature-verified",
      ownerAndDelegation: ownerAndDelegationProvenance,
      expectedClearValues: expectedClearValues
        ? "artifact-asserted"
        : "not-provided",
    },
  };
};

const readBoundedJson = async (
  response: Response,
  maxBytes: number,
): Promise<unknown> => {
  if (!response.body) throw new Error("Relayer response has no body.");
  const reader = response.body.getReader();
  const chunks: Uint8Array[] = [];
  let total = 0;
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      total += value.byteLength;
      if (total > maxBytes) {
        await reader.cancel();
        throw new Error(`Relayer response exceeds ${maxBytes.toString()} bytes.`);
      }
      chunks.push(value);
    }
  } finally {
    reader.releaseLock();
  }
  const bytes = new Uint8Array(total);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  try {
    return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes)) as unknown;
  } catch (error) {
    throw new Error("Relayer response is not valid UTF-8 JSON.", { cause: error });
  }
};

export const verifyUserDecryptResult = async (
  options: VerifyUserDecryptResultOptions,
): Promise<VerifyUserDecryptResult> => {
  const resultUrl = buildUserDecryptResultUrl(options.artifact, {
    url: options.url,
    relayerUrl: options.relayerUrl,
    jobId: options.jobId,
  });
  const url = new URL(resultUrl);
  if (url.protocol !== "http:" && url.protocol !== "https:") {
    throw new Error("Relayer result URL must use HTTP or HTTPS.");
  }
  const timeoutMs = options.timeoutMs ?? 30_000;
  const maxResponseBytes = options.maxResponseBytes ?? 2 * 1024 * 1024;
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs <= 0) {
    throw new RangeError("Verifier timeoutMs must be a positive safe integer.");
  }
  if (!Number.isSafeInteger(maxResponseBytes) || maxResponseBytes <= 0) {
    throw new RangeError("Verifier maxResponseBytes must be a positive safe integer.");
  }
  const timeoutSignal = AbortSignal.timeout(timeoutMs);
  const signal = options.signal
    ? AbortSignal.any([options.signal, timeoutSignal])
    : timeoutSignal;
  const headers = await options.authHeaders?.(url);
  const response = await fetch(url, {
    signal,
    redirect: "error",
    ...(headers ? { headers } : {}),
  });
  const body = await readBoundedJson(response, maxResponseBytes);
  const envelope = asOptionalRecord(body);
  if (!response.ok) {
    const message = boundedRedactedMessage(
      envelope?.error ?? body,
      240,
    );
    throw new Error(`GET returned HTTP ${response.status.toString()}: ${message}`);
  }

  const responseRequestId = typeof envelope?.requestId === "string"
    ? envelope.requestId
    : undefined;
  const responseJobId = typeof envelope?.jobId === "string"
    ? envelope.jobId
    : undefined;
  const artifactRequestId = options.artifact.relayer?.requestId;
  const artifactJobId = options.artifact.relayer?.jobId;
  if (artifactRequestId && responseRequestId && artifactRequestId !== responseRequestId) {
    throw new Error("Relayer response requestId does not match the validation artifact.");
  }
  if (artifactJobId && responseJobId && artifactJobId !== responseJobId) {
    throw new Error("Relayer response jobId does not match the validation artifact.");
  }
  const encodedUrlJobId = url.pathname.split("/").filter(Boolean).at(-1);
  let urlJobId: string | undefined;
  try {
    urlJobId = encodedUrlJobId === undefined
      ? undefined
      : decodeURIComponent(encodedUrlJobId);
  } catch (error) {
    throw new Error("Relayer result URL contains an invalid encoded job id.", { cause: error });
  }
  if (artifactJobId && !responseJobId && urlJobId && artifactJobId !== urlJobId) {
    throw new Error("Relayer result URL job id does not match the validation artifact.");
  }

  const verification = await verifyUserDecryptShares({
    artifact: options.artifact,
    rpcUrl: options.rpcUrl,
    shares: findShareArray(body),
  });

  return {
    ...verification,
    url: resultUrl,
    httpStatus: response.status,
    jobId: responseJobId ?? urlJobId ?? artifactJobId,
    requestId: responseRequestId,
    status: typeof envelope?.status === "string" ? envelope.status : undefined,
    responseIdentity: {
      requestId: artifactRequestId && responseRequestId ? "artifact-matched" : "unbound",
      jobId: artifactJobId && responseJobId
        ? "response-artifact-matched"
        : artifactJobId && urlJobId
          ? "url-artifact-matched"
          : "unbound",
    },
  };
};
