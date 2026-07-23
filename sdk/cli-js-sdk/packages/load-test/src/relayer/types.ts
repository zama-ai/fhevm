/**
 * Wire types for the relayer v2 HTTP API.
 *
 * Field names mirror the relayer's serde `rename_all = "camelCase"` structs
 * (`src/input_proof/handler.rs`, `src/public_decrypt/handlers.rs`,
 * `src/user_decrypt/handlers.rs`). Raw-hex fields are explicitly marked: the
 * relayer rejects `0x` prefixes on `ciphertextWithInputVerification`,
 * `signature`, and `publicKey`, while `extraData` and handles require the
 * prefix.
 */

export const FLOWS = [
  "input-proof",
  "public-decrypt",
  "user-decrypt",
  "delegated-user-decrypt",
] as const;
export type FlowKind = (typeof FLOWS)[number];

/** `status` values in v2 response envelopes. */
export type ApiResponseStatus = "queued" | "succeeded" | "failed";

export type ApiErrorJson = Readonly<{
  label: string;
  message: string;
  details?: readonly Readonly<{ field: string; issue: string }>[];
}>;

// -------------------------------------------------------------------------
// POST request bodies
// -------------------------------------------------------------------------

export type InputProofRequestBody = Readonly<{
  contractChainId: number;
  contractAddress: string;
  userAddress: string;
  /** ABI-encoded ciphertext + ZKPoK. Raw hex, NO `0x` prefix. */
  ciphertextWithInputVerification: string;
  /** `0x`-prefixed hex. */
  extraData: string;
}>;

export type PublicDecryptRequestBody = Readonly<{
  /** `0x` + 64 hex chars each. */
  ciphertextHandles: readonly string[];
  /** `"0x00"` or `"0x01"` + 32-byte context id. */
  extraData: string;
}>;

export type HandleContractPairBody = Readonly<{
  handle: string;
  contractAddress: string;
}>;

export type UserDecryptRequestBody = Readonly<{
  handleContractPairs: readonly HandleContractPairBody[];
  requestValidity: Readonly<{ startTimestamp: string; durationDays: string }>;
  /** Host-chain id encoded as a decimal string for the v2 relayer schema. */
  contractsChainId: string;
  contractAddresses: readonly string[];
  userAddress: string;
  /** 65-byte EIP-712 signature. Raw hex, NO `0x` prefix. */
  signature: string;
  /** Transport public key. Raw hex, NO `0x` prefix. */
  publicKey: string;
  /** `"0x00"` or `"0x01"` + 32-byte context id. */
  extraData: string;
}>;

export type DelegatedUserDecryptRequestBody = Readonly<{
  handleContractPairs: readonly HandleContractPairBody[];
  /** Host-chain id encoded as a decimal string for the v2 relayer schema. */
  contractsChainId: string;
  contractAddresses: readonly string[];
  delegatorAddress: string;
  delegateAddress: string;
  startTimestamp: string;
  durationDays: string;
  /** 65-byte EIP-712 signature. Raw hex, NO `0x` prefix. */
  signature: string;
  /** Transport public key. Raw hex, NO `0x` prefix. */
  publicKey: string;
  extraData: string;
}>;

// -------------------------------------------------------------------------
// Responses
// -------------------------------------------------------------------------

/** 202 envelope returned by every POST. */
export type PostAcceptedResponse = Readonly<{
  status: ApiResponseStatus;
  requestId: string;
  result: Readonly<{ jobId: string }>;
}>;

export type InputProofResultJson =
  | Readonly<{
      accepted: true;
      extraData: string;
      handles: readonly string[];
      signatures: readonly string[];
    }>
  | Readonly<{
      accepted: false;
      extraData: string;
    }>;

export type PublicDecryptResultJson = Readonly<{
  /** ABI-encoded cleartexts. Raw hex, NO `0x` prefix. */
  decryptedValue: string;
  /** 65-byte EIP-712 signatures. Raw hex, NO `0x` prefix. */
  signatures: readonly string[];
  extraData: string;
}>;

export type UserDecryptShareJson = Readonly<{
  /** Signcrypted share payload. Raw hex, NO `0x` prefix. */
  payload: string;
  /** KMS signature. Raw hex, NO `0x` prefix. */
  signature: string;
  extraData: string;
}>;

export type UserDecryptResultJson = Readonly<{
  result: readonly UserDecryptShareJson[];
}>;

export type FlowResultJson =
  | InputProofResultJson
  | PublicDecryptResultJson
  | UserDecryptResultJson;

/** GET envelope shared by all flows. */
export type GetJobResponse<Result extends FlowResultJson = FlowResultJson> =
  Readonly<{
    status: ApiResponseStatus;
    /** Failure responses may omit this behind a throttling proxy. */
    requestId?: string;
    result?: Result;
    error?: ApiErrorJson;
  }>;

export type HealthReadinessResponse = Readonly<{
  status?: string;
  [key: string]: unknown;
}>;

const nonEmpty = z.string().min(1);
export const apiErrorSchema = z.object({
  label: nonEmpty,
  message: z.string(),
  details: z.array(z.object({
    field: z.string(),
    issue: z.string(),
  }).strict()).optional(),
}).strict();

export const postAcceptedResponseSchema = z.object({
  status: z.literal("queued"),
  requestId: nonEmpty,
  result: z.object({ jobId: nonEmpty }).strict(),
}).strict();

export const inputProofResultSchema = z.discriminatedUnion("accepted", [
  z.object({
    accepted: z.literal(true),
    extraData: z.string(),
    handles: z.array(z.string()),
    signatures: z.array(z.string()),
  }).strict(),
  z.object({
    accepted: z.literal(false),
    extraData: z.string(),
  }).strict(),
]);

export const publicDecryptResultSchema = z.object({
  decryptedValue: z.string(),
  signatures: z.array(z.string()),
  extraData: z.string(),
}).strict();

export const userDecryptResultSchema = z.object({
  result: z.array(z.object({
    payload: z.string(),
    signature: z.string(),
    extraData: z.string(),
  }).strict()),
}).strict();

export const flowResultSchemas = {
  "input-proof": inputProofResultSchema,
  "public-decrypt": publicDecryptResultSchema,
  "user-decrypt": userDecryptResultSchema,
  "delegated-user-decrypt": userDecryptResultSchema,
} as const satisfies Record<FlowKind, z.ZodType<FlowResultJson>>;

export const queuedJobResponseSchema = z.object({
  status: z.literal("queued"),
  requestId: nonEmpty,
}).strict();

export const failedJobResponseSchema = z.object({
  status: z.literal("failed"),
  requestId: nonEmpty.optional(),
  error: apiErrorSchema,
}).strict();

export const succeededJobResponseSchema = <Result extends FlowResultJson>(
  result: z.ZodType<Result>,
) => z.object({
  status: z.literal("succeeded"),
  requestId: nonEmpty,
  result,
}).strict();
import { z } from "zod";
