import { readFile } from "node:fs/promises";
import { z } from "zod";

import { scenarioSchema } from "../scenario/schema";
import type { Report } from "./schema";

const number = z.number().finite();
const nonNegative = number.nonnegative();
const integer = z.number().int().nonnegative();
const flow = z.enum(["input-proof", "public-decrypt", "user-decrypt", "delegated-user-decrypt"]);
const target = z.enum(["A", "B"]);
const latency = z.object({
  count: integer,
  meanMs: nonNegative,
  p50Ms: nonNegative,
  p90Ms: nonNegative,
  p95Ms: nonNegative,
  p99Ms: nonNegative,
  maxMs: nonNegative,
}).strict();
const signedStats = z.object({
  count: integer,
  meanMs: number,
  p50Ms: number,
  p90Ms: number,
  p95Ms: number,
  p99Ms: number,
  maxMs: number,
}).strict();
const countStats = z.object({
  count: integer,
  max: nonNegative,
  mean: nonNegative,
  p50: nonNegative,
  p90: nonNegative,
  p95: nonNegative,
  p99: nonNegative,
}).strict();
const loadStage = z.object({
  index: integer,
  label: z.string(),
  model: z.enum(["open", "closed", "drain"]),
  startOffsetMs: nonNegative.optional(),
  endOffsetMs: nonNegative.optional(),
  vus: integer.optional(),
  targetRps: nonNegative.optional(),
  fromRps: nonNegative.optional(),
  toRps: nonNegative.optional(),
}).strict();
const flowFields = {
  flow,
  driver: z.enum(["raw-http", "sdk"]),
  submitted: integer,
  succeeded: integer,
  failed: integer,
  submitFailed: integer,
  verifyFailed: integer,
  timedOut: integer,
  protocolErrors: integer,
  aborted: integer,
  errorRate: nonNegative.max(1),
  byErrorLabel: z.record(z.string(), integer),
  submitLatency: latency.optional(),
  e2eLatency: latency.optional(),
  pollsPerRequest: countStats.optional(),
} as const;
const flowReport = z.object(flowFields).strict();
const clientStage = z.object({ ...flowFields, stage: loadStage }).strict();
const labels = z.record(z.string(), z.string());
const counter = z.object({
  labels,
  delta: number,
  resetDetected: z.boolean().optional(),
  lowerBound: z.boolean().optional(),
}).strict();
const histogram = z.object({
  labels,
  count: nonNegative,
  p50: nonNegative.optional(),
  p90: nonNegative.optional(),
  p95: nonNegative.optional(),
  p99: nonNegative.optional(),
  resetDetected: z.boolean().optional(),
  lowerBound: z.boolean().optional(),
}).strict();
const gauge = z.object({ labels, peak: number, last: number }).strict();
const trend = z.object({
  first: number,
  last: number,
  peak: number,
  delta: number,
  perHour: number,
}).strict();
const processSummary = z.object({
  rss: trend.optional(),
  virtualMemory: trend.optional(),
  openFds: trend.optional(),
  maxFds: nonNegative.optional(),
  avgCpuCores: nonNegative.optional(),
  windowSec: nonNegative.optional(),
}).strict();
const v2Metrics = z.object({
  inputProofInserted: z.array(counter),
  inputProofDuration: z.array(histogram),
  transactionCounts: z.array(counter),
  transactionDurations: z.array(histogram),
  transactionErrors: z.array(counter),
  recoveryRuns: z.array(counter),
  recoveryDurations: z.array(histogram),
  recoveryItems: z.array(counter),
  walletLeaseOwned: z.array(gauge),
  walletLeaseTransitions: z.array(counter),
  dbErrors: z.array(counter),
}).strict();
const http = z.object({
  byEndpointStatus: z.array(counter),
  observedDurationSec: nonNegative.optional(),
  totalRequests: nonNegative,
  totalRequestsPerSec: nonNegative.optional(),
  totalRequestsLowerBound: z.boolean().optional(),
  loadRequests: nonNegative,
  loadRequestsPerSec: nonNegative.optional(),
  loadRequestsLowerBound: z.boolean().optional(),
  nonSuccess: nonNegative,
  nonSuccessLowerBound: z.boolean().optional(),
  resetDetected: z.boolean().optional(),
}).strict();
const metrics = z.object({
  collection: z.object({
    successfulScrapes: integer,
    failedScrapes: integer,
    lastAttemptSucceeded: z.boolean().optional(),
    lastAttemptAt: z.iso.datetime().optional(),
    lastSuccessAt: z.iso.datetime().optional(),
    lastFailureAt: z.iso.datetime().optional(),
    lastError: z.string().optional(),
  }).strict(),
  capabilities: z.object({
    profile: z.enum(["legacy", "v2", "unknown"]),
    signals: z.record(z.string(), z.object({
      available: z.boolean(),
      family: z.string().optional(),
      reason: z.string().optional(),
    }).strict()),
    discoveredFamilies: z.array(z.string()),
  }).strict(),
  stageDurations: z.array(histogram),
  e2eDurations: z.array(histogram),
  terminalTotals: z.array(counter),
  reclaims: z.array(counter),
  throttlerDepth: z.array(gauge),
  limiterUtilization: z.array(gauge),
  dependencyDurations: z.array(histogram),
  v2: v2Metrics.optional(),
  http: http.optional(),
  process: processSummary.optional(),
}).strict();
const queueDepth = z.object({
  source: z.enum(["database", "prometheus"]),
  sampleCount: integer,
  maxPending: nonNegative,
  endPending: nonNegative,
  trend: z.enum(["draining", "growing", "flat"]),
  byStage: z.array(z.object({ status: z.string(), peak: nonNegative, end: nonNegative }).strict()),
  series: z.array(z.object({ tMs: nonNegative, pendingTotal: nonNegative }).strict()),
}).strict();
const targetReport = z.object({
  target,
  url: z.url(),
  apiPrefix: z.string().optional(),
  relayerConfig: z.object({ path: z.string(), raw: z.string() }).strict().optional(),
  flows: z.array(flowReport),
  clientStages: z.array(clientStage).optional(),
  stages: z.array(z.object({
    flow: z.string(), stage: z.string(), stats: latency,
    retriedCount: integer, shareOfE2ePct: nonNegative.optional(),
  }).strict()).optional(),
  correlation: z.array(z.object({
    flow, matched: integer, clientE2e: latency.optional(),
    serverE2e: latency.optional(), pollOverhead: latency.optional(),
  }).strict()).optional(),
  metrics: metrics.optional(),
  queueDepth: queueDepth.optional(),
}).strict();
const comparisonFlow = z.object({
  flow,
  pairs: integer,
  bothSucceeded: integer,
  aOnlySucceeded: integer,
  bOnlySucceeded: integer,
  bothFailed: integer,
  differentTerminalOutcome: integer,
  submitLatencyDelta: signedStats.optional(),
  e2eLatencyDelta: signedStats.optional(),
  pollsDelta: signedStats.optional(),
}).strict();

