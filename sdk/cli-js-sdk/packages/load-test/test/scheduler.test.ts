import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";

import type { FlowExecutor, RequestRecord } from "../src/flows/types";
import { PoolExhaustedError } from "../src/flows/types";
import { Recorder, type TargetRequestRecord } from "../src/runner/recorder";
import { runScheduler } from "../src/runner/scheduler";
import { scenarioSchema } from "../src/scenario/schema";
import { readJsonl } from "../src/shared/jsonl";
import { sleep } from "../src/shared/time";

let dir: string;

beforeEach(async () => {
  dir = await mkdtemp(join(tmpdir(), "load-test-sched-"));
});

afterEach(async () => {
  await rm(dir, { recursive: true, force: true });
});

const makeExecutor = (options: {
  flow?: "input-proof";
  latencyMs?: number;
  failAfter?: number;
}): FlowExecutor & { calls: number[] } => {
  const calls: number[] = [];
  return {
    flow: options.flow ?? "input-proof",
    calls,
    prepare: () => Promise.resolve(),
    close: () => Promise.resolve(),
    async execute(index: number): Promise<RequestRecord> {
      calls.push(index);
      if (options.failAfter !== undefined && calls.length > options.failAfter) {
        throw new PoolExhaustedError("pool dry");
      }
      await sleep(options.latencyMs ?? 1);
      return {
        flow: options.flow ?? "input-proof",
        index,
        startedAtMs: Date.now(),
        sentRequestId: `req-${index.toString()}`,
        pollCount: 1,
        outcome: "succeeded",
        e2eLatencyMs: options.latencyMs ?? 1,
      };
    },
  };
};

const scenario = (shape: unknown, extras: Record<string, unknown> = {}) =>
  scenarioSchema.parse({
    name: "test",
    flows: [{ flow: "input-proof", weight: 1 }],
    shape,
    drainTimeoutSec: 10,
    ...extras,
  });

