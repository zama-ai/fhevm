import type { LatencyStats } from "./histogram";
import type { FlowReport, Report, TargetReport } from "./schema";
import type { CounterDelta, GaugePeak, HistogramQuantiles } from "./prom-analysis";
import { MIN_DISPATCH_HEALTH_SAMPLES } from "../collectors/injector-runtime";
import { plannedRequestCount } from "../scenario/schema";

/** Renders report.json into the human-readable report.md. */

// Percentiles from tiny samples look precise while mostly restating individual
// observations. Below this boundary the Markdown uses n/mean/max descriptions;
// report.json still retains every computed percentile for machine consumers.
const DISTRIBUTION_MIN_COUNT = 20;

const ms = (value: number | undefined): string =>
  value === undefined ? "-" : `${Math.round(value).toString()} ms`;

const seconds = (value: number | undefined): string =>
  value === undefined ? "-" : `${(Math.round(value * 1000) / 1000).toString()} s`;

const percent = (value: number | undefined): string =>
  value === undefined ? "-" : `${(value * 100).toFixed(2)}%`;

const pct = (value: number | undefined): string =>
  value === undefined ? "-" : `${value.toString()}%`;

const mib = (bytes: number | undefined): string =>
  bytes === undefined ? "-" : `${(Math.round((bytes / 1_048_576) * 10) / 10).toString()} MiB`;

const signed = (value: number | undefined, unit = ""): string => {
  if (value === undefined) return "-";
  const rounded = Math.round(value);
  return `${rounded >= 0 ? "+" : ""}${rounded.toString()}${unit}`;
};

const relativePercent = (a: number | undefined, b: number | undefined): string =>
  a === undefined || b === undefined || a === 0
    ? "-"
    : `${b >= a ? "+" : ""}${(((b - a) / a) * 100).toFixed(1)}%`;

const decimal = (value: number | undefined): string =>
  value === undefined ? "-" : value.toFixed(2);

const signedDecimal = (value: number | undefined): string =>
  value === undefined ? "-" : `${value >= 0 ? "+" : ""}${value.toFixed(2)}`;

const severityIcon = (severity: string): string =>
  severity === "critical"
    ? "x"
    : severity === "warn"
      ? "!"
      : severity === "ok"
        ? "ok"
        : "i";

const metricLabels = (labels: Readonly<Record<string, string>>): string =>
  Object.entries(labels).sort(([a], [b]) => a.localeCompare(b))
    .map(([key, value]) => `${key}=${value.replace(/\|/g, "\\|").replace(/\r?\n/g, " ")}`).join(", ") || "(none)";

const estimateNote = (entry: { lowerBound?: boolean }): string =>
  entry.lowerBound ? "yes (reset)" : "no";

const boundedCount = (value: number, lowerBound: boolean | undefined): string =>
  `${lowerBound ? "at least " : ""}${value.toString()}`;

const boundedRate = (value: number | undefined, lowerBound: boolean | undefined): string =>
  value === undefined ? "-" : `${lowerBound ? "at least " : ""}${value.toString()}/s`;

const compactList = (values: readonly string[], limit = 6): string => {
  if (values.length === 0) return "none";
  const shown = values.slice(0, limit);
  const remaining = values.length - shown.length;
  return `${shown.join(", ")}${remaining > 0 ? `, +${remaining.toString()} more` : ""}`;
};

const signalLabel = (signal: string): string =>
  signal
    .replace(/^v2/, "v2 ")
    .replace(/([a-z])([A-Z])/g, "$1 $2")
    .toLowerCase();

const metricsCoverage = (target: TargetReport): string | undefined => {
  const metrics = target.metrics;
  if (!metrics) return undefined;
  const collection = metrics.collection ?? { successfulScrapes: 0, failedScrapes: 0 };
  if (collection.successfulScrapes === 0) {
    return `${target.target} unavailable (0/${(collection.successfulScrapes + collection.failedScrapes).toString()} scrapes)`;
  }
  const capabilities = Object.entries(metrics.capabilities.signals);
  const available = capabilities.filter(([, capability]) => capability.available).length;
  return `${target.target} ${available === capabilities.length ? "complete" : "partial"} (${available.toString()}/${capabilities.length.toString()} mapped signals)`;
};

const executiveLatencyEvidence = (report: Report): string | undefined => {
  const samples = report.targets.flatMap((target) =>
    target.flows.map((flow) => ({
      label: `${target.target}/${flow.flow}`,
      count: flow.e2eLatency?.count ?? 0,
    })),
  );
  if (samples.length === 0 || samples.every((sample) => sample.count >= DISTRIBUTION_MIN_COUNT)) {
    return undefined;
  }
  const summary = compactList(samples.map((sample) => `${sample.label} n=${sample.count.toString()}`));
  return `descriptive only (${summary}); fewer than ${DISTRIBUTION_MIN_COUNT.toString()} successful e2e observations do not support a percentile performance conclusion`;
};

const observedLatency = (stats: LatencyStats | undefined): string => {
  if (!stats) return "-";
  if (stats.count === 1) return `n=1 · observed ${ms(stats.maxMs)}`;
  return `n=${stats.count.toString()} · mean ${ms(stats.meanMs)} · max ${ms(stats.maxMs)}`;
};

