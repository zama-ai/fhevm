import type { MemorySample } from './memorySampler.js';

// ---------------------------------------------------------------------------
// Why this exists
// ---------------------------------------------------------------------------
// WebAssembly.Memory can only grow, never shrink (per spec), and a healthy
// process also grows once during warmup (caches fill, thread pools spawn,
// JIT settles). So "memory grew" is never itself a leak signal — only
// *sustained, non-decaying growth proportional to iteration count* is. This
// module compares the growth rate in the second half of a run against the
// first half: a real leak keeps growing at the same (or a worse) rate, while
// expected behavior decays toward zero once warmup is over.
//
// The thresholds below are placeholders. Per the project's stated plan, v1 is
// a standalone script whose output is meant to be read and judged by a human
// first; these numbers should be re-tuned once real runs against localstack
// have been observed, before anyone treats this as an unattended pass/fail
// gate.

export type MetricSelector = (sample: MemorySample) => number | undefined;

export type TrendClassification = 'plateauing' | 'growing' | 'insufficient-data';

export type TrendVerdict = {
  readonly metric: string;
  readonly classification: TrendClassification;
  readonly sampleCount: number;
  readonly baselineBytes: number;
  readonly peakBytes: number;
  readonly firstHalfBytesPerIteration: number;
  readonly secondHalfBytesPerIteration: number;
  readonly ceilingExceededBytes: number | undefined;
};

export type TrendDetectorOptions = {
  /** Samples whose iteration count is below this are excluded (one-time warmup growth). */
  readonly warmupIterations?: number;
  /** Minimum post-warmup samples required to classify at all. */
  readonly minSamplesAfterWarmup?: number;
  /**
   * A metric is "growing" when the second-half slope is still at least this
   * fraction of the first-half slope (i.e. it hasn't meaningfully decayed).
   * Also triggers when the first-half slope was ~flat but the second half
   * started climbing.
   */
  readonly growthRatioThreshold?: number;
  /** Hard safety-net ceiling, in bytes above baseline, regardless of trend shape. */
  readonly absoluteCeilingBytes?: number;
};

const DEFAULT_OPTIONS: Required<TrendDetectorOptions> = {
  warmupIterations: 50,
  minSamplesAfterWarmup: 10,
  growthRatioThreshold: 0.8,
  absoluteCeilingBytes: 512 * 1024 * 1024, // 512 MiB above baseline
};

/** Least-squares slope of `metric` against `iteration` — robust to single-sample noise. */
function linearRegressionSlope(points: ReadonlyArray<{ x: number; y: number }>): number {
  const n = points.length;
  if (n < 2) {
    return 0;
  }
  let sumX = 0;
  let sumY = 0;
  let sumXY = 0;
  let sumXX = 0;
  for (const { x, y } of points) {
    sumX += x;
    sumY += y;
    sumXY += x * y;
    sumXX += x * x;
  }
  const denominator = n * sumXX - sumX * sumX;
  if (denominator === 0) {
    return 0;
  }
  return (n * sumXY - sumX * sumY) / denominator;
}

function standardDeviation(values: readonly number[]): number {
  if (values.length < 2) {
    return 0;
  }
  const mean = values.reduce((sum, v) => sum + v, 0) / values.length;
  const variance = values.reduce((sum, v) => sum + (v - mean) ** 2, 0) / (values.length - 1);
  return Math.sqrt(variance);
}

