import { existsSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

import { runsDir, type LoadTestEnv } from "../env";
import {
  formatPoolPlan,
  inspectPoolRequirements,
  preparePoolRequirements,
  type PoolPlanArtifact,
} from "../pool/planning";
import { diffReports, type DiffResult } from "../report/diff";
import { readReportFile } from "../report/runtime";
import type { Report } from "../report/schema";
import { executeRun, RunInterruptedError } from "../runner/run";
import { assertRelayerReadiness } from "../runner/readiness";
import { ceilingWarnings } from "../scenario/limits";
import { loadScenario } from "../scenario/load";
import type { Scenario } from "../scenario/schema";
import { logger } from "../shared/logger";
import { safeJoin } from "../shared/paths";
import { safeArtifactText } from "../shared/safe-artifact";
import { isoNow, sleep } from "../shared/time";
import type { Suite } from "./schema";

export type SuiteRunOptions = Readonly<{
  env: LoadTestEnv;
  suite: Suite;
  /** Root for run outputs; defaults to `<dataDir>/runs/<timestamp>-suite-<name>`. */
  outputRoot?: string;
  /** Directory of committed baseline reports (`<network>/<label>.json`). */
  baselinesDir?: string;
  /** Explicitly authorize pool generation/on-chain writes when work is needed. */
  prepare?: boolean;
  /** Wallet lanes for on-chain handle creation. */
  lanes?: number;
  connections?: number;
  /** Skip the readiness precheck on every run (older relayer health paths). */
  skipReadiness?: boolean;
  signal?: AbortSignal;
}>;

export type SuiteEntryResult = Readonly<{
  label: string;
  scenarioName: string;
  outputDir: string;
  submitted: number;
  errorRate: number;
  verifyFailed: number;
  targets: readonly Readonly<{
    target: "A" | "B";
    errorRate: number;
    verifyFailed: number;
  }>[];
  differentTerminalOutcomes: number;
  thresholdsPassed: boolean;
  diff?: Readonly<{ baseline: string; passed: boolean; regressions: number }>;
}>;

export type SuiteResult = Readonly<{
  suite: string;
  startedAt: string;
  endedAt: string;
  outputRoot: string;
  status: "completed" | "blocked" | "interrupted" | "failed";
  passed: boolean;
  entries: readonly SuiteEntryResult[];
  blockedReason?: string;
}>;

const isAbortError = (error: unknown): boolean =>
  error instanceof Error && error.name === "AbortError";

const describePlan = (plan: PoolPlanArtifact): void => {
  for (const line of formatPoolPlan(plan)) logger.info(line);
};

const summarize = (label: string, report: Report, outputDir: string): SuiteEntryResult => {
  const targets = report.targets.map((target) => {
    const measured = target.flows.reduce(
      (total, flow) => total + flow.submitted - flow.aborted,
      0,
    );
    const errors = target.flows.reduce(
      (total, flow) => total + flow.submitted - flow.aborted - flow.succeeded,
      0,
    );
    return {
      target: target.target,
      errorRate: measured === 0 ? 0 : errors / measured,
      verifyFailed: target.flows.reduce((total, flow) => total + flow.verifyFailed, 0),
    };
  });
  return {
    label,
    scenarioName: report.run.scenario.name,
    outputDir,
    submitted: report.run.submitted,
    errorRate: Math.max(0, ...targets.map((target) => target.errorRate)),
    verifyFailed: Math.max(0, ...targets.map((target) => target.verifyFailed)),
    targets,
    differentTerminalOutcomes: report.comparison?.flows.reduce(
      (total, flow) => total + flow.differentTerminalOutcome,
      0,
    ) ?? 0,
    thresholdsPassed: report.thresholds.passed,
  };
};

const renderSuiteMarkdown = (result: SuiteResult): string => {
  const status = result.status === "interrupted"
    ? "⏹ interrupted"
    : result.status === "failed"
      ? "❌ failed"
      : result.status === "blocked"
        ? "⛔ blocked"
      : result.passed
        ? "✅ completed"
        : "❌ completed with breaches";
  const lines = [
    `# Load Test Suite — ${result.suite}`,
    "",
    `- **Window:** ${result.startedAt} → ${result.endedAt}`,
    `- **Status:** ${status}`,
    ...(result.blockedReason ? [`- **Blocked:** ${result.blockedReason}`] : []),
    "",
    "| Scenario | Workflows | A error | B error | Worst verify failed | Pair divergence | Thresholds | Baseline diff | Report |",
    "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
  ];
  for (const entry of result.entries) {
    const diff = entry.diff
      ? entry.diff.passed
        ? "✅"
        : `❌ ${entry.diff.regressions.toString()} regression(s)`
      : "—";
    lines.push(
      `| ${entry.label} | ${entry.submitted.toString()} | ` +
        `${((entry.targets.find((target) => target.target === "A")?.errorRate ?? 0) * 100).toFixed(2)}% | ` +
        `${entry.targets.some((target) => target.target === "B") ? `${((entry.targets.find((target) => target.target === "B")?.errorRate ?? 0) * 100).toFixed(2)}%` : "—"} | ` +
        `${entry.verifyFailed.toString()} | ${entry.differentTerminalOutcomes.toString()} | ` +
        `${entry.thresholdsPassed ? "✅" : "❌"} | ${diff} | ${entry.outputDir} |`,
    );
  }
  return `${lines.join("\n")}\n`;
};

const writeSuiteSummary = async (result: SuiteResult): Promise<void> => {
  await writeFile(
    join(result.outputRoot, "suite-summary.json"),
    `${JSON.stringify(result, null, 2)}\n`,
  );
  await writeFile(
    join(result.outputRoot, "suite-summary.md"),
    renderSuiteMarkdown(result),
  );
};

export type ResolvedSuiteEntry = Readonly<{ label: string; scenario: Scenario }>;

/** Resolves and validates every suite entry before pool inspection. */
export const resolveSuiteScenarios = async (
  suite: Suite,
): Promise<ResolvedSuiteEntry[]> => {
  const resolved: ResolvedSuiteEntry[] = [];
  for (const entry of suite.entries) {
    const scenario = await loadScenario(entry.scenario, entry.params);
    for (const warning of ceilingWarnings(scenario)) logger.warn(warning);
    resolved.push({ label: entry.label ?? scenario.name, scenario });
  }
  const labels = resolved.map((entry) => entry.label);
  if (new Set(labels).size !== labels.length) {
    throw new Error(`Duplicate suite labels: ${labels.join(", ")}. Set distinct "label"s.`);
  }
  return resolved;
};

/**
 * Resolves every scenario, records a live plan, optionally prepares deficits
 * when explicitly authorized, runs ready scenarios sequentially, compares
 * baselines when present, and writes a terminal suite summary.
 */
const executeSuiteLifecycle = async (
  options: SuiteRunOptions,
): Promise<SuiteResult> => {
  const { env, suite } = options;
  const startedAt = isoNow();
  const outputRoot =
    options.outputRoot ??
    join(runsDir(env), `${startedAt.replace(/[:.]/g, "-")}-suite-${suite.name}`);
  await mkdir(outputRoot, { recursive: true });

  let resolved: ResolvedSuiteEntry[] = [];
  const entries: SuiteEntryResult[] = [];
  let interrupted = options.signal?.aborted ?? false;
  let passed = true;
  let suiteError: unknown;
  let blockedReason: string | undefined;

  try {
    // Resolve every entry before pool inspection or mutation.
    resolved = await resolveSuiteScenarios(suite);
    logger.info(
      `Suite "${suite.name}": ${resolved.map((entry) => entry.label).join(" → ")}`,
    );

    // Always persist the live plan under the stable suite root. Mutation is
    // separately and explicitly authorized by `suite prepare` or --prepare.
    if (!interrupted) {
      let plan = await inspectPoolRequirements({
        env,
        scenarios: resolved.map((entry) => entry.scenario),
        pauseSec: suite.pauseSec,
        artifactDir: outputRoot,
      });
      describePlan(plan);
      if (!plan.ready) {
        const preparationAuthorized = options.prepare === true;
        if (!preparationAuthorized) {
          blockedReason =
            "Pool preparation is required. Inspect pool-plan.md, then rerun with --prepare or use `suite prepare`.";
          passed = false;
          logger.warn(blockedReason);
        } else {
          try {
            const prepared = await preparePoolRequirements({
              env,
              scenarios: resolved.map((entry) => entry.scenario),
              pauseSec: suite.pauseSec,
              artifactDir: outputRoot,
              lanes: options.lanes,
              signal: options.signal,
              onProgress: (message) => logger.info(message),
              beforeActions: () => assertRelayerReadiness({
                env,
                connections: options.connections,
                skipReadiness: options.skipReadiness,
                signal: options.signal,
              }),
            });
            plan = prepared.plan;
          } catch (error) {
            if (isAbortError(error)) {
              interrupted = true;
            } else {
              throw error;
            }
          }
          if (!interrupted && !plan.ready) {
            throw new Error(
              "Pool preparation completed with residual work; refusing suite execution.",
            );
          }
        }
      } else {
        logger.success("All pools ready; nothing to prepare.");
        if (options.lanes !== undefined) {
          logger.warn(
            "--lanes has no effect: pools are already ready, so no preparation is needed.",
          );
        }
      }
      interrupted = options.signal?.aborted ?? false;
    }
  } catch (error) {
    passed = false;
    if (isAbortError(error)) interrupted = true;
    else suiteError = error;
  }

  // ---- Execute sequentially.
  if (!suiteError && !blockedReason) {
    try {
      for (const [index, { label, scenario }] of resolved.entries()) {
        if (options.signal?.aborted) {
          interrupted = true;
          break;
        }
        if (index > 0 && suite.pauseSec > 0) {
          logger.info(`Pausing ${suite.pauseSec.toString()}s before "${label}"…`);
          await sleep(suite.pauseSec * 1000, options.signal);
          if (options.signal?.aborted) {
            interrupted = true;
            break;
          }
        }
        logger.start(
          `Running "${label}" (${(index + 1).toString()}/${resolved.length.toString()})…`,
        );
        let runResult;
        try {
          runResult = await executeRun({
            scenario,
            env,
            outputDir: safeJoin(outputRoot, label),
            connections: options.connections,
            skipReadiness: options.skipReadiness,
            signal: options.signal,
          });
        } catch (error) {
          if (error instanceof RunInterruptedError) {
            interrupted = true;
            break;
          }
          throw error;
        }
        const { report, outputDir } = runResult;
        const summary = summarize(label, report, outputDir);
        if (runResult.status === "interrupted" || options.signal?.aborted) {
          entries.push(summary);
          interrupted = true;
          passed = false;
          break;
        }

        let diffSummary: SuiteEntryResult["diff"];
        const baselinePath = options.baselinesDir
          ? safeJoin(options.baselinesDir, env.network, `${label}.json`)
          : undefined;
        if (baselinePath) {
          let diff: DiffResult | undefined;
          if (existsSync(baselinePath)) {
            const baseline = await readReportFile(baselinePath);
            diff = diffReports(baseline, report);
            for (const note of diff.notes) logger.warn(note);
          } else {
            logger.info(`No baseline at ${baselinePath}; skipping diff.`);
          }
          if (diff) {
            diffSummary = {
              baseline: baselinePath,
              passed: diff.passed,
              regressions: diff.regressions.length,
            };
            for (const regression of diff.regressions) {
              logger.error(
                `Regression vs baseline: ${regression.flow} ${regression.metric} ` +
                  `${regression.baseline.toString()} -> ${regression.current.toString()}`,
              );
            }
          }
        }

        if (options.signal?.aborted) {
          entries.push({ ...summary, diff: diffSummary });
          interrupted = true;
          passed = false;
          break;
        }

        const entry = { ...summary, diff: diffSummary };
        entries.push(entry);
        if (!entry.thresholdsPassed || entry.diff?.passed === false) passed = false;
      }
    } catch (error) {
      passed = false;
      if (isAbortError(error)) interrupted = true;
      else suiteError = error;
    }
  }

  const result: SuiteResult = {
    suite: suite.name,
    startedAt,
    endedAt: isoNow(),
    outputRoot,
    status: interrupted
      ? "interrupted"
      : suiteError
        ? "failed"
        : blockedReason
          ? "blocked"
          : "completed",
    passed: passed && !interrupted && !blockedReason,
    entries,
    ...(blockedReason ? { blockedReason } : {}),
  };
  try {
    await writeSuiteSummary(result);
  } catch (summaryError) {
    if (suiteError) {
      throw new AggregateError(
        [suiteError, summaryError],
        "Suite failed and terminal summary persistence also failed",
        { cause: suiteError },
      );
    }
    throw summaryError;
  }
  logger.success(`Suite summary written to ${join(outputRoot, "suite-summary.md")}`);
  if (suiteError) throw suiteError;
  return result;
};

export const runSuite = (options: SuiteRunOptions): Promise<SuiteResult> =>
  executeSuiteLifecycle(options);

export type SuitePrepareOptions = Omit<SuiteRunOptions, "baselinesDir" | "prepare">;

export type SuitePreparationResult = Readonly<{
  suite: string;
  startedAt: string;
  endedAt: string;
  outputRoot: string;
  status: "completed" | "interrupted" | "failed";
  ready: boolean;
  error?: string;
}>;

const renderSuitePreparationMarkdown = (result: SuitePreparationResult): string => `${[
  `# Suite Pool Preparation — ${result.suite}`,
  "",
  `- **Window:** ${result.startedAt} → ${result.endedAt}`,
  `- **Status:** ${result.status}`,
  `- **Final readiness:** ${result.ready ? "ready" : "not ready"}`,
  ...(result.error ? [`- **Error:** ${result.error}`] : []),
].join("\n")}\n`;

const writeSuitePreparationResult = async (
  result: SuitePreparationResult,
): Promise<void> => {
  await writeFile(
    join(result.outputRoot, "suite-preparation.json"),
    `${JSON.stringify(result, null, 2)}\n`,
  );
  await writeFile(
    join(result.outputRoot, "suite-preparation.md"),
    renderSuitePreparationMarkdown(result),
  );
};

/** Explicit preparation lifecycle; never writes a workload suite summary. */
export const prepareSuite = async (
  options: SuitePrepareOptions,
): Promise<SuitePreparationResult> => {
  const startedAt = isoNow();
  const outputRoot = options.outputRoot ?? join(
    options.env.dataDir,
    "preparations",
    `${startedAt.replace(/[:.]/g, "-")}-suite-${options.suite.name}`,
  );
  await mkdir(outputRoot, { recursive: true });
  let operationError: unknown;
  let interrupted = options.signal?.aborted ?? false;
  let ready = false;

  try {
    options.signal?.throwIfAborted();
    const resolved = await resolveSuiteScenarios(options.suite);
    const scenarios = resolved.map((entry) => entry.scenario);
    const initial = await inspectPoolRequirements({
      env: options.env,
      scenarios,
      pauseSec: options.suite.pauseSec,
      artifactDir: outputRoot,
    });
    describePlan(initial);
    options.signal?.throwIfAborted();
    // Always invoke the preparer, including a ready/no-op plan, so every
    // explicit prepare command has durable preparation.json/.md evidence.
    const prepared = await preparePoolRequirements({
      env: options.env,
      scenarios,
      pauseSec: options.suite.pauseSec,
      artifactDir: outputRoot,
      lanes: options.lanes,
      signal: options.signal,
      onProgress: (message) => logger.info(message),
      beforeActions: () => assertRelayerReadiness({
        env: options.env,
        connections: options.connections,
        skipReadiness: options.skipReadiness,
        signal: options.signal,
      }),
    });
    ready = prepared.plan.ready;
    if (!ready) {
      throw new Error("Pool preparation completed with residual work.");
    }
  } catch (error) {
    operationError = error;
    interrupted = isAbortError(error);
  }

  const result: SuitePreparationResult = {
    suite: options.suite.name,
    startedAt,
    endedAt: isoNow(),
    outputRoot,
    status: interrupted ? "interrupted" : operationError ? "failed" : "completed",
    ready: ready && !operationError,
    ...(operationError
      ? { error: safeArtifactText(operationError instanceof Error ? operationError.message : operationError) }
      : {}),
  };
  let persistenceError: unknown;
  try {
    await writeSuitePreparationResult(result);
  } catch (error) {
    persistenceError = error;
  }
  if (operationError && persistenceError) {
    throw new AggregateError(
      [operationError, persistenceError],
      "Suite preparation failed and its terminal evidence could not be persisted",
      { cause: operationError },
    );
  }
  if (operationError && !interrupted) throw operationError;
  if (persistenceError) throw persistenceError;
  logger.success(`Suite preparation evidence written to ${outputRoot}`);
  return result;
};
