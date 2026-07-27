import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";

import { parsePrometheusText } from "../src/collectors/prom-parse";
import { createPrometheusHttpSource, PrometheusCollector } from "../src/collectors/prometheus";

const EXPOSITION = `
# HELP relayer_requests_by_status Requests per status
# TYPE relayer_requests_by_status gauge
relayer_requests_by_status{flow="input_proof",status="queued"} 12
relayer_requests_by_status{flow="input_proof",status="completed"} 100
# TYPE relayer_stage_duration_seconds histogram
relayer_stage_duration_seconds_bucket{flow="input_proof",stage="broadcast",le="0.64"} 3
relayer_stage_duration_seconds_bucket{flow="input_proof",stage="broadcast",le="+Inf"} 3
relayer_stage_duration_seconds_sum{flow="input_proof",stage="broadcast"} 1.44
relayer_stage_duration_seconds_count{flow="input_proof",stage="broadcast"} 3
relayer_stage_duration_seconds_bucket{flow="user_decrypt",stage="readiness_check",le="0.64"} 1
relayer_stage_duration_seconds_bucket{flow="user_decrypt",stage="readiness_check",le="+Inf"} 2
relayer_stage_duration_seconds_sum{flow="user_decrypt",stage="readiness_check"} 2.2
relayer_stage_duration_seconds_count{flow="user_decrypt",stage="readiness_check"} 2
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 42.5
# TYPE odd_label gauge
odd_label{message="say \\"hi\\", ok"} 1
`;

describe("parsePrometheusText", () => {
  const families = parsePrometheusText(EXPOSITION);
  const byName = Object.fromEntries(families.map((family) => [family.name, family]));

  it("parses gauges with labels", () => {
    const gauge = byName.relayer_requests_by_status;
    expect(gauge?.type).toBe("GAUGE");
    expect(gauge?.metrics).toHaveLength(2);
    expect(gauge?.metrics[0]).toEqual({
      labels: { flow: "input_proof", status: "queued" },
      value: "12",
    });
  });

  it("groups histogram buckets per label set, keeping labels", () => {
    const histogram = byName.relayer_stage_duration_seconds;
    expect(histogram?.type).toBe("HISTOGRAM");
    expect(histogram?.metrics).toHaveLength(2);
    const broadcast = histogram?.metrics.find(
      (metric) => metric.labels?.stage === "broadcast",
    );
    expect(broadcast?.labels).toEqual({ flow: "input_proof", stage: "broadcast" });
    expect(broadcast?.buckets).toEqual({ "0.64": "3", "+Inf": "3" });
    expect(broadcast?.count).toBe("3");
    expect(broadcast?.sum).toBe("1.44");
  });

  it("parses unlabeled counters", () => {
    expect(byName.process_cpu_seconds_total?.metrics[0]?.value).toBe("42.5");
  });

  it("unescapes label values", () => {
    expect(byName.odd_label?.metrics[0]?.labels?.message).toBe('say "hi", ok');
  });

  it("uses TYPE metadata even when it follows histogram samples", () => {
    const [family] = parsePrometheusText(`
late_bucket{kind="x",le="1"} 2
late_bucket{kind="x",le="+Inf"} 2
# TYPE late histogram
late_count{kind="x"} 2
`);
    expect(family?.name).toBe("late");
    expect(family?.metrics[0]?.buckets).toEqual({ "1": "2", "+Inf": "2" });
  });

  it("skips malformed labels and values rather than partially parsing them", () => {
    const families = parsePrometheusText(`
valid{kind="ok"} 1e3
bad{kind="ok",broken} 2
also_bad{kind="ok"} nope
`);
    expect(families).toHaveLength(1);
    expect(families[0]?.metrics[0]?.value).toBe("1e3");
  });
});

