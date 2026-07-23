import type { MetricFamily, MetricsSnapshot } from "../collectors/prometheus";

/**
 * Run-window analysis of cumulative Prometheus data: the relayer exports
 * since-process-start counters and histograms, so the run's contribution is
 * the delta between the first and last scrapes.
 */

export type CounterDelta = Readonly<{
  labels: Readonly<Record<string, string>>;
  delta: number;
  /** True when a counter reset makes delta a conservative post-reset lower bound. */
  resetDetected?: boolean;
  lowerBound?: boolean;
}>;

export type HistogramQuantiles = Readonly<{
  labels: Readonly<Record<string, string>>;
  count: number;
  /** Quantiles estimated from bucket deltas; seconds, linear interpolation. */
  p50?: number;
  p90?: number;
  p95?: number;
  p99?: number;
  /** True when final buckets are used as a post-reset lower-bound distribution. */
  resetDetected?: boolean;
  lowerBound?: boolean;
}>;

export type GaugePeak = Readonly<{
  labels: Readonly<Record<string, string>>;
  /** Highest value observed across all run snapshots. */
  peak: number;
  /** Value in the final snapshot. */
  last: number;
}>;

export type TimePoint = Readonly<{ tMs: number; value: number }>;

/** A single value tracked across the run: first/last/peak + linear drift. */
export type GaugeTrend = Readonly<{
  first: number;
  last: number;
  peak: number;
  /** Net change last - first. */
  delta: number;
  /** Least-squares slope in value units per hour (drift signal for soaks). */
  perHour: number;
}>;

const labelKey = (labels: Record<string, string> | undefined): string =>
  JSON.stringify(Object.entries(labels ?? {}).sort());

/**
 * Per-label peak and final value of a gauge across every snapshot. Gauges
 * (limiter utilization, pool connections) are not cumulative, so neither a
 * delta nor a histogram quantile fits — the saturation signal is the peak
 * over the run.
 */
export const gaugePeaks = (
  snapshots: readonly MetricsSnapshot[],
  name: string,
): GaugePeak[] => {
  const peaks = new Map<string, { labels: Record<string, string>; peak: number; last: number }>();
  for (const snapshot of snapshots) {
    for (const metric of familyByName(snapshot, name)?.metrics ?? []) {
      const key = labelKey(metric.labels);
      if (metric.value === undefined) continue;
      const value = Number(metric.value);
      if (!Number.isFinite(value)) continue;
      const existing = peaks.get(key);
      if (existing) {
        existing.peak = Math.max(existing.peak, value);
        existing.last = value;
      } else {
        peaks.set(key, { labels: metric.labels ?? {}, peak: value, last: value });
      }
    }
  }
  return [...peaks.values()];
};

const familyByName = (
  snapshot: MetricsSnapshot | undefined,
  name: string,
): MetricFamily | undefined =>
  snapshot?.families.find((family) => family.name === name);

/** Per-label-set counter delta between the first and last snapshots. */
export const counterDeltas = (
  first: MetricsSnapshot | undefined,
  last: MetricsSnapshot | undefined,
  name: string,
): CounterDelta[] => {
  const start = new Map<string, number>();
  for (const metric of familyByName(first, name)?.metrics ?? []) {
    if (metric.value === undefined) continue;
    const value = Number(metric.value);
    if (Number.isFinite(value)) start.set(labelKey(metric.labels), value);
  }
  const deltas: CounterDelta[] = [];
  for (const metric of familyByName(last, name)?.metrics ?? []) {
    if (metric.value === undefined) continue;
    const initial = start.get(labelKey(metric.labels));
    const final = Number(metric.value);
    // Absence is not zero. A series first observed after the baseline cannot
    // produce a trustworthy run-window delta.
    if (initial === undefined || !Number.isFinite(final)) continue;
    // On reset, the post-reset value is the only conservative lower bound.
    const resetDetected = final < initial;
    const delta = resetDetected ? Math.max(0, final) : final - initial;
    deltas.push({
      labels: metric.labels ?? {},
      delta,
      ...(resetDetected ? { resetDetected: true, lowerBound: true } : {}),
    });
  }
  return deltas;
};

