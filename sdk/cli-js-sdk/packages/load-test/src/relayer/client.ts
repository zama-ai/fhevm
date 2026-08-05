import { Pool, type Dispatcher } from "undici";
import type { ZodType } from "zod";

import { safeArtifactText } from "../shared/safe-artifact";
import { clamp, monotonicNowMs, sleep } from "../shared/time";
import { normalizeApiPrefix } from "./api-prefix";
import {
  failedJobResponseSchema,
  flowResultSchemas,
  postAcceptedResponseSchema,
  queuedJobResponseSchema,
  succeededJobResponseSchema,
  type FlowResultJson,
  type DelegatedUserDecryptRequestBody,
  type FlowKind,
  type GetJobResponse,
  type InputProofRequestBody,
  type InputProofResultJson,
  type PostAcceptedResponse,
  type PublicDecryptRequestBody,
  type PublicDecryptResultJson,
  type UserDecryptRequestBody,
  type UserDecryptResultJson,
} from "./types";

/** Outcome of a single POST submission. */
export type SubmitOutcome = Readonly<{
  httpStatus: number;
  latencyMs: number;
  /** Parsed 202 envelope; undefined on non-202 responses. */
  accepted?: PostAcceptedResponse;
  /** `Retry-After` header in milliseconds, when present. */
  retryAfterMs?: number;
  /** error.label from a non-202 body, when parseable. */
  errorLabel?: string;
  errorMessage?: string;
  /** A success-status response violated the relayer wire contract. */
  protocolError?: boolean;
}>;

/** Outcome of polling one job to a terminal state. */
export type PollOutcome<Result> = Readonly<{
  /** HTTP status of the terminal response. */
  httpStatus: number;
  /** Number of GET requests issued. */
  pollCount: number;
  result?: Result;
  errorLabel?: string;
  errorMessage?: string;
  /** True when the overall deadline elapsed before a terminal response. */
  deadlineExceeded: boolean;
  aborted?: boolean;
  protocolError?: boolean;
}>;

export type RelayerClientOptions = Readonly<{
  /** Relayer origin, e.g. `https://relayer.testnet.zama.cloud`. */
  baseUrl: string;
  /** API route prefix, e.g. `/v2` or `/v3`. Defaults to `/v2`. */
  apiPrefix?: string;
  /** undici Pool connection cap; bounds concurrent sockets to the relayer. */
  connections?: number;
  /** Per-HTTP-call timeouts. */
  headersTimeoutMs?: number;
  bodyTimeoutMs?: number;
  /** Optional API key sent as the SDK does. */
  apiKey?: string;
}>;

export type PollOptions = Readonly<{
  /** Overall deadline for reaching a terminal state. */
  deadlineMs: number;
  /** Retry-After from the accepted POST; falls back to `defaultIntervalMs`. */
  initialRetryAfterMs?: number;
  /** Floor/ceiling applied to server-provided Retry-After waits. */
  minIntervalMs?: number;
  maxIntervalMs?: number;
  /** Fallback interval when the server omits Retry-After. */
  defaultIntervalMs?: number;
  signal?: AbortSignal;
  /** Correlation header sent on every poll. */
  requestId?: string;
}>;

const JSON_HEADERS = { "content-type": "application/json" } as const;

const parseRetryAfterMs = (headers: Dispatcher.ResponseData["headers"]): number | undefined => {
  const raw = headers["retry-after"];
  const value = Array.isArray(raw) ? raw[0] : raw;
  if (value === undefined) return undefined;
  const seconds = Number(value);
  return Number.isFinite(seconds) ? seconds * 1000 : undefined;
};

const safeJson = (text: string): unknown => {
  try {
    return JSON.parse(text);
  } catch {
    return undefined;
  }
};

const errorFromBody = (body: unknown): { label?: string; message?: string } => {
  if (typeof body !== "object" || body === null) return {};
  const error = (body as { error?: { label?: string; message?: string } }).error;
  if (typeof error !== "object" || error === null) {
    // ValidatedJson 400s use a flat `{label, message}` shape.
    const flat = body as { label?: string; message?: string };
    return { label: flat.label, message: safeArtifactText(flat.message) };
  }
  return { label: error.label, message: safeArtifactText(error.message) };
};

/**
 * Minimal relayer v2 client over an explicit undici `Pool`, giving the load
 * tool deterministic keep-alive and connection limits independent of the
 * global fetch dispatcher used by SDK-driven flows.
 */
export class RelayerClient {
  readonly baseUrl: string;
  readonly apiPrefix: string;
  private readonly pool: Pool;
  private readonly apiKey: string | undefined;

