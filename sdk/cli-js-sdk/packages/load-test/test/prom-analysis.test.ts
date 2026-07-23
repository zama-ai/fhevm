import { describe, expect, it } from "vitest";

import type { MetricsSnapshot } from "../src/collectors/prometheus";
import {
  counterDeltas,
  gaugePeaks,
  histogramQuantiles,
  linearTrendPerHour,
  processSummary,
} from "../src/report/prom-analysis";

const snapshot = (tMs: number, families: MetricsSnapshot["families"]): MetricsSnapshot => ({
  tMs,
  families,
});

describe("counterDeltas", () => {
  it("computes per-label deltas and preserves observed zero rows", () => {
    const first = snapshot(0, [
      {
        name: "relayer_requests_terminal_total",
        type: "COUNTER",
        metrics: [
          { labels: { flow: "input_proof", status: "completed" }, value: "10" },
          { labels: { flow: "input_proof", status: "failed" }, value: "2" },
        ],
      },
    ]);
    const last = snapshot(60_000, [
      {
        name: "relayer_requests_terminal_total",
        type: "COUNTER",
        metrics: [
          { labels: { flow: "input_proof", status: "completed" }, value: "110" },
          { labels: { flow: "input_proof", status: "failed" }, value: "2" },
          { labels: { flow: "user_decrypt", status: "completed" }, value: "5" },
        ],
      },
    ]);
    const deltas = counterDeltas(first, last, "relayer_requests_terminal_total");
    expect(deltas).toHaveLength(2);
    expect(deltas).toContainEqual({
      labels: { flow: "input_proof", status: "completed" },
      delta: 100,
    });
    // A series absent from the baseline is unknown, not zero.
    expect(deltas.some((entry) => entry.labels.flow === "user_decrypt")).toBe(false);
    expect(deltas).toContainEqual({
      labels: { flow: "input_proof", status: "failed" },
      delta: 0,
    });
  });

  it("marks a counter reset as a post-reset lower bound", () => {
    const first = snapshot(0, [{ name: "x", type: "COUNTER", metrics: [{ value: "50" }] }]);
    const last = snapshot(1000, [{ name: "x", type: "COUNTER", metrics: [{ value: "7" }] }]);
    expect(counterDeltas(first, last, "x")).toEqual([
      { labels: {}, delta: 7, resetDetected: true, lowerBound: true },
    ]);
  });

  it("returns nothing without snapshots", () => {
    expect(counterDeltas(undefined, undefined, "x")).toEqual([]);
  });
});

describe("histogramQuantiles", () => {
  it("estimates quantiles from bucket deltas with interpolation", () => {
    const first = snapshot(0, [
      {
        name: "relayer_stage_duration_seconds",
        type: "HISTOGRAM",
        metrics: [
          {
            labels: { flow: "input_proof", stage: "broadcast" },
            buckets: { "1": "0", "2": "0", "4": "0", "+Inf": "0" },
            count: "0",
            sum: "0",
          },
        ],
      },
    ]);
    const last = snapshot(60_000, [
      {
        name: "relayer_stage_duration_seconds",
        type: "HISTOGRAM",
        metrics: [
          {
            labels: { flow: "input_proof", stage: "broadcast" },
            // 50 in (0,1], 40 in (1,2], 10 in (2,4].
            buckets: { "1": "50", "2": "90", "4": "100", "+Inf": "100" },
            count: "100",
            sum: "120",
          },
        ],
      },
    ]);
    const quantiles = histogramQuantiles(first, last, "relayer_stage_duration_seconds");
    expect(quantiles).toHaveLength(1);
    const entry = quantiles[0];
    expect(entry?.count).toBe(100);
    expect(entry?.p50).toBeCloseTo(1, 5);
    // p95 target = 95th of 100: bucket (1,2] holds 50..90, (2,4] holds 90..100.
    expect(entry?.p95).toBeCloseTo(3, 5);
  });

  it("skips label sets with no run-window observations", () => {
    const unchanged = snapshot(0, [
      {
        name: "relayer_stage_duration_seconds",
        type: "HISTOGRAM",
        metrics: [
          {
            labels: { flow: "input_proof", stage: "broadcast" },
            buckets: { "1": "50", "+Inf": "50" },
            count: "50",
            sum: "10",
          },
        ],
      },
    ]);
    expect(
      histogramQuantiles(unchanged, unchanged, "relayer_stage_duration_seconds"),
    ).toEqual([]);
  });

  it("uses final histogram buckets as a lower bound after reset", () => {
    const first = snapshot(0, [{
      name: "h", type: "HISTOGRAM", metrics: [{ buckets: { "1": "50", "+Inf": "100" } }],
    }]);
    const last = snapshot(1000, [{
      name: "h", type: "HISTOGRAM", metrics: [{ buckets: { "1": "2", "+Inf": "5" } }],
    }]);
    expect(histogramQuantiles(first, last, "h")[0]).toMatchObject({
      count: 5, resetDetected: true, lowerBound: true,
    });
  });
});