const percentileLatency = (stats: LatencyStats | undefined): string =>
  stats
    ? `n=${stats.count.toString()} · p50 ${ms(stats.p50Ms)} · p90 ${ms(stats.p90Ms)} · p95 ${ms(stats.p95Ms)} · p99 ${ms(stats.p99Ms)}`
    : "-";

const latencyEvidence = (stats: LatencyStats | undefined): string =>
  stats && stats.count >= DISTRIBUTION_MIN_COUNT
    ? percentileLatency(stats)
    : observedLatency(stats);

const pushCounterMetrics = (
  lines: string[], title: string, entries: readonly CounterDelta[],
): void => {
  const observed = entries.filter((entry) => entry.delta !== 0);
  if (observed.length === 0) return;
  lines.push(`#### ${title}`, "", "| Labels | Delta | Lower bound? |", "| --- | ---: | --- |");
  for (const entry of observed) {
    lines.push(`| ${metricLabels(entry.labels)} | ${entry.delta.toString()} | ${estimateNote(entry)} |`);
  }
  lines.push("");
};

const pushHistogramMetrics = (
  lines: string[], title: string, entries: readonly HistogramQuantiles[],
): void => {
  const observed = entries.filter((entry) => entry.count > 0);
  if (observed.length === 0) return;
  lines.push(`#### ${title}`, "", "| Labels | Evidence | Latency | Lower bound? |", "| --- | --- | --- | --- |");
  for (const entry of observed) {
    const latency = entry.count >= DISTRIBUTION_MIN_COUNT
      ? `p50 ${seconds(entry.p50)} · p90 ${seconds(entry.p90)} · p95 ${seconds(entry.p95)} · p99 ${seconds(entry.p99)}`
      : "too few observations for a distribution";
    lines.push(`| ${metricLabels(entry.labels)} | n=${entry.count.toString()} | ${latency} | ${estimateNote(entry)} |`);
  }
  lines.push("");
};

const pushGaugeMetrics = (
  lines: string[], title: string, entries: readonly GaugePeak[],
): void => {
  // A zero gauge is an observed state, not an absent signal. In particular,
  // wallet lease ownership and queue depth are operationally meaningful at 0.
  if (entries.length === 0) return;
  lines.push(`#### ${title}`, "", "| Labels | Peak | Last |", "| --- | ---: | ---: |");
  for (const entry of entries) {
    lines.push(`| ${metricLabels(entry.labels)} | ${entry.peak.toString()} | ${entry.last.toString()} |`);
  }
  lines.push("");
};

const latencyRow = (label: string, stats: LatencyStats | undefined): string =>
  `| ${label} | ${latencyEvidence(stats)} |`;

const stageTarget = (
  stage: NonNullable<TargetReport["clientStages"]>[number]["stage"],
): string => {
  if (stage.vus !== undefined) return `${stage.vus.toString()} VUs`;
  if (stage.targetRps !== undefined) return `${stage.targetRps.toString()} req/s`;
  if (stage.fromRps !== undefined && stage.toRps !== undefined) {
    return `${stage.fromRps.toString()}->${stage.toRps.toString()} req/s`;
  }
  return stage.model;
};

const targetLabel = (target: TargetReport): string =>
  `${target.target}: ${target.url}${target.apiPrefix ? ` (${target.apiPrefix})` : ""}`;

const injectorAssessment = (report: Report): string | undefined => {
  const injector = report.injector;
  if (!injector) return undefined;
  const validity: Readonly<Record<string, string>> = {
    healthy: "valid",
    degraded: "use with caution",
    unhealthy: "invalid",
    indeterminate: "not established",
    unavailable: "unknown",
  };
  return `${validity[injector.health.verdict] ?? "unknown"} (${injector.health.verdict})` +
    (injector.health.reasons.length > 0 ? ` — ${injector.health.reasons.join(" ")}` : "");
};

const pushInjectorAppendix = (lines: string[], report: Report): void => {
  const injector = report.injector;
  if (!injector) return;
  lines.push("## Appendix: Injector Runtime Diagnostics");
  lines.push("");
  lines.push(`Assessment: **${injector.health.verdict}**. ${injector.health.reasons.join(" ")}`);
  lines.push("");
  lines.push("| Signal | Value |");
  lines.push("| --- | ---: |");
  lines.push(`| Runtime samples | ${injector.sampleCount.toString()} |`);
  if (injector.healthSampleCount !== undefined) {
    lines.push(`| Runtime health samples | ${injector.healthSampleCount.toString()} |`);
  }
  const dispatchLags = injector.scheduler.dispatchLagMs;
  if (dispatchLags.length >= MIN_DISPATCH_HEALTH_SAMPLES) {
    lines.push(`| Dispatch lag p95 | ${ms(injector.dispatchLagP95Ms)} |`);
    lines.push(`| Dispatch lag p99 | ${ms(injector.dispatchLagP99Ms)} |`);
  } else if (dispatchLags.length > 0) {
    const max = Math.max(...dispatchLags);
    if (dispatchLags.length === 1) {
      lines.push(`| Dispatch lag observations | n=1 · observed ${ms(max)} |`);
    } else {
      const mean = dispatchLags.reduce((total, value) => total + value, 0) / dispatchLags.length;
      lines.push(
        `| Dispatch lag observations | n=${dispatchLags.length.toString()} · mean ${ms(mean)} · max ${ms(max)} |`,
      );
    }
  } else {
    lines.push("| Dispatch lag observations | none |");
  }
  lines.push(`| Peak event-loop lag p99 | ${ms(injector.maxEventLoopLagP99Ms)} |`);
  lines.push(`| Peak event-loop utilization | ${percent(injector.peakEventLoopUtilization)} |`);
  lines.push(`| Peak RSS | ${mib(injector.peakRssBytes)} |`);
  lines.push(`| CPU user | ${injector.cpuUserMicros?.toString() ?? "-"} µs |`);
  lines.push(`| CPU system | ${injector.cpuSystemMicros?.toString() ?? "-"} µs |`);
  lines.push(`| GC count | ${injector.gcCount.toString()} |`);
  lines.push(`| GC duration | ${ms(injector.gcDurationMs)} |`);
  lines.push(`| Peak inflight | ${injector.scheduler.peakInflight.toString()} |`);
  lines.push(`| Backpressure events | ${injector.scheduler.backpressureEvents.toString()} |`);
  lines.push(`| Dropped | ${injector.scheduler.dropped.toString()} |`);
  lines.push(`| Abandoned | ${injector.scheduler.abandoned.toString()} |`);
  lines.push("");
};