  constructor(options: RelayerClientOptions) {
    this.baseUrl = options.baseUrl.replace(/\/+$/, "");
    this.apiPrefix = normalizeApiPrefix(options.apiPrefix);
    this.apiKey = options.apiKey;
    this.pool = new Pool(this.baseUrl, {
      connections: options.connections ?? 128,
      pipelining: 1,
      headersTimeout: options.headersTimeoutMs ?? 30_000,
      bodyTimeout: options.bodyTimeoutMs ?? 30_000,
    });
  }

  async close(): Promise<void> {
    await this.pool.close();
  }

  private headers(requestId?: string): Record<string, string> {
    return {
      ...JSON_HEADERS,
      ...(requestId ? { "x-request-id": requestId } : {}),
      ...(this.apiKey ? { "x-api-key": this.apiKey } : {}),
    };
  }

  private async submit(
    path: string,
    body: unknown,
    requestId?: string,
    signal?: AbortSignal,
  ): Promise<SubmitOutcome> {
    const startedAt = monotonicNowMs();
    const response = await this.pool.request({
      path,
      method: "POST",
      headers: this.headers(requestId),
      body: JSON.stringify(body),
      signal,
    });
    const text = await response.body.text();
    const latencyMs = monotonicNowMs() - startedAt;
    const parsed = safeJson(text);

    if (response.statusCode === 202) {
      const accepted = postAcceptedResponseSchema.safeParse(parsed);
      if (!accepted.success) {
        return {
          httpStatus: response.statusCode,
          latencyMs,
          retryAfterMs: parseRetryAfterMs(response.headers),
          errorLabel: "client_protocol_error",
          errorMessage: `Invalid relayer 202 response: ${accepted.error.issues[0]?.message ?? "schema mismatch"}`,
          protocolError: true,
        };
      }
      return {
        httpStatus: response.statusCode,
        latencyMs,
        accepted: accepted.data,
        retryAfterMs: parseRetryAfterMs(response.headers),
      };
    }
    const { label, message } = errorFromBody(parsed);
    return {
      httpStatus: response.statusCode,
      latencyMs,
      retryAfterMs: parseRetryAfterMs(response.headers),
      errorLabel: label,
      errorMessage: message ?? safeArtifactText(text),
    };
  }

  submitInputProof(
    body: InputProofRequestBody,
    requestId?: string,
    signal?: AbortSignal,
  ): Promise<SubmitOutcome> {
    return this.submit(`${this.apiPrefix}/input-proof`, body, requestId, signal);
  }

  submitPublicDecrypt(
    body: PublicDecryptRequestBody,
    requestId?: string,
    signal?: AbortSignal,
  ): Promise<SubmitOutcome> {
    return this.submit(`${this.apiPrefix}/public-decrypt`, body, requestId, signal);
  }

  submitUserDecrypt(
    body: UserDecryptRequestBody,
    requestId?: string,
    signal?: AbortSignal,
  ): Promise<SubmitOutcome> {
    return this.submit(`${this.apiPrefix}/user-decrypt`, body, requestId, signal);
  }

  submitDelegatedUserDecrypt(
    body: DelegatedUserDecryptRequestBody,
    requestId?: string,
    signal?: AbortSignal,
  ): Promise<SubmitOutcome> {
    return this.submit(`${this.apiPrefix}/delegated-user-decrypt`, body, requestId, signal);
  }

  /** Single GET of a job's status. */
  async getJob<Result extends GetJobResponse["result"]>(
    flow: FlowKind,
    jobId: string,
    requestId?: string,
    signal?: AbortSignal,
  ): Promise<{
    httpStatus: number;
    body?: GetJobResponse<NonNullable<Result>>;
    retryAfterMs?: number;
  }> {
    const response = await this.pool.request({
      path: `${this.apiPrefix}/${flow}/${encodeURIComponent(jobId)}`,
      method: "GET",
      headers: this.headers(requestId),
      signal,
    });
    const text = await response.body.text();
    return {
      httpStatus: response.statusCode,
      body: safeJson(text) as GetJobResponse<NonNullable<Result>> | undefined,
      retryAfterMs: parseRetryAfterMs(response.headers),
    };
  }

