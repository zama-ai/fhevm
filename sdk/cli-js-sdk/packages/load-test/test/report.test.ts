import { describe, expect, it } from "vitest";

import type { StageRow } from "../src/collectors/stage-rows";
import type { QueueDepthSample } from "../src/collectors/types";
import type { RequestRecord } from "../src/flows/types";
import { buildReport, buildStageReports } from "../src/report/build";
import { diffReports } from "../src/report/diff";
import { renderMarkdownReport } from "../src/report/render-md";
import { parseReport } from "../src/report/runtime";
import { parsePrometheusText } from "../src/collectors/prom-parse";
import { scenarioSchema, type Scenario } from "../src/scenario/schema";

const scenario: Scenario = scenarioSchema.parse({
  name: "test",
  flows: [{ flow: "input-proof", weight: 1 }],
  shape: { kind: "constant", rps: 1, durationSec: 10 },
  thresholds: { maxErrorRate: 0.1, maxVerifyFailures: 0 },
});

const record = (overrides: Partial<RequestRecord>): RequestRecord => ({
  flow: "input-proof",
  index: 0,
  startedAtMs: 0,
  sentRequestId: "id",
  pollCount: 2,
  outcome: "succeeded",
  submitLatencyMs: 50,
  e2eLatencyMs: 1500,
  verified: true,
  ...overrides,
});

const build = (records: RequestRecord[], testScenario = scenario) => {
  const hasB = records.some((entry) => entry.outcomeB !== undefined);
  return buildReport({
    scenario: testScenario,
    network: "testnet",
    relayerUrl: "http://localhost:3000",
    relayerBUrl: hasB ? "http://localhost:3001" : undefined,
    startedAt: "2026-01-01T00:00:00.000Z",
    endedAt: "2026-01-01T00:10:00.000Z",
    records,
    submitted: records.length,
    completed: records.length,
    abandoned: 0,
    poolExhausted: false,
    submissionDurationMs: 10_000,
    targets: [
      { target: "A", relayerUrl: "http://localhost:3000" },
      ...(hasB ? [{ target: "B" as const, relayerUrl: "http://localhost:3001" }] : []),
    ],
  });
};

