import type { StageRow } from "../collectors/stage-rows";
import type { MetricsSnapshot, PrometheusCollectionStatus } from "../collectors/prometheus";
import type { InjectorRuntimeSummary } from "../collectors/injector-runtime";
import {
  discoverPrometheusCapabilities,
  mappedMetricFamily,
  type PrometheusSignal,
} from "../collectors/prom-capabilities";
import type { RelayerConfigSnapshot } from "../collectors/relayer-config";
import type { QueueDepthSample } from "../collectors/types";
import type { RequestRecord } from "../flows/types";
import type { FlowKind } from "../relayer/types";
import { plannedRequestCount, shapeModel, type Scenario } from "../scenario/schema";
import { LatencyHistogram, statsOf, type LatencyStats } from "./histogram";
import {
  counterDeltas,
  gaugePeaks,
  histogramQuantiles,
  processSummary,
} from "./prom-analysis";
import type {
  BacklogStage,
  ClientStageReport,
  CorrelationReport,
  FlowReport,
  FlowComparisonReport,
  HttpReport,
  PairComparisonReport,
  QueueDepthReport,
  RelayerMetricsReport,
  Report,
  RelayerTarget,
  StageReport,
  TargetReport,
} from "./schema";
import { buildCorrelation } from "./correlate";
import { buildDiagnosis } from "./diagnosis";
import { evaluateThresholds } from "./thresholds";

const flowDriver = (flow: FlowKind): FlowReport["driver"] =>
  flow === "input-proof" ? "raw-http" : "sdk";

const buildFlowReport = (flow: FlowKind, records: readonly RequestRecord[]): FlowReport => {
  const submitHistogram = new LatencyHistogram();
  const e2eHistogram = new LatencyHistogram();
  const pollHistogram = new LatencyHistogram();
  const byErrorLabel: Record<string, number> = {};
  const counts = {
    succeeded: 0,
    failed: 0,
    submit_failed: 0,
    verify_failed: 0,
    timed_out: 0,
    protocol_error: 0,
    aborted: 0,
  };

  for (const record of records) {
    counts[record.outcome] += 1;
    if (record.errorLabel) {
      byErrorLabel[record.errorLabel] = (byErrorLabel[record.errorLabel] ?? 0) + 1;
    }
    if (record.submitLatencyMs !== undefined) submitHistogram.record(record.submitLatencyMs);
    if (record.outcome === "succeeded" && record.e2eLatencyMs !== undefined) {
      e2eHistogram.record(record.e2eLatencyMs);
    }
    if (record.pollCount > 0) pollHistogram.record(record.pollCount);
  }

  const submitted = records.length;
  const measured = submitted - counts.aborted;
  const errors = measured - counts.succeeded;
  const pollLatencyStats = pollHistogram.stats();
  return {
    flow,
    driver: flowDriver(flow),
    submitted,
    succeeded: counts.succeeded,
    failed: counts.failed,
    submitFailed: counts.submit_failed,
    verifyFailed: counts.verify_failed,
    timedOut: counts.timed_out,
    protocolErrors: counts.protocol_error,
    aborted: counts.aborted,
    errorRate: measured === 0 ? 0 : errors / measured,
    byErrorLabel,
    submitLatency: submitHistogram.stats(),
    e2eLatency: e2eHistogram.stats(),
    pollsPerRequest: pollLatencyStats
      ? {
          count: pollLatencyStats.count,
          max: pollLatencyStats.maxMs,
          mean: pollLatencyStats.meanMs,
          p50: pollLatencyStats.p50Ms,
          p90: pollLatencyStats.p90Ms,
          p95: pollLatencyStats.p95Ms,
          p99: pollLatencyStats.p99Ms,
        }
      : undefined,
  };
};

type TargetRequestRecord = RequestRecord & Readonly<{ relayerTarget: RelayerTarget }>;

