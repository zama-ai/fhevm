import type { FlowKind } from "../relayer/types";

export type RequestLoadStage = Readonly<{
  /** Zero-based stage index within the scenario shape. */
  index: number;
  /** Human-readable stable label, e.g. `50vu` or `10rps`. */
  label: string;
  model: "open" | "closed" | "drain";
  startOffsetMs?: number;
  endOffsetMs?: number;
  vus?: number;
  targetRps?: number;
  fromRps?: number;
  toRps?: number;
}>;

/** Terminal classification of one load-test request. */
export type RequestOutcome =
  /** Relayer reported success and verification passed. */
  | "succeeded"
  /** POST was rejected (non-202) or transport failed before a job existed. */
  | "submit_failed"
  /** Relayer reported a terminal failure. */
  | "failed"
  /** Relayer reported success but the result failed verification. */
  | "verify_failed"
  /** No terminal status before the request deadline. */
  | "timed_out"
  | "protocol_error"
  /** Operator/run cancellation, excluded from workload timeout rates. */
  | "aborted";

export type RelayerLegRecord = Readonly<{
  /** `requestId` echoed by the relayer (proxy id wins behind Kong/CF). */
  echoedRequestId?: string;
  jobId?: string;
  submitHttpStatus?: number;
  submitLatencyMs?: number;
  /** First Retry-After hint received, in ms. */
  firstRetryAfterMs?: number;
  pollCount: number;
  outcome: RequestOutcome;
  errorLabel?: string;
  errorMessage?: string;
  e2eLatencyMs?: number;
  /** Present on decrypt flows; false implies outcome `verify_failed`. */
  verified?: boolean;
}>;

/**
 * Per-request record persisted to requests.jsonl during a run. Latencies are
 * client-observed milliseconds; `e2eLatencyMs` spans submit start to the
 * terminal poll response and is therefore quantized to the poll interval.
 */
export type RequestRecord = Readonly<{
  flow: FlowKind;
  /** Global submission index within the run. */
  index: number;
  /** Load-shape stage active when this request was submitted. */
  loadStage?: RequestLoadStage;
  /** Wall-clock submit start (epoch ms), for time-series alignment. */
  startedAtMs: number;
  /** UUID sent as `x-request-id` on every HTTP call of this request. */
  sentRequestId: string;
}> &
  RelayerLegRecord &
  Readonly<{
    /** Candidate relayer leg, present only when `--relayer-b-url` is configured. */
    echoedRequestIdB?: string;
    jobIdB?: string;
    submitHttpStatusB?: number;
    submitLatencyMsB?: number;
    firstRetryAfterMsB?: number;
    pollCountB?: number;
    outcomeB?: RequestOutcome;
    errorLabelB?: string;
    errorMessageB?: string;
    e2eLatencyMsB?: number;
    verifiedB?: boolean;
  }>;

/**
 * A flow executor owns everything needed to turn one scheduler tick into a
 * complete request record: payload acquisition, submission, polling, and
 * correctness verification.
 */
export interface FlowExecutor {
  readonly flow: FlowKind;
  /** Validates pools and preloads shared state; `planned` is the expected request count. */
  prepare(planned: number, signal?: AbortSignal): Promise<void>;
  /** Executes one workflow. The signal is aborted when the run must stop in-flight work. */
  execute(index: number, signal: AbortSignal): Promise<RequestRecord>;
  close(): Promise<void>;
}

/** Thrown when a single-use pool cannot serve another request. */
export class PoolExhaustedError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "PoolExhaustedError";
  }
}
