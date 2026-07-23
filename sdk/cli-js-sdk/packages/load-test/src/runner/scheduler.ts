import { setImmediate as yieldEventLoop } from "node:timers/promises";

import {
  PoolExhaustedError,
  type FlowExecutor,
  type RequestLoadStage,
} from "../flows/types";
import type { FlowKind } from "../relayer/types";
import type { RateShape, Scenario } from "../scenario/schema";
import { logger } from "../shared/logger";
import { monotonicNowMs, sleep } from "../shared/time";
import type { Recorder } from "./recorder";
import {
  arrivalOffsetsMs,
  closedStageOffsetsMs,
  createFlowSequencer,
  openLoadStageForOffset,
  segmentBoundariesMs,
} from "./schedule";

export type SegmentVerdict = "continue" | "stop";

export type SchedulerTelemetrySink = Readonly<{
  recordDispatch(lagMs: number, inflight: number): void;
  recordDropped(count?: number): void;
  recordAbandoned(count: number): void;
}>;

export type SchedulerOptions = Readonly<{
  scenario: Scenario;
  executors: ReadonlyMap<FlowKind, FlowExecutor>;
  recorder: Recorder;
  signal?: AbortSignal;
  /**
   * Saturation feedback consulted at each segment boundary; returning "stop"
   * ends submission early (used by ramp scenarios).
   */
  onSegmentEnd?: (segmentIndex: number) => Promise<SegmentVerdict>;
  /** Progress callback after every submitted request. */
  onSubmitted?: (submitted: number) => void;
  /** Progress callback after every completed request. */
  onCompleted?: (done: number, submitted: number) => void;
  telemetry?: SchedulerTelemetrySink;
  cancellationTimeoutMs?: number;
}>;

export type SchedulerResult = Readonly<{
  submitted: number;
  completed: number;
  /** Requests still outstanding when the drain timeout expired. */
  abandoned: number;
  /** Index of the segment after which saturation stopped submission. */
  stoppedAtSegment?: number;
  /** True when a single-use pool ran dry mid-run. */
  poolExhausted: boolean;
  submissionDurationMs: number;
  /** Submission stopped because the caller's signal was aborted. */
  interrupted: boolean;
}>;

const settleOrAbort = async <T>(
  promise: Promise<T>,
  signal: AbortSignal,
): Promise<{ settled: true; value: T } | { settled: false }> => {
  if (signal.aborted) return { settled: false };
  return new Promise((resolve) => {
    const onAbort = (): void => resolve({ settled: false });
    signal.addEventListener("abort", onAbort, { once: true });
    void promise.then((value) => {
      signal.removeEventListener("abort", onAbort);
      resolve({ settled: true, value });
    });
  });
};

const executeOne = async (
  options: SchedulerOptions,
  flow: FlowKind,
  index: number,
  signal: AbortSignal,
  onFatal: (error: Error) => void,
  onPoolExhausted: () => void,
  shouldRecord: () => boolean,
  loadStage?: RequestLoadStage,
): Promise<"completed" | "pool_exhausted"> => {
  const executor = options.executors.get(flow);
  if (!executor) throw new Error(`No executor for flow ${flow}.`);
  try {
    const record = await executor.execute(index, signal);
    if (shouldRecord()) {
      await options.recorder.record(loadStage ? { ...record, loadStage } : record);
    }
    return "completed";
  } catch (error: unknown) {
    if (error instanceof PoolExhaustedError) {
      // Publish exhaustion in the same promise reaction that observes the
      // failed claim. Waiting for the outer task's `.then` allows a zero-gap
      // arrival loop to claim one additional durable cursor position.
      onPoolExhausted();
      logger.error(error.message);
      return "pool_exhausted";
    }
    const cause = error instanceof Error ? error : new Error(String(error));
    const failure = new Error(
      `Executor ${flow} request ${index.toString()} failed unexpectedly: ${cause.message}`,
      { cause },
    );
    onFatal(failure);
    throw failure;
  }
};