const targetRecord = (
  record: RequestRecord,
  target: RelayerTarget,
): TargetRequestRecord | undefined => {
  if (target === "A") return { ...record, relayerTarget: "A" };
  if (!record.outcomeB) return undefined;
  return {
    ...record,
    relayerTarget: "B",
    echoedRequestId: record.echoedRequestIdB,
    jobId: record.jobIdB,
    submitHttpStatus: record.submitHttpStatusB,
    submitLatencyMs: record.submitLatencyMsB,
    firstRetryAfterMs: record.firstRetryAfterMsB,
    pollCount: record.pollCountB ?? 0,
    outcome: record.outcomeB,
    errorLabel: record.errorLabelB,
    errorMessage: record.errorMessageB,
    e2eLatencyMs: record.e2eLatencyMsB,
    verified: record.verifiedB,
  };
};

const recordsForTarget = (
  records: readonly RequestRecord[],
  target: RelayerTarget,
): TargetRequestRecord[] =>
  records
    .map((record) => targetRecord(record, target))
    .filter((record): record is TargetRequestRecord => record !== undefined);

const byFlowRecords = <Record extends { flow: FlowKind }>(
  records: readonly Record[],
): Map<FlowKind, Record[]> => {
  const byFlow = new Map<FlowKind, Record[]>();
  for (const record of records) {
    const list = byFlow.get(record.flow) ?? [];
    list.push(record);
    byFlow.set(record.flow, list);
  }
  return byFlow;
};

const stageKey = (record: RequestRecord): string | undefined =>
  record.loadStage ? `${record.loadStage.index.toString()}/${record.flow}` : undefined;

const buildClientStageReports = (
  records: readonly RequestRecord[],
): ClientStageReport[] | undefined => {
  const grouped = new Map<string, RequestRecord[]>();
  for (const record of records) {
    const key = stageKey(record);
    if (!key) continue;
    const list = grouped.get(key) ?? [];
    list.push(record);
    grouped.set(key, list);
  }
  if (grouped.size === 0) return undefined;

  return [...grouped.values()]
    .map((stageRecords) => {
      const first = stageRecords[0];
      if (!first?.loadStage) throw new Error("Missing load stage for grouped records.");
      return {
        stage: first.loadStage,
        ...buildFlowReport(first.flow, stageRecords),
      };
    })
    .sort((a, b) => a.stage.index - b.stage.index || a.flow.localeCompare(b.flow));
};

/** Stage definitions over the §1.1 timestamp columns, per flow family. */
const STAGES: readonly Readonly<{
  stage: string;
  from: keyof StageRow;
  to: keyof StageRow;
  decryptOnly?: boolean;
  /** Attempt-count column flagging retries that inflate this stage. */
  attemptKey?: "readinessAttemptCount" | "broadcastAttemptCount";
}>[] = [
  { stage: "readiness_wait", from: "createdAt", to: "readinessClaimedAt", decryptOnly: true, attemptKey: "readinessAttemptCount" },
  { stage: "readiness_check", from: "readinessClaimedAt", to: "readyAt", decryptOnly: true, attemptKey: "readinessAttemptCount" },
  { stage: "broadcast_wait", from: "readyAt", to: "claimedAt", decryptOnly: true, attemptKey: "broadcastAttemptCount" },
  { stage: "queue_wait", from: "createdAt", to: "claimedAt" },
  { stage: "broadcast", from: "claimedAt", to: "broadcastedAt", attemptKey: "broadcastAttemptCount" },
  { stage: "confirmation", from: "broadcastedAt", to: "gatewayRequestConfirmedAt" },
  { stage: "gateway_response", from: "gatewayRequestConfirmedAt", to: "completedAt" },
];

/** Server-measured e2e p50 per flow (created_at → completed_at), milliseconds. */
const serverE2eP50ByFlow = (rows: readonly StageRow[]): Map<string, number> => {
  const byFlow = new Map<string, number[]>();
  for (const row of rows) {
    if (!row.createdAt || !row.completedAt) continue;
    const ms = Date.parse(row.completedAt) - Date.parse(row.createdAt);
    if (!Number.isFinite(ms) || ms < 0) continue;
    const list = byFlow.get(row.flow) ?? [];
    list.push(ms);
    byFlow.set(row.flow, list);
  }
  const p50 = new Map<string, number>();
  for (const [flow, durations] of byFlow) {
    const stats = statsOf(durations);
    if (stats) p50.set(flow, stats.p50Ms);
  }
  return p50;
};