const estimateQuantile = (
  bounds: readonly number[],
  cumulative: readonly number[],
  total: number,
  quantile: number,
): number | undefined => {
  const target = quantile * total;
  for (let i = 0; i < bounds.length; i += 1) {
    if ((cumulative[i] ?? 0) >= target) {
      const upper = bounds[i] ?? 0;
      const lower = i > 0 ? (bounds[i - 1] ?? 0) : 0;
      const prevCount = i > 0 ? (cumulative[i - 1] ?? 0) : 0;
      const inBucket = (cumulative[i] ?? 0) - prevCount;
      if (!Number.isFinite(upper)) return lower;
      if (inBucket <= 0) return upper;
      return lower + ((target - prevCount) / inBucket) * (upper - lower);
    }
  }
  return undefined;
};

/**
 * Quantile estimates per label set from histogram bucket deltas. Estimates
 * interpolate linearly inside the winning bucket; +Inf collapses to the last
 * finite bound.
 */
export const histogramQuantiles = (
  first: MetricsSnapshot | undefined,
  last: MetricsSnapshot | undefined,
  name: string,
): HistogramQuantiles[] => {
  const startBuckets = new Map<string, Record<string, string>>();
  for (const metric of familyByName(first, name)?.metrics ?? []) {
    startBuckets.set(labelKey(metric.labels), metric.buckets ?? {});
  }

  const results: HistogramQuantiles[] = [];
  for (const metric of familyByName(last, name)?.metrics ?? []) {
    const start = startBuckets.get(labelKey(metric.labels));
    if (!start) continue;
    let resetDetected = false;
    let entries = Object.entries(metric.buckets ?? {})
      .map(([le, count]) => ({
        bound: le === "+Inf" ? Infinity : Number(le),
        delta: start[le] === undefined ? NaN : Number(count) - Number(start[le]),
      }))
      .sort((a, b) => a.bound - b.bound);
    if (entries.length === 0 || entries.some((entry) =>
      Number.isNaN(entry.bound) || !Number.isFinite(entry.delta),
    )) continue;
    if (entries.some((entry) => entry.delta < 0)) {
      resetDetected = true;
      entries = Object.entries(metric.buckets ?? {})
        .map(([le, count]) => ({
          bound: le === "+Inf" ? Infinity : Number(le),
          delta: Number(count),
        }))
        .sort((a, b) => a.bound - b.bound);
      if (entries.some((entry) => Number.isNaN(entry.bound) || !Number.isFinite(entry.delta) || entry.delta < 0)) continue;
    }
    if (entries.some((entry, index) => index > 0 && entry.delta < (entries[index - 1]?.delta ?? 0))) continue;
    const bounds = entries.map((entry) => entry.bound);
    const cumulative = entries.map((entry) => entry.delta);
    const total = cumulative.at(-1) ?? 0;
    if (total <= 0) continue;
    results.push({
      labels: metric.labels ?? {},
      count: total,
      p50: estimateQuantile(bounds, cumulative, total, 0.5),
      p90: estimateQuantile(bounds, cumulative, total, 0.9),
      p95: estimateQuantile(bounds, cumulative, total, 0.95),
      p99: estimateQuantile(bounds, cumulative, total, 0.99),
      ...(resetDetected ? { resetDetected: true, lowerBound: true } : {}),
    });
  }
  return results;
};

