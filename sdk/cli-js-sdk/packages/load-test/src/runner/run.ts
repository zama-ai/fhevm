import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

import { InjectorRuntimeCollector } from "../collectors/injector-runtime";
import {
  createPrometheusHttpSource,
  PrometheusCollector,
} from "../collectors/prometheus";
import { snapshotRelayerConfig } from "../collectors/relayer-config";
import type { QueueDepthSource } from "../collectors/types";
import { runsDir, type LoadTestEnv } from "../env";
import { createFlowExecutor } from "../flows";
import type { FlowExecutor } from "../flows/types";
import { RelayerClient } from "../relayer/client";
import type { FlowKind } from "../relayer/types";
import { buildReport } from "../report/build";
import type { BuildReportTargetInput } from "../report/build";
import { renderMarkdownReport } from "../report/render-md";
import type { RelayerTarget, Report } from "../report/schema";
import { plannedRequestCount, shapeModel, type Scenario } from "../scenario/schema";
import { plannedFlowAllocations } from "../scenario/allocation";
import { logger } from "../shared/logger";
import { isoNow } from "../shared/time";
import { Recorder } from "./recorder";
import { runScheduler, type SegmentVerdict } from "./scheduler";

export type RunOptions = Readonly<{
  scenario: Scenario;
  env: LoadTestEnv;
  /** Output directory; defaults to `<dataDir>/runs/<timestamp>-<scenario>`. */
  outputDir?: string;
  /** undici connection cap toward the relayer. */
  connections?: number;
  /**
   * Skip the `GET /health/readiness` precheck. Needed for older relayer
   * builds that expose health on a different path (e.g. `/liveness`,
   * `/healthz`) or a separate port. The v2 job routes still work.
   */
  skipReadiness?: boolean;
  signal?: AbortSignal;
  /** Maximum wait per owned-resource close operation. */
  cleanupTimeoutMs?: number;
}>;

export type RunResult = Readonly<{
  report: Report;
  outputDir: string;
  status: "completed" | "interrupted";
}>;

export class RunInterruptedError extends Error {
  constructor() {
    super("Load-test run was interrupted before execution could start.");
    this.name = "RunInterruptedError";
  }
}

/**
 * Saturation feedback for ramp scenarios: queue depth growing monotonically
 * across consecutive steps at a fixed arrival rate IS the saturation signal
 * (design §1.3). Without a queue-depth source the ramp runs all steps.
 */
const createSaturationMonitor = (
  scenario: Scenario,
  source: () => QueueDepthSource | undefined,
): ((segmentIndex: number) => Promise<SegmentVerdict>) => {
  let strikes = 0;
  let lastPending: number | undefined;
  return (segmentIndex: number): Promise<SegmentVerdict> => {
    if (!scenario.saturationStop.enabled) return Promise.resolve("continue");
    const samples = source()?.samples;
    if (!samples || samples.length === 0) return Promise.resolve("continue");
    const pending = samples.at(-1)?.pendingTotal ?? 0;
    const growth = lastPending === undefined ? 0 : pending - lastPending;
    lastPending = pending;
    if (growth >= scenario.saturationStop.minQueueGrowth) {
      strikes += 1;
      logger.warn(
        `Queue depth grew by ${growth.toString()} over segment ${segmentIndex.toString()} (strike ${strikes.toString()}/${scenario.saturationStop.consecutiveSteps.toString()}).`,
      );
    } else {
      strikes = 0;
    }
    if (strikes >= scenario.saturationStop.consecutiveSteps) {
      logger.warn("Saturation detected; stopping submission.");
      return Promise.resolve("stop");
    }
    return Promise.resolve("continue");
  };
};

