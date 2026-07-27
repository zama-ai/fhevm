import type { MetricFamily } from "./prom-parse";

export type PrometheusSignal =
  | "queueDepth" | "throttlerDepth" | "stageDuration" | "e2eDuration"
  | "terminalTotal" | "reclaims" | "limiterUtilization" | "dependencyDuration"
  | "httpRequests" | "process"
  | "v2InputProofInserted" | "v2InputProofDuration"
  | "v2TransactionCount" | "v2TransactionDuration" | "v2TransactionErrors"
  | "v2RecoveryRuns" | "v2RecoveryDuration" | "v2RecoveryItems"
  | "v2WalletLeaseOwned" | "v2WalletLeaseTransitions" | "v2DbErrors";

export type SignalCapability = Readonly<{ available: boolean; family?: string; reason?: string }>;
export type PrometheusCapabilities = Readonly<{
  profile: "legacy" | "v2" | "unknown";
  signals: Readonly<Record<PrometheusSignal, SignalCapability>>;
  discoveredFamilies: readonly string[];
}>;

type Mapping = Readonly<{ family: string; labels?: Readonly<Record<string, string>> }>;

const LEGACY_EVIDENCE = [
  "relayer_request_count",
  "relayer_request_status_duration_seconds",
  "relayer_queue_size_count",
] as const;
const V2_EVIDENCE = [
  "input_proof_requests_inserted_total",
  "input_proof_request_duration_seconds",
  "relayer_recovery_runs_total",
  "relayer_wallet_lease_transitions_total",
] as const;

const COMMON: Partial<Record<PrometheusSignal, Mapping>> = {
  httpRequests: { family: "relayer_http_responses_total" },
  process: { family: "process_" },
};
const LEGACY: Partial<Record<PrometheusSignal, Mapping>> = {
  queueDepth: { family: "relayer_request_count", labels: { req_type: "flow" } },
  throttlerDepth: { family: "relayer_queue_size_count", labels: { queue_type: "queue" } },
  stageDuration: {
    family: "relayer_request_status_duration_seconds",
    labels: { req_type: "flow", previous_status: "stage" },
  },
};
// These mappings deliberately remain implementation-specific. For example,
// a v2 transaction terminal outcome is not a relayer request terminal outcome,
// and recovery items are not the legacy queue/reclaim signal.
const V2: Partial<Record<PrometheusSignal, Mapping>> = {
  v2InputProofInserted: { family: "input_proof_requests_inserted_total" },
  v2InputProofDuration: { family: "input_proof_request_duration_seconds" },
  v2TransactionCount: { family: "relayer_transaction_count" },
  v2TransactionDuration: { family: "relayer_transaction_duration_seconds" },
  v2TransactionErrors: { family: "relayer_transaction_errors_total" },
  v2RecoveryRuns: { family: "relayer_recovery_runs_total" },
  v2RecoveryDuration: { family: "relayer_recovery_duration_seconds" },
  v2RecoveryItems: { family: "relayer_recovery_items_total" },
  v2WalletLeaseOwned: { family: "relayer_wallet_lease_owned" },
  v2WalletLeaseTransitions: { family: "relayer_wallet_lease_transitions_total" },
  v2DbErrors: { family: "relayer_db_errors_total" },
};

const SIGNALS: readonly PrometheusSignal[] = [
  "queueDepth", "throttlerDepth", "stageDuration", "e2eDuration", "terminalTotal", "reclaims",
  "limiterUtilization", "dependencyDuration", "httpRequests", "process",
  "v2InputProofInserted", "v2InputProofDuration",
  "v2TransactionCount", "v2TransactionDuration", "v2TransactionErrors",
  "v2RecoveryRuns", "v2RecoveryDuration", "v2RecoveryItems",
  "v2WalletLeaseOwned", "v2WalletLeaseTransitions", "v2DbErrors",
];

const hasFamily = (names: ReadonlySet<string>, family: string): boolean =>
  family.endsWith("_") ? [...names].some((name) => name.startsWith(family)) : names.has(family);

const mappingFor = (profile: PrometheusCapabilities["profile"], signal: PrometheusSignal): Mapping | undefined =>
  COMMON[signal] ?? (profile === "legacy" ? LEGACY[signal] : profile === "v2" ? V2[signal] : undefined);

export const discoverPrometheusCapabilities = (families: readonly MetricFamily[]): PrometheusCapabilities => {
  const names = new Set(families.map((family) => family.name));
  const legacyScore = LEGACY_EVIDENCE.filter((family) => names.has(family)).length;
  const v2Score = V2_EVIDENCE.filter((family) => names.has(family)).length;
  // Shared HTTP/transaction families are deliberately not profile evidence.
  const profile = legacyScore > v2Score ? "legacy" : v2Score > legacyScore ? "v2" : "unknown";
  const signals = Object.fromEntries(SIGNALS.map((signal) => {
    const mapping = mappingFor(profile, signal);
    const available = mapping !== undefined && hasFamily(names, mapping.family);
    return [signal, available
      ? { available: true, family: mapping.family }
      : { available: false, reason: mapping
          ? `Prometheus family ${mapping.family} was not present.`
          : `No recognized ${signal} family is exported by the ${profile} profile.` }];
  })) as Record<PrometheusSignal, SignalCapability>;
  return { profile, signals, discoveredFamilies: [...names].sort() };
};

export const mappedMetricFamily = (
  families: readonly MetricFamily[], capabilities: PrometheusCapabilities, signal: PrometheusSignal,
): MetricFamily | undefined => {
  const mapping = mappingFor(capabilities.profile, signal);
  if (!mapping || !capabilities.signals[signal].available || mapping.family.endsWith("_")) return undefined;
  const family = families.find((candidate) => candidate.name === mapping.family);
  if (!family) return undefined;
  return {
    ...family,
    metrics: family.metrics.map((metric) => ({
      ...metric,
      labels: metric.labels
        ? Object.fromEntries(Object.entries(metric.labels).map(([key, value]) => [mapping.labels?.[key] ?? key, value]))
        : undefined,
    })),
  };
};
