import type { MemorySample } from './memorySampler.js';
import { GATING_METRICS, type TrendVerdict } from './trendDetector.js';

export function formatBytes(bytes: number): string {
  const sign = bytes < 0 ? '-' : '';
  const abs = Math.abs(bytes);
  if (abs < 1024) {
    return `${sign}${abs.toFixed(0)}B`;
  }
  if (abs < 1024 ** 2) {
    return `${sign}${(abs / 1024).toFixed(1)}KB`;
  }
  if (abs < 1024 ** 3) {
    return `${sign}${(abs / 1024 ** 2).toFixed(1)}MB`;
  }
  return `${sign}${(abs / 1024 ** 3).toFixed(2)}GB`;
}

function pad(value: string, width: number): string {
  return value.length >= width ? value : ' '.repeat(width - value.length) + value;
}

/** Formats a duration as `<minutes>m<seconds>s` (e.g. `12m34s`), or `-` when unknown. */
export function formatDuration(ms: number | undefined): string {
  if (ms === undefined || !Number.isFinite(ms) || ms < 0) {
    return '-';
  }
  const totalSeconds = Math.round(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}m${String(seconds).padStart(2, '0')}s`;
}

export function printHeader(scenarioName: string): void {
  console.log(`\n=== ${scenarioName} ===`);
  console.log(
    [
      pad('iter', 8),
      pad('elapsed', 9),
      pad('eta', 8),
      pad('rss', 10),
      pad('Δrss', 10),
      pad('tfheMem', 10),
      pad('Δtfhe', 10),
      pad('tkmsMem', 10),
      pad('Δtkms', 10),
      pad('gc', 6),
    ].join('  '),
  );
}

/**
 * Per-metric baselines, tracked separately per metric rather than as one
 * "first sample" snapshot. RSS is defined from the very first tick, but
 * tfhe/tkms WASM memory only becomes defined once a client has actually
 * initialized that module — using a single shared baseline sample would
 * leave the WASM delta columns permanently blank (baseline.tfheMemory stays
 * `undefined` forever) even once the metric itself is being read correctly.
 */
export type MetricBaselines = {
  rssBytes: number | undefined;
  tfheMemoryBytes: number | undefined;
  tkmsMemoryBytes: number | undefined;
};

export function updateMetricBaselines(baselines: MetricBaselines, sample: MemorySample): void {
  baselines.rssBytes ??= sample.rssBytes;
  baselines.tfheMemoryBytes ??= sample.tfheMemory?.byteLength;
  baselines.tkmsMemoryBytes ??= sample.tkmsMemory?.byteLength;
}

export function printSampleLine(
  sample: MemorySample,
  baselines: MetricBaselines,
  startedAtMs: number,
  etaMs: number | undefined,
): void {
  const line = [
    pad(String(sample.iteration), 8),
    pad(formatDuration(sample.tMs - startedAtMs), 9),
    pad(formatDuration(etaMs), 8),
    pad(formatBytes(sample.rssBytes), 10),
    pad(baselines.rssBytes !== undefined ? formatBytes(sample.rssBytes - baselines.rssBytes) : '-', 10),
    pad(sample.tfheMemory ? formatBytes(sample.tfheMemory.byteLength) : '-', 10),
    pad(
      sample.tfheMemory && baselines.tfheMemoryBytes !== undefined
        ? formatBytes(sample.tfheMemory.byteLength - baselines.tfheMemoryBytes)
        : '-',
      10,
    ),
    pad(sample.tkmsMemory ? formatBytes(sample.tkmsMemory.byteLength) : '-', 10),
    pad(
      sample.tkmsMemory && baselines.tkmsMemoryBytes !== undefined
        ? formatBytes(sample.tkmsMemory.byteLength - baselines.tkmsMemoryBytes)
        : '-',
      10,
    ),
    pad(String(sample.gcCount), 6),
  ].join('  ');
  console.log(line);
}

export function printTrendSummary(scenarioName: string, verdicts: readonly TrendVerdict[]): void {
  console.log(`\n--- ${scenarioName}: trend summary ---`);
  for (const v of verdicts) {
    const gating = GATING_METRICS.has(v.metric);
    const note = gating ? '' : '  (informational — process-level, does not gate)';
    if (v.classification === 'insufficient-data') {
      console.log(`  ${pad(v.metric, 14)} insufficient data (${v.sampleCount} post-warmup samples)${note}`);
      continue;
    }
    const marker = v.classification === 'growing' ? '⚠ GROWING' : 'plateauing';
    const ceiling =
      v.ceilingExceededBytes !== undefined ? ` CEILING EXCEEDED (+${formatBytes(v.ceilingExceededBytes)})` : '';
    console.log(
      `  ${pad(v.metric, 14)} ${pad(marker, 12)}  first-half ${formatBytes(v.firstHalfBytesPerIteration)}/iter -> second-half ${formatBytes(v.secondHalfBytesPerIteration)}/iter  (peak ${formatBytes(v.peakBytes - v.baselineBytes)} above baseline)${ceiling}${note}`,
    );
  }
}