describe("createPrometheusHttpSource", () => {
  it("scrapes raw text without a relayer client dependency", async () => {
    const fetcher = async (): Promise<Response> => new Response("metric 1", { status: 200 });
    await expect(createPrometheusHttpSource("https://metrics.example/metrics", fetcher as typeof fetch)())
      .resolves.toBe("metric 1");
  });

  it("reports non-success scrape responses", async () => {
    const fetcher = async (): Promise<Response> => new Response("no", { status: 503 });
    await expect(createPrometheusHttpSource("https://metrics.example/metrics", fetcher as typeof fetch)())
      .rejects.toThrow("HTTP 503");
  });

  it("bounds scrape response size", async () => {
    const fetcher = async (): Promise<Response> => new Response("metric 12345", { status: 200 });
    await expect(createPrometheusHttpSource(
      "https://metrics.example/metrics", fetcher as typeof fetch, { maxResponseBytes: 5 },
    )()).rejects.toThrow("response limit");
  });
});

describe("PrometheusCollector", () => {
  it("retains real v2 families and records scrape health", async () => {
    const directory = await mkdtemp(join(tmpdir(), "load-test-prom-"));
    const source = async (): Promise<string> => `
# TYPE input_proof_requests_inserted_total counter
input_proof_requests_inserted_total 1
# TYPE relayer_transaction_count counter
relayer_transaction_count{transaction_type="input_proof",status="confirmed"} 1
# TYPE relayer_recovery_runs_total counter
relayer_recovery_runs_total{request_type="input_proof",result="ok"} 1
# TYPE relayer_wallet_lease_transitions_total counter
relayer_wallet_lease_transitions_total{result="acquired"} 1
# TYPE relayer_db_errors_total counter
relayer_db_errors_total{operation="lease_acquire"} 0
`;
    try {
      const collector = new PrometheusCollector(source, join(directory, "metrics.jsonl"), 60_000);
      await collector.start();
      await collector.stop();
      expect(collector.collectionStatus).toMatchObject({ successfulScrapes: 2, failedScrapes: 0 });
      expect(collector.capabilities.profile).toBe("v2");
      expect(collector.snapshots[0]?.families.map((family) => family.name)).toEqual(
        expect.arrayContaining([
          "input_proof_requests_inserted_total",
          "relayer_transaction_count",
          "relayer_wallet_lease_transitions_total",
          "relayer_db_errors_total",
        ]),
      );
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  });

  it("keeps an unavailable scrape explicit without failing an optional collector", async () => {
    const directory = await mkdtemp(join(tmpdir(), "load-test-prom-fail-"));
    try {
      const collector = new PrometheusCollector(
        async () => Promise.reject(new Error("connection refused")),
        join(directory, "metrics.jsonl"),
        60_000,
      );
      await collector.start();
      await collector.stop();
      expect(collector.collectionStatus).toMatchObject({
        successfulScrapes: 0,
        failedScrapes: 2,
        lastAttemptSucceeded: false,
        lastError: "connection refused",
      });
      expect(collector.collectionStatus.lastAttemptAt).toBeDefined();
      expect(collector.collectionStatus.lastFailureAt).toBeDefined();
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  });

  it("preserves the most recent scrape failure after collection recovers", async () => {
    const directory = await mkdtemp(join(tmpdir(), "load-test-prom-recovered-"));
    let attempts = 0;
    try {
      const collector = new PrometheusCollector(
        async () => {
          attempts += 1;
          if (attempts === 1) throw new Error("temporary timeout");
          return "# TYPE input_proof_requests_inserted_total counter\ninput_proof_requests_inserted_total 0\n";
        },
        join(directory, "metrics.jsonl"),
        60_000,
      );
      await collector.start();
      await collector.stop();
      expect(collector.collectionStatus).toMatchObject({
        successfulScrapes: 1,
        failedScrapes: 1,
        lastAttemptSucceeded: true,
        lastError: "temporary timeout",
      });
      expect(collector.collectionStatus.lastSuccessAt).toBeDefined();
      expect(collector.collectionStatus.lastFailureAt).toBeDefined();
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  });
});