const runClosedScheduler = async (
  options: SchedulerOptions,
  executionSignal: AbortSignal,
  onFatal: (error: Error) => void,
): Promise<SchedulerResult> => {
  const { scenario } = options;
  if (scenario.shape.kind !== "closed") {
    throw new Error("Closed scheduler requires a closed scenario shape.");
  }
  const shape = scenario.shape;

  const sequencer = createFlowSequencer(scenario.flows);
  const stages = closedStageOffsetsMs(shape);
  const startMono = monotonicNowMs();
  let submitted = 0;
  let completed = 0;
  let poolExhausted = false;
  let active = 0;
  let acceptingRecords = true;
  const stopSignal = options.signal
    ? AbortSignal.any([options.signal, executionSignal])
    : executionSignal;

  const nextIndex = (): number | undefined => {
    if (poolExhausted || stopSignal.aborted) return undefined;
    if (shape.maxIterations !== undefined && submitted >= shape.maxIterations) {
      return undefined;
    }
    const index = submitted;
    submitted += 1;
    options.onSubmitted?.(submitted);
    return index;
  };

  const runVu = async (stage: (typeof stages)[number]): Promise<void> => {
    const delayMs = stage.startMs - (monotonicNowMs() - startMono);
    if (delayMs > 0) await sleep(delayMs, stopSignal);
    while (!stopSignal.aborted && !poolExhausted) {
      const nowMs = monotonicNowMs() - startMono;
      if (nowMs >= stage.endMs) break;
      const index = nextIndex();
      if (index === undefined) break;
      active += 1;
      options.telemetry?.recordDispatch(0, active);
      let outcome: "completed" | "pool_exhausted";
      try {
        outcome = await executeOne(
          options,
          sequencer().flow,
          index,
          stopSignal,
          onFatal,
          () => { poolExhausted = true; },
          () => acceptingRecords,
          stage.loadStage,
        );
      } finally {
        active -= 1;
      }
      if (outcome === "pool_exhausted") {
        poolExhausted = true;
        break;
      }
      if (!acceptingRecords) break;
      completed += 1;
      options.onCompleted?.(completed, submitted);
      if (shape.thinkTimeMs > 0 && monotonicNowMs() - startMono < stage.endMs) {
        await sleep(shape.thinkTimeMs, stopSignal);
      }
    }
  };

  const loops: Promise<void>[] = [];
  for (const stage of stages) {
    for (let i = 0; i < stage.vus; i += 1) {
      loops.push(runVu(stage));
    }
  }
  const settledPromise = Promise.allSettled(loops);
  let settled: PromiseSettledResult<void>[];
  const initial = await settleOrAbort(settledPromise, stopSignal);
  if (!initial.settled) {
    const timeoutMs = options.cancellationTimeoutMs ?? 5_000;
    const outcome = await Promise.race([
      settledPromise.then((value) => ({ settled: true as const, value })),
      sleep(timeoutMs).then(() => ({ settled: false as const })),
    ]);
    if (!outcome.settled) {
      acceptingRecords = false;
      options.telemetry?.recordAbandoned(active);
      void settledPromise;
      const fatal = executionSignal.reason;
      if (fatal instanceof Error) throw fatal;
      return {
        submitted,
        completed,
        abandoned: active,
        poolExhausted,
        submissionDurationMs: monotonicNowMs() - startMono,
        interrupted: options.signal?.aborted ?? false,
      };
    }
    settled = outcome.value;
  } else {
    settled = initial.value;
  }
  const rejected = settled.find(
    (result): result is PromiseRejectedResult => result.status === "rejected",
  );
  const chronologicalFatal = executionSignal.reason;
  if (chronologicalFatal instanceof Error) throw chronologicalFatal;
  if (rejected) throw rejected.reason;
  options.telemetry?.recordAbandoned(0);

  return {
    submitted,
    completed,
    abandoned: 0,
    poolExhausted,
    submissionDurationMs: monotonicNowMs() - startMono,
    interrupted: options.signal?.aborted ?? false,
  };
};

/**
 * Open-model scheduler: walks the precomputed arrival schedule, fires each
 * request without awaiting it, and lets a per-request poller run to its own
 * terminal state. If the injector falls behind the schedule (event-loop
 * saturation), requests fire back-to-back without reducing the offered count.
 */
