import type { RelayerConfigSnapshot } from "../collectors/relayer-config";
import type { InjectorRuntimeSummary } from "../collectors/injector-runtime";
import type { PrometheusCapabilities } from "../collectors/prom-capabilities";
import type { PrometheusCollectionStatus } from "../collectors/prometheus";
import type { RequestLoadStage } from "../flows/types";
import type { FlowKind } from "../relayer/types";
import type { Scenario } from "../scenario/schema";
import type { LatencyStats } from "./histogram";
import type {
  CounterDelta,
  GaugePeak,
  HistogramQuantiles,
  ProcessSummary,
} from "./prom-analysis";

/**
 * Canonical machine-readable run report. Regression detection is a diff of
 * two of these documents; everything a human dashboard would show must be
 * derivable from here.
 */

export type FlowReport = Readonly<{
  flow: FlowKind;
  driver: "raw-http" | "sdk";
  submitted: number;
  succeeded: number;
  failed: number;
  submitFailed: number;
  verifyFailed: number;
  timedOut: number;
  protocolErrors: number;
  aborted: number;
  /** (failed + submitFailed + verifyFailed + timedOut) / submitted. */
  errorRate: number;
  byErrorLabel: Readonly<Record<string, number>>;
  submitLatency?: LatencyStats;
  e2eLatency?: LatencyStats;
  pollsPerRequest?: CountStats;
}>;

export type CountStats = Readonly<{
  count: number;
  max: number;
  mean: number;
  p50: number;
  p90: number;
  p95: number;
  p99: number;
}>;

export type ClientStageReport = FlowReport &
  Readonly<{
    stage: RequestLoadStage;
  }>;

export type RelayerTarget = "A" | "B";

/** DB-derived stage durations (§1.1 columns); milliseconds. */
export type StageReport = Readonly<{
  flow: string;
  stage: string;
  stats: LatencyStats;
  /** Rows whose stage includes at least one retry (attempt count > 1). */
  retriedCount: number;
  /** Stage p50 as a percentage of this flow's server-measured e2e p50. */
  shareOfE2ePct?: number;
}>;

/** Peak/end backlog in one non-terminal status, summed across flows. */
export type BacklogStage = Readonly<{
  status: string;
  peak: number;
  end: number;
}>;

export type QueueDepthReport = Readonly<{
  source: "database" | "prometheus";
  sampleCount: number;
  maxPending: number;
  endPending: number;
  /** Did the backlog grow monotonically (saturation) or drain? */
  trend: "draining" | "growing" | "flat";
  /** Per-non-terminal-status peak/end — where the backlog actually sat. */
  byStage: readonly BacklogStage[];
  /** Downsampled (tMs, pendingTotal) pairs for plotting. */
  series: readonly Readonly<{ tMs: number; pendingTotal: number }>[];
}>;

export type RelayerMetricsReport = Readonly<{
  collection: PrometheusCollectionStatus;
  capabilities: PrometheusCapabilities;
  stageDurations: readonly HistogramQuantiles[];
  e2eDurations: readonly HistogramQuantiles[];
  terminalTotals: readonly CounterDelta[];
  reclaims: readonly CounterDelta[];
  /** Legacy in-process throttler queue depth, as peak and final gauge values. */
  throttlerDepth: readonly GaugePeak[];
  /** Peak in-use vs configured max per limiter/RPC-semaphore (§1.4). */
  limiterUtilization: readonly GaugePeak[];
  /** Per-dependency RPC latency quantiles (host chain, gateway, tx submit). */
  dependencyDurations: readonly HistogramQuantiles[];
  /** V2-native families. They are not normalized into legacy request stages. */
  v2?: V2RelayerMetricsReport;
  /** Relayer-side HTTP request view (endpoint/status deltas). */
  http?: HttpReport;
  /** Process/host metrics (Linux `process_*`); primary soak readout. */
  process?: ProcessSummary;
}>;

export type V2RelayerMetricsReport = Readonly<{
  inputProofInserted: readonly CounterDelta[];
  inputProofDuration: readonly HistogramQuantiles[];
  transactionCounts: readonly CounterDelta[];
  transactionDurations: readonly HistogramQuantiles[];
  transactionErrors: readonly CounterDelta[];
  recoveryRuns: readonly CounterDelta[];
  recoveryDurations: readonly HistogramQuantiles[];
  recoveryItems: readonly CounterDelta[];
  walletLeaseOwned: readonly GaugePeak[];
  walletLeaseTransitions: readonly CounterDelta[];
  dbErrors: readonly CounterDelta[];
}>;