export const buildStageReports = (rows: readonly StageRow[]): StageReport[] => {
  const reports: StageReport[] = [];
  const flows = [...new Set(rows.map((row) => row.flow))];
  const e2eP50 = serverE2eP50ByFlow(rows);
  for (const flow of flows) {
    const flowRows = rows.filter((row) => row.flow === flow);
    const isDecrypt = flow !== "input-proof";
    const flowE2e = e2eP50.get(flow);
    for (const definition of STAGES) {
      if (definition.decryptOnly && !isDecrypt) continue;
      // For decrypt flows queue_wait is covered by readiness_wait.
      if (definition.stage === "queue_wait" && isDecrypt) continue;
      const durations: number[] = [];
      let retriedCount = 0;
      for (const row of flowRows) {
        const from = row[definition.from];
        const to = row[definition.to];
        if (typeof from !== "string" || typeof to !== "string" || !from || !to) continue;
        const durationMs = Date.parse(to) - Date.parse(from);
        if (!Number.isFinite(durationMs) || durationMs < 0) continue;
        durations.push(durationMs);
        if (definition.attemptKey && row[definition.attemptKey] > 1) retriedCount += 1;
      }
      const stats = statsOf(durations);
      if (!stats) continue;
      // queue_wait overlaps broadcast/confirmation/gateway for input-proof
      // (created→claimed includes the broadcast wait), so exclude it from the
      // share so the additive stages sum to ~100%.
      const shareOfE2ePct =
        flowE2e && flowE2e > 0 && definition.stage !== "queue_wait"
          ? Math.round((stats.p50Ms / flowE2e) * 1000) / 10
          : undefined;
      reports.push({ flow, stage: definition.stage, stats, retriedCount, shareOfE2ePct });
    }
  }
  return reports;
};

/** 2xx status labels count as success; everything else is an error/throttle. */
const isSuccessStatus = (status: string | undefined): boolean =>
  status !== undefined && /^2\d\d$/.test(status);

const buildHttpReport = (
  first: MetricsSnapshot | undefined,
  last: MetricsSnapshot | undefined,
): HttpReport | undefined => {
  const rawDeltas = counterDeltas(first, last, "http");
  // Collapse bounded transport/version dimensions into the fields the report
  // actually presents. This also keeps the report stable across relayer SDK
  // label changes without retaining unnecessary cardinality.
  const grouped = new Map<string, { labels: Record<string, string>; delta: number; resetDetected?: boolean; lowerBound?: boolean }>();
  for (const entry of rawDeltas) {
    const labels = {
      endpoint: entry.labels.endpoint ?? "unknown",
      status: entry.labels.status ?? "unknown",
    };
    const key = JSON.stringify(labels);
    const existing = grouped.get(key);
    if (existing) {
      existing.delta += entry.delta;
      if (entry.resetDetected) {
        existing.resetDetected = true;
        existing.lowerBound = true;
      }
    } else {
      grouped.set(key, {
        labels,
        delta: entry.delta,
        ...(entry.resetDetected ? { resetDetected: true, lowerBound: true } : {}),
      });
    }
  }
  const deltas = [...grouped.values()];
  if (deltas.length === 0) return undefined;
  const loadEntries = deltas.filter((entry) => {
      const endpoint = (entry.labels.endpoint ?? "").replace(/^\/v\d+/, "");
      return /^\/(input-proof|public-decrypt|user-decrypt|delegated-user-decrypt)(?:\/|$)/.test(endpoint);
    });
  const nonSuccessEntries = deltas.filter((entry) => !isSuccessStatus(entry.labels.status));
  const totalRequests = deltas.reduce((total, entry) => total + entry.delta, 0);
  const loadRequests = loadEntries.reduce((total, entry) => total + entry.delta, 0);
  const nonSuccess = nonSuccessEntries.reduce((total, entry) => total + entry.delta, 0);
  const totalRequestsLowerBound = deltas.some((entry) => entry.lowerBound);
  const loadRequestsLowerBound = loadEntries.some((entry) => entry.lowerBound);
  const nonSuccessLowerBound = nonSuccessEntries.some((entry) => entry.lowerBound);
  const observedDurationSec =
    first && last && last.tMs > first.tMs ? (last.tMs - first.tMs) / 1000 : undefined;
  const perSec = (count: number): number | undefined =>
    observedDurationSec && observedDurationSec > 0
      ? Math.round((count / observedDurationSec) * 100) / 100
      : undefined;
  return {
    byEndpointStatus: [...deltas].sort((a, b) => b.delta - a.delta),
    observedDurationSec,
    totalRequests,
    totalRequestsPerSec: perSec(totalRequests),
    ...(totalRequestsLowerBound ? { totalRequestsLowerBound: true } : {}),
    loadRequests,
    loadRequestsPerSec: perSec(loadRequests),
    ...(loadRequestsLowerBound ? { loadRequestsLowerBound: true } : {}),
    nonSuccess,
    ...(nonSuccessLowerBound ? { nonSuccessLowerBound: true } : {}),
    ...(totalRequestsLowerBound ? { resetDetected: true } : {}),
  };
};