export const reportSchema = z.object({
  version: z.literal(1),
  run: z.object({
    status: z.enum(["completed", "interrupted"]),
    scenario: scenarioSchema,
    model: z.enum(["open", "closed", "drain"]),
    network: z.string().min(1),
    relayerUrl: z.url(),
    relayerApiPrefix: z.string().optional(),
    relayerBUrl: z.url().optional(),
    relayerBApiPrefix: z.string().optional(),
    startedAt: z.iso.datetime(),
    endedAt: z.iso.datetime(),
    plannedRequests: integer,
    submitted: integer,
    completed: integer,
    abandoned: integer,
    poolExhausted: z.boolean(),
    submissionDurationMs: nonNegative,
    achievedWorkflowsPerSec: nonNegative,
    stoppedAtSegment: integer.optional(),
  }).strict(),
  diagnosis: z.object({
    verdict: z.string(),
    bottleneckStage: z.object({
      flow: z.string(), stage: z.string(), sharePct: nonNegative,
    }).strict().optional(),
    saturatedLimiters: z.array(z.string()),
    flags: z.array(z.object({
      severity: z.enum(["ok", "info", "warn", "critical"]), message: z.string(),
    }).strict()),
    recommendations: z.array(z.string()),
  }).strict().optional(),
  injector: z.object({
    sampleCount: integer,
    healthSampleCount: integer.optional(),
    scheduler: z.object({
      dispatchLagMs: z.array(nonNegative), peakInflight: integer,
      backpressureEvents: integer, dropped: integer, abandoned: integer,
    }).strict(),
    dispatchLagP95Ms: nonNegative.optional(),
    dispatchLagP99Ms: nonNegative.optional(),
    maxEventLoopLagP99Ms: nonNegative.optional(),
    peakEventLoopUtilization: nonNegative.optional(),
    peakRssBytes: nonNegative.optional(),
    cpuUserMicros: nonNegative.optional(),
    cpuSystemMicros: nonNegative.optional(),
    gcCount: integer,
    gcDurationMs: nonNegative,
    health: z.object({
      verdict: z.enum(["healthy", "degraded", "unhealthy", "indeterminate", "unavailable"]),
      reasons: z.array(z.string()),
    }).strict(),
  }).strict().optional(),
  targets: z.array(targetReport).min(1),
  comparison: z.object({ flows: z.array(comparisonFlow) }).strict().optional(),
  thresholds: z.object({
    passed: z.boolean(),
    breaches: z.array(z.object({
      threshold: z.string(), limit: number, actual: number,
      flow: flow.optional(), target: target.optional(),
    }).strict()),
  }).strict(),
}).strict();

export const parseReport = (value: unknown): Report => reportSchema.parse(value) as Report;

export const readReportFile = async (path: string): Promise<Report> => {
  let value: unknown;
  try {
    value = JSON.parse(await readFile(path, "utf8")) as unknown;
  } catch (error) {
    throw new Error(`Could not read report JSON at ${path}.`, { cause: error });
  }
  try {
    return parseReport(value);
  } catch (error) {
    throw new Error(`Invalid report schema at ${path}.`, { cause: error });
  }
};
