import { suiteSchema, type Suite } from "./schema";

/**
 * Built-in suites: opinionated scenario groupings so an operator runs one
 * command instead of choosing rates by hand. Durations are chosen so the
 * whole suite (including drain phases) fits the stated budget.
 */

const definitions: Record<string, unknown> = {
  /** ~1 min. Fast end-to-end correctness across all flows. */
  smoke: {
    name: "smoke",
    description: "burst of ~5 requests per flow (ip/pd/ud); end-to-end correctness across all flows in ~1 min",
    pauseSec: 0,
    entries: [{ scenario: "smoke", label: "smoke" }],
  },

  /**
   * ~15 min. The nightly regression set: per-flow reference numbers, SLO
   * compliance at expected load, flow interference, and backlog correctness.
   */
  standard: {
    name: "standard",
    description: "baseline + open-steady-5 + open-mixed (6 rps) + drain-200; the regression set (~15 min)",
    pauseSec: 30,
    entries: [
      { scenario: "baseline", label: "baseline" },
      { scenario: "open-steady", params: { rps: 5, durationSec: 300 }, label: "open-steady" },
      { scenario: "open-mixed", params: { rps: 6, durationSec: 300 }, label: "open-mixed" },
      { scenario: "drain", params: { count: 200, rps: 20 }, label: "drain" },
    ],
  },

  /**
   * Open-ended (ramp stops at saturation). Finds max sustainable throughput
   * near the ~20 rps input-proof ceiling, then checks recovery from a spike to
   * it. Early stopping requires a compatible queue-depth signal from the
   * Prometheus collector.
   */
  capacity: {
    name: "capacity",
    description: "open-ramp to saturation + open-spike recovery; capacity discovery near the ~20 rps ceiling",
    pauseSec: 120,
    entries: [
      { scenario: "open-ramp" },
      { scenario: "open-spike" },
    ],
  },

  /** ≥ 60 min. Leak and drift detection via the relayer process metrics. */
  endurance: {
    name: "endurance",
    description: "60 min open soak at 3 req/s input-proof; watches process metrics for leaks and drift",
    pauseSec: 30,
    entries: [{ scenario: "open-soak", params: { rps: 3, durationSec: 3600 } }],
  },
};

export const BUILTIN_SUITES = Object.keys(definitions);

export const createBuiltinSuite = (name: string): Suite => {
  const definition = definitions[name];
  if (!definition) {
    throw new Error(`Unknown suite "${name}". Built-ins: ${BUILTIN_SUITES.join(", ")}.`);
  }
  return suiteSchema.parse(definition);
};
