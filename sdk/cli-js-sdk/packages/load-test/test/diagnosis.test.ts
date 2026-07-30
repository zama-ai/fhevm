import { describe, expect, it } from "vitest";

import { discoverPrometheusCapabilities } from "../src/collectors/prom-capabilities";
import { buildDiagnosis } from "../src/report/diagnosis";
import type { Report } from "../src/report/schema";
import { scenarioSchema } from "../src/scenario/schema";

type Core = Omit<Report, "thresholds" | "diagnosis">;
type Target = Report["targets"][number];
type Flow = Target["flows"][number];
type Metrics = NonNullable<Target["metrics"]>;
type QueueDepth = NonNullable<Target["queueDepth"]>;
type CoreOverrides = Partial<Core> &
  Readonly<{
    flows?: Target["flows"];
    stages?: Target["stages"];
    relayerMetrics?: Target["metrics"];
    queueDepth?: Target["queueDepth"];
  }>;

const baseFlow = (over: Partial<Flow> = {}): Flow => ({
  flow: "input-proof",
  driver: "raw-http",
  submitted: 100,
  succeeded: 100,
  failed: 0,
  submitFailed: 0,
  verifyFailed: 0,
  timedOut: 0,
  protocolErrors: 0,
  aborted: 0,
  errorRate: 0,
  byErrorLabel: {},
  ...over,
});

const core = (over: CoreOverrides = {}): Core => {
  const { flows, stages, relayerMetrics, queueDepth, ...coreOver } = over;
  return {
    version: 1,
    run: {
      status: "completed",
      scenario: scenarioSchema.parse({
        name: "drain",
        flows: [{ flow: "input-proof", weight: 1 }],
        shape: { kind: "burst", count: 100 },
      }),
      model: "drain",
      network: "devnet",
      relayerUrl: "http://localhost:3000",
      startedAt: "2026-06-15T00:00:00.000Z",
      endedAt: "2026-06-15T00:01:00.000Z",
      plannedRequests: 100,
      submitted: 100,
      completed: 100,
      abandoned: 0,
      poolExhausted: false,
      submissionDurationMs: 500,
      achievedWorkflowsPerSec: 200,
    },
    targets: [
      {
        target: "A",
        url: "http://localhost:3000",
        flows: flows ?? [baseFlow()],
        stages,
        metrics: relayerMetrics,
        queueDepth,
      },
    ],
    ...coreOver,
  };
};

const limiter = (name: string, inUse: number, max: number) => [
  { labels: { limiter: name, state: "in_use" }, peak: inUse, last: 0 },
  { labels: { limiter: name, state: "max" }, peak: max, last: max },
];

describe("buildDiagnosis", () => {
  it("flags correctness failures as critical and refuses to bless", () => {
    const d = buildDiagnosis(
      core({ flows: [baseFlow({ succeeded: 98, verifyFailed: 2, errorRate: 0.02 })] }),
    );
    expect(d.verdict).toMatch(/FAILED/);
    expect(d.flags.some((f) => f.severity === "critical")).toBe(true);
    expect(d.recommendations.join(" ")).toMatch(/do not treat this run as a baseline/i);
  });

  it("identifies a downstream bottleneck and advises against raising broadcast limits", () => {
    const d = buildDiagnosis(
      core({
        stages: [
          { flow: "input-proof", stage: "broadcast", stats: stat(0.06), retriedCount: 0, shareOfE2ePct: 0.3 },
          { flow: "input-proof", stage: "gateway_response", stats: stat(16), retriedCount: 0, shareOfE2ePct: 72 },
        ],
        relayerMetrics: metrics({ limiterUtilization: limiter("input_proof_broadcast", 10, 10) }),
        queueDepth: queue("draining"),
      }),
    );
    expect(d.bottleneckStage).toEqual({ flow: "input-proof", stage: "gateway_response", sharePct: 72 });
    expect(d.verdict).toMatch(/gateway_response/);
    expect(d.recommendations.join(" ")).toMatch(/will not help|downstream/i);
    // Saturated limiter but backlog draining ⇒ no "raise config" recommendation.
    expect(d.recommendations.join(" ")).not.toMatch(/max_broadcasts_in_flight/);
  });

  it("recommends raising the config when a limiter saturates AND the backlog grows", () => {
    const d = buildDiagnosis(
      core({
        relayerMetrics: metrics({ limiterUtilization: limiter("input_proof_broadcast", 10, 10) }),
        queueDepth: queue("growing"),
      }),
    );
    expect(d.saturatedLimiters[0]).toMatch(/input_proof_broadcast/);
    expect(d.recommendations.join(" ")).toMatch(/max_broadcasts_in_flight/);
    expect(d.verdict).toMatch(/saturated|backlog growing/i);
  });

  it("calls a clean run healthy with no limiter saturated", () => {
    const d = buildDiagnosis(
      core({
        relayerMetrics: metrics({ limiterUtilization: limiter("input_proof_broadcast", 3, 10) }),
        queueDepth: queue("draining"),
      }),
    );
    expect(d.verdict).toMatch(/Healthy/);
    expect(d.saturatedLimiters).toHaveLength(0);
    expect(d.flags.some((f) => f.message.includes("No limiter saturated"))).toBe(true);
  });

  it("flags memory growth on a long run but stays quiet on short runs", () => {
    const leaky = {
      rss: { first: 100 * 2 ** 20, last: 400 * 2 ** 20, peak: 400 * 2 ** 20, delta: 300 * 2 ** 20, perHour: 300 * 2 ** 20 },
      windowSec: 3600,
    };
    const dLong = buildDiagnosis(core({ relayerMetrics: metrics({ process: leaky }) }));
    expect(dLong.recommendations.join(" ")).toMatch(/memory leak/i);

    const dShort = buildDiagnosis(
      core({ relayerMetrics: metrics({ process: { ...leaky, windowSec: 120 } }) }),
    );
    expect(dShort.flags.some((f) => f.message.includes("RSS"))).toBe(false);
  });
});

// --- helpers -------------------------------------------------------------
function stat(p50Seconds: number) {
  const v = p50Seconds * 1000;
  return { count: 100, meanMs: v, p50Ms: v, p90Ms: v, p95Ms: v, p99Ms: v, maxMs: v };
}
function metrics(over: Partial<Metrics & object> = {}): Metrics {
  return {
    collection: { successfulScrapes: 0, failedScrapes: 0 },
    capabilities: discoverPrometheusCapabilities([]),
    stageDurations: [],
    e2eDurations: [],
    terminalTotals: [],
    reclaims: [],
    throttlerDepth: [],
    limiterUtilization: [],
    dependencyDurations: [],
    ...over,
  };
}
function queue(trend: "draining" | "growing" | "flat"): QueueDepth {
  return {
    source: "database",
    sampleCount: 10,
    maxPending: trend === "flat" ? 0 : 100,
    endPending: trend === "growing" ? 90 : 0,
    trend,
    byStage: [],
    series: [],
  };
}
