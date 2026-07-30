import { monitorEventLoopDelay, performance, PerformanceObserver } from "node:perf_hooks";

import { JsonlWriter } from "../shared/jsonl";
import { epochNowMs } from "../shared/time";
import type { Collector } from "./types";

export type InjectorRuntimeSample = Readonly<{
  tMs: number;
  /** Monotonic interval represented by ELU and event-loop lag fields. */
  observationDurationMs: number;
  eventLoopUtilization: number;
  eventLoopLagMeanMs: number;
  eventLoopLagP99Ms: number;
  cpuUserMicros: number;
  cpuSystemMicros: number;
  rssBytes: number;
  heapUsedBytes: number;
  gcCount: number;
  gcDurationMs: number;
}>;

export type SchedulerTelemetry = Readonly<{
  dispatchLagMs: readonly number[];
  peakInflight: number;
  /** Dispatches delayed by more than 25 ms beyond their planned offset. */
  backpressureEvents: number;
  dropped: number;
  abandoned: number;
}>;

export type InjectorHealth = Readonly<{
  verdict: "healthy" | "degraded" | "unhealthy" | "indeterminate" | "unavailable";
  reasons: readonly string[];
}>;

/** Ten one-second samples provide a minimum sustained runtime observation window. */
export const MIN_RUNTIME_HEALTH_SAMPLES = 10;
/** A p99 needs at least 100 dispatches so its tail represents more than one observation. */
export const MIN_DISPATCH_HEALTH_SAMPLES = 100;

export type InjectorRuntimeSummary = Readonly<{
  /** All raw samples, including a potentially short final resource snapshot. */
  sampleCount: number;
  /** Samples with a long enough interval to support ELU/lag health assessment. */
  healthSampleCount?: number;
  scheduler: SchedulerTelemetry;
  dispatchLagP95Ms?: number;
  dispatchLagP99Ms?: number;
  maxEventLoopLagP99Ms?: number;
  peakEventLoopUtilization?: number;
  peakRssBytes?: number;
  cpuUserMicros?: number;
  cpuSystemMicros?: number;
  gcCount: number;
  gcDurationMs: number;
  health: InjectorHealth;
}>;

const percentile = (values: readonly number[], p: number): number | undefined => {
  if (values.length === 0) return undefined;
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.ceil(sorted.length * p) - 1)];
};

export const assessInjectorHealth = (input: {
  sampleCount: number;
  dispatchCount: number;
  dispatchLagP99Ms?: number;
  maxEventLoopLagP99Ms?: number;
  peakEventLoopUtilization?: number;
  dropped: number;
  abandoned: number;
}): InjectorHealth => {
  const critical: string[] = [];
  const warnings: string[] = [];
  if (input.dropped > 0) critical.push(`${input.dropped.toString()} scheduled dispatch(es) were dropped.`);
  if (input.abandoned > 0) critical.push(`${input.abandoned.toString()} request(s) remained inflight after drain.`);
  if (input.sampleCount === 0 && input.dispatchCount === 0 && critical.length === 0) {
    return { verdict: "unavailable", reasons: ["No injector runtime or scheduler samples were collected."] };
  }

  const runtimeSufficient = input.sampleCount >= MIN_RUNTIME_HEALTH_SAMPLES;
  const dispatchSufficient = input.dispatchCount >= MIN_DISPATCH_HEALTH_SAMPLES;
  if (dispatchSufficient) {
    if ((input.dispatchLagP99Ms ?? 0) > 100) critical.push(`Dispatch lag p99 exceeded 100 ms (${input.dispatchLagP99Ms?.toFixed(1)} ms).`);
    else if ((input.dispatchLagP99Ms ?? 0) > 25) warnings.push(`Dispatch lag p99 exceeded 25 ms (${input.dispatchLagP99Ms?.toFixed(1)} ms).`);
  }
  if (runtimeSufficient) {
    if ((input.maxEventLoopLagP99Ms ?? 0) > 100) critical.push(`Event-loop lag p99 exceeded 100 ms (${input.maxEventLoopLagP99Ms?.toFixed(1)} ms).`);
    else if ((input.maxEventLoopLagP99Ms ?? 0) > 25) warnings.push(`Event-loop lag p99 exceeded 25 ms (${input.maxEventLoopLagP99Ms?.toFixed(1)} ms).`);
    if ((input.peakEventLoopUtilization ?? 0) > 0.95) warnings.push(`Event-loop utilization peaked above 95% (${((input.peakEventLoopUtilization ?? 0) * 100).toFixed(1)}%).`);
  }
  if (critical.length > 0) return { verdict: "unhealthy", reasons: [...critical, ...warnings] };
  if (warnings.length > 0) return { verdict: "degraded", reasons: warnings };
  const insufficient: string[] = [];
  if (!dispatchSufficient) {
    insufficient.push(`Dispatch health needs at least ${MIN_DISPATCH_HEALTH_SAMPLES.toString()} samples; collected ${input.dispatchCount.toString()}.`);
  }
  if (!runtimeSufficient) {
    insufficient.push(`Runtime health needs at least ${MIN_RUNTIME_HEALTH_SAMPLES.toString()} samples; collected ${input.sampleCount.toString()}.`);
  }
  if (insufficient.length > 0) return { verdict: "indeterminate", reasons: insufficient };
  return { verdict: "healthy", reasons: ["Injector scheduling and runtime signals stayed within health thresholds."] };
};