const createRunProgress = (
  scenario: Scenario,
  planned: number | undefined,
): {
  onSubmitted: (submitted: number) => void;
  onCompleted: (completed: number, submitted: number) => void;
  heartbeat: () => void;
  finish: () => void;
} => {
  let submitted = 0;
  let completed = 0;
  let lastLogAt = 0;

  const plannedLabel = planned === undefined ? "duration-bound" : `${planned.toString()} planned`;
  const progressLine = (): string =>
    planned === undefined
      ? `${submitted.toString()} submitted · ${completed.toString()} completed`
      : `${submitted.toString()}/${planned.toString()} submitted · ${completed.toString()}/${planned.toString()} completed`;

  const log = (force = false): void => {
    const now = Date.now();
    if (!force && now - lastLogAt < 10_000) return;
    lastLogAt = now;
    logger.info(`Progress ${scenario.name}: ${progressLine()} (${plannedLabel})`);
  };

  return {
    onSubmitted: (value) => {
      submitted = value;
      if (submitted === 1 || submitted === planned) log(true);
      else log();
    },
    onCompleted: (done, currentSubmitted) => {
      completed = done;
      submitted = currentSubmitted;
      if (done === 1 || done === planned || done === currentSubmitted) log(true);
      else log();
    },
    heartbeat: () => log(true),
    finish: () => {
      logger.success(`Execution finished: ${progressLine()}`);
    },
  };
};

type CollectorRuntime = {
  target: RelayerTarget;
  client: RelayerClient;
  relayerUrl: string;
  relayerApiPrefix?: string;
  relayerConfigPath?: string;
  relayerConfig?: BuildReportTargetInput["relayerConfig"];
  prometheus: PrometheusCollector;
};

const queueDepthSource = (
  runtime: CollectorRuntime,
): Readonly<{ source: "prometheus"; collector: QueueDepthSource }> | undefined => {
  if (runtime.prometheus.samples.length > 0) {
    return { source: "prometheus", collector: runtime.prometheus };
  }
  return undefined;
};

const aggregateQueueDepthSource = (
  runtimes: readonly CollectorRuntime[],
): QueueDepthSource | undefined => {
  const latestSamples = runtimes
    .map((runtime) => ({
      target: runtime.target,
      sample: queueDepthSource(runtime)?.collector.samples.at(-1),
    }))
    .filter(
      (entry): entry is { target: RelayerTarget; sample: NonNullable<typeof entry.sample> } =>
        entry.sample !== undefined,
    );
  if (latestSamples.length === 0) return undefined;
  const first = latestSamples[0];
  if (!first) return undefined;
  if (latestSamples.length === 1) return { samples: [first.sample] };
  return {
    samples: [
      {
        tMs: Math.max(...latestSamples.map((entry) => entry.sample.tMs)),
        byFlowStatus: Object.assign(
          {},
          ...latestSamples.map((entry) =>
            Object.fromEntries(
              Object.entries(entry.sample.byFlowStatus).map(([key, value]) => [
                `${entry.target}/${key}`,
                value,
              ]),
            ),
          ),
        ) as Record<string, number>,
        pendingTotal: latestSamples.reduce(
          (total, entry) => total + entry.sample.pendingTotal,
          0,
        ),
      },
    ],
  };
};

