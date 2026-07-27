import type { FlowKind } from "../relayer/types";
import type { Report } from "./schema";

/**
 * Baseline comparison: regression detection is a diff between two
 * report.json documents, not a human reading two dashboards.
 */

export type Regression = Readonly<{
  flow: FlowKind;
  target?: string;
  metric: string;
  baseline: number;
  current: number;
  /** current / baseline - 1. */
  relativeChange: number;
}>;

export type DiffResult = Readonly<{
  passed: boolean;
  regressions: readonly Regression[];
  notes: readonly string[];
}>;

export type DiffOptions = Readonly<{
  /** Allowed relative latency increase before flagging; >= 0 (default 0.20). */
  latencyTolerance?: number;
  /** Allowed absolute error-rate increase before flagging; in [0, 1] (default 0.01). */
  errorRateTolerance?: number;
}>;

export const diffReports = (
  baseline: Report,
  current: Report,
  options: DiffOptions = {},
): DiffResult => {
  const latencyTolerance = options.latencyTolerance ?? 0.2;
  const errorRateTolerance = options.errorRateTolerance ?? 0.01;
  if (!Number.isFinite(latencyTolerance) || latencyTolerance < 0) {
    throw new Error(`latencyTolerance must be >= 0, got ${latencyTolerance.toString()}.`);
  }
  if (
    !Number.isFinite(errorRateTolerance) ||
    errorRateTolerance < 0 ||
    errorRateTolerance > 1
  ) {
    throw new Error(
      `errorRateTolerance must be between 0 and 1, got ${errorRateTolerance.toString()}.`,
    );
  }
  const regressions: Regression[] = [];
  const notes: string[] = [];

  if (
    baseline.version !== current.version ||
    baseline.run.network !== current.run.network ||
    baseline.run.model !== current.run.model ||
    !isDeepStrictEqual(baseline.run.scenario, current.run.scenario)
  ) {
    throw new Error("Baseline is incompatible with the current report version, network, model, or resolved scenario.");
  }
  const baselineTargets = baseline.targets.map((entry) => entry.target).sort();
  const currentTargets = current.targets.map((entry) => entry.target).sort();
  if (!isDeepStrictEqual(baselineTargets, currentTargets)) {
    throw new Error("Baseline target set is incompatible with the current report.");
  }

  for (const currentTarget of current.targets) {
    const baselineTarget = baseline.targets.find(
      (target) => target.target === currentTarget.target,
    );
    if (!baselineTarget) {
      notes.push(`Target ${currentTarget.target} has no baseline; skipped.`);
      continue;
    }

  for (const currentFlow of currentTarget.flows) {
    const baselineFlow = baselineTarget.flows.find((flow) => flow.flow === currentFlow.flow);
    if (!baselineFlow) {
      notes.push(`Flow ${currentFlow.flow} target ${currentTarget.target} has no baseline; skipped.`);
      continue;
    }

    for (const metric of ["p95Ms", "p99Ms"] as const) {
      const baselineValue = baselineFlow.e2eLatency?.[metric];
      const currentValue = currentFlow.e2eLatency?.[metric];
      if (baselineValue === undefined || currentValue === undefined || baselineValue === 0) {
        continue;
      }
      const relativeChange = currentValue / baselineValue - 1;
      if (relativeChange > latencyTolerance) {
        regressions.push({
          flow: currentFlow.flow,
          target: currentTarget.target,
          metric: `e2e.${metric}`,
          baseline: baselineValue,
          current: currentValue,
          relativeChange: Math.round(relativeChange * 1000) / 1000,
        });
      }
    }

    const errorRateIncrease = currentFlow.errorRate - baselineFlow.errorRate;
    if (errorRateIncrease > errorRateTolerance) {
      regressions.push({
        flow: currentFlow.flow,
        target: currentTarget.target,
        metric: "errorRate",
        baseline: baselineFlow.errorRate,
        current: currentFlow.errorRate,
        relativeChange: Math.round(errorRateIncrease * 1000) / 1000,
      });
    }

    if (currentFlow.verifyFailed > 0) {
      regressions.push({
        flow: currentFlow.flow,
        target: currentTarget.target,
        metric: "verifyFailed",
        baseline: baselineFlow.verifyFailed,
        current: currentFlow.verifyFailed,
        relativeChange: Infinity,
      });
    }
  }
  }

  return { passed: regressions.length === 0, regressions, notes };
};
import { isDeepStrictEqual } from "node:util";