const buildRelayerMetrics = (
  snapshots: readonly MetricsSnapshot[],
  fallbackCapabilities?: ReturnType<typeof discoverPrometheusCapabilities>,
  collection?: PrometheusCollectionStatus,
): RelayerMetricsReport | undefined => {
  if (snapshots.length === 0 && !fallbackCapabilities && !collection) return undefined;
  const capabilities = snapshots.at(-1)?.capabilities ??
    fallbackCapabilities ?? discoverPrometheusCapabilities(snapshots.at(-1)?.families ?? []);
  const mapped = (signal: PrometheusSignal, canonicalName: string): MetricsSnapshot[] =>
    snapshots.map((snapshot) => {
      const family = mappedMetricFamily(snapshot.families, capabilities, signal);
      return { ...snapshot, families: family ? [{ ...family, name: canonicalName }] : [] };
    });
  const stage = mapped("stageDuration", "stage");
  const e2e = mapped("e2eDuration", "e2e");
  const terminal = mapped("terminalTotal", "terminal");
  const reclaims = mapped("reclaims", "reclaims");
  const throttler = mapped("throttlerDepth", "throttler");
  const limiter = mapped("limiterUtilization", "limiter");
  const dependency = mapped("dependencyDuration", "dependency");
  const http = mapped("httpRequests", "http");
  const v2InputProofInserted = mapped("v2InputProofInserted", "v2_input_proof_inserted");
  const v2InputProofDuration = mapped("v2InputProofDuration", "v2_input_proof_duration");
  const v2TransactionCount = mapped("v2TransactionCount", "v2_transaction_count");
  const v2TransactionDuration = mapped("v2TransactionDuration", "v2_transaction_duration");
  const v2TransactionErrors = mapped("v2TransactionErrors", "v2_transaction_errors");
  const v2RecoveryRuns = mapped("v2RecoveryRuns", "v2_recovery_runs");
  const v2RecoveryDuration = mapped("v2RecoveryDuration", "v2_recovery_duration");
  const v2RecoveryItems = mapped("v2RecoveryItems", "v2_recovery_items");
  const v2WalletLeaseOwned = mapped("v2WalletLeaseOwned", "v2_wallet_lease_owned");
  const v2WalletLeaseTransitions = mapped("v2WalletLeaseTransitions", "v2_wallet_lease_transitions");
  const v2DbErrors = mapped("v2DbErrors", "v2_db_errors");
  const v2 = capabilities.profile === "v2" ? {
    inputProofInserted: counterDeltas(v2InputProofInserted[0], v2InputProofInserted.at(-1), "v2_input_proof_inserted"),
    inputProofDuration: histogramQuantiles(v2InputProofDuration[0], v2InputProofDuration.at(-1), "v2_input_proof_duration"),
    transactionCounts: counterDeltas(v2TransactionCount[0], v2TransactionCount.at(-1), "v2_transaction_count"),
    transactionDurations: histogramQuantiles(v2TransactionDuration[0], v2TransactionDuration.at(-1), "v2_transaction_duration"),
    transactionErrors: counterDeltas(v2TransactionErrors[0], v2TransactionErrors.at(-1), "v2_transaction_errors"),
    recoveryRuns: counterDeltas(v2RecoveryRuns[0], v2RecoveryRuns.at(-1), "v2_recovery_runs"),
    recoveryDurations: histogramQuantiles(v2RecoveryDuration[0], v2RecoveryDuration.at(-1), "v2_recovery_duration"),
    recoveryItems: counterDeltas(v2RecoveryItems[0], v2RecoveryItems.at(-1), "v2_recovery_items"),
    walletLeaseOwned: gaugePeaks(v2WalletLeaseOwned, "v2_wallet_lease_owned"),
    walletLeaseTransitions: counterDeltas(v2WalletLeaseTransitions[0], v2WalletLeaseTransitions.at(-1), "v2_wallet_lease_transitions"),
    dbErrors: counterDeltas(v2DbErrors[0], v2DbErrors.at(-1), "v2_db_errors"),
  } : undefined;
  return {
    collection: collection ?? { successfulScrapes: snapshots.length, failedScrapes: 0 },
    capabilities,
    stageDurations: histogramQuantiles(stage[0], stage.at(-1), "stage"),
    e2eDurations: histogramQuantiles(e2e[0], e2e.at(-1), "e2e"),
    terminalTotals: counterDeltas(terminal[0], terminal.at(-1), "terminal"),
    reclaims: counterDeltas(reclaims[0], reclaims.at(-1), "reclaims"),
    throttlerDepth: gaugePeaks(throttler, "throttler"),
    limiterUtilization: gaugePeaks(limiter, "limiter"),
    dependencyDurations: histogramQuantiles(dependency[0], dependency.at(-1), "dependency"),
    v2,
    http: buildHttpReport(http[0], http.at(-1)),
    process: processSummary(snapshots),
  };
};