describe("gaugePeaks", () => {
  it("tracks peak and last value per label set across snapshots", () => {
    const mk = (t: number, inUse: number): MetricsSnapshot =>
      snapshot(t, [
        {
          name: "relayer_limiter_utilization",
          type: "GAUGE",
          metrics: [
            { labels: { limiter: "input_proof_broadcast", state: "in_use" }, value: String(inUse) },
            { labels: { limiter: "input_proof_broadcast", state: "max" }, value: "10" },
          ],
        },
      ]);
    // in_use rises to the cap mid-run, then falls back.
    const peaks = gaugePeaks([mk(0, 1), mk(5000, 10), mk(10000, 2)], "relayer_limiter_utilization");
    const inUse = peaks.find((p) => p.labels.state === "in_use");
    const max = peaks.find((p) => p.labels.state === "max");
    expect(inUse?.peak).toBe(10); // saturation caught even though it's gone by the end
    expect(inUse?.last).toBe(2);
    expect(max?.peak).toBe(10);
  });

  it("returns empty when the family is absent", () => {
    expect(gaugePeaks([snapshot(0, [])], "relayer_sqlx_pool_connections")).toEqual([]);
  });
});

describe("linearTrendPerHour", () => {
  it("recovers a known slope scaled to per-hour", () => {
    // +10 units per hour over 3 half-hour samples.
    const pts = [
      { tMs: 0, value: 100 },
      { tMs: 1_800_000, value: 105 },
      { tMs: 3_600_000, value: 110 },
    ];
    expect(linearTrendPerHour(pts)).toBeCloseTo(10, 6);
  });
  it("is zero for flat or single-point series", () => {
    expect(linearTrendPerHour([{ tMs: 0, value: 5 }])).toBe(0);
    expect(linearTrendPerHour([{ tMs: 0, value: 5 }, { tMs: 1000, value: 5 }])).toBe(0);
  });
});

describe("processSummary", () => {
  const procSnap = (tMs: number, rss: number, cpu: number, fds: number): MetricsSnapshot =>
    snapshot(tMs, [
      { name: "process_resident_memory_bytes", type: "GAUGE", metrics: [{ value: String(rss) }] },
      { name: "process_cpu_seconds_total", type: "COUNTER", metrics: [{ value: String(cpu) }] },
      { name: "process_open_fds", type: "GAUGE", metrics: [{ value: String(fds) }] },
    ]);
  it("summarizes RSS trend and average CPU cores", () => {
    // 600s window, cpu 0→300 ⇒ 0.5 cores; RSS 100MiB→160MiB ⇒ +60 over 600s.
    const s = processSummary([
      procSnap(0, 100 * 2 ** 20, 0, 50),
      procSnap(600_000, 160 * 2 ** 20, 300, 60),
    ]);
    expect(s?.avgCpuCores).toBeCloseTo(0.5, 3);
    expect(s?.windowSec).toBe(600);
    expect(s?.rss?.delta).toBe(60 * 2 ** 20);
    expect(s?.openFds?.delta).toBe(10);
    expect(s?.rss?.perHour).toBeCloseTo(360 * 2 ** 20, -3); // 60MiB/600s = 360MiB/hr
  });
  it("returns undefined when no process metrics present", () => {
    expect(processSummary([snapshot(0, [])])).toBeUndefined();
  });
});