const flowByName = (
  target: TargetReport | undefined,
  flow: string,
): FlowReport | undefined => target?.flows.find((candidate) => candidate.flow === flow);

const correctnessSummary = (report: Report): string => {
  const flows = report.targets.flatMap((target) => target.flows);
  const submitted = flows.reduce((total, flow) => total + flow.submitted, 0);
  const succeeded = flows.reduce((total, flow) => total + flow.succeeded, 0);
  const verifyFailed = flows.reduce((total, flow) => total + flow.verifyFailed, 0);
  const passed = submitted > 0 && succeeded === submitted && verifyFailed === 0;
  return `${passed ? "PASS" : "FAIL"} — ${succeeded.toString()}/${submitted.toString()} target ` +
    `request${submitted === 1 ? "" : "s"} succeeded; ${verifyFailed.toString()} verification ` +
    `failure${verifyFailed === 1 ? "" : "s"}`;
};

const isFullySuccessfulDualReport = (report: Report): boolean => {
  if (!report.targets.some((target) => target.target === "B")) return false;
  return report.targets.every((target) =>
    target.flows.length > 0 && target.flows.every((flow) =>
      flow.submitted > 0 &&
      flow.succeeded === flow.submitted &&
      flow.errorRate === 0 &&
      report.targets.every((candidate) => flowByName(candidate, flow.flow) !== undefined)
    ),
  );
};

const latencyMetric = (
  stats: LatencyStats | undefined,
  metric: keyof Pick<LatencyStats, "p50Ms" | "p90Ms" | "p95Ms" | "p99Ms">,
): number | undefined => stats?.[metric];

const comparisonValues = (
  aValue: number | undefined,
  bValue: number | undefined,
  deltaFormat: (value: number | undefined) => string,
): Readonly<{ delta: string; relative: string }> => {
  const delta =
    aValue === undefined || bValue === undefined ? undefined : bValue - aValue;
  return { delta: deltaFormat(delta), relative: relativePercent(aValue, bValue) };
};