export function evaluateTrend(
  samples: readonly MemorySample[],
  metricName: string,
  select: MetricSelector,
  options: TrendDetectorOptions = {},
): TrendVerdict {
  const opts = { ...DEFAULT_OPTIONS, ...options };

  const points = samples
    .map((sample) => ({ iteration: sample.iteration, value: select(sample) }))
    .filter((p): p is { iteration: number; value: number } => p.value !== undefined);

  // Baseline/peak/ceiling are computed over the POST-warmup window, not the
  // full run: one-time startup cost (module init, thread-pool spawn, first
  // network fetch) legitimately dwarfs steady-state growth, and including it
  // here would report a misleading "peak Xxx MB above baseline" driven
  // entirely by warmup, not by anything the loop itself did.
  const postWarmup = points.filter((p) => p.iteration >= opts.warmupIterations);

  const baselineBytes = postWarmup[0]?.value ?? 0;
  const peakBytes = postWarmup.reduce((max, p) => Math.max(max, p.value), baselineBytes);
  const ceilingExceededBytes =
    peakBytes - baselineBytes > opts.absoluteCeilingBytes ? peakBytes - baselineBytes : undefined;

  if (postWarmup.length < opts.minSamplesAfterWarmup) {
    return {
      metric: metricName,
      classification: 'insufficient-data',
      sampleCount: postWarmup.length,
      baselineBytes,
      peakBytes,
      firstHalfBytesPerIteration: 0,
      secondHalfBytesPerIteration: 0,
      ceilingExceededBytes,
    };
  }

  const mid = Math.floor(postWarmup.length / 2);
  const firstHalf = postWarmup.slice(0, mid);
  const secondHalf = postWarmup.slice(mid);

  const firstHalfSlope = linearRegressionSlope(firstHalf.map((p) => ({ x: p.iteration, y: p.value })));
  const secondHalfSlope = linearRegressionSlope(secondHalf.map((p) => ({ x: p.iteration, y: p.value })));

  // A flat, metric-agnostic noise floor is miscalibrated: RSS/external
  // naturally jitter by single-digit MB between samples from GC/allocator
  // behavior alone, while a WASM page count only ever moves in fixed 64KB
  // steps (or not at all). Deriving the floor from this run's own observed
  // scatter — one standard deviation of the post-warmup values, spread over
  // the run — self-calibrates per metric instead of guessing one constant
  // for everything: noisy metrics get a forgiving floor, near-deterministic
  // ones stay tightly sensitive.
  const noiseFloorBytesPerIteration = Math.max(
    1,
    standardDeviation(postWarmup.map((p) => p.value)) / postWarmup.length,
  );
  const isGrowing =
    ceilingExceededBytes !== undefined ||
    (secondHalfSlope > noiseFloorBytesPerIteration &&
      (firstHalfSlope <= noiseFloorBytesPerIteration || secondHalfSlope >= firstHalfSlope * opts.growthRatioThreshold));

  return {
    metric: metricName,
    classification: isGrowing ? 'growing' : 'plateauing',
    sampleCount: postWarmup.length,
    baselineBytes,
    peakBytes,
    firstHalfBytesPerIteration: firstHalfSlope,
    secondHalfBytesPerIteration: secondHalfSlope,
    ceilingExceededBytes,
  };
}

export const METRIC_SELECTORS: Record<string, MetricSelector> = {
  rss: (s) => s.rssBytes,
  heapUsed: (s) => s.heapUsedBytes,
  external: (s) => s.externalBytes,
  arrayBuffers: (s) => s.arrayBuffersBytes,
  tfheMemory: (s) => s.tfheMemory?.byteLength,
  tkmsMemory: (s) => s.tkmsMemory?.byteLength,
};

/**
 * Metrics whose "growing" classification actually gates the process exit
 * code. Process-level metrics (rss/heapUsed/external/arrayBuffers) can
 * legitimately oscillate by tens of MB per iteration — a big transient
 * allocation during one iteration's work, mostly reclaimed by GC before the
 * next — and a fixed-interval sampler catches different phases of that
 * sawtooth on every run. A two-half linear fit over that kind of data can
 * show a spurious slope purely from which points happened to land near a
 * peak vs. a trough, no matter how the noise floor is tuned; two separate
 * real runs against localstack produced exactly that false signal on rss and
 * external before this distinction was added.
 *
 * `tfheMemory`/`tkmsMemory` don't have this problem: `WebAssembly.Memory`
 * only ever grows, never shrinks, so there is no trough to be out of phase
 * with — any sustained climb there is real. They're the direct, trustworthy
 * signal for the thing this harness actually exists to catch (native/WASM
 * memory not being freed), so they're the only classifications that gate.
 *
 * Process-level metrics are still computed, printed, and worth a human
 * glancing at (that's what `insufficient-data`/`growing` labels are for) —
 * they just don't fail the run on their own. The absolute ceiling check
 * (`ceilingExceededBytes`) still applies across every metric regardless of
 * this list: a hard ceiling breach isn't subject to sawtooth
 * misinterpretation the way a small fitted slope is.
 */
export const GATING_METRICS: ReadonlySet<string> = new Set(['tfheMemory', 'tkmsMemory']);

export function evaluateAllTrends(
  samples: readonly MemorySample[],
  options: TrendDetectorOptions = {},
): readonly TrendVerdict[] {
  return Object.entries(METRIC_SELECTORS).map(([metricName, select]) =>
    evaluateTrend(samples, metricName, select, options),
  );
}

/**
 * True when the run should be treated as anomalous: a gating metric
 * (tfhe/tkms WASM memory) was classified "growing", or any metric — gating
 * or not — breached the absolute ceiling.
 */
export function hasActionableGrowth(verdicts: readonly TrendVerdict[]): boolean {
  return verdicts.some(
    (v) => v.ceilingExceededBytes !== undefined || (GATING_METRICS.has(v.metric) && v.classification === 'growing'),
  );
}
