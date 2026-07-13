import { poolDir, type LoadTestEnv } from "../env";
import { binomial } from "./combinations";
import { HANDLE_POOLS } from "./handles";
import { INPUT_PROOF_POOL } from "./input-proof";
import { PoolStore } from "./store";
import type { FlowKind } from "../relayer/types";
import { plannedFlowAllocations } from "../scenario/allocation";
import { shapeDurationSec, type Scenario } from "../scenario/schema";

/**
 * Pool requirements: derives what pools must hold for one or more scenarios,
 * compares those requirements with live storage, and reports deficits.
 *
 * Uses the same exact smooth-weighted allocation as executor preflight.
 */

/** Reusable decrypt pools get a small handle variety even at low need. */
const MIN_REUSABLE_HANDLES = 4;

export type FlowNeed = Readonly<{
  flow: FlowKind;
  /** Finite budget, or an explicitly duration-bound reusable workload. */
  workload: Readonly<
    | { mode: "finite"; requestBudget: number }
    | { mode: "duration-bound" }
  >;
  /** public-decrypt only: combination sizes requested, with per-k needs. */
  byHandleCount: ReadonlyMap<number, number>;
}>;

export const computeFlowNeeds = (scenarios: readonly Scenario[]): FlowNeed[] => {
  const requests = new Map<FlowKind, { finite: number; durationBound: boolean }>();
  const byK = new Map<FlowKind, Map<number, number>>();
  for (const scenario of scenarios) {
    for (const allocation of plannedFlowAllocations(scenario)) {
      if (
        allocation.requests === undefined &&
        (allocation.flow === "input-proof" || allocation.flow === "public-decrypt")
      ) {
        throw new Error(
          `Scenario "${scenario.name}" uses closed ${allocation.flow} without maxIterations; ` +
            "single-use pools require a finite request cap.",
        );
      }
      if (allocation.requests === undefined) {
        const existing = requests.get(allocation.flow) ?? { finite: 0, durationBound: false };
        requests.set(allocation.flow, { ...existing, durationBound: true });
        continue;
      }
      const existing = requests.get(allocation.flow) ?? { finite: 0, durationBound: false };
      requests.set(allocation.flow, {
        ...existing,
        finite: existing.finite + allocation.requests,
      });
      const perK = byK.get(allocation.flow) ?? new Map<number, number>();
      perK.set(
        allocation.handlesPerRequest,
        (perK.get(allocation.handlesPerRequest) ?? 0) + allocation.requests,
      );
      byK.set(allocation.flow, perK);
    }
  }
  return [...requests.entries()].map(([flow, count]) => ({
    flow,
    workload: count.durationBound
      ? { mode: "duration-bound" as const }
      : { mode: "finite" as const, requestBudget: count.finite },
    byHandleCount: byK.get(flow) ?? new Map(),
  }));
};

export type PoolDeficit = Readonly<{
  pool: string;
  flow: FlowKind;
  /** Items currently in the pool. */
  current: number;
  /** Items still unconsumed (after cursors). */
  /** Scalar availability when pool items map directly to usable work. */
  available?: number;
  /** Public-decrypt availability is a per-combination-size space, not a scalar. */
  combinationCapacity?: readonly Readonly<{
    handlesPerRequest: number;
    totalCombinations: string;
    consumedCombinations: string;
    availableCombinations: string;
    neededRequests: number;
  }>[];
  /** Finite request budget, or an explicitly duration-bound reusable workload. */
  workload: FlowNeed["workload"];
  /** Reusable handle variety target; not a synthetic request count. */
  targetItems?: number;
  /** Items to generate/create to cover the need. */
  deficit: number;
  /** Delegated ACL state needs repair even when no new handles are needed. */
  refreshRequired?: boolean;
  /** Earliest unix timestamp every delegated owner must remain valid through. */
  requiredValidUntil?: string;
  /** Human reason, e.g. the binding combination constraint. */
  detail: string;
}>;

const inputProofDeficit = async (
  env: LoadTestEnv,
  needed: number,
): Promise<PoolDeficit> => {
  const store = await PoolStore.openIfExists(poolDir(env, INPUT_PROOF_POOL));
  const current = store?.meta.count ?? 0;
  const consumed = store ? Number(store.cursor("submit").position) : 0;
  const available = Math.max(0, current - consumed);
  return {
    pool: INPUT_PROOF_POOL,
    flow: "input-proof",
    current,
    available,
    workload: { mode: "finite", requestBudget: needed },
    deficit: Math.max(0, needed - available),
    detail: "single-use payloads; one per request",
  };
};

/**
 * Smallest handle-pool size n >= current such that every requested
 * combination size k still has `needed_k` unconsumed ranks. Growing n is
 * safe: colex unranking keeps already-consumed ranks pointing at the same
 * combinations.
 */
export const minHandleCountForCombos = (
  current: number,
  perK: ReadonlyMap<number, { needed: number; consumedRanks: bigint }>,
): number => {
  let n = current;
  const satisfies = (candidate: number): boolean =>
    [...perK.entries()].every(
      ([k, { needed, consumedRanks }]) =>
        binomial(candidate, k) - consumedRanks >= BigInt(needed),
    );
  while (!satisfies(n)) n += 1;
  return n;
};