describe("runScheduler", () => {
  it("writes normalized per-relayer request files", async () => {
    const recorder = await Recorder.open(join(dir, "requests.jsonl"), {
      relayerAPath: join(dir, "requests-a.jsonl"),
      relayerBPath: join(dir, "requests-b.jsonl"),
    });
    await recorder.record({
      flow: "input-proof",
      index: 7,
      startedAtMs: 10,
      sentRequestId: "rid",
      jobId: "job-a",
      pollCount: 2,
      outcome: "succeeded",
      e2eLatencyMs: 100,
      jobIdB: "job-b",
      pollCountB: 3,
      outcomeB: "timed_out",
      errorLabelB: "deadline",
      e2eLatencyMsB: 200,
    });
    await recorder.close();

    const paired = await readJsonl<RequestRecord>(join(dir, "requests.jsonl"));
    const relayerA = await readJsonl<TargetRequestRecord>(join(dir, "requests-a.jsonl"));
    const relayerB = await readJsonl<TargetRequestRecord>(join(dir, "requests-b.jsonl"));

    expect(paired[0]?.jobIdB).toBe("job-b");
    expect(relayerA[0]?.relayerTarget).toBe("A");
    expect(relayerA[0]?.jobId).toBe("job-a");
    expect(relayerA[0]?.outcome).toBe("succeeded");
    expect(relayerB[0]?.relayerTarget).toBe("B");
    expect(relayerB[0]?.jobId).toBe("job-b");
    expect(relayerB[0]?.outcome).toBe("timed_out");
    expect(relayerB[0]?.errorLabel).toBe("deadline");
  });

  it("submits the planned count and records every completion", async () => {
    const executor = makeExecutor({});
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const result = await runScheduler({
      scenario: scenario({ kind: "constant", rps: 50, durationSec: 0.4 }),
      executors: new Map([["input-proof", executor]]),
      recorder,
    });
    await recorder.close();
    expect(result.submitted).toBe(20);
    expect(result.completed).toBe(20);
    expect(result.abandoned).toBe(0);
    expect(recorder.records).toHaveLength(20);
  });

  it("keeps the offered load open-model: slow responses do not delay submissions", async () => {
    // 10 requests at 50 rps with 400 ms latency each: closed-model would take
    // > 4 s, open-model finishes submitting in ~200 ms.
    const executor = makeExecutor({ latencyMs: 400 });
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const result = await runScheduler({
      scenario: scenario({ kind: "constant", rps: 50, durationSec: 0.2 }),
      executors: new Map([["input-proof", executor]]),
      recorder,
    });
    await recorder.close();
    expect(result.submitted).toBe(10);
    expect(result.submissionDurationMs).toBeLessThan(1000);
  });

  it("emits dispatch lag, inflight, and abandonment telemetry", async () => {
    const executor = makeExecutor({ latencyMs: 20 });
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const dispatches: Array<{ lagMs: number; inflight: number }> = [];
    let abandoned = -1;
    await runScheduler({
      scenario: scenario({ kind: "burst", count: 5, maxRps: 100 }),
      executors: new Map([["input-proof", executor]]),
      recorder,
      telemetry: {
        recordDispatch: (lagMs, inflight) => dispatches.push({ lagMs, inflight }),
        recordDropped: () => undefined,
        recordAbandoned: (count) => { abandoned = count; },
      },
    });
    await recorder.close();
    expect(dispatches).toHaveLength(5);
    expect(dispatches.every((entry) => entry.lagMs >= 0)).toBe(true);
    expect(Math.max(...dispatches.map((entry) => entry.inflight))).toBeGreaterThan(0);
    expect(abandoned).toBe(0);
  });

  it("records dispatch telemetry before invoking synchronous executor work", async () => {
    const order: string[] = [];
    const executor: FlowExecutor = {
      flow: "input-proof",
      prepare: () => Promise.resolve(),
      close: () => Promise.resolve(),
      async execute(index): Promise<RequestRecord> {
        order.push("execute");
        return {
          flow: "input-proof",
          index,
          startedAtMs: Date.now(),
          sentRequestId: "request",
          pollCount: 1,
          outcome: "succeeded",
        };
      },
    };
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    await runScheduler({
      scenario: scenario({ kind: "burst", count: 1, maxRps: 1 }),
      executors: new Map([["input-proof", executor]]),
      recorder,
      telemetry: {
        recordDispatch: () => order.push("telemetry"),
        recordDropped: () => undefined,
        recordAbandoned: () => undefined,
      },
    });
    await recorder.close();
    expect(order).toEqual(["telemetry", "execute"]);
  });

  it("closed-model submissions are bounded by active VUs and request completion", async () => {
    const executor = makeExecutor({ latencyMs: 120 });
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const result = await runScheduler({
      scenario: scenario({
        kind: "closed",
        vus: 2,
        durationSec: 0.25,
        thinkTimeMs: 0,
        maxIterations: 100,
      }),
      executors: new Map([["input-proof", executor]]),
      recorder,
    });
    await recorder.close();
    expect(result.submitted).toBeGreaterThanOrEqual(2);
    expect(result.submitted).toBeLessThanOrEqual(6);
    expect(result.completed).toBe(result.submitted);
    expect(new Set(recorder.records.map((record) => record.loadStage?.label))).toEqual(
      new Set(["2vu"]),
    );
  });

  it("stops submitting when a pool runs dry", async () => {
    const executor = makeExecutor({ failAfter: 5 });
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const result = await runScheduler({
      scenario: scenario({ kind: "burst", count: 100, maxRps: 1000 }),
      executors: new Map([["input-proof", executor]]),
      recorder,
    });
    await recorder.close();
    expect(result.poolExhausted).toBe(true);
    expect(result.submitted).toBe(6);
    expect(executor.calls).toEqual([0, 1, 2, 3, 4, 5]);
  });

  it("does not overclaim a zero-spacing burst after the first dry claim", async () => {
    const executor = makeExecutor({ failAfter: 0 });
    const recorder = await Recorder.open(join(dir, "dry-zero-gap.jsonl"));
    const result = await runScheduler({
      scenario: scenario({ kind: "burst", count: 20 }),
      executors: new Map([["input-proof", executor]]),
      recorder,
    });
    await recorder.close();
    expect(result.poolExhausted).toBe(true);
    expect(result.submitted).toBe(1);
    expect(executor.calls).toEqual([0]);
  });

  it("halts at a segment boundary when feedback says stop", async () => {
    const executor = makeExecutor({});
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const result = await runScheduler({
      scenario: scenario({
        kind: "segments",
        segments: [
          { fromRps: 50, toRps: 50, durationSec: 0.2 },
          { fromRps: 100, toRps: 100, durationSec: 0.2 },
        ],
      }),
      executors: new Map([["input-proof", executor]]),
      recorder,
      onSegmentEnd: () => Promise.resolve("stop"),
    });
    await recorder.close();
    expect(result.stoppedAtSegment).toBe(0);
    // Only the first segment's arrivals fired.
    expect(result.submitted).toBeLessThanOrEqual(10);
  });

  it("aborts promptly on signal", async () => {
    const executor = makeExecutor({});
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const controller = new AbortController();
    setTimeout(() => controller.abort(), 50);
    const result = await runScheduler({
      scenario: scenario({ kind: "constant", rps: 10, durationSec: 60 }),
      executors: new Map([["input-proof", executor]]),
      recorder,
      signal: controller.signal,
    });
    await recorder.close();
    expect(result.submitted).toBeLessThan(20);
    expect(result.interrupted).toBe(true);
  });

  it("aborts and settles every live request before returning from drain timeout", async () => {
    let liveTasks = 0;
    const executor: FlowExecutor = {
      flow: "input-proof",
      prepare: () => Promise.resolve(),
      close: () => Promise.resolve(),
      execute: (index, signal) =>
        new Promise<RequestRecord>((resolve) => {
          liveTasks += 1;
          const finish = (): void => {
            liveTasks -= 1;
            resolve({
              flow: "input-proof",
              index,
              startedAtMs: Date.now(),
              sentRequestId: `req-${index.toString()}`,
              pollCount: 0,
              outcome: "timed_out",
              errorLabel: "aborted_after_drain_timeout",
            });
          };
          if (signal.aborted) finish();
          else signal.addEventListener("abort", finish, { once: true });
        }),
    };
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));
    const result = await runScheduler({
      scenario: scenario(
        { kind: "burst", count: 1 },
        { drainTimeoutSec: 0.02 },
      ),
      executors: new Map([["input-proof", executor]]),
      recorder,
    });

    expect(result.abandoned).toBe(1);
    expect(liveTasks).toBe(0);
    expect(recorder.records).toHaveLength(1);
    await recorder.close();
    await sleep(20);
    expect(recorder.records).toHaveLength(1);
  });

  it("rejects unexpected executor errors instead of counting them completed", async () => {
    const executor: FlowExecutor = {
      flow: "input-proof",
      prepare: () => Promise.resolve(),
      close: () => Promise.resolve(),
      execute: () => Promise.reject(new Error("driver exploded")),
    };
    const recorder = await Recorder.open(join(dir, "requests.jsonl"));

    await expect(
      runScheduler({
        scenario: scenario({ kind: "burst", count: 1 }),
        executors: new Map([["input-proof", executor]]),
        recorder,
      }),
    ).rejects.toThrow("driver exploded");
    expect(recorder.records).toHaveLength(0);
    await recorder.close();
  });

  it("passes caller cancellation to active closed requests and settles promptly", async () => {
    const controller = new AbortController();
    let receivedSignal: AbortSignal | undefined;
    const executor: FlowExecutor = {
      flow: "input-proof",
      prepare: () => Promise.resolve(),
      close: () => Promise.resolve(),
      execute: (index, signal) => new Promise((resolve) => {
        receivedSignal = signal;
        signal.addEventListener("abort", () => resolve({
          flow: "input-proof", index, startedAtMs: Date.now(), sentRequestId: "cancelled",
          pollCount: 0, outcome: "timed_out", errorLabel: "client_aborted",
        }), { once: true });
      }),
    };
    const recorder = await Recorder.open(join(dir, "cancelled.jsonl"));
    setTimeout(() => controller.abort(), 10);
    const result = await runScheduler({
      scenario: scenario({ kind: "closed", vus: 1, durationSec: 60, maxIterations: 1 }),
      executors: new Map([["input-proof", executor]]), recorder,
      signal: controller.signal, cancellationTimeoutMs: 50,
    });
    expect(receivedSignal?.aborted).toBe(true);
    expect(result.interrupted).toBe(true);
    expect(result.abandoned).toBe(0);
    await recorder.close();
  });

  it("bounds settlement when a cancelled closed request ignores its signal", async () => {
    const controller = new AbortController();
    const executor: FlowExecutor = {
      flow: "input-proof", prepare: () => Promise.resolve(), close: () => Promise.resolve(),
      execute: () => new Promise(() => undefined),
    };
    const recorder = await Recorder.open(join(dir, "ignored-cancel.jsonl"));
    setTimeout(() => controller.abort(), 10);
    const result = await runScheduler({
      scenario: scenario({ kind: "closed", vus: 1, durationSec: 60, maxIterations: 1 }),
      executors: new Map([["input-proof", executor]]), recorder,
      signal: controller.signal, cancellationTimeoutMs: 20,
    });
    expect(result.interrupted).toBe(true);
    expect(result.abandoned).toBe(1);
    await recorder.close();
  });

  it("yields a zero-spacing burst and stops after the first fatal request", async () => {
    let calls = 0;
    const executor: FlowExecutor = {
      flow: "input-proof", prepare: () => Promise.resolve(), close: () => Promise.resolve(),
      execute: () => { calls += 1; return Promise.reject(new Error("fatal burst")); },
    };
    const recorder = await Recorder.open(join(dir, "fatal-burst.jsonl"));
    await expect(runScheduler({
      scenario: scenario({ kind: "burst", count: 20 }),
      executors: new Map([["input-proof", executor]]), recorder,
    })).rejects.toThrow("fatal burst");
    expect(calls).toBe(1);
    await recorder.close();
  });
});
