import * as hdr from "hdr-histogram-js";

/** Latency summary serialized into report.json. */
export type LatencyStats = Readonly<{
  count: number;
  meanMs: number;
  p50Ms: number;
  p90Ms: number;
  p95Ms: number;
  p99Ms: number;
  maxMs: number;
}>;

/**
 * hdr-histogram accumulator for client-observed millisecond latencies.
 * Tracks 1 ms .. 2 h at 3 significant digits.
 */
export class LatencyHistogram {
  private readonly histogram = hdr.build({
    lowestDiscernibleValue: 1,
    highestTrackableValue: 7_200_000,
    numberOfSignificantValueDigits: 3,
  });

  record(valueMs: number): void {
    this.histogram.recordValue(Math.max(1, Math.round(valueMs)));
  }

  get count(): number {
    return this.histogram.totalCount;
  }

  stats(): LatencyStats | undefined {
    if (this.histogram.totalCount === 0) return undefined;
    return {
      count: this.histogram.totalCount,
      meanMs: Math.round(this.histogram.mean * 100) / 100,
      p50Ms: this.histogram.getValueAtPercentile(50),
      p90Ms: this.histogram.getValueAtPercentile(90),
      p95Ms: this.histogram.getValueAtPercentile(95),
      p99Ms: this.histogram.getValueAtPercentile(99),
      maxMs: this.histogram.maxValue,
    };
  }
}

export const statsOf = (values: readonly number[]): LatencyStats | undefined => {
  const histogram = new LatencyHistogram();
  for (const value of values) histogram.record(value);
  return histogram.stats();
};