describe("buildReport", () => {
  it("round-trips through the strict versioned runtime schema", () => {
    const report = build([record({})]);
    expect(parseReport(JSON.parse(JSON.stringify(report)))).toEqual(report);
    expect(() => parseReport({ ...report, version: 2 })).toThrow();
    expect(() => parseReport({ ...report, unexpected: true })).toThrow();
  });

  it("aggregates outcomes, error labels, and latency percentiles per flow", () => {
    const report = build([
      record({ index: 0 }),
      record({ index: 1, e2eLatencyMs: 2500 }),
      record({
        index: 2,
        outcome: "failed",
        errorLabel: "gw_ciphertext_failed",
        verified: undefined,
      }),
    ]);
    const flow = report.targets[0]?.flows[0];
    expect(flow?.submitted).toBe(3);
    expect(flow?.driver).toBe("raw-http");
    expect(flow?.succeeded).toBe(2);
    expect(flow?.failed).toBe(1);
    expect(flow?.errorRate).toBeCloseTo(1 / 3, 5);
    expect(flow?.byErrorLabel.gw_ciphertext_failed).toBe(1);
    // Failed requests are excluded from the e2e histogram.
    expect(flow?.e2eLatency?.count).toBe(2);
    expect(report.run.achievedWorkflowsPerSec).toBeCloseTo(0.3, 5);
    expect(report.run.model).toBe("open");
  });

  it("aggregates paired relayer target summaries", () => {
    const report = build([
      record({
        index: 0,
        jobId: "job-a",
        jobIdB: "job-b",
        outcomeB: "succeeded",
        pollCountB: 3,
        submitLatencyMsB: 70,
        e2eLatencyMsB: 1800,
        verifiedB: true,
      }),
      record({
        index: 1,
        outcome: "failed",
        errorLabel: "primary_failed",
        outcomeB: "timed_out",
        pollCountB: 5,
        errorLabelB: "candidate_timeout",
      }),
    ]);

    expect(report.targets).toHaveLength(2);
    const primary = report.targets.find((target) => target.target === "A")?.flows[0];
    const candidate = report.targets.find((target) => target.target === "B")?.flows[0];
    expect(primary?.failed).toBe(1);
    expect(primary?.byErrorLabel.primary_failed).toBe(1);
    expect(candidate?.succeeded).toBe(1);
    expect(candidate?.timedOut).toBe(1);
    expect(candidate?.byErrorLabel.candidate_timeout).toBe(1);
    expect(candidate?.e2eLatency?.p50Ms).toBe(1800);
    expect(report.thresholds.breaches).toContainEqual(expect.objectContaining({
      threshold: "maxErrorRate",
      target: "B",
      actual: 0.5,
    }));

    const markdown = renderMarkdownReport(report);
    expect(markdown).toContain("## Client Outcome Summary");
    expect(markdown).toContain("## Client Latency Comparison");
    expect(markdown).toContain("## Polling Comparison");
    expect(markdown).toContain("| input-proof | e2e p50 | 1500 ms | 1800 ms | +300 ms | +20.0% |");
    expect(markdown).toContain("| input-proof | 2.00 | 4.00 | +2.00 | 2.00 | 5.00 | +3.00 |");
    expect(markdown).toContain("## Paired Latency Delta");
    expect(markdown).toContain("Deltas are client-observed `B - A` for the paired workload item");
  });

  it("round-trips signed paired deltas when the candidate is faster", () => {
    const report = build([record({
      submitLatencyMs: 100,
      e2eLatencyMs: 1_000,
      pollCount: 3,
      outcomeB: "succeeded",
      submitLatencyMsB: 50,
      e2eLatencyMsB: 500,
      pollCountB: 1,
      verifiedB: true,
    })]);

    expect(report.comparison?.flows[0]?.e2eLatencyDelta?.p50Ms).toBe(-500);
    expect(parseReport(JSON.parse(JSON.stringify(report)))).toEqual(report);
  });

  it("marks decrypt flows as SDK-driven in flow reports", () => {
    const sdkScenario = scenarioSchema.parse({
      name: "sdk",
      flows: [{ flow: "user-decrypt", weight: 1 }],
      shape: { kind: "closed", vus: 1, durationSec: 1 },
    });
    const report = build([record({ flow: "user-decrypt" })], sdkScenario);
    expect(report.run.model).toBe("closed");
    expect(report.targets[0]?.flows[0]?.driver).toBe("sdk");
  });

  it("aggregates client metrics by load stage", () => {
    const report = build([
      record({
        index: 0,
        loadStage: { index: 0, label: "10vu", model: "closed", vus: 10 },
        e2eLatencyMs: 1000,
      }),
      record({
        index: 1,
        loadStage: { index: 0, label: "10vu", model: "closed", vus: 10 },
        e2eLatencyMs: 2000,
      }),
      record({
        index: 2,
        loadStage: { index: 1, label: "20vu", model: "closed", vus: 20 },
        e2eLatencyMs: 4000,
      }),
    ]);
    expect(report.targets[0]?.clientStages?.map((stage) => stage.stage.label)).toEqual([
      "10vu",
      "20vu",
    ]);
    expect(report.targets[0]?.clientStages?.[0]?.submitted).toBe(2);
    expect(report.targets[0]?.clientStages?.[1]?.e2eLatency?.p50Ms).toBeGreaterThanOrEqual(3990);
  });

  it("evaluates thresholds into breaches", () => {
    const strict = scenarioSchema.parse({
      ...scenario,
      thresholds: { maxErrorRate: 0, maxVerifyFailures: 0 },
    });
    const report = build(
      [record({}), record({ index: 1, outcome: "verify_failed", verified: false })],
      strict,
    );
    expect(report.thresholds.passed).toBe(false);
    const names = report.thresholds.breaches.map((breach) => breach.threshold);
    expect(names).toContain("maxErrorRate");
    expect(names).toContain("maxVerifyFailures");
  });
});

