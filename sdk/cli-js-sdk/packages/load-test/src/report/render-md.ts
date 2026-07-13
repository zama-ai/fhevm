import type { LatencyStats } from "./histogram";
import type { FlowReport, Report, TargetReport } from "./schema";
import type { CounterDelta, GaugePeak, HistogramQuantiles } from "./prom-analysis";

/** Renders report.json into the human-readable report.md. */

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

const pushCounterMetrics = (
  lines: string[], title: string, entries: readonly CounterDelta[],
): void => {
  if (entries.length === 0) return;
  lines.push(`#### ${title}`, "", "| Labels | Delta | Lower bound? |", "| --- | ---: | --- |");
  for (const entry of entries) {
    lines.push(`| ${metricLabels(entry.labels)} | ${entry.delta.toString()} | ${estimateNote(entry)} |`);
  }
  lines.push("");
};

const pushHistogramMetrics = (
  lines: string[], title: string, entries: readonly HistogramQuantiles[],
): void => {
  if (entries.length === 0) return;
  lines.push(`#### ${title}`, "", "| Labels | Count | p50 | p90 | p95 | p99 | Lower bound? |", "| --- | ---: | ---: | ---: | ---: | ---: | --- |");
  for (const entry of entries) {
    lines.push(`| ${metricLabels(entry.labels)} | ${entry.count.toString()} | ${seconds(entry.p50)} | ${seconds(entry.p90)} | ${seconds(entry.p95)} | ${seconds(entry.p99)} | ${estimateNote(entry)} |`);
  }
  lines.push("");
};

const pushGaugeMetrics = (
  lines: string[], title: string, entries: readonly GaugePeak[],
): void => {
  if (entries.length === 0) return;
  lines.push(`#### ${title}`, "", "| Labels | Peak | Last |", "| --- | ---: | ---: |");
  for (const entry of entries) {
    lines.push(`| ${metricLabels(entry.labels)} | ${entry.peak.toString()} | ${entry.last.toString()} |`);
  }
  lines.push("");
};

const latencyRow = (label: string, stats: LatencyStats | undefined): string =>
  stats
    ? `| ${label} | ${stats.count.toString()} | ${ms(stats.meanMs)} | ${ms(stats.p50Ms)} | ${ms(stats.p90Ms)} | ${ms(stats.p95Ms)} | ${ms(stats.p99Ms)} | ${ms(stats.maxMs)} |`
    : `| ${label} | - | - | - | - | - | - | - |`;

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

const flowByName = (
  target: TargetReport | undefined,
  flow: string,
): FlowReport | undefined => target?.flows.find((candidate) => candidate.flow === flow);

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
    for (const metric of ["p50Ms", "p90Ms", "p95Ms", "p99Ms"] as const) {
      pushLatency(
        `submit ${metric.replace("Ms", "")}`,
        latencyMetric(aFlow?.submitLatency, metric),
        latencyMetric(bFlow?.submitLatency, metric),
      );
    }
    for (const metric of ["p50Ms", "p90Ms", "p95Ms", "p99Ms"] as const) {
      pushLatency(
        `e2e ${metric.replace("Ms", "")}`,
        latencyMetric(aFlow?.e2eLatency, metric),
        latencyMetric(bFlow?.e2eLatency, metric),
      );
    }
  }
  lines.push("");

  lines.push("## Polling Comparison");
  lines.push("");
  lines.push("| Flow | A mean | B mean | Delta | A p95 | B p95 | Delta |");
  lines.push("| --- | ---: | ---: | ---: | ---: | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    const bFlow = flowByName(b, flow);
    const mean = comparisonValues(
      aFlow?.pollsPerRequest?.mean,
      bFlow?.pollsPerRequest?.mean,
      signedDecimal,
    );
    const p95 = comparisonValues(
      aFlow?.pollsPerRequest?.p95,
      bFlow?.pollsPerRequest?.p95,
      signedDecimal,
    );
    lines.push(
      `| ${flow} | ${decimal(aFlow?.pollsPerRequest?.mean)} | ` +
        `${decimal(bFlow?.pollsPerRequest?.mean)} | ${mean.delta} | ` +
        `${decimal(aFlow?.pollsPerRequest?.p95)} | ` +
        `${decimal(bFlow?.pollsPerRequest?.p95)} | ${p95.delta} |`,
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
  lines.push("| Flow | Metric | Value |");
  lines.push("| --- | --- | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    for (const metric of ["p50Ms", "p90Ms", "p95Ms", "p99Ms"] as const) {
      lines.push(
        `| ${flow} | submit ${metric.replace("Ms", "")} | ${ms(latencyMetric(aFlow?.submitLatency, metric))} |`,
      );
    }
    for (const metric of ["p50Ms", "p90Ms", "p95Ms", "p99Ms"] as const) {
      lines.push(
        `| ${flow} | e2e ${metric.replace("Ms", "")} | ${ms(latencyMetric(aFlow?.e2eLatency, metric))} |`,
      );
    }
  }
  lines.push("");

  lines.push("## Polling");
  lines.push("");
  lines.push("| Flow | mean | p95 |");
  lines.push("| --- | ---: | ---: |");
  for (const flow of flows) {
    const aFlow = flowByName(a, flow);
    lines.push(
      `| ${flow} | ${decimal(aFlow?.pollsPerRequest?.mean)} | ` +
        `${decimal(aFlow?.pollsPerRequest?.p95)} |`,
    );
  }
  lines.push("");
};