/** Relayer-side HTTP view from the mapped response counter (run-window deltas). */
export type HttpReport = Readonly<{
  /** Per (endpoint, status) request counts. */
  byEndpointStatus: readonly CounterDelta[];
  observedDurationSec?: number;
  totalRequests: number;
  totalRequestsPerSec?: number;
  totalRequestsLowerBound?: boolean;
  /** Requests to relayer API routes under test, excluding /metrics and health probes. */
  loadRequests: number;
  loadRequestsPerSec?: number;
  loadRequestsLowerBound?: boolean;
  /** Requests whose status label is not 2xx (4xx/5xx/throttle). */
  nonSuccess: number;
  nonSuccessLowerBound?: boolean;
  /** At least one contributing response counter reset during the run window. */
  resetDetected?: boolean;
}>;

/**
 * Client-vs-server latency reconciliation. Joins each request's client e2e
 * (poll-quantized) to the DB-measured server e2e (created→completed) by job
 * id, quantifying the overhead polling adds.
 */
export type CorrelationReport = Readonly<{
  flow: FlowKind;
  matched: number;
  clientE2e?: LatencyStats;
  serverE2e?: LatencyStats;
  /** client - server per matched request: the poll-quantization overhead. */
  pollOverhead?: LatencyStats;
}>;

export type DiagnosisSeverity = "ok" | "info" | "warn" | "critical";

export type DiagnosisFlag = Readonly<{
  severity: DiagnosisSeverity;
  message: string;
}>;

/** Synthesized verdict: the one-paragraph "what happened and why". */
export type Diagnosis = Readonly<{
  /** One-line headline rendered at the top of the report. */
  verdict: string;
  /** Stage consuming the largest share of end-to-end latency, if known. */
  bottleneckStage?: Readonly<{ flow: string; stage: string; sharePct: number }>;
  /** Limiters whose peak in-use reached (near) their configured max. */
  saturatedLimiters: readonly string[];
  flags: readonly DiagnosisFlag[];
  recommendations: readonly string[];
}>;

export type ThresholdBreach = Readonly<{
  threshold: string;
  limit: number;
  actual: number;
  flow?: FlowKind;
  target?: RelayerTarget;
}>;

export type TargetReport = Readonly<{
  target: RelayerTarget;
  url: string;
  apiPrefix?: string;
  relayerConfig?: RelayerConfigSnapshot;
  flows: readonly FlowReport[];
  clientStages?: readonly ClientStageReport[];
  stages?: readonly StageReport[];
  correlation?: readonly CorrelationReport[];
  metrics?: RelayerMetricsReport;
  queueDepth?: QueueDepthReport;
}>;

export type FlowComparisonReport = Readonly<{
  flow: FlowKind;
  pairs: number;
  bothSucceeded: number;
  aOnlySucceeded: number;
  bOnlySucceeded: number;
  bothFailed: number;
  differentTerminalOutcome: number;
  submitLatencyDelta?: LatencyStats;
  e2eLatencyDelta?: LatencyStats;
  pollsDelta?: LatencyStats;
}>;

export type PairComparisonReport = Readonly<{
  flows: readonly FlowComparisonReport[];
}>;

export type Report = Readonly<{
  version: 1;
  run: Readonly<{
    status: "completed" | "interrupted";
    scenario: Scenario;
    model: "open" | "closed" | "drain";
    network: string;
    relayerUrl: string;
    relayerApiPrefix?: string;
    relayerBUrl?: string;
    relayerBApiPrefix?: string;
    startedAt: string;
    endedAt: string;
    plannedRequests: number;
    submitted: number;
    completed: number;
    abandoned: number;
    poolExhausted: boolean;
    submissionDurationMs: number;
    /**
     * Model-aware workflow rate: delivered arrivals over the configured open
     * window, completed throughput for closed runs, or burst injection rate.
     * This is never the target's raw HTTP request rate.
     */
    achievedWorkflowsPerSec: number;
    /** Set when ramp saturation feedback stopped submission early. */
    stoppedAtSegment?: number;
  }>;
  /** Synthesized verdict + recommendations (top of the rendered report). */
  diagnosis?: Diagnosis;
  injector?: InjectorRuntimeSummary;
  targets: readonly TargetReport[];
  /** Paired A/B deltas when dual dispatch is enabled. */
  comparison?: PairComparisonReport;
  thresholds: Readonly<{
    passed: boolean;
    breaches: readonly ThresholdBreach[];
  }>;
}>;