  /**
   * Polls a job until a terminal response, honoring server `Retry-After`
   * (clamped) and the caller's overall deadline. A 202 keeps polling; any
   * other status is terminal. Transient transport errors are retried until
   * the deadline.
   */
  async pollJob<Result extends GetJobResponse["result"]>(
    flow: FlowKind,
    jobId: string,
    options: PollOptions,
  ): Promise<PollOutcome<NonNullable<Result>>> {
    const minInterval = options.minIntervalMs ?? 250;
    const maxInterval = options.maxIntervalMs ?? 30_000;
    const defaultInterval = options.defaultIntervalMs ?? 1_000;
    const startedAt = monotonicNowMs();
    let pollCount = 0;
    let lastHttpStatus = 0;

    const initialWait = clamp(
      options.initialRetryAfterMs ?? defaultInterval,
      minInterval,
      maxInterval,
    );
    const remainingMs = Math.max(0, options.deadlineMs - (monotonicNowMs() - startedAt));
    await sleep(Math.min(initialWait, remainingMs), options.signal);
    if (!(options.signal?.aborted ?? false) && initialWait >= remainingMs) {
      return {
        httpStatus: lastHttpStatus,
        pollCount,
        errorLabel: "client_poll_deadline_exceeded",
        deadlineExceeded: true,
        aborted: false,
      };
    }

    while (monotonicNowMs() - startedAt < options.deadlineMs) {
      if (options.signal?.aborted) break;
      let httpStatus: number;
      let body: GetJobResponse<NonNullable<Result>> | undefined;
      let retryAfterMs: number | undefined;
      try {
        pollCount += 1;
        ({ httpStatus, body, retryAfterMs } = await this.getJob<Result>(
          flow,
          jobId,
          options.requestId,
          options.signal,
        ));
      } catch {
        if (options.signal?.aborted) break;
        // Transport error: back off briefly and retry until the deadline.
        try {
          await sleep(defaultInterval, options.signal);
        } catch {
          if (options.signal?.aborted) break;
          throw new Error("Relayer transport retry wait failed unexpectedly.");
        }
        continue;
      }
      lastHttpStatus = httpStatus;

      if (httpStatus === 202) {
        const queued = queuedJobResponseSchema.safeParse(body);
        if (!queued.success) {
          return {
            httpStatus,
            pollCount,
            errorLabel: "client_protocol_error",
            errorMessage: `Invalid relayer queued response: ${queued.error.issues[0]?.message ?? "schema mismatch"}`,
            deadlineExceeded: false,
            protocolError: true,
          };
        }
        const wait = clamp(retryAfterMs ?? defaultInterval, minInterval, maxInterval);
        try {
          await sleep(wait, options.signal);
        } catch {
          if (options.signal?.aborted) break;
          throw new Error("Relayer polling wait failed unexpectedly.");
        }
        continue;
      }

      if (httpStatus === 200) {
        const resultSchema = flowResultSchemas[flow] as ZodType<FlowResultJson>;
        const succeeded = succeededJobResponseSchema(resultSchema).safeParse(body);
        if (!succeeded.success) {
          return {
            httpStatus,
            pollCount,
            errorLabel: "client_protocol_error",
            errorMessage: `Invalid relayer success response: ${succeeded.error.issues[0]?.message ?? "schema mismatch"}`,
            deadlineExceeded: false,
            protocolError: true,
          };
        }
        return {
          httpStatus,
          pollCount,
          result: succeeded.data.result as unknown as NonNullable<Result>,
          deadlineExceeded: false,
        };
      }

      const failed = failedJobResponseSchema.safeParse(body);
      if (!failed.success) {
        return {
          httpStatus,
          pollCount,
          errorLabel: "client_protocol_error",
          errorMessage: `Invalid relayer failure response: ${failed.error.issues[0]?.message ?? "schema mismatch"}`,
          deadlineExceeded: false,
          protocolError: true,
        };
      }
      return {
        httpStatus,
        pollCount,
        errorLabel: failed.data.error.label,
        errorMessage: safeArtifactText(failed.data.error.message),
        deadlineExceeded: false,
      };
    }

    return {
      httpStatus: lastHttpStatus,
      pollCount,
      errorLabel: "client_poll_deadline_exceeded",
      deadlineExceeded: !(options.signal?.aborted ?? false),
      aborted: options.signal?.aborted ?? false,
    };
  }

  // TODO: readiness path is hardcoded; some relayer implementations expose a
  // different one. Support a future per-target { healthPath } override.
  /** GET /health/readiness; returns true on HTTP 200. */
  async isReady(): Promise<boolean> {
    try {
      const response = await this.pool.request({
        path: "/health/readiness",
        method: "GET",
      });
      await response.body.text();
      return response.statusCode === 200;
    } catch {
      return false;
    }
  }

  // TODO: metrics path is hardcoded; the metrics URL may differ per relayer
  // implementation. Support a future per-target { metricsUrl } override.
  /** Raw Prometheus exposition text from GET /metrics. */
  async metricsText(): Promise<string> {
    const response = await this.pool.request({ path: "/metrics", method: "GET" });
    if (response.statusCode !== 200) {
      await response.body.text();
      throw new Error(`GET /metrics returned ${response.statusCode.toString()}`);
    }
    return response.body.text();
  }
}

export type InputProofPoll = PollOutcome<InputProofResultJson>;
export type PublicDecryptPoll = PollOutcome<PublicDecryptResultJson>;
export type UserDecryptPoll = PollOutcome<UserDecryptResultJson>;