const pushOutcomeComparison = (lines: string[], report: Report): void => {
  const a = report.targets.find((target) => target.target === "A");
  const b = report.targets.find((target) => target.target === "B");
  if (!b) return;
  lines.push("## Outcome Comparison");
  lines.push("");
  lines.push("| Flow | Outcome | A | B | B - A |");
  lines.push("| --- | --- | ---: | ---: | ---: |");
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
  lines.push("| Flow | Pairs | Both succeeded | A only succeeded | B only succeeded | Both failed | Different outcome | E2E delta mean | p50 | p90 | p95 | p99 | max |");
  lines.push("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
  for (const flow of report.comparison.flows) {
    lines.push(
      `| ${flow.flow} | ${flow.pairs.toString()} | ${flow.bothSucceeded.toString()} | ${flow.aOnlySucceeded.toString()} | ${flow.bOnlySucceeded.toString()} | ${flow.bothFailed.toString()} | ${flow.differentTerminalOutcome.toString()} | ${signed(flow.e2eLatencyDelta?.meanMs, " ms")} | ${signed(flow.e2eLatencyDelta?.p50Ms, " ms")} | ${signed(flow.e2eLatencyDelta?.p90Ms, " ms")} | ${signed(flow.e2eLatencyDelta?.p95Ms, " ms")} | ${signed(flow.e2eLatencyDelta?.p99Ms, " ms")} | ${signed(flow.e2eLatencyDelta?.maxMs, " ms")} |`,
    );
  }
  lines.push("");
};

const pushTargetDetails = (lines: string[], target: TargetReport): void => {
  lines.push(`## Target ${target.target} Details`);
  lines.push("");
  lines.push(`Relayer: ${targetLabel(target)}`);
  lines.push("");

  for (const flow of target.flows) {
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
    lines.push("| Latency | Count | Mean | p50 | p90 | p95 | p99 | Max |");
    lines.push("| --- | --- | --- | --- | --- | --- | --- | --- |");
    lines.push(latencyRow("Submit (POST)", flow.submitLatency));
    lines.push(latencyRow("End-to-end*", flow.e2eLatency));
    lines.push("");
    lines.push("*end-to-end is quantized to the poll interval; relayer-side histograms are poll-free.");
    if (flow.pollsPerRequest) {
      lines.push("");
      lines.push(
        `Polls per request: mean ${flow.pollsPerRequest.mean.toString()}, p95 ${flow.pollsPerRequest.p95.toString()}.`,
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

  if (target.clientStages && target.clientStages.length > 0) {
    lines.push(`### ${target.target} Client Results by Load Stage`);
    lines.push("");
    lines.push("| Stage | Target | Flow | Driver | Submitted | Error rate | e2e p50 | e2e p90 | e2e p95 | e2e p99 |");
    lines.push("| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |");
    for (const stage of target.clientStages) {
      lines.push(
        `| ${stage.stage.label} | ${stageTarget(stage.stage)} | ${stage.flow} | ${stage.driver} | ` +
          `${stage.submitted.toString()} | ${percent(stage.errorRate)} | ${ms(stage.e2eLatency?.p50Ms)} | ` +
          `${ms(stage.e2eLatency?.p90Ms)} | ${ms(stage.e2eLatency?.p95Ms)} | ${ms(stage.e2eLatency?.p99Ms)} |`,
      );
    }
    lines.push("");
  }

  if (target.correlation && target.correlation.length > 0) {
    lines.push(`### ${target.target} Poll Quantization`);
    lines.push("");
    lines.push("| Flow | Matched | Client e2e p50 | Server e2e p50 | Poll overhead p50 | Overhead p95 |");
    lines.push("| --- | ---: | ---: | ---: | ---: | ---: |");
    for (const c of target.correlation) {
      lines.push(
        `| ${c.flow} | ${c.matched.toString()} | ${ms(c.clientE2e?.p50Ms)} | ${ms(c.serverE2e?.p50Ms)} | ` +
          `${ms(c.pollOverhead?.p50Ms)} | ${ms(c.pollOverhead?.p95Ms)} |`,
      );
    }
    lines.push("");
  }

  if (target.stages && target.stages.length > 0) {
    lines.push(`### ${target.target} Pipeline Stages (database timestamps)`);
    lines.push("");
    lines.push("| Flow | Stage | Count | Retried | p50 | p90 | p95 | p99 | Max | Share |");
    lines.push("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for (const stage of target.stages) {
      lines.push(
        `| ${stage.flow} | ${stage.stage} | ${stage.stats.count.toString()} | ${stage.retriedCount.toString()} | ` +
          `${ms(stage.stats.p50Ms)} | ${ms(stage.stats.p90Ms)} | ${ms(stage.stats.p95Ms)} | ${ms(stage.stats.p99Ms)} | ${ms(stage.stats.maxMs)} | ${pct(stage.shareOfE2ePct)} |`,
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
    lines.push(
      `Scrapes: ${collection.successfulScrapes.toString()} successful, ` +
        `${collection.failedScrapes.toString()} failed.` +
        (collection.lastAttemptSucceeded !== undefined
          ? ` Last attempt: ${collection.lastAttemptSucceeded ? "succeeded" : "failed"}${collection.lastAttemptAt ? ` at ${collection.lastAttemptAt}` : ""}.`
          : "") +
        (collection.lastError
          ? ` Most recent failure${collection.lastFailureAt ? ` at ${collection.lastFailureAt}` : ""}: ${collection.lastError.replace(/\r?\n/g, " ")}`
          : ""),
    );
    lines.push("");
    lines.push(`Detected profile: **${metrics.capabilities.profile}**.`);
    lines.push("");
    if (metrics.capabilities.discoveredFamilies.length > 0) {
      lines.push(`Retained/discovered families: ${metrics.capabilities.discoveredFamilies.join(", ")}.`);
      lines.push("");
    }
    lines.push("| Capability | Available | Family / reason |");
    lines.push("| --- | --- | --- |");
    for (const [signal, capability] of Object.entries(metrics.capabilities.signals)) {
      lines.push(`| ${signal} | ${capability.available ? "yes" : "no"} | ${capability.family ?? capability.reason ?? "-"} |`);
    }
    lines.push("");
    if (metrics.e2eDurations.length > 0) {
      lines.push("| Flow | Terminal status | Count | p50 | p90 | p95 | p99 | Lower bound? |");
      lines.push("| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |");
      for (const entry of metrics.e2eDurations) {
        lines.push(
          `| ${entry.labels.flow ?? "?"} | ${entry.labels.terminal_status ?? "?"} | ${entry.count.toString()} | ` +
            `${seconds(entry.p50)} | ${seconds(entry.p90)} | ${seconds(entry.p95)} | ${seconds(entry.p99)} | ${estimateNote(entry)} |`,
        );
      }
      lines.push("");
    }
    if (metrics.stageDurations.length > 0) {
      lines.push("| Flow | Stage | Count | p50 | p90 | p95 | p99 | Lower bound? |");
      lines.push("| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |");
      for (const entry of metrics.stageDurations) {
        lines.push(
          `| ${entry.labels.flow ?? "?"} | ${entry.labels.stage ?? "?"} | ${entry.count.toString()} | ` +
            `${seconds(entry.p50)} | ${seconds(entry.p90)} | ${seconds(entry.p95)} | ${seconds(entry.p99)} | ${estimateNote(entry)} |`,
        );
      }
      lines.push("");
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
    if (metrics.http) {
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
      for (const entry of http.byEndpointStatus) {
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
  lines.push(`- **Network:** ${run.network}`);
  lines.push(`- **Relayers:** ${report.targets.map(targetLabel).join(" · ")}`);
  lines.push(`- **Model:** ${run.model}`);
  lines.push(`- **Status:** ${run.status}`);
  lines.push(`- **Window:** ${run.startedAt} -> ${run.endedAt}`);
  lines.push(
    `- **Workflows:** ${run.submitted.toString()} submitted of ${run.plannedRequests.toString()} planned · achieved ${run.achievedWorkflowsPerSec.toString()} workflows/s` +
      (run.abandoned > 0 ? ` · **${run.abandoned.toString()} abandoned at drain timeout**` : ""),
  );
  if (run.stoppedAtSegment !== undefined) {
    lines.push(
      `- **Saturation stop:** submission halted after segment ${run.stoppedAtSegment.toString()} (queue depth grew).`,
    );
  }
  if (run.poolExhausted) {
    lines.push("- **Pool exhausted mid-run** - results cover a partial run.");
  }
  lines.push(
    `- **Thresholds:** ${report.thresholds.passed ? "passed" : `${report.thresholds.breaches.length.toString()} breach(es)`}`,
  );
  if (report.diagnosis) lines.push(`- **Verdict:** ${report.diagnosis.verdict}`);
  if (report.injector) {
    lines.push(
      `- **Injector health:** ${report.injector.health.verdict}` +
        (report.injector.health.reasons.length > 0
          ? ` - ${report.injector.health.reasons.join(" ")}`
          : ""),
    );
  }
  lines.push("");

  if (report.diagnosis) {
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

  if (report.injector) {
    const injector = report.injector;
    lines.push("## Injector Runtime");
    lines.push("");
    lines.push(`Health: **${injector.health.verdict}**. ${injector.health.reasons.join(" ")}`);
    lines.push("");
    lines.push("| Signal | Value |");
    lines.push("| --- | ---: |");
    lines.push(`| Runtime samples | ${injector.sampleCount.toString()} |`);
    lines.push(`| Dispatch lag p95 | ${ms(injector.dispatchLagP95Ms)} |`);
    lines.push(`| Dispatch lag p99 | ${ms(injector.dispatchLagP99Ms)} |`);
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
  for (const target of report.targets) pushTargetDetails(lines, target);

  return `${lines.join("\n")}\n`;
};
