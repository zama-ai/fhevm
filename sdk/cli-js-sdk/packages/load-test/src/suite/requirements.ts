import { poolDir, type LoadTestEnv } from "../env";
import { binomial } from "../pool/combinations";
import { HANDLE_POOLS } from "../pool/handles";
import { INPUT_PROOF_POOL } from "../pool/input-proof";
import { PoolStore } from "../pool/store";
import type { FlowKind } from "../relayer/types";
import { plannedFlowAllocations } from "../scenario/allocation";
import { shapeDurationSec, type Scenario } from "../scenario/schema";

/**
 * Pool planning: derives what each pool must hold to serve a list of
 * scenarios, compares against what is on disk (item counts minus consumed
 * cursors), and reports the deficit to prepare.
 *
 * Uses the same exact smooth-weighted allocation as executor preflight.
 */

/** Reusable decrypt pools get a small handle variety even at low need. */
const MIN_REUSABLE_HANDLES = 4;

export type FlowNeed = Readonly<{
  flow: FlowKind;
  /** Requests this flow must serve across the whole suite. */
  requests: number;
  /** public-decrypt only: combination sizes requested, with per-k needs. */
  byHandleCount: ReadonlyMap<number, number>;
}>;

export const computeFlowNeeds = (scenarios: readonly Scenario[]): FlowNeed[] => {
  const requests = new Map<FlowKind, number>();
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
        requests.set(allocation.flow, requests.get(allocation.flow) ?? 1);
        continue;
      }
      requests.set(
        allocation.flow,
        (requests.get(allocation.flow) ?? 0) + allocation.requests,
      );
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
    requests: count,
    byHandleCount: byK.get(flow) ?? new Map(),
  }));
};

export type PoolDeficit = Readonly<{
  pool: string;
  flow: FlowKind;
  /** Items currently in the pool. */
  current: number;
  /** Items still unconsumed (after cursors). */
  available: number;
  /** Requests the suite will send through this pool. */
  needed: number;
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
    needed,
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
  const available = ks
    .map((k) => {
      const consumed = perK.get(k)?.consumedRanks ?? 0n;
      const left = binomial(current, k) - consumed;
      return `C(n,${k.toString()})-used=${left.toString()}`;
    })
    .join(", ");
  return {
    pool: HANDLE_POOLS["public-decrypt"],
    flow: "public-decrypt",
    current,
    available: Number.MAX_SAFE_INTEGER, // combination space; see detail
    needed: need.requests,
    deficit: Math.max(0, target - current),
    detail: `unique handle combinations consumed per request; ${available}; target n=${target.toString()}`,
  };
};

const reusableHandleDeficit = async (
  env: LoadTestEnv,
  flow: "user-decrypt" | "delegated-user-decrypt",
  needed: number,
  requiredValidUntil: bigint,
): Promise<PoolDeficit> => {
  const store = await PoolStore.openIfExists(poolDir(env, HANDLE_POOLS[flow]));
  const current = store?.meta.count ?? 0;
  const target = Math.min(MIN_REUSABLE_HANDLES, Math.max(1, needed));
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
    needed,
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
        plan.push(await inputProofDeficit(env, need.requests));
        break;
      case "public-decrypt":
        plan.push(await publicDecryptDeficit(env, need));
        break;
      case "user-decrypt":
      case "delegated-user-decrypt":
        plan.push(await reusableHandleDeficit(
          env,
          need.flow,
          need.requests,
          delegatedValidUntil,
        ));
        break;
    }
  }
  return plan;
};