export class InjectorRuntimeCollector implements Collector {
  readonly name = "injector-runtime";
  private timer: NodeJS.Timeout | undefined;
  private writer: JsonlWriter<InjectorRuntimeSample> | undefined;
  private readonly histogram = monitorEventLoopDelay({ resolution: 20 });
  private previousElu = performance.eventLoopUtilization();
  private startCpu = process.cpuUsage();
  private readonly allSamples: InjectorRuntimeSample[] = [];
  private readonly healthSamples: InjectorRuntimeSample[] = [];
  private previousSampleAtMs = performance.now();
  private readonly dispatchLagMs: number[] = [];
  private peakInflight = 0;
  private backpressureEvents = 0;
  private dropped = 0;
  private abandoned = 0;
  private gcCount = 0;
  private gcDurationMs = 0;
  private started = false;
  private readonly gcObserver = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      this.gcCount += 1;
      this.gcDurationMs += entry.duration;
    }
  });

  constructor(private readonly outputPath: string, private readonly intervalMs = 1_000) {}

  get samples(): readonly InjectorRuntimeSample[] { return this.allSamples; }

  recordDispatch(lagMs: number, inflight: number): void {
    if (!Number.isFinite(lagMs) || !Number.isFinite(inflight)) return;
    this.dispatchLagMs.push(Math.max(0, lagMs));
    if (lagMs > 25) this.backpressureEvents += 1;
    this.peakInflight = Math.max(this.peakInflight, inflight);
  }

  recordDropped(count = 1): void {
    if (Number.isFinite(count) && count > 0) this.dropped += count;
  }
  /** Scheduler reports current/final abandoned work; repeated reports are idempotent. */
  recordAbandoned(count: number): void {
    if (Number.isFinite(count) && count >= 0) this.abandoned = Math.max(this.abandoned, count);
  }

  async start(): Promise<void> {
    if (this.started) return;
    this.writer = await JsonlWriter.open<InjectorRuntimeSample>(this.outputPath);
    this.started = true;
    this.previousElu = performance.eventLoopUtilization();
    this.previousSampleAtMs = performance.now();
    this.startCpu = process.cpuUsage();
    try {
      this.histogram.enable();
      this.gcObserver.observe({ entryTypes: ["gc"] });
      // The first utilization delta needs a real observation interval. An
      // immediate sample is dominated by collector startup and often reads 1.
      this.timer = setInterval(() => { void this.sample().catch(() => undefined); }, this.intervalMs);
      this.timer.unref();
    } catch (error) {
      this.started = false;
      this.histogram.disable();
      this.gcObserver.disconnect();
      await this.writer.close();
      this.writer = undefined;
      throw error;
    }
  }

  private async sample(): Promise<void> {
    const sampledAtMs = performance.now();
    const observationDurationMs = Math.max(0, sampledAtMs - this.previousSampleAtMs);
    this.previousSampleAtMs = sampledAtMs;
    const elu = performance.eventLoopUtilization(this.previousElu);
    this.previousElu = performance.eventLoopUtilization();
    const cpu = process.cpuUsage(this.startCpu);
    const memory = process.memoryUsage();
    const sample: InjectorRuntimeSample = {
      tMs: epochNowMs(),
      observationDurationMs,
      eventLoopUtilization: elu.utilization,
      eventLoopLagMeanMs: Number(this.histogram.mean) / 1e6 || 0,
      eventLoopLagP99Ms: Number(this.histogram.percentile(99)) / 1e6 || 0,
      cpuUserMicros: cpu.user,
      cpuSystemMicros: cpu.system,
      rssBytes: memory.rss,
      heapUsedBytes: memory.heapUsed,
      gcCount: this.gcCount,
      gcDurationMs: this.gcDurationMs,
    };
    this.allSamples.push(sample);
    // Preserve every final CPU/memory snapshot in JSONL, but do not let a
    // teardown sample taken just after a periodic tick dominate ELU/lag peaks.
    if (observationDurationMs >= this.intervalMs / 2) this.healthSamples.push(sample);
    await this.writer?.write(sample);
    this.histogram.reset();
  }

  async stop(): Promise<void> {
    if (!this.started) return;
    this.started = false;
    if (this.timer) clearInterval(this.timer);
    this.timer = undefined;
    const errors: unknown[] = [];
    try {
      await this.sample();
    } catch (error) {
      errors.push(error);
    }
    this.histogram.disable();
    this.gcObserver.disconnect();
    try {
      await this.writer?.close();
    } catch (error) {
      errors.push(error);
    }
    this.writer = undefined;
    if (errors.length > 0) {
      throw new AggregateError(errors, "Failed to stop injector runtime collector");
    }
  }

  summary(): InjectorRuntimeSummary {
    const dispatchLagP95Ms = percentile(this.dispatchLagMs, 0.95);
    const dispatchLagP99Ms = percentile(this.dispatchLagMs, 0.99);
    const maxEventLoopLagP99Ms = this.healthSamples.length > 0
      ? Math.max(...this.healthSamples.map((sample) => sample.eventLoopLagP99Ms)) : undefined;
    const peakEventLoopUtilization = this.healthSamples.length > 0
      ? Math.max(...this.healthSamples.map((sample) => sample.eventLoopUtilization)) : undefined;
    const peakRssBytes = this.allSamples.length > 0
      ? Math.max(...this.allSamples.map((sample) => sample.rssBytes)) : undefined;
    const last = this.allSamples.at(-1);
    const scheduler = {
      dispatchLagMs: this.dispatchLagMs,
      peakInflight: this.peakInflight,
      backpressureEvents: this.backpressureEvents,
      dropped: this.dropped,
      abandoned: this.abandoned,
    };
    const health = assessInjectorHealth({
      sampleCount: this.healthSamples.length,
      dispatchCount: this.dispatchLagMs.length,
      dispatchLagP99Ms,
      maxEventLoopLagP99Ms,
      peakEventLoopUtilization,
      dropped: this.dropped,
      abandoned: this.abandoned,
    });
    return {
      sampleCount: this.allSamples.length,
      healthSampleCount: this.healthSamples.length,
      scheduler, dispatchLagP95Ms, dispatchLagP99Ms,
      maxEventLoopLagP99Ms, peakEventLoopUtilization, peakRssBytes,
      cpuUserMicros: last?.cpuUserMicros, cpuSystemMicros: last?.cpuSystemMicros,
      gcCount: this.gcCount, gcDurationMs: this.gcDurationMs, health,
    };
  }
}
