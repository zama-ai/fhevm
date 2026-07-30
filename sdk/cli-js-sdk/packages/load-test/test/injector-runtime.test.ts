import { describe, expect, it } from "vitest";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
  assessInjectorHealth,
  InjectorRuntimeCollector,
  MIN_DISPATCH_HEALTH_SAMPLES,
  MIN_RUNTIME_HEALTH_SAMPLES,
} from "../src/collectors/injector-runtime";

describe("assessInjectorHealth", () => {
  it("is unavailable without samples", () => {
    expect(assessInjectorHealth({
      sampleCount: 0, dispatchCount: 0, dropped: 0, abandoned: 0,
    }).verdict).toBe("unavailable");
  });
  it("reports healthy injector signals", () => {
    expect(assessInjectorHealth({
      sampleCount: MIN_RUNTIME_HEALTH_SAMPLES,
      dispatchCount: MIN_DISPATCH_HEALTH_SAMPLES,
      dispatchLagP99Ms: 4,
      maxEventLoopLagP99Ms: 8,
      peakEventLoopUtilization: 0.5,
      dropped: 0,
      abandoned: 0,
    }).verdict).toBe("healthy");
  });
  it("is indeterminate when statistical sample floors are not met", () => {
    const health = assessInjectorHealth({
      sampleCount: MIN_RUNTIME_HEALTH_SAMPLES - 1,
      dispatchCount: MIN_DISPATCH_HEALTH_SAMPLES - 1,
      dispatchLagP99Ms: 30,
      maxEventLoopLagP99Ms: 30,
      peakEventLoopUtilization: 1,
      dropped: 0,
      abandoned: 0,
    });
    expect(health.verdict).toBe("indeterminate");
    expect(health.reasons).toHaveLength(2);
  });
  it("degrades once meaningful lag has enough evidence without duplicate reasons", () => {
    const health = assessInjectorHealth({
      sampleCount: MIN_RUNTIME_HEALTH_SAMPLES,
      dispatchCount: MIN_DISPATCH_HEALTH_SAMPLES,
      dispatchLagP99Ms: 30,
      dropped: 0,
      abandoned: 0,
    });
    expect(health.verdict).toBe("degraded");
    expect(health.reasons).toEqual(["Dispatch lag p99 exceeded 25 ms (30.0 ms)."]);
  });
  it.each([
    { dropped: 1, abandoned: 0 },
    { dropped: 0, abandoned: 1 },
  ])("fails immediately on lost work with low samples: %o", ({ dropped, abandoned }) => {
    expect(assessInjectorHealth({
      sampleCount: 0,
      dispatchCount: 0,
      dropped,
      abandoned,
    }).verdict).toBe("unhealthy");
  });

  it("starts and stops idempotently without leaving the GC observer active", async () => {
    const directory = await mkdtemp(join(tmpdir(), "load-test-injector-"));
    try {
      const collector = new InjectorRuntimeCollector(join(directory, "runtime.jsonl"), 60_000);
      await collector.start();
      await collector.start();
      expect(collector.samples).toHaveLength(0);
      await collector.stop();
      const samplesAfterStop = collector.samples.length;
      const summary = collector.summary();
      await collector.stop();
      expect(samplesAfterStop).toBe(1);
      expect(collector.samples).toHaveLength(samplesAfterStop);
      expect(collector.samples[0]?.observationDurationMs).toBeLessThan(30_000);
      expect(summary).toMatchObject({
        sampleCount: 1,
        healthSampleCount: 0,
        maxEventLoopLagP99Ms: undefined,
        peakEventLoopUtilization: undefined,
      });
    } finally {
      await rm(directory, { recursive: true, force: true });
    }
  });

  it("treats repeated abandoned snapshots idempotently", () => {
    const collector = new InjectorRuntimeCollector("unused.jsonl");
    collector.recordAbandoned(4);
    collector.recordAbandoned(4);
    collector.recordAbandoned(2);
    expect(collector.summary().scheduler.abandoned).toBe(4);
  });
});
