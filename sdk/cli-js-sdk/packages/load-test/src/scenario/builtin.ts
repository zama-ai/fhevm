import type { FlowKind } from "../relayer/types";
import { scenarioSchema, type Scenario, type ScenarioInput } from "./schema";

/**
 * Built-in scenario matrix from the load-test design (§3.3). Each factory
 * accepts the knobs that vary between runs; everything else is fixed so two
 * runs of the same built-in are comparable.
 */

export type BuiltinParams = Readonly<{
  /** Target arrival rate in requests/second (steady, soak, mixed). */
  rps?: number;
  /** Active clients for closed-model scenarios. */
  vus?: number;
  /** Pause between completed workflows in closed-model scenarios. */
  thinkTimeMs?: number;
  /** Optional total request cap for closed-model scenarios. */
  maxIterations?: number;
  /** Submission-phase duration in seconds. */
  durationSec?: number;
  /** Flow selection for single-flow scenarios (steady, drain). */
  flow?: FlowKind;
  /** Request count for drain. */
  count?: number;
}>;

/**
 * Public-decrypt requests consume unique handle combinations (relayer dedup
 * is permanent), so built-ins request 2 handles per call: a pool of n
 * handles then serves C(n, 2) requests instead of n — ~40 on-chain setter
 * transactions cover a 750-request suite rather than 750.
 */
const PUBLIC_DECRYPT_HANDLES_PER_REQUEST = 2;

const allFlows = (weight: number): ScenarioInput["flows"] => [
  { flow: "input-proof", weight },
  {
    flow: "public-decrypt",
    weight,
    handlesPerRequest: PUBLIC_DECRYPT_HANDLES_PER_REQUEST,
  },
  { flow: "user-decrypt", weight },
  { flow: "delegated-user-decrypt", weight },
];