/** Non-terminal request statuses, ordered along the pipeline. */
const NON_TERMINAL_ORDER = [
  "queued",
  "processing",
  "tx_in_flight",
  "receipt_received",
  "acl_check_claimed",
  "ciphertext_ready",
  "claimed",
  "broadcasting",
  "awaiting_gateway_response",
];

/** Classify the backlog trajectory: net growth over the run vs the peak. */
const classifyTrend = (
  samples: readonly QueueDepthSample[],
): "draining" | "growing" | "flat" => {
  const first = samples[0]?.pendingTotal ?? 0;
  const last = samples.at(-1)?.pendingTotal ?? 0;
  const peak = Math.max(...samples.map((s) => s.pendingTotal));
  if (peak === 0) return "flat";
  // Ended near the peak with meaningful depth ⇒ still growing (saturation).
  if (last >= peak * 0.8 && last > 1) return "growing";
  if (last <= Math.max(1, first)) return "draining";
  return "flat";
};

const buildBacklogByStage = (samples: readonly QueueDepthSample[]): BacklogStage[] => {
  // Peak/end summed across flows per non-terminal status.
  const peak = new Map<string, number>();
  const lastSample = samples.at(-1);
  const end = new Map<string, number>();
  for (const sample of samples) {
    const perStatus = new Map<string, number>();
    for (const [key, value] of Object.entries(sample.byFlowStatus)) {
      const status = key.includes("/") ? (key.split("/")[1] ?? key) : key;
      perStatus.set(status, (perStatus.get(status) ?? 0) + value);
    }
    for (const [status, value] of perStatus) {
      peak.set(status, Math.max(peak.get(status) ?? 0, value));
    }
  }
  if (lastSample) {
    for (const [key, value] of Object.entries(lastSample.byFlowStatus)) {
      const status = key.includes("/") ? (key.split("/")[1] ?? key) : key;
      end.set(status, (end.get(status) ?? 0) + value);
    }
  }
  const statuses = [...peak.keys()].filter((status) => NON_TERMINAL_ORDER.includes(status));
  statuses.sort((a, b) => NON_TERMINAL_ORDER.indexOf(a) - NON_TERMINAL_ORDER.indexOf(b));
  return statuses
    .map((status) => ({ status, peak: peak.get(status) ?? 0, end: end.get(status) ?? 0 }))
    .filter((entry) => entry.peak > 0);
};

