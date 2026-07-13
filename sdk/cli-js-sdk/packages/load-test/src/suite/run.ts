import { existsSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

import { runsDir, type LoadTestEnv } from "../env";
import {
  createHandlePool,
  refreshDelegatedHandlePool,
  type HandlePoolFlow,
} from "../pool/handles";
import { generateInputProofPool } from "../pool/input-proof";
import { diffReports, type DiffResult } from "../report/diff";
import { readReportFile } from "../report/runtime";
import type { Report } from "../report/schema";
import { executeRun, RunInterruptedError } from "../runner/run";
import { loadScenario } from "../scenario/load";
import type { Scenario } from "../scenario/schema";
import { logger } from "../shared/logger";
import { safeJoin } from "../shared/paths";
import { isoNow, sleep } from "../shared/time";
import {
  planPools,
  requiredDelegationValidUntil,
  type PoolDeficit,
} from "./requirements";
import type { Suite } from "./schema";

export type SuiteRunOptions = Readonly<{
  env: LoadTestEnv;
  suite: Suite;
  /** Root for run outputs; defaults to `<dataDir>/runs/<timestamp>-suite-<name>`. */
  outputRoot?: string;
  /** Directory of committed baseline reports (`<network>/<label>.json`). */
  baselinesDir?: string;
  /** Prepare pools and exit without running. */
  prepareOnly?: boolean;
  /** Fail instead of generating/creating missing pool items. */
  skipPrepare?: boolean;
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
  status: "completed" | "interrupted" | "failed";
  passed: boolean;
  entries: readonly SuiteEntryResult[];
}>;

const describePlan = (plan: readonly PoolDeficit[]): void => {
  for (const item of plan) {
    const status = item.deficit > 0
      ? `needs ${item.deficit.toString()} more`
      : item.refreshRequired
        ? "needs ACL refresh"
        : "ready";
    logger.info(
      `pool ${item.pool}: ${item.current.toString()} item(s), suite needs ${item.needed.toString()} request(s) — ${status} (${item.detail})`,
    );
  }
};

const preparePools = async (
  env: LoadTestEnv,
  plan: readonly PoolDeficit[],
  scenarios: readonly Scenario[],
  pauseSec: number,
  lanes: number | undefined,
  signal?: AbortSignal,
): Promise<void> => {
  for (const item of plan) {
    if (signal?.aborted) return;
    if (item.deficit === 0 && !item.refreshRequired) continue;
    if (item.flow === "input-proof") {
      logger.start(`Generating ${item.deficit.toString()} input-proof payload(s)…`);
      await generateInputProofPool(env, {
        count: item.deficit,
        signal,
        onProgress: (done, total) => {
          if (done % 50 === 0 || done === total) {
            logger.info(`payloads ${done.toString()}/${total.toString()}`);
          }
        },
      });
    } else if (item.deficit > 0) {
      logger.start(
        `Creating ${item.deficit.toString()} ${item.flow} handle(s) on-chain (funded wallet required)…`,
      );
      await createHandlePool(env, {
        flow: item.flow as HandlePoolFlow,
        count: item.deficit,
        lanes,
        signal,
        onProgress: (done, total) =>
          logger.info(`handles ${done.toString()}/${total.toString()}`),
      });
    }
    if (item.flow === "delegated-user-decrypt" && item.requiredValidUntil) {
      logger.start("Refreshing every delegated pool owner ACL grant…");
      await refreshDelegatedHandlePool(env, {
        // Preparation may itself take a long time. Re-anchor the complete
        // remaining suite horizon immediately before the ACL refresh.
        requiredValidUntil: requiredDelegationValidUntil(scenarios, { pauseSec }),
        signal,
        onProgress: (message) => logger.info(message),
      });
    }
    if (signal?.aborted) return;
  }
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
      : result.passed
        ? "✅ completed"
        : "❌ completed with breaches";
  const lines = [
    `# Load Test Suite — ${result.suite}`,
    "",
    `- **Window:** ${result.startedAt} → ${result.endedAt}`,
    `- **Status:** ${status}`,
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

/**
 * One-command operation: resolve every scenario in the suite, plan and
 * prepare pool deficits, run the scenarios sequentially (pausing between
 * runs so the relayer queue drains), diff against baselines when present,
 * and write a suite-level summary next to the per-run artifacts.
 */
export const runSuite = async (options: SuiteRunOptions): Promise<SuiteResult> => {
  const { env, suite } = options;
  const startedAt = isoNow();
  const outputRoot =
    options.outputRoot ??
    join(runsDir(env), `${startedAt.replace(/[:.]/g, "-")}-suite-${suite.name}`);
  await mkdir(outputRoot, { recursive: true });

  const resolved: { label: string; scenario: Scenario }[] = [];
  const entries: SuiteEntryResult[] = [];
  let interrupted = options.signal?.aborted ?? false;
  let passed = true;
  let suiteError: unknown;
  let prepareOnlyComplete = false;

  try {
    // ---- Resolve scenarios up front so a typo fails before any preparation.
    for (const entry of suite.entries) {
      const scenario = await loadScenario(entry.scenario, entry.params);
      resolved.push({ label: entry.label ?? scenario.name, scenario });
    }
    const labels = resolved.map((entry) => entry.label);
    if (new Set(labels).size !== labels.length) {
      throw new Error(`Duplicate suite labels: ${labels.join(", ")}. Set distinct "label"s.`);
    }
    logger.info(
      `Suite "${suite.name}": ${resolved.map((entry) => entry.label).join(" → ")}`,
    );

    // ---- Plan and prepare pools.
    if (!interrupted) {
      const plan = await planPools(
        env,
        resolved.map((entry) => entry.scenario),
        { pauseSec: suite.pauseSec },
      );
      describePlan(plan);
      const preparationRequired = plan.some(
        (item) => item.deficit > 0 || item.refreshRequired,
      );
      if (preparationRequired) {
        if (options.skipPrepare) {
          throw new Error(
            "Pools cannot serve this suite and --skip-prepare is set; run `suite plan` for details.",
          );
        }
        try {
          await preparePools(
            env,
            plan,
            resolved.map((entry) => entry.scenario),
            suite.pauseSec,
            options.lanes,
            options.signal,
          );
        } catch (error) {
          if (
            options.signal?.aborted &&
            error instanceof Error &&
            error.name === "AbortError"
          ) {
            interrupted = true;
          } else {
            throw error;
          }
        }
      } else {
        logger.success("All pools ready; nothing to prepare.");
      }
      interrupted = options.signal?.aborted ?? false;
    }
    if (options.prepareOnly) {
      if (!interrupted) logger.success("Prepare-only requested; pools are ready.");
      prepareOnlyComplete = true;
      passed = !interrupted;
    }
  } catch (error) {
    passed = false;
    suiteError = error;
  }

  // ---- Execute sequentially.
  if (!suiteError && !prepareOnlyComplete) {
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
    logger.start(`Running "${label}" (${(index + 1).toString()}/${resolved.length.toString()})…`);
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
      suiteError = error;
    }
  }

  const result: SuiteResult = {
    suite: suite.name,
    startedAt,
    endedAt: isoNow(),
    outputRoot,
    status: interrupted ? "interrupted" : suiteError ? "failed" : "completed",
    passed: passed && !interrupted,
    entries,
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