const factories: Record<string, (params: BuiltinParams) => ScenarioInput> = {
  /** Sanity + per-stage reference: 1 req/s per flow for 2 minutes. */
  baseline: (params) => ({
    name: "baseline",
    description: "Open model: 1 req/s per flow; sanity check and per-stage reference numbers",
    flows: allFlows(1),
    shape: {
      kind: "constant",
      rps: 4,
      durationSec: params.durationSec ?? 120,
    },
    thresholds: { maxErrorRate: 0, maxVerifyFailures: 0, perFlow: {} },
  }),

  /** SLO compliance at expected load: fixed N req/s for 10 minutes. */
  "open-steady": (params) => ({
    name: `open-steady-${(params.rps ?? 10).toString()}`,
    description: "Open model: fixed arrival rate; SLO compliance at expected load",
    flows: [{ flow: params.flow ?? "input-proof", weight: 1 }],
    shape: {
      kind: "constant",
      rps: params.rps ?? 10,
      durationSec: params.durationSec ?? 600,
    },
  }),

  /**
   * Max sustainable throughput: stepped rate; stops early when the queue-depth
   * collector sees sustained growth (requires the DB or metrics collector).
   */
  "open-ramp": (params) => {
    const startRps = params.rps ?? 5;
    const stepDurationSec = params.durationSec ?? 120;
    const steps = 8;
    return {
      name: "open-ramp",
      description: "Open model: stepped arrival rate until queue depth grows; finds max sustainable throughput",
      flows: [{ flow: params.flow ?? "input-proof", weight: 1 }],
      shape: {
        kind: "segments",
        segments: Array.from({ length: steps }, (_, index) => ({
          fromRps: startRps * (index + 1),
          toRps: startRps * (index + 1),
          durationSec: stepDurationSec,
        })),
      },
      saturationStop: { enabled: true, consecutiveSteps: 2, minQueueGrowth: 10 },
      // The point of a ramp is to exceed capacity; errors past saturation are data.
      thresholds: { maxErrorRate: 1, maxVerifyFailures: 0, perFlow: {} },
    };
  },

  /** Recovery behavior: baseline, 10x for 60s, baseline again. */
  "open-spike": (params) => {
    const baseRps = params.rps ?? 2;
    return {
      name: "open-spike",
      description: "Open model: baseline, 10x arrival spike for 60s, baseline; measures recovery",
      flows: [{ flow: params.flow ?? "input-proof", weight: 1 }],
      shape: {
        kind: "segments",
        segments: [
          { fromRps: baseRps, toRps: baseRps, durationSec: params.durationSec ?? 120 },
          { fromRps: baseRps * 10, toRps: baseRps * 10, durationSec: 60 },
          { fromRps: baseRps, toRps: baseRps, durationSec: params.durationSec ?? 120 },
        ],
      },
    };
  },

  /** Leaks and drift: moderate rate for at least an hour; read process metrics. */
  "open-soak": (params) => ({
    name: "open-soak",
    description: "Open model: moderate arrival rate for >= 60 min; watches for leaks and drift via process metrics",
    flows: [{ flow: params.flow ?? "input-proof", weight: 1 }],
    shape: {
      kind: "constant",
      rps: params.rps ?? 5,
      durationSec: params.durationSec ?? 3600,
    },
  }),

  /** Interference between flows at a realistic input:user:public ratio. */
  "open-mixed": (params) => ({
    name: "open-mixed",
    description: "Open model: input-proof:user-decrypt:public-decrypt mix; flow interference",
    flows: [
      { flow: "input-proof", weight: 6 },
      { flow: "user-decrypt", weight: 3 },
      {
        flow: "public-decrypt",
        weight: 1,
        handlesPerRequest: PUBLIC_DECRYPT_HANDLES_PER_REQUEST,
      },
    ],
    shape: {
      kind: "constant",
      rps: params.rps ?? 10,
      durationSec: params.durationSec ?? 600,
    },
  }),

  /** Fixed active client loops: request → terminal/timeout → optional think time → next. */
  "closed-steady": (params) => ({
    name: `closed-steady-${(params.vus ?? 10).toString()}vu`,
    description: "Closed model: fixed active clients; resulting throughput and latency are outputs",
    flows: [{ flow: params.flow ?? "user-decrypt", weight: 1 }],
    shape: {
      kind: "closed",
      vus: params.vus ?? 10,
      durationSec: params.durationSec ?? 600,
      thinkTimeMs: params.thinkTimeMs ?? 0,
      maxIterations: params.maxIterations,
    },
  }),

  /** Finds how many active clients remain within acceptable latency/error bounds. */
  "closed-ramp": (params) => {
    const startVus = params.vus ?? 5;
    const stepDurationSec = params.durationSec ?? 120;
    return {
      name: "closed-ramp",
      description: "Closed model: stepped active clients; finds acceptable client concurrency",
      flows: [{ flow: params.flow ?? "user-decrypt", weight: 1 }],
      shape: {
        kind: "closed",
        stages: Array.from({ length: 6 }, (_, index) => ({
          vus: startVus * (index + 1),
          durationSec: stepDurationSec,
        })),
        thinkTimeMs: params.thinkTimeMs ?? 0,
        maxIterations: params.maxIterations,
      },
    };
  },

  /** Long fixed-client run for SDK/client behavior, leaks, and drift. */
  "closed-soak": (params) => ({
    name: "closed-soak",
    description: "Closed model: active SDK clients for >= 60 min; watches latency, errors, leaks, and drift",
    flows: [{ flow: params.flow ?? "user-decrypt", weight: 1 }],
    shape: {
      kind: "closed",
      vus: params.vus ?? 10,
      durationSec: params.durationSec ?? 3600,
      thinkTimeMs: params.thinkTimeMs ?? 0,
      maxIterations: params.maxIterations,
    },
  }),

  /**
   * Backlog correctness: submit N near-instantly, then poll all to completion.
   * Validates the configured throttle; `ramp` measures capacity.
   */
  drain: (params) => ({
    name: "drain",
    description: "Drain model: submit N near-instantly, poll all to completion; validates configured drain rate",
    flows: [{ flow: params.flow ?? "input-proof", weight: 1 }],
    shape: {
      kind: "burst",
      count: params.count ?? 500,
      maxRps: params.rps ?? 100,
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
  return scenarioSchema.parse(factory(params));
};