const buildQueueDepth = (
  source: "database" | "prometheus",
  samples: readonly QueueDepthSample[],
): QueueDepthReport | undefined => {
  if (samples.length === 0) return undefined;
  const maxSeriesPoints = 500;
  const stride = Math.max(1, Math.ceil(samples.length / maxSeriesPoints));
  return {
    source,
    sampleCount: samples.length,
    maxPending: Math.max(...samples.map((sample) => sample.pendingTotal)),
    endPending: samples.at(-1)?.pendingTotal ?? 0,
    trend: classifyTrend(samples),
    byStage: buildBacklogByStage(samples),
    series: samples
      .filter((_, index) => index % stride === 0)
      .map((sample) => ({ tMs: sample.tMs, pendingTotal: sample.pendingTotal })),
  };
};

const percentile = (sorted: readonly number[], p: number): number => {
  if (sorted.length === 0) return 0;
  const index = Math.min(
    sorted.length - 1,
    Math.max(0, Math.ceil((p / 100) * sorted.length) - 1),
  );
  return sorted[index] ?? 0;
};

const signedStatsOf = (values: readonly number[]): LatencyStats | undefined => {
  if (values.length === 0) return undefined;
  const sorted = [...values].sort((a, b) => a - b);
  const mean = values.reduce((total, value) => total + value, 0) / values.length;
  return {
    count: values.length,
    meanMs: Math.round(mean * 100) / 100,
    p50Ms: percentile(sorted, 50),
    p90Ms: percentile(sorted, 90),
    p95Ms: percentile(sorted, 95),
    p99Ms: percentile(sorted, 99),
    maxMs: sorted.at(-1) ?? 0,
  };
};

const buildPairComparison = (
  records: readonly RequestRecord[],
): PairComparisonReport | undefined => {
  const paired = records.filter((record) => record.outcomeB !== undefined);
  if (paired.length === 0) return undefined;

  const flows: FlowComparisonReport[] = [];
  for (const [flow, flowRecords] of byFlowRecords(paired)) {
    const submitDelta: number[] = [];
    const e2eDelta: number[] = [];
    const pollsDelta: number[] = [];
    let bothSucceeded = 0;
    let aOnlySucceeded = 0;
    let bOnlySucceeded = 0;
    let bothFailed = 0;
    let differentTerminalOutcome = 0;

    for (const record of flowRecords) {
      const aSucceeded = record.outcome === "succeeded";
      const bSucceeded = record.outcomeB === "succeeded";
      if (aSucceeded && bSucceeded) bothSucceeded += 1;
      else if (aSucceeded) aOnlySucceeded += 1;
      else if (bSucceeded) bOnlySucceeded += 1;
      else bothFailed += 1;
      if (record.outcomeB !== record.outcome) differentTerminalOutcome += 1;

      if (
        record.submitLatencyMs !== undefined &&
        record.submitLatencyMsB !== undefined
      ) {
        submitDelta.push(record.submitLatencyMsB - record.submitLatencyMs);
      }
      if (
        aSucceeded &&
        bSucceeded &&
        record.e2eLatencyMs !== undefined &&
        record.e2eLatencyMsB !== undefined
      ) {
        e2eDelta.push(record.e2eLatencyMsB - record.e2eLatencyMs);
      }
      if (record.pollCountB !== undefined) {
        pollsDelta.push(record.pollCountB - record.pollCount);
      }
    }

    flows.push({
      flow,
      pairs: flowRecords.length,
      bothSucceeded,
      aOnlySucceeded,
      bOnlySucceeded,
      bothFailed,
      differentTerminalOutcome,
      submitLatencyDelta: signedStatsOf(submitDelta),
      e2eLatencyDelta: signedStatsOf(e2eDelta),
      pollsDelta: signedStatsOf(pollsDelta),
    });
  }

  return { flows };
};

