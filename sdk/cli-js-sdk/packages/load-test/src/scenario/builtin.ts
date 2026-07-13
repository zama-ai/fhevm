import { scenarioSchema, type Scenario, type ScenarioInput } from "./schema";
import {
  applyScenarioOverrides,
  scenarioOverrideSchema,
  type ScenarioOverrides,
} from "./overrides";

/**
 * Built-in scenario matrix from the load-test design (§3.3). Factories return
 * canonical defaults; the shared model-aware override resolver is applied
 * afterward so built-in and JSON scenarios follow the same rules.
 *
 * Defaults are deliberately gentle: the tool is used on light workloads first,
 * and every default peak stays well under the protocol ceilings (input-proof
 * ~20 rps; public + user + delegated decrypt combined ~10 rps). Scenarios that
 * probe the ceilings (`open-ramp`, `open-spike`, `drain`) approach but do not
 * casually exceed them. `ceilingWarnings` (see `limits.ts`) flags overrides
 * that push past the ceilings.
 */

export type BuiltinParams = ScenarioOverrides;

/**
 * Public-decrypt requests consume unique handle combinations (relayer dedup
 * is permanent), so built-ins request 2 handles per call: a pool of n handles
 * then serves C(n, 2) requests instead of n, keeping on-chain setter
 * transactions far below the number of decrypt requests they support.
 *
 * delegated-user-decrypt is deliberately excluded from every default flow mix
 * (it behaves like user-decrypt); it stays reachable via `--flow` overrides.
 */
const PUBLIC_DECRYPT_HANDLES_PER_REQUEST = 2;

/** Equal-weight input-proof / public-decrypt / user-decrypt mix. */
const defaultMix = (weight: number): ScenarioInput["flows"] => [
  { flow: "input-proof", weight },
  {
    flow: "public-decrypt",
    weight,
    handlesPerRequest: PUBLIC_DECRYPT_HANDLES_PER_REQUEST,
  },
  { flow: "user-decrypt", weight },
];