/** Time series of one unlabeled-or-single gauge family across all snapshots. */
export const gaugeSeries = (
  snapshots: readonly MetricsSnapshot[],
  name: string,
  /** Optional label filter to disambiguate multi-series families. */
  match?: Readonly<Record<string, string>>,
): TimePoint[] => {
  const points: TimePoint[] = [];
  for (const snapshot of snapshots) {
    const family = familyByName(snapshot, name);
    if (!family) continue;
    const metric = family.metrics.find((m) =>
      match
        ? Object.entries(match).every(([k, v]) => m.labels?.[k] === v)
        : true,
    );
    if (metric?.value !== undefined) {
      const value = Number(metric.value);
      if (Number.isFinite(value)) points.push({ tMs: snapshot.tMs, value });
    }
  }
  return points;
};

/**
 * Least-squares slope of points, scaled to per-hour. Returns 0 for fewer than
 * two points or a zero time span (the drift signal is undefined there).
 */
export const linearTrendPerHour = (points: readonly TimePoint[]): number => {
  if (points.length < 2) return 0;
  const t0 = points[0]?.tMs ?? 0;
  const xs = points.map((p) => (p.tMs - t0) / 3_600_000); // hours since start
  const ys = points.map((p) => p.value);
  const n = points.length;
  const sumX = xs.reduce((a, b) => a + b, 0);
  const sumY = ys.reduce((a, b) => a + b, 0);
  const sumXY = xs.reduce((acc, x, i) => acc + x * (ys[i] ?? 0), 0);
  const sumXX = xs.reduce((acc, x) => acc + x * x, 0);
  const denom = n * sumXX - sumX * sumX;
  if (Math.abs(denom) < 1e-12) return 0;
  return (n * sumXY - sumX * sumY) / denom;
};

/** Summarizes one gauge family (optionally filtered) as a trend. */
export const gaugeTrend = (
  snapshots: readonly MetricsSnapshot[],
  name: string,
  match?: Readonly<Record<string, string>>,
): GaugeTrend | undefined => {
  const points = gaugeSeries(snapshots, name, match);
  if (points.length === 0) return undefined;
  const values = points.map((p) => p.value);
  const first = values[0] ?? 0;
  const last = values.at(-1) ?? 0;
  return {
    first,
    last,
    peak: Math.max(...values),
    delta: last - first,
    perHour: linearTrendPerHour(points),
  };
};

/** Process-collector summary (Linux `process_*`); the primary soak readout. */
export type ProcessSummary = Readonly<{
  /** Resident memory, bytes. */
  rss?: GaugeTrend;
  /** Virtual memory, bytes. */
  virtualMemory?: GaugeTrend;
  /** Open file descriptors. */
  openFds?: GaugeTrend;
  /** Configured FD ceiling, if exported. */
  maxFds?: number;
  /** Average CPU cores used over the run window (Δcpu_seconds / wall seconds). */
  avgCpuCores?: number;
  /** Run wall-clock span in seconds. */
  windowSec?: number;
}>;

export const processSummary = (
  snapshots: readonly MetricsSnapshot[],
): ProcessSummary | undefined => {
  if (snapshots.length === 0) return undefined;
  const rss = gaugeTrend(snapshots, "process_resident_memory_bytes");
  const virtualMemory = gaugeTrend(snapshots, "process_virtual_memory_bytes");
  const openFds = gaugeTrend(snapshots, "process_open_fds");
  const maxFdsPoints = gaugeSeries(snapshots, "process_max_fds");
  const cpu = gaugeSeries(snapshots, "process_cpu_seconds_total");

  let avgCpuCores: number | undefined;
  let windowSec: number | undefined;
  if (cpu.length >= 2) {
    const firstCpu = cpu[0];
    const lastCpu = cpu.at(-1);
    if (firstCpu && lastCpu) {
      windowSec = (lastCpu.tMs - firstCpu.tMs) / 1000;
      if (windowSec > 0 && lastCpu.value >= firstCpu.value) {
        avgCpuCores = (lastCpu.value - firstCpu.value) / windowSec;
      }
    }
  }

  if (!rss && !openFds && avgCpuCores === undefined) return undefined;
  return {
    rss,
    virtualMemory,
    openFds,
    maxFds: maxFdsPoints.at(-1)?.value,
    avgCpuCores,
    windowSec,
  };
};