const pushSideBySideSummary = (lines: string[], report: Report): void => {
  const a = report.targets.find((target) => target.target === "A");
  const b = report.targets.find((target) => target.target === "B");
  const flows = [
    ...new Set(report.targets.flatMap((target) => target.flows.map((flow) => flow.flow))),
  ].sort();

  // With a single relayer the B/delta columns are all empty, so collapse the
  // comparison tables down to A-only summaries.
  if (!b) {
    pushSingleTargetSummary(lines, a, flows);
    return;
  }

  lines.push("## Client Outcome Summary");
  lines.push("");
  lines.push("| Flow | Submitted A | Submitted B | Success A | Success B | Error A | Error B |");
  lines.push("| --- | ---: | ---: | ---: | ---: | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    const bFlow = flowByName(b, flow);
    lines.push(
      `| ${flow} | ${aFlow?.submitted.toString() ?? "-"} | ${bFlow?.submitted.toString() ?? "-"} | ` +
        `${percent(aFlow ? aFlow.succeeded / aFlow.submitted : undefined)} | ` +
        `${percent(bFlow ? bFlow.succeeded / bFlow.submitted : undefined)} | ` +
        `${percent(aFlow?.errorRate)} | ${percent(bFlow?.errorRate)} |`,
    );
  }
  lines.push("");

  lines.push("## Client Latency Comparison");
  lines.push("");
  lines.push(
    `Percentile distributions are shown only with at least ${DISTRIBUTION_MIN_COUNT.toString()} observations; smaller samples show observed mean/max values.`,
  );
  lines.push("");
  lines.push("| Flow | Metric | A | B | B - A | B vs A |");
  lines.push("| --- | --- | ---: | ---: | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    const bFlow = flowByName(b, flow);
    const pushLatency = (
      metricName: string,
      aValue: number | undefined,
      bValue: number | undefined,
    ): void => {
      const comparison = comparisonValues(aValue, bValue, (value) => signed(value, " ms"));
      lines.push(
        `| ${flow} | ${metricName} | ${ms(aValue)} | ${ms(bValue)} | ${comparison.delta} | ${comparison.relative} |`,
      );
    };
    for (const [phase, aStats, bStats] of [
      ["submit", aFlow?.submitLatency, bFlow?.submitLatency],
      ["e2e", aFlow?.e2eLatency, bFlow?.e2eLatency],
    ] as const) {
      const enough =
        (aStats?.count ?? 0) >= DISTRIBUTION_MIN_COUNT &&
        (bStats?.count ?? 0) >= DISTRIBUTION_MIN_COUNT;
      if (!enough) {
        lines.push(
          `| ${flow} | ${phase} descriptive (n=${aStats?.count.toString() ?? "-"}/${bStats?.count.toString() ?? "-"}) | ` +
            `${latencyEvidence(aStats)} | ${latencyEvidence(bStats)} | - | - |`,
        );
        continue;
      }
      for (const metric of ["p50Ms", "p90Ms", "p95Ms", "p99Ms"] as const) {
        pushLatency(
          `${phase} ${metric.replace("Ms", "")}`,
          latencyMetric(aStats, metric),
          latencyMetric(bStats, metric),
        );
      }
    }
  }
  lines.push("");

  lines.push("## Polling Comparison");
  lines.push("");
  lines.push("| Flow | Evidence | A mean | B mean | Delta | A tail | B tail | Delta |");
  lines.push("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    const bFlow = flowByName(b, flow);
    const mean = comparisonValues(
      aFlow?.pollsPerRequest?.mean,
      bFlow?.pollsPerRequest?.mean,
      signedDecimal,
    );
    const enough =
      (aFlow?.pollsPerRequest?.count ?? 0) >= DISTRIBUTION_MIN_COUNT &&
      (bFlow?.pollsPerRequest?.count ?? 0) >= DISTRIBUTION_MIN_COUNT;
    const aTail = enough ? aFlow?.pollsPerRequest?.p95 : aFlow?.pollsPerRequest?.max;
    const bTail = enough ? bFlow?.pollsPerRequest?.p95 : bFlow?.pollsPerRequest?.max;
    const tail = comparisonValues(
      aTail,
      bTail,
      signedDecimal,
    );
    lines.push(
      `| ${flow} | n=${aFlow?.pollsPerRequest?.count.toString() ?? "-"}/${bFlow?.pollsPerRequest?.count.toString() ?? "-"} · ${enough ? "p95" : "observed max"} | ${decimal(aFlow?.pollsPerRequest?.mean)} | ` +
        `${decimal(bFlow?.pollsPerRequest?.mean)} | ${mean.delta} | ` +
        `${decimal(aTail)} | ${decimal(bTail)} | ${tail.delta} |`,
    );
  }
  lines.push("");

};

const pushSingleTargetSummary = (
  lines: string[],
  a: Report["targets"][number] | undefined,
  flows: readonly string[],
): void => {
  lines.push("## Client Outcome Summary");
  lines.push("");
  lines.push("| Flow | Submitted | Success | Error |");
  lines.push("| --- | ---: | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    lines.push(
      `| ${flow} | ${aFlow?.submitted.toString() ?? "-"} | ` +
        `${percent(aFlow ? aFlow.succeeded / aFlow.submitted : undefined)} | ` +
        `${percent(aFlow?.errorRate)} |`,
    );
  }
  lines.push("");

  lines.push("## Client Latency");
  lines.push("");
  lines.push(
    `Percentile distributions are shown only with at least ${DISTRIBUTION_MIN_COUNT.toString()} observations.`,
  );
  lines.push("");
  lines.push("| Flow | Phase | Evidence |");
  lines.push("| --- | --- | --- |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    lines.push(`| ${flow} | submit | ${latencyEvidence(aFlow?.submitLatency)} |`);
    lines.push(`| ${flow} | e2e | ${latencyEvidence(aFlow?.e2eLatency)} |`);
  }
  lines.push("");

  lines.push("## Polling");
  lines.push("");
  lines.push("| Flow | Evidence | Mean | Tail |");
  lines.push("| --- | --- | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    lines.push(
      `| ${flow} | n=${aFlow?.pollsPerRequest?.count.toString() ?? "-"} · ${(aFlow?.pollsPerRequest?.count ?? 0) >= DISTRIBUTION_MIN_COUNT ? "p95" : "observed max"} | ${decimal(aFlow?.pollsPerRequest?.mean)} | ` +
        `${decimal((aFlow?.pollsPerRequest?.count ?? 0) >= DISTRIBUTION_MIN_COUNT ? aFlow?.pollsPerRequest?.p95 : aFlow?.pollsPerRequest?.max)} |`,
    );
  }
  lines.push("");
};

const pushOutcomeComparison = (lines: string[], report: Report): void => {
  const a = report.targets.find((target) => target.target === "A");
  const b = report.targets.find((target) => target.target === "B");
  if (!b) return;
  const flows = [
    ...new Set(report.targets.flatMap((target) => target.flows.map((flow) => flow.flow))),
  ].sort();
  const outcomes = [
    ["succeeded", "succeeded"],
    ["failed", "failed"],
    ["submit_failed", "submitFailed"],
    ["verify_failed", "verifyFailed"],
    ["timed_out", "timedOut"],
    ["protocol_error", "protocolErrors"],
    ["aborted", "aborted"],
  ] as const;
  const hasDistinctOutcomeEvidence = flows.some((flow) => {
    const aFlow = flowByName(a, flow);
    const bFlow = flowByName(b, flow);
    return !aFlow || !bFlow || aFlow.succeeded !== bFlow.succeeded ||
      outcomes.slice(1).some(([, key]) => (aFlow[key] ?? 0) > 0 || (bFlow[key] ?? 0) > 0);
  });
  if (!hasDistinctOutcomeEvidence) return;
  lines.push("## Outcome Comparison");
  lines.push("");
  lines.push("| Flow | Outcome | A | B | B - A |");
  lines.push("| --- | --- | ---: | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    const bFlow = flowByName(b, flow);
    for (const [label, key] of outcomes) {
      const aValue = aFlow?.[key] ?? 0;
      const bValue = bFlow?.[key] ?? 0;
      lines.push(
        `| ${flow} | ${label} | ${aValue.toString()} | ${bValue.toString()} | ${signed(bValue - aValue)} |`,
      );
    }
  }
  lines.push("");
};