const factories: Record<string, () => ScenarioInput> = {
  /** Sanity + per-flow reference numbers: 1 req/s per flow for 60s. */
  baseline: () => ({
    name: "baseline",
    description:
      "Open model: 1 req/s per flow (ip/pd/ud) for 60s; sanity check and per-flow reference numbers",
    flows: defaultMix(1),
    shape: {
      kind: "constant",
      rps: 3,
      durationSec: 60,
    },
    thresholds: { maxErrorRate: 0, maxVerifyFailures: 0, perFlow: {} },
  }),

  /** Fast end-to-end correctness across all flows (~5 requests per flow). */
  smoke: () => ({
    name: "smoke",
    description:
      "Burst model: ~5 requests per flow (ip/pd/ud) capped at 3 rps; fast end-to-end correctness check",
    flows: defaultMix(1),
    shape: {
      kind: "burst",
      count: 15,
      maxRps: 3,
    },
    thresholds: { maxErrorRate: 0, maxVerifyFailures: 0, perFlow: {} },
  }),

  /** SLO compliance at expected load: fixed N req/s input-proof for 5 min. */
  "open-steady": () => ({
    name: "open-steady-5",
    description:
      "Open model: fixed input-proof arrival rate (5 rps, well under the 20 rps ceiling); SLO compliance",
    flows: [{ flow: "input-proof", weight: 1 }],
    shape: {
      kind: "constant",
      rps: 5,
      durationSec: 300,
    },
  }),

  /**
   * Max sustainable throughput: stepped input-proof rate up to the ~20 rps
   * ceiling; stops early when the queue-depth collector sees sustained growth
   * (requires compatible Prometheus metrics).
   */
  "open-ramp": () => {
    const stepDurationSec = 120;
    const rates = [4, 8, 12, 16, 20, 24];
    return {
      name: "open-ramp",
      description:
        "Open model: input-proof stepped 4→24 rps across the ~20 rps ceiling; finds max sustainable throughput",
      flows: [{ flow: "input-proof", weight: 1 }],
      shape: {
        kind: "segments",
        segments: rates.map((rps) => ({
          fromRps: rps,
          toRps: rps,
          durationSec: stepDurationSec,
        })),
      },
      saturationStop: { enabled: true, consecutiveSteps: 2, minQueueGrowth: 10 },
      // The point of a ramp is to reach capacity; errors past saturation are data.
      thresholds: { maxErrorRate: 1, maxVerifyFailures: 0, perFlow: {} },
    };
  },

  /** Recovery behavior: baseline, spike to the ~20 rps ceiling for 60s, baseline. */
  "open-spike": () => ({
    name: "open-spike",
    description:
      "Open model: input-proof 4 rps, spike to the 20 rps ceiling for 60s, back to 4 rps; measures recovery",
    flows: [{ flow: "input-proof", weight: 1 }],
    shape: {
      kind: "segments",
      segments: [
        { fromRps: 4, toRps: 4, durationSec: 120 },
        { fromRps: 20, toRps: 20, durationSec: 60 },
        { fromRps: 4, toRps: 4, durationSec: 120 },
      ],
    },
  }),

  /** Leaks and drift: gentle input-proof rate for an hour; read process metrics. */
  "open-soak": () => ({
    name: "open-soak",
    description:
      "Open model: gentle input-proof rate (3 rps) for 60 min; watches for leaks and drift via process metrics",
    flows: [{ flow: "input-proof", weight: 1 }],
    shape: {
      kind: "constant",
      rps: 3,
      durationSec: 3600,
    },
  }),

  /** Interference between flows: equal ip/ud/pd thirds at ~2 rps per flow. */
  "open-mixed": () => ({
    name: "open-mixed",
    description:
      "Open model: equal input-proof/user-decrypt/public-decrypt thirds at ~2 rps each (decrypt 4 rps combined); flow interference",
    flows: [
      { flow: "input-proof", weight: 2 },
      { flow: "user-decrypt", weight: 2 },
      {
        flow: "public-decrypt",
        weight: 2,
        handlesPerRequest: PUBLIC_DECRYPT_HANDLES_PER_REQUEST,
      },
    ],
    shape: {
      kind: "constant",
      rps: 6,
      durationSec: 300,
    },
  }),

  /** Fixed active client loops: request → terminal/timeout → optional think time → next. */
  "closed-steady": () => ({
    name: "closed-steady-5vu",
    description:
      "Closed model: 5 fixed user-decrypt clients for 5 min; resulting throughput and latency are outputs",
    flows: [{ flow: "user-decrypt", weight: 1 }],
    shape: {
      kind: "closed",
      vus: 5,
      durationSec: 300,
      thinkTimeMs: 0,
    },
  }),

  /** Finds how many active clients remain within acceptable latency/error bounds. */
  "closed-ramp": () => {
    const stepDurationSec = 120;
    const vuStages = [2, 4, 6, 8, 10];
    return {
      name: "closed-ramp",
      description:
        "Closed model: user-decrypt clients stepped 2→10; finds acceptable client concurrency",
      flows: [{ flow: "user-decrypt", weight: 1 }],
      shape: {
        kind: "closed",
        stages: vuStages.map((vus) => ({ vus, durationSec: stepDurationSec })),
        thinkTimeMs: 0,
      },
    };
  },

  /** Long fixed-client run for SDK/client behavior, leaks, and drift. */
  "closed-soak": () => ({
    name: "closed-soak",
    description:
      "Closed model: 5 active user-decrypt clients for 60 min; watches latency, errors, leaks, and drift",
    flows: [{ flow: "user-decrypt", weight: 1 }],
    shape: {
      kind: "closed",
      vus: 5,
      durationSec: 3600,
      thinkTimeMs: 0,
    },
  }),

  /**
   * Backlog correctness: submit N near-instantly (capped at the ~20 rps
   * ceiling), then poll all to completion. Validates the configured throttle;
   * `open-ramp` measures capacity.
   */
  drain: () => ({
    name: "drain",
    description:
      "Drain model: submit 200 input-proof requests capped at the 20 rps ceiling, poll all to completion; validates backlog handling",
    flows: [{ flow: "input-proof", weight: 1 }],
    shape: {
      kind: "burst",
      count: 200,
      maxRps: 20,
    },
    drainTimeoutSec: 3600,
    thresholds: { maxErrorRate: 0, maxVerifyFailures: 0, perFlow: {} },
  }),
};

export const BUILTIN_SCENARIOS = Object.keys(factories);

export const createBuiltinScenario = (
  name: string,
  params: BuiltinParams = {},
): Scenario => {
  const factory = factories[name];
  if (!factory) {
    throw new Error(
      `Unknown scenario "${name}". Built-ins: ${BUILTIN_SCENARIOS.join(", ")}.`,
    );
  }
  const overrides = scenarioOverrideSchema.parse(params);
  const resolved = applyScenarioOverrides(scenarioSchema.parse(factory()), overrides);
  if (name === "open-steady" && resolved.shape.kind === "constant") {
    return scenarioSchema.parse({
      ...resolved,
      name: `open-steady-${resolved.shape.rps.toString()}`,
    });
  }
  if (name === "closed-steady" && resolved.shape.kind === "closed") {
    return scenarioSchema.parse({
      ...resolved,
      name: `closed-steady-${resolved.shape.vus?.toString() ?? "staged"}vu`,
    });
  }
  return resolved;
};
