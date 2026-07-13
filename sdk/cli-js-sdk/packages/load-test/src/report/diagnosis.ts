import type {
  Diagnosis,
  DiagnosisFlag,
  Report,
  StageReport,
} from "./schema";
import type { GaugePeak, HistogramQuantiles } from "./prom-analysis";
import { plannedRequestCount } from "../scenario/schema";

/**
 * Synthesizes the "what happened and why" verdict from the assembled report.
 *
 * The raw tables answer specific questions; this layer answers the operator's
 * actual question — is it healthy, where is the bottleneck, and is the fix a
 * config knob or downstream capacity? It is deliberately conservative: it only
 * asserts what the collected data supports, and stays silent on signals that
 * weren't captured (e.g. no process metrics ⇒ no leak verdict).
 */

type ReportCore = Omit<Report, "thresholds" | "diagnosis">;

const LIMITER_CONFIG: Record<string, string> = {
  input_proof_broadcast: "queue.input_proof.max_broadcasts_in_flight",
  public_decrypt_broadcast: "queue.public_decrypt.max_broadcasts_in_flight",
  user_decrypt_broadcast: "queue.user_decrypt.max_broadcasts_in_flight",
  public_decrypt_readiness: "gateway.readiness_checker.public_decrypt.max_concurrency",
  user_decrypt_readiness: "gateway.readiness_checker.user_decrypt.max_concurrency",
  tx_engine: "gateway.tx_engine.max_concurrency",
};

/** Stages downstream of the relayer's own work (gateway / host chain). */
const DOWNSTREAM_STAGES = new Set(["confirmation", "gateway_response"]);

type Limiter = Readonly<{ name: string; peakInUse: number; max: number; ratio: number }>;

const limiterPeaks = (gauges: readonly GaugePeak[]): Limiter[] => {
  const byName = new Map<string, { inUse?: number; max?: number }>();
  for (const g of gauges) {
    const name = g.labels.limiter;
    if (!name) continue;
    const row = byName.get(name) ?? {};
    if (g.labels.state === "max") row.max = g.peak;
    else if (g.labels.state === "in_use") row.inUse = g.peak;
    byName.set(name, row);
  }
  const out: Limiter[] = [];
  for (const [name, row] of byName) {
    if (row.inUse === undefined || row.max === undefined || row.max <= 0) continue;
    out.push({ name, peakInUse: row.inUse, max: row.max, ratio: row.inUse / row.max });
  }
  return out;
};

type Bottleneck = Readonly<{ flow: string; stage: string; sharePct: number }>;

/** Prefer exact DB stage shares; fall back to relayer histogram ratios. */
const findBottleneck = (
  stages: readonly StageReport[] | undefined,
  stageDurations: readonly HistogramQuantiles[] | undefined,
  e2eDurations: readonly HistogramQuantiles[] | undefined,
): Bottleneck | undefined => {
  let best: Bottleneck | undefined;
  for (const stage of stages ?? []) {
    if (stage.shareOfE2ePct === undefined) continue;
    if (!best || stage.shareOfE2ePct > best.sharePct) {
      best = { flow: stage.flow, stage: stage.stage, sharePct: stage.shareOfE2ePct };
    }
  }
  if (best) return best;

  // Fallback: relayer-side histograms (e2e p50 per flow as the denominator).
  const e2eByFlow = new Map<string, number>();
  for (const e of e2eDurations ?? []) {
    if (e.labels.flow && e.p50 !== undefined) e2eByFlow.set(e.labels.flow, e.p50);
  }
  for (const s of stageDurations ?? []) {
    const flow = s.labels.flow;
    const stage = s.labels.stage;
    const e2e = flow ? e2eByFlow.get(flow) : undefined;
    if (!flow || !stage || !e2e || e2e <= 0 || s.p50 === undefined) continue;
    if (stage === "queue_wait") continue; // overlaps later stages
    const sharePct = Math.round((s.p50 / e2e) * 1000) / 10;
    if (!best || sharePct > best.sharePct) best = { flow, stage, sharePct };
  }
  return best;
};

const round1 = (value: number): number => Math.round(value * 10) / 10;
const mib = (bytes: number): number => Math.round((bytes / 1_048_576) * 10) / 10;