const pushPairComparison = (lines: string[], report: Report): void => {
  if (!report.comparison || report.comparison.flows.length === 0) return;
  lines.push("## Paired Latency Delta");
  lines.push("");
  lines.push(
    "Deltas are client-observed `B - A` for the paired workload item; SDK drivers execute independent target journeys, and e2e values include polling quantization.",
  );
  lines.push("");
  lines.push(
    `Percentile deltas require at least ${DISTRIBUTION_MIN_COUNT.toString()} paired successful observations; smaller samples are descriptive only.`,
  );
  lines.push("");
  lines.push("| Flow | Pairs | Both succeeded | A only succeeded | B only succeeded | Both failed | Different outcome | E2E evidence |");
  lines.push("| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |");
  for (const flow of report.comparison.flows) {
    const delta = flow.e2eLatencyDelta;
    const evidence = !delta
      ? "no paired success latency"
      : delta.count === 1
        ? `n=1 · observed delta ${signed(delta.meanMs, " ms")}`
        : delta.count < DISTRIBUTION_MIN_COUNT
          ? `n=${delta.count.toString()} · mean ${signed(delta.meanMs, " ms")} · observed max ${signed(delta.maxMs, " ms")}`
          : `n=${delta.count.toString()} · mean ${signed(delta.meanMs, " ms")} · p50 ${signed(delta.p50Ms, " ms")} · p90 ${signed(delta.p90Ms, " ms")} · p95 ${signed(delta.p95Ms, " ms")} · p99 ${signed(delta.p99Ms, " ms")} · max ${signed(delta.maxMs, " ms")}`;
    lines.push(
      `| ${flow.flow} | ${flow.pairs.toString()} | ${flow.bothSucceeded.toString()} | ${flow.aOnlySucceeded.toString()} | ${flow.bOnlySucceeded.toString()} | ${flow.bothFailed.toString()} | ${flow.differentTerminalOutcome.toString()} | ${evidence} |`,
    );
  }
  lines.push("");
};