export type BuildReportTargetInput = Readonly<{
  target: RelayerTarget;
  relayerUrl: string;
  relayerApiPrefix?: string;
  relayerConfig?: RelayerConfigSnapshot;
  stageRows?: readonly StageRow[];
  metricsSnapshots?: readonly MetricsSnapshot[];
  metricsCapabilities?: ReturnType<typeof discoverPrometheusCapabilities>;
  metricsCollection?: PrometheusCollectionStatus;
  queueDepth?: Readonly<{
    source: "database" | "prometheus";
    samples: readonly QueueDepthSample[];
  }>;
}>;

export type BuildReportInput = Readonly<{
  scenario: Scenario;
  network: string;
  relayerUrl: string;
  relayerApiPrefix?: string;
  relayerBUrl?: string;
  relayerBApiPrefix?: string;
  startedAt: string;
  endedAt: string;
  records: readonly RequestRecord[];
  submitted: number;
  completed: number;
  abandoned: number;
  poolExhausted: boolean;
  submissionDurationMs: number;
  stoppedAtSegment?: number;
  interrupted?: boolean;
  injector?: InjectorRuntimeSummary;
  targets: readonly BuildReportTargetInput[];
}>;

export const buildReport = (input: BuildReportInput): Report => {
  const targets: TargetReport[] = input.targets.map((targetInput) => {
    const targetRecords = recordsForTarget(input.records, targetInput.target);
    const flows = [...byFlowRecords(targetRecords).entries()].map(([flow, records]) =>
      buildFlowReport(flow, records),
    );
    const stages = targetInput.stageRows?.length
      ? buildStageReports(targetInput.stageRows)
      : undefined;
    const relayerMetrics = targetInput.metricsSnapshots || targetInput.metricsCapabilities || targetInput.metricsCollection
      ? buildRelayerMetrics(targetInput.metricsSnapshots ?? [], targetInput.metricsCapabilities, targetInput.metricsCollection)
      : undefined;
    const queueDepth = targetInput.queueDepth
      ? buildQueueDepth(targetInput.queueDepth.source, targetInput.queueDepth.samples)
      : undefined;
    const correlation = targetInput.stageRows?.length
      ? buildCorrelation(targetRecords, targetInput.stageRows)
      : undefined;
    const clientStages = buildClientStageReports(targetRecords);
    return {
      target: targetInput.target,
      url: targetInput.relayerUrl,
      apiPrefix: targetInput.relayerApiPrefix,
      relayerConfig: targetInput.relayerConfig,
      flows,
      clientStages,
      stages,
      correlation: correlation && correlation.length > 0 ? correlation : undefined,
      metrics: relayerMetrics,
      queueDepth,
    };
  });

  const comparison = buildPairComparison(input.records);

  const core: Omit<Report, "thresholds" | "diagnosis"> = {
    version: 1,
    run: {
      status: input.interrupted ? "interrupted" : "completed",
      scenario: input.scenario,
      model: shapeModel(input.scenario.shape),
      network: input.network,
      relayerUrl: input.relayerUrl,
      relayerApiPrefix: input.relayerApiPrefix,
      relayerBUrl: input.relayerBUrl,
      relayerBApiPrefix: input.relayerBApiPrefix,
      startedAt: input.startedAt,
      endedAt: input.endedAt,
      plannedRequests: plannedRequestCount(input.scenario.shape) ?? input.submitted,
      submitted: input.submitted,
      completed: input.completed,
      abandoned: input.abandoned,
      poolExhausted: input.poolExhausted,
      submissionDurationMs: input.submissionDurationMs,
      achievedWorkflowsPerSec:
        input.submissionDurationMs > 0
          ? Math.round((input.submitted / (input.submissionDurationMs / 1000)) * 100) / 100
          : 0,
      stoppedAtSegment: input.stoppedAtSegment,
    },
    targets,
    comparison,
    injector: input.injector,
  };

  const diagnosis = buildDiagnosis(core);
  const report: Omit<Report, "thresholds"> = { ...core, diagnosis };
  return { ...report, thresholds: evaluateThresholds(report, input.scenario.thresholds) };
};