describe("buildStageReports", () => {
  it("computes stage durations from timestamp columns", () => {
    const rows: StageRow[] = [
      {
        flow: "input-proof",
        externalJobId: "a",
        status: "completed",
        createdAt: "2026-01-01T00:00:00.000Z",
        claimedAt: "2026-01-01T00:00:01.000Z",
        broadcastedAt: "2026-01-01T00:00:01.500Z",
        gatewayRequestConfirmedAt: "2026-01-01T00:00:05.500Z",
        completedAt: "2026-01-01T00:00:07.000Z",
        readinessAttemptCount: 0,
        broadcastAttemptCount: 1,
      },
    ];
    const stages = buildStageReports(rows);
    const byStage = Object.fromEntries(stages.map((stage) => [stage.stage, stage]));
    // hdr-histogram stores 3 significant digits; allow its quantization.
    const closeTo = (actual: number | undefined, expected: number): void => {
      expect(Math.abs((actual ?? 0) - expected)).toBeLessThanOrEqual(expected * 0.002);
    };
    closeTo(byStage.queue_wait?.stats.p50Ms, 1000);
    closeTo(byStage.broadcast?.stats.p50Ms, 500);
    closeTo(byStage.confirmation?.stats.p50Ms, 4000);
    closeTo(byStage.gateway_response?.stats.p50Ms, 1500);
    // No decrypt-only stages for input-proof.
    expect(byStage.readiness_wait).toBeUndefined();
  });

  it("computes each stage's share of server e2e (additive stages, not queue_wait)", () => {
    const rows: StageRow[] = [
      {
        flow: "input-proof",
        externalJobId: "a",
        status: "completed",
        createdAt: "2026-01-01T00:00:00.000Z",
        claimedAt: "2026-01-01T00:00:01.000Z",
        broadcastedAt: "2026-01-01T00:00:02.000Z", // broadcast 1s
        gatewayRequestConfirmedAt: "2026-01-01T00:00:03.000Z", // confirmation 1s
        completedAt: "2026-01-01T00:00:10.000Z", // gateway_response 7s; e2e 10s
        readinessAttemptCount: 0,
        broadcastAttemptCount: 1,
      },
    ];
    const byStage = Object.fromEntries(buildStageReports(rows).map((s) => [s.stage, s]));
    expect(byStage.gateway_response?.shareOfE2ePct).toBeCloseTo(70, 0);
    expect(byStage.broadcast?.shareOfE2ePct).toBeCloseTo(10, 0);
    // queue_wait overlaps later stages, so it carries no share.
    expect(byStage.queue_wait?.shareOfE2ePct).toBeUndefined();
  });

  it("flags rows whose stage includes a retry", () => {
    const rows: StageRow[] = [
      {
        flow: "user-decrypt",
        externalJobId: "b",
        status: "completed",
        createdAt: "2026-01-01T00:00:00.000Z",
        readinessClaimedAt: "2026-01-01T00:00:02.000Z",
        readyAt: "2026-01-01T00:00:03.000Z",
        claimedAt: "2026-01-01T00:00:04.000Z",
        broadcastedAt: "2026-01-01T00:00:05.000Z",
        gatewayRequestConfirmedAt: "2026-01-01T00:00:06.000Z",
        completedAt: "2026-01-01T00:00:07.000Z",
        readinessAttemptCount: 3,
        broadcastAttemptCount: 1,
      },
    ];
    const stages = buildStageReports(rows);
    const readinessWait = stages.find((stage) => stage.stage === "readiness_wait");
    expect(readinessWait?.retriedCount).toBe(1);
    // queue_wait is replaced by readiness stages for decrypt flows.
    expect(stages.find((stage) => stage.stage === "queue_wait")).toBeUndefined();
  });
});

describe("diffReports", () => {
  it("flags latency regressions beyond tolerance", () => {
    const baseline = build([record({})]);
    const slower = build([record({ e2eLatencyMs: 5000 })]);
    const diff = diffReports(baseline, slower, { latencyTolerance: 0.2 });
    expect(diff.passed).toBe(false);
    expect(diff.regressions.some((r) => r.metric === "e2e.p95Ms")).toBe(true);
  });

  it("passes within tolerance", () => {
    const baseline = build([record({})]);
    const similar = build([record({ e2eLatencyMs: 1600 })]);
    expect(diffReports(baseline, similar, { latencyTolerance: 0.2 }).passed).toBe(true);
  });

  it("always flags verification failures", () => {
    const baseline = build([record({})]);
    const broken = build([
      record({ outcome: "verify_failed", verified: false, errorLabel: "values_mismatch" }),
    ]);
    const diff = diffReports(baseline, broken);
    expect(diff.regressions.some((r) => r.metric === "verifyFailed")).toBe(true);
  });
});

describe("renderMarkdownReport", () => {
  it("renders headline, flow tables, and breaches", () => {
    const report = build([
      record({}),
      record({ index: 1, outcome: "failed", errorLabel: "response_timed_out" }),
      record({ index: 2, outcome: "protocol_error", errorLabel: "bad_envelope" }),
      record({ index: 3, outcome: "aborted", errorLabel: "client_aborted" }),
    ]);
    const markdown = renderMarkdownReport(report);
    expect(markdown).toContain("# Load Test Report - test");
    expect(markdown).toContain("**Model:** open");
    expect(markdown).toContain("achieved 0.4 workflows/s");
    expect(markdown).toContain("### A input-proof (raw-http)");
    expect(markdown).toContain("response_timed_out");
    expect(markdown).toContain("Submit (POST)");
    expect(markdown).toContain("1 protocol errors · 1 aborted");
  });

  it("renders client stage tables", () => {
    const report = build([
      record({
        loadStage: { index: 0, label: "10vu", model: "closed", vus: 10 },
      }),
    ]);
    const markdown = renderMarkdownReport(report);
    expect(markdown).toContain("### A Client Results by Load Stage");
    expect(markdown).toContain("| 10vu | 10 VUs | input-proof | raw-http |");
  });
});

