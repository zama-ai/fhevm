import { JsonlWriter } from "../shared/jsonl";
import { logger } from "../shared/logger";
import { epochNowMs } from "../shared/time";
import { parsePrometheusText, type MetricFamily } from "./prom-parse";
import {
  discoverPrometheusCapabilities,
  mappedMetricFamily,
  type PrometheusCapabilities,
} from "./prom-capabilities";
import type { Collector, QueueDepthSample, QueueDepthSource } from "./types";

export type { MetricFamily } from "./prom-parse";

export type MetricsSnapshot = Readonly<{
  tMs: number;
  families: readonly MetricFamily[];
  capabilities?: PrometheusCapabilities;
}>;

export type MetricsTextSource = () => Promise<string>;

export type PrometheusCollectionStatus = Readonly<{
  successfulScrapes: number;
  failedScrapes: number;
  /** Outcome and timestamp of the most recent attempt. */
  lastAttemptSucceeded?: boolean;
  lastAttemptAt?: string;
  lastSuccessAt?: string;
  lastFailureAt?: string;
  /** Most recent failure message, retained after a later successful scrape. */
  lastError?: string;
}>;

export type PrometheusHttpSourceOptions = Readonly<{
  timeoutMs?: number;
  maxResponseBytes?: number;
}>;

export const createPrometheusHttpSource = (
  endpoint: string,
  fetcher: typeof fetch = fetch,
  options: PrometheusHttpSourceOptions = {},
): MetricsTextSource => async () => {
  const timeoutMs = options.timeoutMs ?? 10_000;
  const maxResponseBytes = options.maxResponseBytes ?? 2 * 1024 * 1024;
  const response = await fetcher(endpoint, {
    headers: { accept: "text/plain; version=0.0.4, application/openmetrics-text; version=1.0.0" },
    signal: AbortSignal.timeout(timeoutMs),
  });
  if (!response.ok) throw new Error(`Prometheus scrape returned HTTP ${response.status.toString()}.`);
  const declaredLength = Number(response.headers.get("content-length"));
  if (Number.isFinite(declaredLength) && declaredLength > maxResponseBytes) {
    throw new Error(`Prometheus scrape exceeded the ${maxResponseBytes.toString()} byte response limit.`);
  }
  if (!response.body) return "";
  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let bytes = 0;
  let text = "";
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      bytes += value.byteLength;
      if (bytes > maxResponseBytes) {
        await reader.cancel();
        throw new Error(`Prometheus scrape exceeded the ${maxResponseBytes.toString()} byte response limit.`);
      }
      text += decoder.decode(value, { stream: true });
    }
    return text + decoder.decode();
  } finally {
    reader.releaseLock();
  }
};

const NON_TERMINAL_STATUSES = new Set([
  "queued",
  "acl_check_claimed",
  "ciphertext_ready",
  "claimed",
  "broadcasting",
  "awaiting_gateway_response",
  "processing",
  "tx_in_flight",
  "receipt_received",
]);

const retainForReport = (family: MetricFamily): boolean =>
  family.name.startsWith("process_") || [
    "relayer_request_count",
    "relayer_queue_size_count",
    "relayer_request_status_duration_seconds",
    "relayer_http_responses_total",
    "input_proof_requests_inserted_total",
    "input_proof_request_duration_seconds",
    "relayer_transaction_count",
    "relayer_transaction_duration_seconds",
    "relayer_transaction_errors_total",
    "relayer_recovery_runs_total",
    "relayer_recovery_duration_seconds",
    "relayer_recovery_items_total",
    "relayer_wallet_lease_owned",
    "relayer_wallet_lease_transitions_total",
    "relayer_db_errors_total",
  ].includes(family.name);

/**
 * Scrapes a Prometheus endpoint on an interval. Capability discovery sees the
 * full bounded response, while artifacts retain only report-consumed families
 * so an unrelated high-cardinality metric cannot inflate every JSONL sample.
 * Cumulative counters/histograms are stored as-is for run-window deltas.
 */
export class PrometheusCollector implements Collector, QueueDepthSource {
  readonly name = "prometheus";
  private timer: NodeJS.Timeout | undefined;
  private writer: JsonlWriter<MetricsSnapshot> | undefined;
  private readonly allSnapshots: MetricsSnapshot[] = [];
  private readonly depthSamples: QueueDepthSample[] = [];
  private scrapePromise: Promise<void> | undefined;
  private started = false;
  private successfulScrapes = 0;
  private failedScrapes = 0;
  private lastScrapeError: string | undefined;
  private lastAttemptSucceeded: boolean | undefined;
  private lastAttemptAt: string | undefined;
  private lastSuccessAt: string | undefined;
  private lastFailureAt: string | undefined;