const publicDecryptDeficit = async (
  env: LoadTestEnv,
  need: FlowNeed,
): Promise<PoolDeficit> => {
  const store = await PoolStore.openIfExists(poolDir(env, HANDLE_POOLS["public-decrypt"]));
  const current = store?.meta.count ?? 0;
  const perK = new Map<number, { needed: number; consumedRanks: bigint }>();
  for (const [k, needed] of need.byHandleCount) {
    const consumedRanks = store ? store.cursor(`combos-k${k.toString()}`).position : 0n;
    perK.set(k, { needed, consumedRanks });
  }
  const target = minHandleCountForCombos(current, perK);
  const ks = [...need.byHandleCount.keys()].sort((a, b) => a - b);
  const combinationCapacity = ks.map((k) => {
    const consumed = perK.get(k)?.consumedRanks ?? 0n;
    const total = binomial(current, k);
    const available = total > consumed ? total - consumed : 0n;
    return {
      handlesPerRequest: k,
      totalCombinations: total.toString(),
      consumedCombinations: consumed.toString(),
      availableCombinations: available.toString(),
      neededRequests: need.byHandleCount.get(k) ?? 0,
    };
  });
  const available = combinationCapacity
    .map((capacity) =>
      `C(n,${capacity.handlesPerRequest.toString()})-used=${capacity.availableCombinations}`,
    )
    .join(", ");
  return {
    pool: HANDLE_POOLS["public-decrypt"],
    flow: "public-decrypt",
    current,
    combinationCapacity,
    workload: need.workload,
    deficit: Math.max(0, target - current),
    detail: `unique handle combinations consumed per request; ${available}; target n=${target.toString()}`,
  };
};

const reusableHandleDeficit = async (
  env: LoadTestEnv,
  flow: "user-decrypt" | "delegated-user-decrypt",
  workload: FlowNeed["workload"],
  requiredValidUntil: bigint,
): Promise<PoolDeficit> => {
  const store = await PoolStore.openIfExists(poolDir(env, HANDLE_POOLS[flow]));
  const current = store?.meta.count ?? 0;
  const target = workload.mode === "duration-bound"
    ? MIN_REUSABLE_HANDLES
    : Math.min(MIN_REUSABLE_HANDLES, Math.max(1, workload.requestBudget));
  const recordedExpirations = store?.meta.delegationExpirations;
  const ownerIndices = store?.meta.ownerIndices ?? [];
  const hasEveryOwnerExpiration =
    flow !== "delegated-user-decrypt" ||
    (recordedExpirations !== undefined &&
      ownerIndices.every((index) => recordedExpirations[index.toString()] !== undefined));
  const earliestRecorded = recordedExpirations
    ? ownerIndices
        .map((index) => recordedExpirations[index.toString()])
        .filter((value): value is string => value !== undefined)
        .map(BigInt)
        .reduce<bigint | undefined>(
          (earliest, value) => earliest === undefined || value < earliest ? value : earliest,
          undefined,
        )
    : undefined;
  const refreshRequired = flow === "delegated-user-decrypt" && current > 0 &&
    (!hasEveryOwnerExpiration || earliestRecorded === undefined || earliestRecorded < requiredValidUntil);
  return {
    pool: HANDLE_POOLS[flow],
    flow,
    current,
    available: current,
    workload,
    targetItems: target,
    deficit: Math.max(0, target - current),
    ...(flow === "delegated-user-decrypt"
      ? {
          refreshRequired,
          requiredValidUntil: requiredValidUntil.toString(),
        }
      : {}),
    detail: refreshRequired
      ? `handles are reusable; ACL delegations need refresh through ${requiredValidUntil.toString()}`
      : "handles are reusable (dedup includes the per-request transport key)",
  };
};

export const requiredDelegationValidUntil = (
  scenarios: readonly Scenario[],
  options: Readonly<{ pauseSec?: number; nowSeconds?: bigint }> = {},
): bigint => {
  const runSeconds = scenarios.reduce((total, scenario) => {
    const submissionSeconds = shapeDurationSec(scenario.shape) ??
      (scenario.shape.kind === "burst" && scenario.shape.maxRps
        ? scenario.shape.count / scenario.shape.maxRps
        : 0);
    return total + submissionSeconds + Math.max(
      scenario.drainTimeoutSec,
      scenario.requestTimeoutSec,
    );
  }, 0);
  const pauses = Math.max(0, scenarios.length - 1) * (options.pauseSec ?? 0);
  const marginSeconds = 120;
  const now = options.nowSeconds ?? BigInt(Math.floor(Date.now() / 1000));
  return now + BigInt(Math.ceil(runSeconds + pauses + marginSeconds));
};

/** Full pool plan for a list of scenarios; deficits of 0 mean ready to run. */
export const planPools = async (
  env: LoadTestEnv,
  scenarios: readonly Scenario[],
  options: Readonly<{ pauseSec?: number; nowSeconds?: bigint }> = {},
): Promise<PoolDeficit[]> => {
  const needs = computeFlowNeeds(scenarios);
  const delegatedValidUntil = requiredDelegationValidUntil(scenarios, options);
  const plan: PoolDeficit[] = [];
  for (const need of needs) {
    switch (need.flow) {
      case "input-proof":
        if (need.workload.mode !== "finite") throw new Error("input-proof requires a finite budget");
        plan.push(await inputProofDeficit(env, need.workload.requestBudget));
        break;
      case "public-decrypt":
        if (need.workload.mode !== "finite") throw new Error("public-decrypt requires a finite budget");
        plan.push(await publicDecryptDeficit(env, need));
        break;
      case "user-decrypt":
      case "delegated-user-decrypt":
        plan.push(await reusableHandleDeficit(
          env,
          need.flow,
          need.workload,
          delegatedValidUntil,
        ));
        break;
    }
  }
  return plan;
};