const pushTargetDetails = (lines: string[], target: TargetReport, report: Report): void => {
  lines.push(`## Target ${target.target} Details`);
  lines.push("");
  lines.push(`Relayer: ${targetLabel(target)}`);
  lines.push("");

  const suppressSuccessfulDualClientDetail = isFullySuccessfulDualReport(report);
  for (const flow of suppressSuccessfulDualClientDetail ? [] : target.flows) {
    lines.push(`### ${target.target} ${flow.flow} (${flow.driver})`);
    lines.push("");
    lines.push(
      `${flow.submitted.toString()} submitted · ${flow.succeeded.toString()} succeeded · ` +
        `${flow.failed.toString()} failed · ${flow.submitFailed.toString()} submit-failed · ` +
        `${flow.verifyFailed.toString()} verify-failed · ${flow.timedOut.toString()} timed out · ` +
        `${flow.protocolErrors.toString()} protocol errors · ${flow.aborted.toString()} aborted · ` +
        `error rate ${percent(flow.errorRate)}`,
    );
    lines.push("");
    lines.push("| Latency | Evidence |");
    lines.push("| --- | --- |");
    lines.push(latencyRow("Submit (POST)", flow.submitLatency));
    lines.push(latencyRow("End-to-end*", flow.e2eLatency));
    lines.push("");
    lines.push("*end-to-end is quantized to the poll interval; relayer-side histograms are poll-free.");
    if (flow.pollsPerRequest) {
      lines.push("");
      const enough = flow.pollsPerRequest.count >= DISTRIBUTION_MIN_COUNT;
      lines.push(
        `Polls per request: n=${flow.pollsPerRequest.count.toString()}, mean ${flow.pollsPerRequest.mean.toString()}, ` +
          `${enough ? `p95 ${flow.pollsPerRequest.p95.toString()}` : `observed max ${flow.pollsPerRequest.max.toString()}`}.`,
      );
    }
    const errorLabels = Object.entries(flow.byErrorLabel);
    if (errorLabels.length > 0) {
      lines.push("");
      lines.push("| Error label | Count |");
      lines.push("| --- | --- |");
      for (const [label, count] of errorLabels.sort((a, b) => b[1] - a[1])) {
        lines.push(`| ${label} | ${count.toString()} |`);
      }
    }
    lines.push("");
  }

  const showClientStages = target.clientStages && target.clientStages.length > 0 &&
    !(suppressSuccessfulDualClientDetail && target.clientStages.length === 1);
  if (showClientStages && target.clientStages) {
    lines.push(`### ${target.target} Client Results by Load Stage`);
    lines.push("");
    lines.push("| Stage | Target | Flow | Driver | Submitted | Error rate | E2E evidence |");
    lines.push("| --- | --- | --- | --- | ---: | ---: | --- |");
    for (const stage of target.clientStages) {
      lines.push(
        `| ${stage.stage.label} | ${stageTarget(stage.stage)} | ${stage.flow} | ${stage.driver} | ` +
          `${stage.submitted.toString()} | ${percent(stage.errorRate)} | ${latencyEvidence(stage.e2eLatency)} |`,
      );
    }
    lines.push("");
  }

  if (target.correlation && target.correlation.length > 0) {
    lines.push(`### ${target.target} Poll Quantization`);
    lines.push("");
    lines.push("| Flow | Matched | Client E2E | Server E2E | Poll overhead |");
    lines.push("| --- | ---: | --- | --- | --- |");
    for (const c of target.correlation) {
      lines.push(
        `| ${c.flow} | ${c.matched.toString()} | ${latencyEvidence(c.clientE2e)} | ${latencyEvidence(c.serverE2e)} | ` +
          `${latencyEvidence(c.pollOverhead)} |`,
      );
    }
    lines.push("");
  }

  if (target.stages && target.stages.length > 0) {
    lines.push(`### ${target.target} Pipeline Stages (database timestamps)`);
    lines.push("");
    lines.push("| Flow | Stage | Count | Retried | Evidence | Share |");
    lines.push("| --- | --- | ---: | ---: | --- | ---: |");
    for (const stage of target.stages) {
      lines.push(
        `| ${stage.flow} | ${stage.stage} | ${stage.stats.count.toString()} | ${stage.retriedCount.toString()} | ` +
          `${latencyEvidence(stage.stats)} | ${pct(stage.shareOfE2ePct)} |`,
      );
    }
    lines.push("");
  }

  const metrics = target.metrics;
  if (metrics) {
    // Keep rendering additive-schema v1 reports written before scrape health
    // became explicit.
    const collection = metrics.collection ?? { successfulScrapes: 0, failedScrapes: 0 };
    lines.push(`### ${target.target} Relayer Metrics (run-window deltas)`);
    lines.push("");
    if (collection.successfulScrapes === 0) {
      const attempts = collection.successfulScrapes + collection.failedScrapes;
      const failure = (collection.lastError?.replace(/\r?\n/g, " ") ?? "no successful response")
        .replace(/[.!?]+$/, "");
      lines.push(
        `**Unavailable:** 0/${attempts.toString()} scrapes succeeded; ${failure}` +
          `${collection.lastFailureAt ? ` (last failure ${collection.lastFailureAt})` : ""}. ` +
          `No relayer-side performance, saturation, or resource conclusions were drawn for target ${target.target}.`,
      );
      lines.push("");
    } else {
      const capabilities = Object.entries(metrics.capabilities.signals);
      const available = capabilities
        .filter(([, capability]) => capability.available)
        .map(([signal]) => signalLabel(signal));
      const missing = capabilities
        .filter(([, capability]) => !capability.available)
        .map(([signal]) => signalLabel(signal));
      lines.push(
        `Collection: ${collection.failedScrapes > 0 ? "**partial**" : "successful"} ` +
          `(${collection.successfulScrapes.toString()} successful, ${collection.failedScrapes.toString()} failed scrape(s)); ` +
          `profile **${metrics.capabilities.profile}**.`,
      );
      lines.push(`Available coverage: ${compactList(available)}.`);
      lines.push(`Missing coverage: ${compactList(missing)}. Full capability reasons remain in report.json.`);
      if (collection.lastError) {
        lines.push(
          `Most recent scrape failure${collection.lastFailureAt ? ` at ${collection.lastFailureAt}` : ""}: ` +
            `${collection.lastError.replace(/\r?\n/g, " ")}`,
        );
      }
      lines.push("");
    if (metrics.e2eDurations.length > 0) {
      const entries = metrics.e2eDurations.filter((entry) => entry.count > 0);
      if (entries.length > 0) {
      lines.push("| Flow | Terminal status | Evidence | Latency | Lower bound? |");
      lines.push("| --- | --- | --- | --- | --- |");
      for (const entry of entries) {
        const latency = entry.count >= DISTRIBUTION_MIN_COUNT
          ? `p50 ${seconds(entry.p50)} · p90 ${seconds(entry.p90)} · p95 ${seconds(entry.p95)} · p99 ${seconds(entry.p99)}`
          : "too few observations for a distribution";
        lines.push(
          `| ${entry.labels.flow ?? "?"} | ${entry.labels.terminal_status ?? "?"} | n=${entry.count.toString()} | ` +
            `${latency} | ${estimateNote(entry)} |`,
        );
      }
      lines.push("");
      }
    }
    if (metrics.stageDurations.length > 0) {
      const entries = metrics.stageDurations.filter((entry) => entry.count > 0);
      if (entries.length > 0) {
      lines.push("| Flow | Stage | Evidence | Latency | Lower bound? |");
      lines.push("| --- | --- | --- | --- | --- |");
      for (const entry of entries) {
        const latency = entry.count >= DISTRIBUTION_MIN_COUNT
          ? `p50 ${seconds(entry.p50)} · p90 ${seconds(entry.p90)} · p95 ${seconds(entry.p95)} · p99 ${seconds(entry.p99)}`
          : "too few observations for a distribution";
        lines.push(
          `| ${entry.labels.flow ?? "?"} | ${entry.labels.stage ?? "?"} | n=${entry.count.toString()} | ` +
            `${latency} | ${estimateNote(entry)} |`,
        );
      }
      lines.push("");
      }
    }
    pushCounterMetrics(lines, "Terminal counters", metrics.terminalTotals);
    pushCounterMetrics(lines, "Reclaims", metrics.reclaims);
    pushGaugeMetrics(lines, "Legacy throttler queue depth", metrics.throttlerDepth ?? []);
    pushHistogramMetrics(lines, "Dependency durations", metrics.dependencyDurations);
    if (metrics.v2) {
      pushCounterMetrics(lines, "V2 input-proof inserts", metrics.v2.inputProofInserted);
      pushHistogramMetrics(lines, "V2 input-proof duration", metrics.v2.inputProofDuration);
      pushCounterMetrics(lines, "V2 transaction outcomes", metrics.v2.transactionCounts);
      pushHistogramMetrics(lines, "V2 transaction durations", metrics.v2.transactionDurations);
      pushCounterMetrics(lines, "V2 transaction error events", metrics.v2.transactionErrors);
      pushCounterMetrics(lines, "V2 recovery runs", metrics.v2.recoveryRuns);
      pushHistogramMetrics(lines, "V2 recovery durations", metrics.v2.recoveryDurations);
      pushCounterMetrics(lines, "V2 recovery items", metrics.v2.recoveryItems);
      pushGaugeMetrics(lines, "V2 wallet lease owned", metrics.v2.walletLeaseOwned);
      pushCounterMetrics(lines, "V2 wallet lease transitions", metrics.v2.walletLeaseTransitions);
      pushCounterMetrics(lines, "V2 database errors", metrics.v2.dbErrors);
    }
    if (metrics.limiterUtilization.length > 0) {
      const byLimiter = new Map<string, { peakInUse?: number; max?: number }>();
      for (const entry of metrics.limiterUtilization) {
        const name = entry.labels.limiter ?? "?";
        const row = byLimiter.get(name) ?? {};
        if (entry.labels.state === "max") row.max = entry.peak;
        else row.peakInUse = entry.peak;
        byLimiter.set(name, row);
      }
      lines.push(`### ${target.target} Limiter / semaphore utilization`);
      lines.push("");
      lines.push("| Limiter | Peak in use | Configured max | Saturated? |");
      lines.push("| --- | ---: | ---: | --- |");
      for (const [name, row] of [...byLimiter].sort()) {
        const saturated =
          row.max !== undefined && row.peakInUse !== undefined && row.peakInUse >= row.max;
        lines.push(
          `| ${name} | ${row.peakInUse ?? "-"} | ${row.max ?? "-"} | ${saturated ? "yes" : "no"} |`,
        );
      }
      lines.push("");
    }
    if (metrics.http && metrics.http.totalRequests > 0) {
      const http = metrics.http;
      lines.push(`### ${target.target} HTTP requests (relayer-side)`);
      lines.push("");
      lines.push(
        `${boundedCount(http.totalRequests, http.totalRequestsLowerBound)} HTTP request(s); ` +
          `${boundedCount(http.nonSuccess, http.nonSuccessLowerBound)} non-2xx. ` +
          `${boundedCount(http.loadRequests, http.loadRequestsLowerBound)} load API request(s) to workload routes.`,
      );
      lines.push(
        `Observed rates: ${boundedRate(http.totalRequestsPerSec, http.totalRequestsLowerBound)} total, ` +
          `${boundedRate(http.loadRequestsPerSec, http.loadRequestsLowerBound)} load API.`,
      );
      lines.push("");
      lines.push("| Endpoint | Status | Count | Lower bound? |");
      lines.push("| --- | --- | ---: | --- |");
      for (const entry of http.byEndpointStatus.filter((entry) => entry.delta !== 0)) {
        lines.push(
          `| ${entry.labels.endpoint ?? "?"} | ${entry.labels.status ?? "?"} | ${entry.delta.toString()} | ${estimateNote(entry)} |`,
        );
      }
      lines.push("");
    }
    if (metrics.process) {
      const proc = metrics.process;
      lines.push(`### ${target.target} Process / host`);
      lines.push("");
      lines.push("| Resource | First | Peak | Last | Delta | Drift/hour |");
      lines.push("| --- | ---: | ---: | ---: | ---: | ---: |");
      if (proc.rss) {
        lines.push(
          `| RSS | ${mib(proc.rss.first)} | ${mib(proc.rss.peak)} | ${mib(proc.rss.last)} | ${mib(proc.rss.delta)} | ${mib(proc.rss.perHour)} |`,
        );
      }
      if (proc.virtualMemory) {
        lines.push(
          `| Virtual | ${mib(proc.virtualMemory.first)} | ${mib(proc.virtualMemory.peak)} | ${mib(proc.virtualMemory.last)} | ${mib(proc.virtualMemory.delta)} | ${mib(proc.virtualMemory.perHour)} |`,
        );
      }
      if (proc.openFds) {
        lines.push(
          `| Open FDs | ${proc.openFds.first.toString()} | ${proc.openFds.peak.toString()} | ${proc.openFds.last.toString()} | ${signed(proc.openFds.delta)} | ${signed(proc.openFds.perHour)} |`,
        );
      }
      if (proc.avgCpuCores !== undefined) {
        lines.push(`| Average CPU cores | - | - | ${proc.avgCpuCores.toFixed(2)} | - | - |`);
      }
      if (proc.maxFds !== undefined) {
        lines.push(`| Max FDs | - | ${proc.maxFds.toString()} | ${proc.maxFds.toString()} | - | - |`);
      }
      lines.push("");
    }
    }
  } else {
    lines.push(`### ${target.target} Relayer Metrics`);
    lines.push("");
    lines.push("Relayer-side metrics were not collected for this target.");
    lines.push("");
  }

  if (target.queueDepth) {
    const qd = target.queueDepth;
    lines.push(`### ${target.target} Queue Depth / Backlog`);
    lines.push("");
    lines.push(
      `Source: ${qd.source} · trend **${qd.trend}** · max pending ${qd.maxPending.toString()} · ` +
        `pending at end ${qd.endPending.toString()} · ${qd.sampleCount.toString()} samples`,
    );
    lines.push("");
    if (qd.byStage.length > 0) {
      lines.push("| Status | Peak | At end |");
      lines.push("| --- | ---: | ---: |");
      for (const entry of qd.byStage) {
        lines.push(`| ${entry.status} | ${entry.peak.toString()} | ${entry.end.toString()} |`);
      }
      lines.push("");
    }
  }

  if (target.relayerConfig) {
    lines.push(`### ${target.target} Relayer Config Snapshot`);
    lines.push("");
    lines.push(`From \`${target.relayerConfig.path}\`:`);
    lines.push("");
    lines.push("```yaml");
    lines.push(target.relayerConfig.raw.trimEnd());
    lines.push("```");
    lines.push("");
  }
};