describe("buildReport — collector-derived sections", () => {
  const snap = (tMs: number) => ({
    tMs,
    families: [
      {
        name: "relayer_http_responses_total",
        type: "COUNTER",
        metrics: [
          { labels: { endpoint: "/input-proof", method: "POST", version: "v2", status: "202" }, value: tMs === 0 ? "0" : "100" },
          { labels: { endpoint: "/input-proof", method: "POST", version: "v2", status: "500" }, value: tMs === 0 ? "0" : "3" },
          { labels: { endpoint: "/input-proof", method: "POST", version: "v3", status: "202" }, value: tMs === 0 ? "0" : "4" },
          { labels: { endpoint: "/metrics", method: "GET", version: "", status: "200" }, value: tMs === 0 ? "0" : "7" },
        ],
      },
      {
        name: "relayer_request_count",
        type: "GAUGE",
        metrics: [{ labels: { req_type: "input_proof", status: "queued" }, value: "0" }],
      },
      {
        name: "relayer_queue_size_count",
        type: "GAUGE",
        metrics: [
          {
            labels: { queue_type: "input_proof_tx_throttler" },
            value: tMs === 0 ? "2" : "7",
          },
        ],
      },
    ],
  });
  const samples: QueueDepthSample[] = [
    { tMs: 0, byFlowStatus: { "input-proof/queued": 50, "input-proof/awaiting_gateway_response": 10 }, pendingTotal: 60 },
    { tMs: 5000, byFlowStatus: { "input-proof/queued": 5, "input-proof/awaiting_gateway_response": 40 }, pendingTotal: 45 },
    { tMs: 10000, byFlowStatus: { "input-proof/completed": 100 }, pendingTotal: 0 },
  ];

  const report = buildReport({
    scenario,
    network: "testnet",
    relayerUrl: "http://localhost:3000",
    startedAt: "2026-01-01T00:00:00.000Z",
    endedAt: "2026-01-01T00:10:00.000Z",
    records: [record({})],
    submitted: 1,
    completed: 1,
    abandoned: 0,
    poolExhausted: false,
    submissionDurationMs: 1000,
    targets: [
      {
        target: "A",
        relayerUrl: "http://localhost:3000",
        metricsSnapshots: [snap(0), snap(10000)],
        metricsCollection: {
          successfulScrapes: 2,
          failedScrapes: 1,
          lastAttemptSucceeded: true,
          lastAttemptAt: "2026-01-01T00:00:10.000Z",
          lastSuccessAt: "2026-01-01T00:00:10.000Z",
          lastFailureAt: "2026-01-01T00:00:05.000Z",
          lastError: "temporary timeout",
        },
        queueDepth: { source: "database", samples },
      },
    ],
  });
  const targetA = report.targets[0];

  it("does not invent unavailable limiter telemetry", () => {
    expect(targetA?.metrics?.limiterUtilization).toEqual([]);
    expect(report.diagnosis?.saturatedLimiters).toEqual([]);
  });

  it("summarizes http requests and counts non-2xx", () => {
    expect(targetA?.metrics?.http?.totalRequests).toBe(114);
    expect(targetA?.metrics?.http?.loadRequests).toBe(107);
    expect(targetA?.metrics?.http?.totalRequestsPerSec).toBeCloseTo(11.4, 1);
    expect(targetA?.metrics?.http?.loadRequestsPerSec).toBeCloseTo(10.7, 1);
    expect(targetA?.metrics?.http?.nonSuccess).toBe(3);
  });

  it("reports the real legacy throttler queue label as peak and current depth", () => {
    expect(targetA?.metrics?.throttlerDepth).toEqual([
      {
        labels: { queue: "input_proof_tx_throttler" },
        peak: 7,
        last: 7,
      },
    ]);
    expect(renderMarkdownReport(report)).toContain("Legacy throttler queue depth");
  });

  it("qualifies HTTP totals and rates after a response counter reset", () => {
    const resetReport = buildReport({
      scenario,
      network: "testnet",
      relayerUrl: "http://localhost:3000",
      startedAt: "2026-01-01T00:00:00.000Z",
      endedAt: "2026-01-01T00:00:10.000Z",
      records: [record({})],
      submitted: 1,
      completed: 1,
      abandoned: 0,
      poolExhausted: false,
      submissionDurationMs: 1000,
      targets: [{
        target: "A",
        relayerUrl: "http://localhost:3000",
        metricsSnapshots: [
          { tMs: 0, families: [{ name: "relayer_http_responses_total", type: "COUNTER", metrics: [{ labels: { endpoint: "/input-proof", status: "500" }, value: "100" }] }] },
          { tMs: 10_000, families: [{ name: "relayer_http_responses_total", type: "COUNTER", metrics: [{ labels: { endpoint: "/input-proof", status: "500" }, value: "7" }] }] },
        ],
      }],
    });
    const http = resetReport.targets[0]?.metrics?.http;
    expect(http).toMatchObject({
      totalRequests: 7,
      loadRequests: 7,
      nonSuccess: 7,
      resetDetected: true,
      totalRequestsLowerBound: true,
      loadRequestsLowerBound: true,
      nonSuccessLowerBound: true,
    });
    const markdown = renderMarkdownReport(resetReport);
    expect(markdown).toContain("at least 7 HTTP request(s)");
    expect(markdown).toContain("at least 0.7/s total");
  });

  it("locates where the backlog sat and classifies the trend as draining", () => {
    const qd = targetA?.queueDepth;
    expect(qd?.trend).toBe("draining");
    const byStatus = Object.fromEntries((qd?.byStage ?? []).map((s) => [s.status, s]));
    expect(byStatus.queued?.peak).toBe(50);
    expect(byStatus.awaiting_gateway_response?.peak).toBe(40);
    expect(byStatus.queued?.end).toBe(0);
  });

  it("renders the new sections in markdown", () => {
    const md = renderMarkdownReport(report);
    expect(md).toContain("## Diagnosis");
    expect(md).not.toContain("Limiter / semaphore utilization");
    expect(md).toContain("HTTP requests (relayer-side)");
    expect(md).toContain("Queue Depth / Backlog");
    expect(md).toContain("Last attempt: succeeded at 2026-01-01T00:00:10.000Z");
    expect(md).toContain("Most recent failure at 2026-01-01T00:00:05.000Z: temporary timeout");
  });
});

