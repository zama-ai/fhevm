import { suiteSchema, type Suite } from "./schema";

/**
 * Built-in suites: opinionated scenario groupings so an operator runs one
 * command instead of choosing rates by hand. Durations are chosen so the
 * whole suite (including drain phases) fits the stated budget.
 */

const definitions: Record<string, unknown> = {
  /** ~3 min. Quick functional pass: is the deployment healthy at trivial load? */
  smoke: {
    name: "smoke",
    description: "open steady 1 req/s input-proof for 60s plus a tiny drain; deployment sanity in ~3 min",
    pauseSec: 10,
    entries: [
      { scenario: "open-steady", params: { rps: 1, durationSec: 60 }, label: "smoke-open-steady-1" },
      { scenario: "drain", params: { count: 20, rps: 20 }, label: "smoke-drain-20" },
    ],
  },

  /**
   * ~45 min. The nightly regression set: per-flow reference numbers, SLO
   * compliance at expected load, flow interference, and backlog correctness.
   */
  standard: {
    name: "standard",
    description: "baseline + open-steady-10 + open-mixed-10 + drain-500; the regression set (~45 min)",
    entries: [
      { scenario: "baseline" },
      { scenario: "open-steady", params: { rps: 10, durationSec: 600 } },
      { scenario: "open-mixed", params: { rps: 10, durationSec: 600 } },
      { scenario: "drain", params: { count: 500, rps: 100 } },
    ],
  },

  /**
   * Open-ended (ramp stops at saturation). Finds max sustainable throughput,
   * then checks recovery from a burst above it. Early stopping requires a
   * compatible queue-depth signal from the Prometheus collector.
   */
  capacity: {
    name: "capacity",
    description: "ramp to saturation + spike recovery; capacity discovery (up to ~40 min)",
    pauseSec: 120,
    entries: [
      { scenario: "open-ramp", params: { rps: 5, durationSec: 120 } },
      { scenario: "open-spike", params: { rps: 2, durationSec: 120 } },
    ],
  },

  /** ≥ 60 min. Leak and drift detection via the relayer process metrics. */
  endurance: {
    name: "endurance",
    description: "60 min open soak at 5 req/s; watches process metrics for leaks and drift",
    entries: [{ scenario: "open-soak", params: { rps: 5, durationSec: 3600 } }],
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