export const buildDiagnosis = (report: ReportCore): Diagnosis => {
  const flags: DiagnosisFlag[] = [];
  const recommendations: string[] = [];
  const allFlows = report.targets.flatMap((target) =>
    target.flows.map((flow) => ({ target: target.target, flow })),
  );
  const allStages = report.targets.flatMap((target) => target.stages ?? []);
  const stageDurations = report.targets.flatMap(
    (target) => target.metrics?.stageDurations ?? [],
  );
  const e2eDurations = report.targets.flatMap(
    (target) => target.metrics?.e2eDurations ?? [],
  );
  const limiterUtilization = report.targets.flatMap(
    (target) => target.metrics?.limiterUtilization ?? [],
  );
  const reclaims = report.targets.flatMap((target) => target.metrics?.reclaims ?? []);
  const httpNonSuccess = report.targets.reduce(
    (total, target) => total + (target.metrics?.http?.nonSuccess ?? 0),
    0,
  );
  const httpTotal = report.targets.reduce(
    (total, target) => total + (target.metrics?.http?.totalRequests ?? 0),
    0,
  );

  // --- Correctness and outcomes (client-side, always available). ----------
  const totalSubmitted = allFlows.reduce((t, entry) => t + entry.flow.submitted, 0);
  const totalErrors = allFlows.reduce((t, entry) => t + (entry.flow.submitted - entry.flow.succeeded), 0);
  const verifyFailed = allFlows.reduce((t, entry) => t + entry.flow.verifyFailed, 0);
  const errorRate = totalSubmitted === 0 ? 0 : totalErrors / totalSubmitted;
  const finitePlan = plannedRequestCount(report.run.scenario.shape);
  const submissionWasPartial = finitePlan === undefined || report.run.submitted < finitePlan;

  if (verifyFailed > 0) {
    flags.push({
      severity: "critical",
      message: `${verifyFailed.toString()} request(s) failed correctness verification — the relayer returned wrong plaintexts or invalid signatures.`,
    });
    recommendations.push(
      "STOP: do not treat this run as a baseline. Investigate the relayer/gateway — a fast wrong answer is worse than a slow right one.",
    );
  }
  if (errorRate > 0) {
    flags.push({
      severity: errorRate > 0.05 ? "critical" : "warn",
      message: `Error rate ${(errorRate * 100).toFixed(2)}% (${totalErrors.toString()}/${totalSubmitted.toString()}).`,
    });
  }
  if (submissionWasPartial && report.run.poolExhausted) {
    flags.push({
      severity: "warn",
      message: "A single-use pool ran dry mid-run; results cover only a partial run.",
    });
  }
  if (report.run.abandoned > 0) {
    flags.push({
      severity: "warn",
      message: `${report.run.abandoned.toString()} request(s) never reached a terminal state before the drain timeout.`,
    });
  }

  // --- Bottleneck stage. --------------------------------------------------
  const bottleneck = findBottleneck(
    allStages,
    stageDurations,
    e2eDurations,
  );
  if (bottleneck) {
    const downstream = DOWNSTREAM_STAGES.has(bottleneck.stage);
    flags.push({
      severity: "info",
      message: `Dominant stage: ${bottleneck.stage} (${bottleneck.sharePct.toString()}% of ${bottleneck.flow} e2e)${downstream ? " — downstream of the relayer (gateway/host chain)" : ""}.`,
    });
    if (downstream) {
      recommendations.push(
        `Latency is dominated by ${bottleneck.stage}, which is gateway/host-chain work — raising relayer broadcast limits will not help; profile the downstream dependency.`,
      );
    }
  }

  // --- Limiter saturation. ------------------------------------------------
  const limiters = limiterPeaks(limiterUtilization);
  const saturated = limiters.filter((l) => l.ratio >= 0.9);
  const backlogGrowing = report.targets.some((target) => target.queueDepth?.trend === "growing");
  for (const l of saturated) {
    flags.push({
      severity: backlogGrowing ? "warn" : "info",
      message: `Limiter ${l.name} peaked at ${l.peakInUse.toString()}/${l.max.toString()} (${Math.round(l.ratio * 100).toString()}% of cap).`,
    });
    const knob = LIMITER_CONFIG[l.name];
    if (backlogGrowing && knob) {
      recommendations.push(
        `${l.name} hit its cap while the backlog grew — if this stage is the bottleneck, raising ${knob} may increase throughput.`,
      );
    }
  }
  if (limiters.length > 0 && saturated.length === 0) {
    flags.push({
      severity: "ok",
      message: `No limiter saturated (peak utilization ${Math.round(Math.max(...limiters.map((l) => l.ratio)) * 100).toString()}% of cap) — throughput was not relayer-config-bound.`,
    });
  }

  // --- Reclaims (instability under load). ---------------------------------
  const reclaimTotal = reclaims.reduce((t, r) => t + r.delta, 0);
  if (reclaimTotal > 0) {
    flags.push({
      severity: "warn",
      message: `${reclaimTotal.toString()} reclaim(s) fired (stale readiness / wallet-lease churn) — a source of tail latency under stress.`,
    });
  }

  // --- HTTP-side errors / throttling. -------------------------------------
  if (httpNonSuccess > 0) {
    flags.push({
      severity: "warn",
      message: `${httpNonSuccess.toString()} non-2xx relayer responses out of ${httpTotal.toString()} (throttling or errors).`,
    });
  }

  // --- Soak: memory / FD drift. -------------------------------------------
  const proc = report.targets.find((target) => target.metrics?.process)?.metrics?.process;
  const longRun = (proc?.windowSec ?? 0) >= 600; // only trust drift over ≥10 min
  if (proc?.rss && longRun) {
    const growthMiBPerHour = mib(proc.rss.perHour);
    if (growthMiBPerHour > 50) {
      flags.push({
        severity: "warn",
        message: `RSS grew ${growthMiBPerHour.toString()} MiB/hour (now ${mib(proc.rss.last).toString()} MiB) — possible memory leak.`,
      });
      recommendations.push(
        "Investigate a memory leak: RSS trends upward over the soak rather than plateauing.",
      );
    } else {
      flags.push({
        severity: "ok",
        message: `RSS stable (${growthMiBPerHour.toString()} MiB/hour drift over ${(proc.windowSec! / 60).toFixed(0)} min).`,
      });
    }
  }
  if (proc?.openFds && longRun && proc.openFds.perHour > 5) {
    flags.push({
      severity: "warn",
      message: `Open FDs grew ${round1(proc.openFds.perHour).toString()}/hour (now ${proc.openFds.last.toString()}) — possible descriptor leak.`,
    });
  }

  // --- Saturation / capacity framing. -------------------------------------
  if (backlogGrowing) {
    const maxPending = Math.max(
      ...report.targets.map((target) => target.queueDepth?.maxPending ?? 0),
    );
    const endPending = Math.max(
      ...report.targets.map((target) => target.queueDepth?.endPending ?? 0),
    );
    flags.push({
      severity: "info",
      message: `Backlog grew through the run (peaked at ${maxPending.toString()}, ended at ${endPending.toString()}) — arrival rate exceeded sustainable throughput.`,
    });
  }
  if (submissionWasPartial && report.run.stoppedAtSegment !== undefined) {
    flags.push({
      severity: "info",
      message: `Ramp stopped at segment ${report.run.stoppedAtSegment.toString()} on the saturation signal — max sustainable rate is near the previous step.`,
    });
  }

  // --- One-line verdict. --------------------------------------------------
  let verdict: string;
  if (verifyFailed > 0) {
    verdict = `❌ FAILED — ${verifyFailed.toString()} correctness failure(s); do not bless this run.`;
  } else if (errorRate > 0.05) {
    verdict = `❌ Unhealthy — ${(errorRate * 100).toFixed(1)}% error rate.`;
  } else {
    const parts: string[] = [];
    parts.push(errorRate === 0 ? "0 errors" : `${(errorRate * 100).toFixed(2)}% errors`);
    if (bottleneck) parts.push(`bottleneck ${bottleneck.stage} (${bottleneck.sharePct.toString()}% of e2e)`);
    if (saturated.length > 0) parts.push(`${saturated.map((l) => l.name).join(", ")} at cap`);
    else if (limiters.length > 0) parts.push("no limiter saturated");
    if (backlogGrowing) parts.push("backlog growing (saturated)");
    else if (report.targets.some((target) => target.queueDepth)) parts.push("backlog drained");
    const ok = errorRate === 0 && reclaimTotal === 0;
    verdict = `${ok ? "✅ Healthy" : "⚠️ Completed with warnings"} — ${parts.join("; ")}.`;
  }

  return {
    verdict,
    bottleneckStage: bottleneck,
    saturatedLimiters: saturated.map((l) => `${l.name} (${l.peakInUse.toString()}/${l.max.toString()})`),
    flags,
    recommendations,
  };
};