describe("buildReport — v2-native Prometheus sections", () => {
  const metrics = (value: number) => parsePrometheusText(`
# TYPE input_proof_requests_inserted_total counter
input_proof_requests_inserted_total ${value.toString()}
# TYPE input_proof_request_duration_seconds histogram
input_proof_request_duration_seconds_bucket{le="1"} ${value.toString()}
input_proof_request_duration_seconds_bucket{le="+Inf"} ${value.toString()}
input_proof_request_duration_seconds_count ${value.toString()}
# TYPE relayer_transaction_count counter
relayer_transaction_count{transaction_type="input_proof",status="confirmed"} ${value.toString()}
# TYPE relayer_recovery_runs_total counter
relayer_recovery_runs_total{request_type="input_proof",result="ok"} ${value.toString()}
# TYPE relayer_wallet_lease_transitions_total counter
relayer_wallet_lease_transitions_total{result="acquired"} ${value.toString()}
# TYPE relayer_db_errors_total counter
relayer_db_errors_total{operation="lease_acquire"} ${value.toString()}
`);

  it("keeps v2 semantics separate and renders their native labels", () => {
    const report = buildReport({
      scenario,
      network: "testnet",
      relayerUrl: "http://localhost:3000",
      startedAt: "2026-01-01T00:00:00.000Z",
      endedAt: "2026-01-01T00:00:10.000Z",
      records: [record({})], submitted: 1, completed: 1, abandoned: 0,
      poolExhausted: false, submissionDurationMs: 1000,
      targets: [{
        target: "A", relayerUrl: "http://localhost:3000",
        metricsSnapshots: [{ tMs: 0, families: metrics(10) }, { tMs: 10_000, families: metrics(15) }],
      }],
    });
    const v2 = report.targets[0]?.metrics?.v2;
    expect(report.targets[0]?.metrics?.capabilities.profile).toBe("v2");
    expect(v2?.inputProofInserted[0]?.delta).toBe(5);
    expect(v2?.transactionCounts[0]?.labels).toEqual({ transaction_type: "input_proof", status: "confirmed" });
    expect(v2?.walletLeaseTransitions[0]?.delta).toBe(5);
    expect(v2?.dbErrors[0]?.delta).toBe(5);
    const markdown = renderMarkdownReport(report);
    expect(markdown).toContain("V2 transaction outcomes");
    expect(markdown).toContain("V2 wallet lease transitions");
    expect(markdown).toContain("Detected profile: **v2**");
  });
});