export const executeRun = async (options: RunOptions): Promise<RunResult> => {
  const { scenario, env } = options;
  const startedAt = isoNow();
  const outputDir =
    options.outputDir ??
    join(
      runsDir(env),
      `${startedAt.replace(/[:.]/g, "-")}-${scenario.name}`,
    );
  let client: RelayerClient | undefined;
  let clientB: RelayerClient | undefined;
  const executors = new Map<FlowKind, FlowExecutor>();
  const collectorRuntimes: CollectorRuntime[] = [];
  let injectorRuntime: InjectorRuntimeCollector | undefined;
  let recorder: Recorder | undefined;
  let heartbeat: ReturnType<typeof setInterval> | undefined;
  let cleaned = false;
  let cleanupErrors: unknown[] = [];
  let interrupted = options.signal?.aborted ?? false;
  const latchInterruption = (): void => { interrupted = true; };
  options.signal?.addEventListener("abort", latchInterruption, { once: true });

  const cleanup = async (): Promise<unknown[]> => {
    if (cleaned) return [];
    cleaned = true;
    if (heartbeat) clearInterval(heartbeat);
    const attempt = async (
      operation: () => Promise<unknown>,
      label: string,
    ): Promise<unknown> => {
      let timeout: ReturnType<typeof setTimeout> | undefined;
      const timeoutMs = options.cleanupTimeoutMs ?? 10_000;
      try {
        return await Promise.race([
          Promise.resolve().then(operation),
          new Promise<never>((_, reject) => {
            // Keep this timer referenced while cleanup is outstanding. An
            // unresolved close promise alone does not keep Node alive.
            timeout = setTimeout(
              () => reject(new Error(`Timed out cleaning up ${label}`)),
              timeoutMs,
            );
          }),
        ]);
      } finally {
        if (timeout) clearTimeout(timeout);
      }
    };
    const currentInjectorRuntime = injectorRuntime;
    const currentRecorder = recorder;
    const currentClient = client;
    const currentClientB = clientB;
    const errors: unknown[] = [];
    const phase = async (operations: Promise<unknown>[]): Promise<void> => {
      const settled = await Promise.allSettled(operations);
      errors.push(...settled.flatMap((result) =>
        result.status === "rejected" ? [result.reason] : [],
      ));
    };
    await phase([
      ...collectorRuntimes.map((runtime) =>
        attempt(() => runtime.prometheus.stop(), `prometheus ${runtime.target}`),
      ),
      ...(currentInjectorRuntime
        ? [attempt(() => currentInjectorRuntime.stop(), "injector collector")]
        : []),
    ]);
    if (currentRecorder) await phase([attempt(() => currentRecorder.close(), "recorder")]);
    await phase([...executors.values()].map((executor) =>
      attempt(() => executor.close(), `executor ${executor.flow}`),
    ));
    await phase([
      ...(currentClient ? [attempt(() => currentClient.close(), "relayer client A")] : []),
      ...(currentClientB ? [attempt(() => currentClientB.close(), "relayer client B")] : []),
    ]);
    return errors;
  };

  const assertNotInterrupted = (): void => {
    if (interrupted) throw new RunInterruptedError();
  };

  let result: RunResult | undefined;
  let primaryError: unknown;
  try {
    assertNotInterrupted();
    await mkdir(outputDir, { recursive: true });
    logger.info(
      `Run "${scenario.name}" against ${env.relayerUrl}` +
        `${env.relayerBUrl ? ` and ${env.relayerBUrl}` : ""}; artifacts in ${outputDir}`,
    );

    client = new RelayerClient({
      baseUrl: env.relayerUrl,
      apiPrefix: env.relayerApiPrefix,
      connections: options.connections,
      apiKey: process.env.ZAMA_FHEVM_API_KEY,
    });
    clientB = env.relayerBUrl
      ? new RelayerClient({
          baseUrl: env.relayerBUrl,
          apiPrefix: env.relayerBApiPrefix ?? env.relayerApiPrefix,
          connections: options.connections,
          apiKey: process.env.ZAMA_FHEVM_API_KEY,
        })
      : undefined;

    // ---- Prepare ---------------------------------------------------------
    if (options.skipReadiness) {
      logger.warn(
        "Skipping readiness check (--skip-readiness); assuming the relayer's v2 routes are live.",
      );
    } else if (!(await client.isReady())) {
      throw new Error(
        `Relayer at ${env.relayerUrl} failed the readiness check (GET /health/readiness). ` +
          `Older relayers expose health elsewhere (e.g. /liveness, /healthz); pass --skip-readiness to proceed.`,
      );
    } else if (clientB && !(await clientB.isReady())) {
      throw new Error(
        `Candidate relayer at ${env.relayerBUrl ?? "<unset>"} failed the readiness check (GET /health/readiness). ` +
          `Older relayers expose health elsewhere (e.g. /liveness, /healthz); pass --skip-readiness to proceed.`,
      );
    } else {
      logger.success("Relayer readiness check passed.");
    }
    assertNotInterrupted();

    const planned = plannedRequestCount(scenario.shape);
    const allocations = new Map(
      plannedFlowAllocations(scenario).map((allocation) => [allocation.flow, allocation]),
    );
    const requestTimeoutMs = scenario.requestTimeoutSec * 1000;
    for (const mix of scenario.flows) {
      assertNotInterrupted();
      if (
        planned === undefined &&
        (mix.flow === "input-proof" || mix.flow === "public-decrypt")
      ) {
        throw new Error(
          `Closed ${mix.flow} scenarios require shape.maxIterations so single-use pools can be planned safely.`,
        );
      }
      const executor = await createFlowExecutor({
        flow: mix.flow,
        env,
        client,
        clientB,
        requestTimeoutMs,
        handlesPerRequest: mix.handlesPerRequest,
      });
      executors.set(mix.flow, executor);
      const flowPlanned = allocations.get(mix.flow)?.requests ?? 1;
      await executor.prepare(flowPlanned, options.signal);
    }
    assertNotInterrupted();
    logger.success(
      planned === undefined
        ? `Prepared ${executors.size.toString()} flow executor(s) for duration-bound ${shapeModel(scenario.shape)} run.`
        : `Prepared ${executors.size.toString()} flow executor(s) for ${planned.toString()} planned request(s).`,
    );

    // ---- Collect (start) --------------------------------------------------
    logger.start("Starting collectors.");
    collectorRuntimes.push({
      target: "A",
      client,
      relayerUrl: env.relayerUrl,
      relayerApiPrefix: env.relayerApiPrefix,
      relayerConfigPath: env.relayerConfigPath,
      prometheus: new PrometheusCollector(
        createPrometheusHttpSource(new URL("/metrics", env.relayerUrl).toString()),
        join(outputDir, "metrics-a.jsonl"),
      ),
    });
    if (clientB && env.relayerBUrl) {
      collectorRuntimes.push({
        target: "B",
        client: clientB,
        relayerUrl: env.relayerBUrl,
        relayerApiPrefix: env.relayerBApiPrefix ?? env.relayerApiPrefix,
        relayerConfigPath: env.relayerBConfigPath,
        prometheus: new PrometheusCollector(
          createPrometheusHttpSource(
            new URL("/metrics", env.relayerBUrl).toString(),
          ),
          join(outputDir, "metrics-b.jsonl"),
        ),
      });
    }
    for (const runtime of collectorRuntimes) {
      runtime.relayerConfig = await snapshotRelayerConfig(runtime.relayerConfigPath);
      try {
        await runtime.prometheus.start();
      } catch (error) {
        logger.warn(
          `Prometheus collector ${runtime.target} unavailable: ${(error as Error).message}`,
        );
      }
    }
    injectorRuntime = new InjectorRuntimeCollector(
      join(outputDir, "injector-runtime.jsonl"),
    );
    await injectorRuntime.start().catch((error) => {
      logger.warn(`Injector runtime collector unavailable: ${(error as Error).message}`);
    });
    assertNotInterrupted();
    const depthSource = (): QueueDepthSource | undefined =>
      aggregateQueueDepthSource(collectorRuntimes);

    // ---- Execute ----------------------------------------------------------
    logger.start(`Executing "${scenario.name}" (${shapeModel(scenario.shape)} model).`);
    const progress = createRunProgress(scenario, planned);
    heartbeat = setInterval(progress.heartbeat, 30_000);
    heartbeat.unref();
    recorder = await Recorder.open(join(outputDir, "requests.jsonl"), {
      relayerAPath: join(outputDir, "requests-a.jsonl"),
      relayerBPath: env.relayerBUrl
        ? join(outputDir, "requests-b.jsonl")
        : undefined,
    });
    const schedulerResult = await runScheduler({
      scenario,
      executors,
      recorder,
      signal: options.signal,
      onSegmentEnd: createSaturationMonitor(scenario, depthSource),
      onSubmitted: progress.onSubmitted,
      onCompleted: progress.onCompleted,
      telemetry: injectorRuntime,
    });
    clearInterval(heartbeat);
    heartbeat = undefined;
    progress.finish();

    // Freeze all owned resources before report construction.
    logger.start("Stopping collectors.");
    const stopErrors = await cleanup();
    if (stopErrors.length > 0) {
      throw new AggregateError(stopErrors, "Run resource cleanup failed");
    }

    // Cleanup is the terminal boundary. Freeze interruption state once all
    // owned work is stopped so report.json and the returned status cannot
    // diverge if a signal arrives while artifacts are being written.
    const runInterrupted = schedulerResult.interrupted || interrupted;
    options.signal?.removeEventListener("abort", latchInterruption);

    // ---- Report -----------------------------------------------------------
    const report = buildReport({
      scenario,
      network: env.network,
      relayerUrl: env.relayerUrl,
      relayerApiPrefix: env.relayerApiPrefix,
      relayerBUrl: env.relayerBUrl,
      relayerBApiPrefix: env.relayerBApiPrefix ?? env.relayerApiPrefix,
      startedAt,
      endedAt: isoNow(),
      records: recorder.records,
      submitted: schedulerResult.submitted,
      completed: schedulerResult.completed,
      abandoned: schedulerResult.abandoned,
      poolExhausted: schedulerResult.poolExhausted,
      submissionDurationMs: schedulerResult.submissionDurationMs,
      stoppedAtSegment: schedulerResult.stoppedAtSegment,
      interrupted: runInterrupted,
      injector: injectorRuntime.summary(),
      targets: collectorRuntimes.map((runtime) => {
        const queueSource = queueDepthSource(runtime);
        return {
          target: runtime.target,
          relayerUrl: runtime.relayerUrl,
          relayerApiPrefix: runtime.relayerApiPrefix,
          relayerConfig: runtime.relayerConfig,
          metricsSnapshots: runtime.prometheus.snapshots,
          metricsCapabilities: runtime.prometheus.capabilities,
          metricsCollection: runtime.prometheus.collectionStatus,
          queueDepth: queueSource
            ? {
                source: queueSource.source,
                samples: queueSource.collector.samples,
              }
            : undefined,
        };
      }),
    });

    await writeFile(
      join(outputDir, "report.json"),
      `${JSON.stringify(report, null, 2)}\n`,
    );
    await writeFile(join(outputDir, "report.md"), renderMarkdownReport(report));
    logger.success(`Report written to ${join(outputDir, "report.json")}`);
    if (!report.thresholds.passed) {
      for (const breach of report.thresholds.breaches) {
        logger.error(
          `Threshold breach: ${breach.threshold}${breach.flow ? ` (${breach.flow})` : ""} — limit ${breach.limit.toString()}, actual ${breach.actual.toString()}`,
        );
      }
    }

    result = {
      report,
      outputDir,
      status: runInterrupted ? "interrupted" : "completed",
    };
  } catch (error) {
    primaryError =
      interrupted && error instanceof Error && error.name === "AbortError"
        ? new RunInterruptedError()
        : error;
  } finally {
    cleanupErrors = await cleanup();
    options.signal?.removeEventListener("abort", latchInterruption);
  }

  if (primaryError !== undefined) {
    if (cleanupErrors.length > 0) {
      throw new AggregateError(
        [primaryError, ...cleanupErrors],
        "Load-test run failed and resource cleanup also failed",
        { cause: primaryError },
      );
    }
    throw primaryError;
  }
  if (cleanupErrors.length > 0) {
    throw new AggregateError(cleanupErrors, "Run resource cleanup failed");
  }
  if (!result) throw new Error("Load-test run completed without a result");
  return result;
};