  constructor(
    private readonly source: MetricsTextSource,
    private readonly outputPath: string,
    private readonly intervalMs = 5_000,
  ) {}

  get snapshots(): readonly MetricsSnapshot[] {
    return this.allSnapshots;
  }

  get samples(): readonly QueueDepthSample[] {
    return this.depthSamples;
  }

  get capabilities(): PrometheusCapabilities {
    const last = this.allSnapshots.at(-1);
    return last?.capabilities ?? discoverPrometheusCapabilities(last?.families ?? []);
  }

  get collectionStatus(): PrometheusCollectionStatus {
    return {
      successfulScrapes: this.successfulScrapes,
      failedScrapes: this.failedScrapes,
      lastAttemptSucceeded: this.lastAttemptSucceeded,
      lastAttemptAt: this.lastAttemptAt,
      lastSuccessAt: this.lastSuccessAt,
      lastFailureAt: this.lastFailureAt,
      lastError: this.lastScrapeError,
    };
  }

  async start(): Promise<void> {
    if (this.started) return;
    this.writer = await JsonlWriter.open<MetricsSnapshot>(this.outputPath);
    this.started = true;
    try {
      await this.scrape();
      this.timer = setInterval(() => { void this.scrape(); }, this.intervalMs);
      this.timer.unref();
    } catch (error) {
      this.started = false;
      await this.writer.close();
      this.writer = undefined;
      throw error;
    }
  }

  private async scrape(): Promise<void> {
    if (this.scrapePromise) return this.scrapePromise;
    this.scrapePromise = (async () => {
      const attemptedAt = new Date(epochNowMs()).toISOString();
      try {
        const text = await this.source();
        const parsedFamilies = parsePrometheusText(text);
        const capabilities = discoverPrometheusCapabilities(parsedFamilies);
        const families = parsedFamilies.filter(retainForReport);
        const snapshot: MetricsSnapshot = { tMs: epochNowMs(), families, capabilities };
        this.allSnapshots.push(snapshot);
        await this.writer?.write(snapshot);
        this.successfulScrapes += 1;
        this.lastAttemptSucceeded = true;
        this.lastAttemptAt = attemptedAt;
        this.lastSuccessAt = attemptedAt;
        this.recordQueueDepth(snapshot);
      } catch (error) {
        const message = (error as Error).message;
        this.failedScrapes += 1;
        this.lastAttemptSucceeded = false;
        this.lastAttemptAt = attemptedAt;
        this.lastFailureAt = attemptedAt;
        this.lastScrapeError = message;
        if (this.successfulScrapes === 0 && this.failedScrapes === 1) {
          logger.warn(`Prometheus collector unavailable: ${message}`);
        } else {
          logger.debug(`metrics scrape failed: ${message}`);
        }
      }
    })().finally(() => {
      this.scrapePromise = undefined;
    });
    return this.scrapePromise;
  }

  private recordQueueDepth(snapshot: MetricsSnapshot): void {
    const family = mappedMetricFamily(
      snapshot.families,
      snapshot.capabilities ?? discoverPrometheusCapabilities(snapshot.families),
      "queueDepth",
    );
    if (!family) return;
    const byFlowStatus: Record<string, number> = {};
    let pendingTotal = 0;
    for (const metric of family.metrics) {
      const flow = metric.labels?.flow ?? "unknown";
      const status = metric.labels?.status ?? "unknown";
      if (metric.value === undefined) continue;
      const value = Number(metric.value);
      if (!Number.isFinite(value) || value < 0) continue;
      const key = `${flow}/${status}`;
      byFlowStatus[key] = (byFlowStatus[key] ?? 0) + value;
      if (NON_TERMINAL_STATUSES.has(status)) pendingTotal += value;
    }
    this.depthSamples.push({ tMs: snapshot.tMs, byFlowStatus, pendingTotal });
  }

  async stop(): Promise<void> {
    if (!this.started) return;
    this.started = false;
    if (this.timer) clearInterval(this.timer);
    this.timer = undefined;
    await this.scrape();
    await this.writer?.close();
    this.writer = undefined;
  }
}