export const runScheduler = async (
  options: SchedulerOptions,
): Promise<SchedulerResult> => {
  const executionController = new AbortController();
  const onFatal = (error: Error): void => {
    if (!executionController.signal.aborted) executionController.abort(error);
  };
  if (options.scenario.shape.kind === "closed") {
    return runClosedScheduler(options, executionController.signal, onFatal);
  }

  const { scenario, executors, recorder } = options;
  const shape = scenario.shape as Exclude<RateShape, { kind: "closed" }>;
  const sequencer = createFlowSequencer(scenario.flows);
  const boundaries = segmentBoundariesMs(shape);
  const startMono = monotonicNowMs();
  const outstanding = new Set<Promise<void>>();

  let submitted = 0;
  let completed = 0;
  let nextBoundary = 0;
  let stoppedAtSegment: number | undefined;
  let poolExhausted = false;
  let fatalError: Error | undefined;
  let acceptingRecords = true;
  const submissionSignal = options.signal
    ? AbortSignal.any([options.signal, executionController.signal])
    : executionController.signal;

  const fire = (
    flow: FlowKind,
    index: number,
    loadStage: RequestLoadStage,
    scheduledOffsetMs: number,
  ): void => {
    // Capture scheduler lateness before invoking the async executor: its body
    // runs synchronously until the first await and may perform non-trivial
    // request setup that is workload cost, not dispatch lag.
    options.telemetry?.recordDispatch(
      Math.max(0, monotonicNowMs() - startMono - scheduledOffsetMs),
      outstanding.size + 1,
    );
    const task = executeOne(
      options,
      flow,
      index,
      submissionSignal,
      onFatal,
      () => { poolExhausted = true; },
      () => acceptingRecords,
      loadStage,
    )
      .then((outcome) => {
        if (outcome === "pool_exhausted") {
          poolExhausted = true;
          return;
        }
        if (!acceptingRecords) return;
        completed += 1;
        options.onCompleted?.(completed, submitted);
      })
      .catch((error: unknown) => {
        const abortReason = executionController.signal.reason;
        fatalError ??=
          abortReason instanceof Error
            ? abortReason
            : error instanceof Error
              ? error
              : new Error(String(error));
      })
      .finally(() => {
        outstanding.delete(task);
      });
    outstanding.add(task);
  };

  for (const offsetMs of arrivalOffsetsMs(shape)) {
    if (submissionSignal.aborted || poolExhausted) break;

    // Saturation feedback between segments.
    while (nextBoundary < boundaries.length && offsetMs >= (boundaries[nextBoundary] ?? Infinity)) {
      const verdict = (await options.onSegmentEnd?.(nextBoundary)) ?? "continue";
      if (verdict === "stop") {
        stoppedAtSegment = nextBoundary;
        break;
      }
      nextBoundary += 1;
    }
    if (stoppedAtSegment !== undefined) break;

    const waitMs = offsetMs - (monotonicNowMs() - startMono);
    if (waitMs > 0) await sleep(waitMs, submissionSignal);
    if (fatalError) break;
    if (submissionSignal.aborted || poolExhausted) break;

    fire(sequencer().flow, submitted, openLoadStageForOffset(shape, offsetMs), offsetMs);
    submitted += 1;
    options.onSubmitted?.(submitted);
    if (submitted % 64 === 0) await yieldEventLoop();
    else await Promise.resolve();
  }

  const submissionDurationMs = monotonicNowMs() - startMono;

  // Drain: outstanding pollers run to their own request deadline, bounded by
  // the scenario drain timeout.
  const cancellationTimeoutMs = options.cancellationTimeoutMs ?? 5_000;
  const drainDeadline = monotonicNowMs() +
    (options.signal?.aborted ? cancellationTimeoutMs : scenario.drainTimeoutSec * 1000);
  while (outstanding.size > 0 && monotonicNowMs() < drainDeadline) {
    const remainingMs = Math.max(0, drainDeadline - monotonicNowMs());
    await Promise.race([...outstanding, sleep(Math.min(1000, remainingMs))]);
    if (fatalError || options.signal?.aborted) break;
  }
  const callerInterrupted = options.signal?.aborted ?? false;
  let abandoned = fatalError || callerInterrupted ? 0 : outstanding.size;
  options.telemetry?.recordAbandoned(abandoned);
  if (abandoned > 0) {
    logger.warn(
      `${abandoned.toString()} request(s) still outstanding after the ${scenario.drainTimeoutSec.toString()}s drain timeout.`,
    );
    executionController.abort(
      new Error(
        `Run drain timeout expired with ${abandoned.toString()} request(s) outstanding`,
      ),
    );
  }
  if (fatalError && !executionController.signal.aborted) {
    executionController.abort(fatalError);
  }
  const settling = Promise.allSettled([...outstanding]);
  const settledInTime = await Promise.race([
    settling.then(() => true),
    sleep(cancellationTimeoutMs).then(() => false),
  ]);
  if (!settledInTime) {
    acceptingRecords = false;
    abandoned = Math.max(abandoned, outstanding.size);
    options.telemetry?.recordAbandoned(abandoned);
    void settling;
  }
  if (fatalError) throw fatalError;

  return {
    submitted,
    completed,
    abandoned,
    stoppedAtSegment,
    poolExhausted,
    submissionDurationMs,
    interrupted: options.signal?.aborted ?? false,
  };
};
