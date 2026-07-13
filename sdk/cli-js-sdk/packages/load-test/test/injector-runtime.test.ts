import { describe, expect, it } from "vitest";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { assessInjectorHealth, InjectorRuntimeCollector } from "../src/collectors/injector-runtime";

describe("assessInjectorHealth", () => {
  it("is unavailable without samples", () => {
    expect(assessInjectorHealth({ sampleCount: 0, dropped: 0, abandoned: 0 }).verdict).toBe("unavailable");
  });
  it("reports healthy injector signals", () => {
    expect(assessInjectorHealth({ sampleCount: 2, dispatchLagP99Ms: 4, maxEventLoopLagP99Ms: 8, peakEventLoopUtilization: 0.5, dropped: 0, abandoned: 0 }).verdict).toBe("healthy");
  });
  it("degrades on meaningful lag and fails on dropped work", () => {
    expect(assessInjectorHealth({ sampleCount: 2, dispatchLagP99Ms: 30, dropped: 0, abandoned: 0 }).verdict).toBe("degraded");
    expect(assessInjectorHealth({ sampleCount: 2, dispatchLagP99Ms: 2, dropped: 1, abandoned: 0 }).verdict).toBe("unhealthy");
  });

  it("starts and stops idempotently without leaving the GC observer active", async () => {
    const directory = await mkdtemp(join(tmpdir(), "load-test-injector-"));
    try {
      const collector = new InjectorRuntimeCollector(join(directory, "runtime.jsonl"), 60_000);
      await collector.start();
      await collector.start();
      await collector.stop();
      const samplesAfterStop = collector.samples.length;
      await collector.stop();
      expect(samplesAfterStop).toBe(2);
      expect(collector.samples).toHaveLength(samplesAfterStop);
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