export const renderMarkdownReport = (report: Report): string => {
  const lines: string[] = [];
  const { run } = report;

  lines.push(`# Load Test Report - ${run.scenario.name}`);
  lines.push("");
  lines.push("## Executive Summary");
  lines.push("");
  lines.push(`- **Network:** ${run.network}`);
  lines.push(`- **Relayers:** ${report.targets.map(targetLabel).join(" · ")}`);
  lines.push(`- **Model:** ${run.model}`);
  lines.push(`- **Status:** ${run.status}`);
  lines.push(`- **Window:** ${run.startedAt} -> ${run.endedAt}`);
  lines.push(
    `- **Workflows:** ${run.submitted.toString()} submitted of ${run.plannedRequests.toString()} planned · achieved ${run.achievedWorkflowsPerSec.toString()} ${run.achievedWorkflowsPerSec === 1 ? "workflow/s" : "workflows/s"}` +
      (run.abandoned > 0 ? ` · **${run.abandoned.toString()} abandoned at drain timeout**` : ""),
  );
  const finitePlan = plannedRequestCount(run.scenario.shape);
  const submissionWasPartial = finitePlan === undefined || run.submitted < finitePlan;
  if (submissionWasPartial && run.stoppedAtSegment !== undefined) {
    lines.push(
      `- **Saturation stop:** submission halted after segment ${run.stoppedAtSegment.toString()} (queue depth grew).`,
    );
  }
  if (submissionWasPartial && run.poolExhausted) {
    lines.push("- **Pool exhausted mid-run** - results cover a partial run.");
  }
  lines.push(
    `- **Thresholds:** ${report.thresholds.passed ? "passed" : `${report.thresholds.breaches.length.toString()} breach(es)`}`,
  );
  lines.push(`- **Correctness:** ${correctnessSummary(report)}`);
  const latencyEvidenceSummary = executiveLatencyEvidence(report);
  if (latencyEvidenceSummary) lines.push(`- **Performance evidence:** ${latencyEvidenceSummary}.`);
  const assessment = injectorAssessment(report);
  if (assessment) lines.push(`- **Injector assessment:** ${assessment}`);
  const coverage = report.targets.map(metricsCoverage).filter((value): value is string => value !== undefined);
  if (coverage.length > 0) lines.push(`- **Relayer telemetry:** ${coverage.join(" · ")}`);
  lines.push("");

  if (
    report.diagnosis &&
    (report.diagnosis.flags.length > 0 || report.diagnosis.recommendations.length > 0)
  ) {
    const { diagnosis } = report;
    lines.push("## Diagnosis");
    lines.push("");
    for (const flag of diagnosis.flags) {
      lines.push(`- ${severityIcon(flag.severity)} ${flag.message}`);
    }
    if (diagnosis.recommendations.length > 0) {
      lines.push("");
      lines.push("**Recommendations**");
      lines.push("");
      for (const rec of diagnosis.recommendations) lines.push(`- ${rec}`);
    }
    lines.push("");
  }

  if (report.thresholds.breaches.length > 0) {
    lines.push("## Threshold Breaches");
    lines.push("");
    lines.push("| Threshold | Target | Flow | Limit | Actual |");
    lines.push("| --- | --- | --- | ---: | ---: |");
    for (const breach of report.thresholds.breaches) {
      lines.push(
        `| ${breach.threshold} | ${breach.target ?? "all"} | ${breach.flow ?? "all"} | ${breach.limit.toString()} | ${breach.actual.toString()} |`,
      );
    }
    lines.push("");
  }

  pushSideBySideSummary(lines, report);
  pushOutcomeComparison(lines, report);
  pushPairComparison(lines, report);
  for (const target of report.targets) pushTargetDetails(lines, target, report);
  pushInjectorAppendix(lines, report);

  return `${lines.join("\n")}\n`;
};
