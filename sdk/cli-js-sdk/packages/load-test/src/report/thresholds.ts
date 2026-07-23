import type { Thresholds } from "../scenario/schema";
import type { Report, ThresholdBreach } from "./schema";

/**
 * Threshold evaluation: scenario-declared limits checked against the built
 * report. Breaches make the run exit non-zero so CI can gate on them.
 */
export const evaluateThresholds = (
  report: Omit<Report, "thresholds">,
  thresholds: Thresholds,
): Report["thresholds"] => {
  const breaches: ThresholdBreach[] = [];
  const allFlows = report.targets.flatMap((target) =>
    target.flows.map((flow) => ({ target: target.target, flow })),
  );
  for (const targetReport of report.targets) {
    const measured = targetReport.flows.reduce(
      (total, flow) => total + flow.submitted - flow.aborted,
      0,
    );
    const errors = targetReport.flows.reduce(
      (total, flow) => total + flow.submitted - flow.aborted - flow.succeeded,
      0,
    );
    const errorRate = measured === 0 ? 0 : errors / measured;
    if (errorRate > thresholds.maxErrorRate) {
      breaches.push({
        threshold: "maxErrorRate",
        limit: thresholds.maxErrorRate,
        actual: Math.round(errorRate * 10_000) / 10_000,
        target: targetReport.target,
      });
    }
    const verifyFailures = targetReport.flows.reduce(
      (total, flow) => total + flow.verifyFailed,
      0,
    );
    if (verifyFailures > thresholds.maxVerifyFailures) {
      breaches.push({
        threshold: "maxVerifyFailures",
        limit: thresholds.maxVerifyFailures,
        actual: verifyFailures,
        target: targetReport.target,
      });
    }
  }

  for (const { target, flow } of allFlows) {
    const flowThresholds = thresholds.perFlow[flow.flow];
    if (!flowThresholds) continue;
    if (
      flowThresholds.maxErrorRate !== undefined &&
      flow.errorRate > flowThresholds.maxErrorRate
    ) {
      breaches.push({
        threshold: "perFlow.maxErrorRate",
        limit: flowThresholds.maxErrorRate,
        actual: Math.round(flow.errorRate * 10_000) / 10_000,
        flow: flow.flow,
        target,
      });
    }
    if (
      flowThresholds.e2eP95Ms !== undefined &&
      (flow.e2eLatency?.p95Ms ?? 0) > flowThresholds.e2eP95Ms
    ) {
      breaches.push({
        threshold: "perFlow.e2eP95Ms",
        limit: flowThresholds.e2eP95Ms,
        actual: flow.e2eLatency?.p95Ms ?? 0,
        flow: flow.flow,
        target,
      });
    }
    if (
      flowThresholds.e2eP99Ms !== undefined &&
      (flow.e2eLatency?.p99Ms ?? 0) > flowThresholds.e2eP99Ms
    ) {
      breaches.push({
        threshold: "perFlow.e2eP99Ms",
        limit: flowThresholds.e2eP99Ms,
        actual: flow.e2eLatency?.p99Ms ?? 0,
        flow: flow.flow,
        target,
      });
    }
  }

  return { passed: breaches.length === 0, breaches };
};
