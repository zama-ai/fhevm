import { appendFile, mkdir } from 'node:fs/promises';
import { dirname } from 'node:path';
import { PerformanceObserver } from 'node:perf_hooks';

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

export type WasmMemoryInfo = {
  readonly byteLength: number;
  readonly pages: number;
};

export type MemorySample = {
  readonly tMs: number;
  readonly iteration: number;
  readonly rssBytes: number;
  readonly heapUsedBytes: number;
  readonly externalBytes: number;
  readonly arrayBuffersBytes: number;
  readonly tfheMemory: WasmMemoryInfo | undefined;
  readonly tkmsMemory: WasmMemoryInfo | undefined;
  readonly gcCount: number;
  readonly gcDurationMs: number;
};

export type MemorySamplerOptions = {
  /** Sampling cadence. Defaults to 2s — real leak trends need minutes, not milliseconds. */
  readonly intervalMs?: number;
  /** Called on every tick with the current iteration count for this scenario. */
  readonly getIteration: () => number;
  /** Live WASM linear-memory reader for the tfhe module, if this scenario touches it. */
  readonly readTfheMemory?: () => WasmMemoryInfo | undefined;
  /** Live WASM linear-memory reader for the tkms module, if this scenario touches it. */
  readonly readTkmsMemory?: () => WasmMemoryInfo | undefined;
  /** Optional JSONL sink — one line per sample, for post-run inspection/plotting. */
  readonly jsonlPath?: string;
  /** Called after every sample is recorded (used by the console reporter). */
  readonly onSample?: (sample: MemorySample) => void;
};

// ---------------------------------------------------------------------------
// MemorySampler
// ---------------------------------------------------------------------------

/**
 * Periodically snapshots process + WASM memory so a long-running scenario's
 * growth trend can be inspected after the fact. Forces `global.gc()` before
 * every sample when available (`--expose-gc`) — without it, GC scheduling
 * noise dominates the signal and hides real trends.
 */
export class MemorySampler {
  private readonly intervalMs: number;
  private readonly options: MemorySamplerOptions;
  private timer: NodeJS.Timeout | undefined;
  private jsonlReady: Promise<void> | undefined;
  private gcCount = 0;
  private gcDurationMs = 0;
  private readonly gcObserver = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      this.gcCount += 1;
      this.gcDurationMs += entry.duration;
    }
  });
  private readonly _samples: MemorySample[] = [];

  constructor(options: MemorySamplerOptions) {
    this.options = options;
    this.intervalMs = options.intervalMs ?? 2_000;
    if (options.jsonlPath !== undefined) {
      this.jsonlReady = mkdir(dirname(options.jsonlPath), { recursive: true }).then(() => undefined);
    }
  }

  get samples(): readonly MemorySample[] {
    return this._samples;
  }

  start(): void {
    this.gcObserver.observe({ entryTypes: ['gc'] });
    this.timer = setInterval(() => {
      void this.tick();
    }, this.intervalMs);
    this.timer.unref();
  }

  async stop(): Promise<void> {
    if (this.timer !== undefined) {
      clearInterval(this.timer);
      this.timer = undefined;
    }
    this.gcObserver.disconnect();
    // Final snapshot so the last measured state is always captured, even if it
    // lands between two periodic ticks.
    await this.tick();
  }

  /** Forces a GC pass (if available) and records one sample immediately. */
  async tick(): Promise<MemorySample> {
    if (typeof global.gc === 'function') {
      global.gc();
    }

    const memoryUsage = process.memoryUsage();
    const sample: MemorySample = {
      tMs: performanceNowMs(),
      iteration: this.options.getIteration(),
      rssBytes: memoryUsage.rss,
      heapUsedBytes: memoryUsage.heapUsed,
      externalBytes: memoryUsage.external,
      arrayBuffersBytes: memoryUsage.arrayBuffers,
      tfheMemory: this.options.readTfheMemory?.(),
      tkmsMemory: this.options.readTkmsMemory?.(),
      gcCount: this.gcCount,
      gcDurationMs: this.gcDurationMs,
    };

    this._samples.push(sample);
    this.options.onSample?.(sample);
    await this.appendJsonl(sample);
    return sample;
  }

  private async appendJsonl(sample: MemorySample): Promise<void> {
    if (this.options.jsonlPath === undefined) {
      return;
    }
    await this.jsonlReady;
    await appendFile(this.options.jsonlPath, `${JSON.stringify(sample)}\n`, 'utf-8');
  }
}

function performanceNowMs(): number {
  // Wall-clock, not monotonic-since-process-start: samples are compared across
  // a run that can last tens of minutes, and JSONL output is easiest to read
  // when timestamps are absolute.
  return Date.now();
}

/** `global.gc` only exists under `--expose-gc`; `run.mjs` re-execs itself with that flag set. */
export function assertGcIsExposed(): void {
  if (typeof global.gc !== 'function') {
    throw new Error(
      'global.gc() is not available. Run this script with `node --expose-gc` (test/memleaks/run.mjs does this automatically).',
    );
  }
}
