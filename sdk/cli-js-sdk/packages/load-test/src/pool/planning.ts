import { createHash, randomUUID } from "node:crypto";
import { mkdir, open, rename, rm } from "node:fs/promises";
import { join } from "node:path";

import { resolveNetworkConfig } from "@cli-fhevm-sdk/toolkit";

import type { LoadTestEnv } from "../env";
import type { FlowKind } from "../relayer/types";
import type { Scenario } from "../scenario/schema";
import { safeArtifactText } from "../shared/safe-artifact";
import { createHandlePool, refreshDelegatedHandlePool, type HandlePoolFlow } from "./handles";
import { generateInputProofPool } from "./input-proof";
import {
  planPools,
  requiredDelegationValidUntil,
  type PoolDeficit,
} from "./requirements";

export * from "./requirements";

export const POOL_PLAN_ARTIFACT_VERSION = 2 as const;
export const POOL_PREPARATION_ARTIFACT_VERSION = 2 as const;

export type PoolEnvironmentIdentity = Readonly<{
  network: string;
  contractChainId: number;
  contractAddress: string;
  relayer: string;
  relayerApiPrefix?: string;
  relayerB?: string;
  relayerBApiPrefix?: string;
}>;

export type PoolPlanItem = Readonly<{
  pool: string;
  flow: FlowKind;
  requirement: Readonly<{
    workload: Readonly<
      | { mode: "finite"; requestBudget: number }
      | { mode: "duration-bound" }
    >;
    targetItems?: number;
    requiredValidUntil?: string;
  }>;
  observation: Readonly<{
    currentItems: number;
    availableItems?: number;
    combinationCapacity?: readonly Readonly<{
      handlesPerRequest: number;
      totalCombinations: string;
      consumedCombinations: string;
      availableCombinations: string;
      neededRequests: number;
    }>[];
  }>;
  decision: Readonly<{
    deficitItems: number;
    refreshRequired: boolean;
    ready: boolean;
    detail: string;
  }>;
}>;

export type PlannedPoolAction = Readonly<{
  kind: "generate-input-proof" | "create-handles" | "refresh-delegation-acl";
  pool: string;
  flow: FlowKind;
  items?: number;
  requiredValidUntil?: string;
}>;

export type PoolPlanArtifact = Readonly<{
  version: typeof POOL_PLAN_ARTIFACT_VERSION;
  kind: "load-test-pool-plan";
  observedAt: string;
  scenarioDigest: string;
  scenarios: readonly string[];
  pauseSec: number;
  environment: PoolEnvironmentIdentity;
  requiredDelegationValidUntil: string;
  ready: boolean;
  items: readonly PoolPlanItem[];
  plannedActions: readonly PlannedPoolAction[];
}>;

export type PoolPreparationAction = PlannedPoolAction & Readonly<{
  startedAt: string;
  endedAt: string;
  status: "completed" | "interrupted" | "failed";
  actualItems?: number;
  error?: string;
}>;

export type PoolPreparationArtifact = Readonly<{
  version: typeof POOL_PREPARATION_ARTIFACT_VERSION;
  kind: "load-test-pool-preparation";
  startedAt: string;
  endedAt: string;
  status: "completed" | "interrupted" | "failed";
  scenarioDigest: string;
  scenarios: readonly string[];
  pauseSec: number;
  environment: PoolEnvironmentIdentity;
  requested: Readonly<{ lanes?: number }>;
  initialReady: boolean;
  actions: readonly PoolPreparationAction[];
  finalInspection: Readonly<{
    status: "completed" | "failed";
    observedAt?: string;
    error?: string;
  }>;
  finalReady?: boolean;
  finalItems?: readonly PoolPlanItem[];
  error?: string;
}>;

type PlanPools = typeof planPools;

export type PoolPlanningOperations = Readonly<{
  inspect: PlanPools;
  generateInputProof: typeof generateInputProofPool;
  createHandles: typeof createHandlePool;
  refreshDelegationAcl: typeof refreshDelegatedHandlePool;
  now: () => Date;
}>;

const defaultOperations: PoolPlanningOperations = {
  inspect: planPools,
  generateInputProof: generateInputProofPool,
  createHandles: createHandlePool,
  refreshDelegationAcl: refreshDelegatedHandlePool,
  now: () => new Date(),
};

export type InspectPoolRequirementsOptions = Readonly<{
  env: LoadTestEnv;
  scenarios: readonly Scenario[];
  pauseSec?: number;
  /** When present, writes durable `pool-plan.json` and `pool-plan.md` evidence. */
  artifactDir?: string;
  operations?: Partial<PoolPlanningOperations>;
}>;

export type PreparePoolRequirementsOptions = InspectPoolRequirementsOptions & Readonly<{
  /** Preparation is explicit: calling this API authorizes the planned writes. */
  artifactDir: string;
  lanes?: number;
  signal?: AbortSignal;
  onProgress?: (message: string) => void;
  /**
   * Mutation gate evaluated against the authoritative fresh plan. It runs
   * exactly once, immediately before the first planned action, and is skipped
   * for a ready/no-op plan.
   */
  beforeActions?: (plan: PoolPlanArtifact) => Promise<void>;
}>;

const operationsFor = (
  overrides: Partial<PoolPlanningOperations> | undefined,
): PoolPlanningOperations => ({ ...defaultOperations, ...overrides });

const stableValue = (value: unknown): unknown => {
  if (Array.isArray(value)) return value.map(stableValue);
  if (typeof value !== "object" || value === null) return value;
  return Object.fromEntries(
    Object.entries(value)
      .sort(([left], [right]) => left.localeCompare(right))
      .map(([key, child]) => [key, stableValue(child)]),
  );
};

const scenarioDigest = (scenarios: readonly Scenario[], pauseSec: number): string =>
  createHash("sha256")
    .update(JSON.stringify(stableValue({ pauseSec, scenarios })))
    .digest("hex");

const endpointIdentity = (value: string): string => {
  const url = new URL(value);
  const pathname = url.pathname === "/" ? "" : url.pathname.replace(/\/+$/, "");
  // Preserve a custom base path because it may identify a distinct deployment,
  // while stripping userinfo, query credentials, and fragments.
  return `${url.origin}${pathname}`;
};

const routeIdentity = (value: string): string =>
  new URL(value, "https://route.invalid").pathname;

export const poolEnvironmentIdentity = (env: LoadTestEnv): PoolEnvironmentIdentity => ({
  network: env.network,
  contractChainId: env.contractChainId,
  contractAddress: (
    env.contractAddress ?? resolveNetworkConfig(env.network).fheTestAddress
  ).toLowerCase(),
  relayer: endpointIdentity(env.relayerUrl),
  ...(env.relayerApiPrefix
    ? { relayerApiPrefix: routeIdentity(env.relayerApiPrefix) }
    : {}),
  ...(env.relayerBUrl ? { relayerB: endpointIdentity(env.relayerBUrl) } : {}),
  ...(env.relayerBApiPrefix
    ? { relayerBApiPrefix: routeIdentity(env.relayerBApiPrefix) }
    : {}),
});

const toPlanItem = (item: PoolDeficit): PoolPlanItem => ({
  pool: item.pool,
  flow: item.flow,
  requirement: {
    workload: item.workload,
    ...(item.targetItems !== undefined ? { targetItems: item.targetItems } : {}),
    ...(item.requiredValidUntil
      ? { requiredValidUntil: item.requiredValidUntil }
      : {}),
  },
  observation: {
    currentItems: item.current,
    ...(item.available !== undefined ? { availableItems: item.available } : {}),
    ...(item.combinationCapacity
      ? { combinationCapacity: item.combinationCapacity }
      : {}),
  },
  decision: {
    deficitItems: item.deficit,
    refreshRequired: item.refreshRequired ?? false,
    ready: item.deficit === 0 && !(item.refreshRequired ?? false),
    detail: item.detail,
  },
});

const plannedActions = (items: readonly PoolPlanItem[]): PlannedPoolAction[] =>
  items.flatMap((item) => {
    const actions: PlannedPoolAction[] = [];
    if (item.decision.deficitItems > 0) {
      actions.push({
        kind: item.flow === "input-proof" ? "generate-input-proof" : "create-handles",
        pool: item.pool,
        flow: item.flow,
        items: item.decision.deficitItems,
      });
    }
    if (
      item.flow === "delegated-user-decrypt" &&
      (item.decision.deficitItems > 0 || item.decision.refreshRequired) &&
      item.requirement.requiredValidUntil
    ) {
      actions.push({
        kind: "refresh-delegation-acl",
        pool: item.pool,
        flow: item.flow,
        requiredValidUntil: item.requirement.requiredValidUntil,
      });
    }
    return actions;
  });

const buildPlan = async (
  options: InspectPoolRequirementsOptions,
  operations: PoolPlanningOperations,
): Promise<PoolPlanArtifact> => {
  const pauseSec = options.pauseSec ?? 0;
  const observed = operations.now();
  const nowSeconds = BigInt(Math.floor(observed.getTime() / 1000));
  const deficits = await operations.inspect(options.env, options.scenarios, {
    pauseSec,
    nowSeconds,
  });
  const items = deficits.map(toPlanItem);
  return {
    version: POOL_PLAN_ARTIFACT_VERSION,
    kind: "load-test-pool-plan",
    observedAt: observed.toISOString(),
    scenarioDigest: scenarioDigest(options.scenarios, pauseSec),
    scenarios: options.scenarios.map((scenario) => scenario.name),
    pauseSec,
    environment: poolEnvironmentIdentity(options.env),
    requiredDelegationValidUntil: requiredDelegationValidUntil(options.scenarios, {
      pauseSec,
      nowSeconds,
    }).toString(),
    ready: items.every((item) => item.decision.ready),
    items,
    plannedActions: plannedActions(items),
  };
};

const jsonText = (value: unknown): string => `${JSON.stringify(value, null, 2)}\n`;

/** Flushes a complete sibling file before atomically publishing its name. */
const writeDurableFile = async (
  directory: string,
  name: string,
  contents: string,
): Promise<void> => {
  await mkdir(directory, { recursive: true });
  const target = join(directory, name);
  const temporary = join(directory, `.${name}.${process.pid.toString()}.${randomUUID()}.tmp`);
  try {
    const handle = await open(temporary, "wx", 0o600);
    try {
      await handle.writeFile(contents, "utf8");
      await handle.sync();
    } finally {
      await handle.close();
    }
    await rename(temporary, target);
    try {
      const directoryHandle = await open(directory, "r");
      try {
        await directoryHandle.sync();
      } finally {
        await directoryHandle.close();
      }
    } catch {
      // Some platforms/filesystems reject directory fsync; file+rename still
      // provides an atomic complete artifact there.
    }
  } finally {
    await rm(temporary, { force: true });
  }
};

const inventorySummary = (item: PoolPlanItem): string => {
  const available = item.observation.availableItems === undefined
    ? "availability is expressed by combination capacity"
    : `${item.observation.availableItems.toString()} available`;
  const combinations = item.observation.combinationCapacity?.map((capacity) =>
    `k=${capacity.handlesPerRequest.toString()}: ${capacity.availableCombinations} available combination(s) ` +
      `for ${capacity.neededRequests.toString()} required request(s)`,
  ).join("; ");
  return `${item.observation.currentItems.toString()} inventory item(s), ${available}` +
    (combinations ? ` (${combinations})` : "");
};

const actionCostSummary = (action: PlannedPoolAction): string => {
  if (action.kind === "generate-input-proof") {
    return `[LOCAL CPU] generate ${(action.items ?? 0).toString()} input-proof payload(s) ` +
      `for ${action.pool}; 0 funded transactions`;
  }
  if (action.kind === "create-handles") {
    const items = action.items ?? 0;
    const delegation = action.flow === "delegated-user-decrypt"
      ? "; delegated ACL setup may add one funded transaction per owner lane that needs it"
      : "";
    return `[ON-CHAIN] create ${items.toString()} handle(s) for ${action.pool}; ` +
      `${items.toString()} funded setter transaction(s), one per handle${delegation}`;
  }
  return `[ON-CHAIN] refresh delegation ACL for ${action.pool}; funded transaction count is ` +
    "runtime-dependent, up to one per existing owner lane that needs renewal";
};

/** Stable human-facing plan summary shared by CLI output and Markdown evidence. */
export const formatPoolPlan = (plan: PoolPlanArtifact): readonly string[] => {
  const lines = [
    `Pool plan: ${plan.ready ? "ready; no preparation actions" : "preparation required"}.`,
  ];
  for (const item of plan.items) {
    const workload = item.requirement.workload.mode === "finite"
      ? `finite budget ${item.requirement.workload.requestBudget.toString()} request(s)`
      : "duration-bound workload";
    const target = item.requirement.targetItems === undefined
      ? ""
      : `; reusable pool target ${item.requirement.targetItems.toString()} handle(s)`;
    lines.push(
      `Pool ${item.pool}: ${workload}${target}; ${inventorySummary(item)}; ` +
        `deficit ${item.decision.deficitItems.toString()} creation unit(s); ` +
        `${item.decision.refreshRequired ? "ACL refresh required; " : ""}` +
        `status ${item.decision.ready ? "ready" : "action required"}. ${item.decision.detail}`,
    );
  }
  if (plan.plannedActions.length === 0) lines.push("Planned actions: none.");
  else {
    for (const action of plan.plannedActions) {
      lines.push(`Planned action: ${actionCostSummary(action)}.`);
    }
  }
  return lines;
};

const planMarkdown = (plan: PoolPlanArtifact): string => {
  const lines = [
    "# Pool Plan",
    "",
    `- **Observed:** ${plan.observedAt}`,
    `- **Scenarios:** ${plan.scenarios.join(" → ")}`,
    `- **Scenario digest:** \`${plan.scenarioDigest}\``,
    `- **Environment:** ${plan.environment.network} / chain ${plan.environment.contractChainId.toString()} / ${plan.environment.relayer}`,
    `- **Ready:** ${plan.ready ? "yes" : "no"}`,
    "",
    "## Operator Summary",
    "",
    ...formatPoolPlan(plan).map((line) => `- ${line}`),
    "",
    "| Pool | Workload | Current | Available | Deficit | ACL refresh | Status |",
    "| --- | ---: | ---: | ---: | ---: | --- | --- |",
  ];
  for (const item of plan.items) {
    lines.push(
      `| ${item.pool} | ${item.requirement.workload.mode === "finite" ? `${item.requirement.workload.requestBudget.toString()} requests` : "duration-bound"} | ` +
        `${item.observation.currentItems.toString()} | ${item.observation.availableItems?.toString() ?? "—"} | ` +
        `${item.decision.deficitItems.toString()} | ${item.decision.refreshRequired ? "yes" : "no"} | ` +
        `${item.decision.ready ? "ready" : "action required"} |`,
    );
  }
  return `${lines.join("\n")}\n`;
};

const preparationMarkdown = (artifact: PoolPreparationArtifact): string => {
  const lines = [
    "# Pool Preparation",
    "",
    `- **Window:** ${artifact.startedAt} → ${artifact.endedAt}`,
    `- **Status:** ${artifact.status}`,
    `- **Scenarios:** ${artifact.scenarios.join(" → ")}`,
    `- **Scenario digest:** \`${artifact.scenarioDigest}\``,
    `- **Initial readiness:** ${artifact.initialReady ? "ready" : "action required"}`,
    `- **Final inspection:** ${artifact.finalInspection.status}`,
    `- **Final readiness:** ${artifact.finalInspection.status === "failed" ? "unavailable" : artifact.finalReady ? "ready" : "not ready"}`,
    "",
    "| Action | Pool | Planned | Actual | Status |",
    "| --- | --- | ---: | ---: | --- |",
  ];
  for (const action of artifact.actions) {
    lines.push(
      `| ${action.kind} | ${action.pool} | ${action.items?.toString() ?? "—"} | ` +
        `${action.actualItems?.toString() ?? "—"} | ${action.status} |`,
    );
  }
  if (artifact.error) lines.push("", `Error: ${artifact.error}`);
  return `${lines.join("\n")}\n`;
};

const writePlanEvidence = async (
  artifactDir: string,
  plan: PoolPlanArtifact,
): Promise<void> => {
  await writeDurableFile(artifactDir, "pool-plan.json", jsonText(plan));
  await writeDurableFile(artifactDir, "pool-plan.md", planMarkdown(plan));
};

export const inspectPoolRequirements = async (
  options: InspectPoolRequirementsOptions,
): Promise<PoolPlanArtifact> => {
  const plan = await buildPlan(options, operationsFor(options.operations));
  if (options.artifactDir) await writePlanEvidence(options.artifactDir, plan);
  return plan;
};

const errorText = (error: unknown): string =>
  safeArtifactText(error instanceof Error ? error.message : error) ?? "Unknown error";

const interrupted = (error: unknown): boolean =>
  error instanceof Error && error.name === "AbortError";

/**
 * Explicitly prepares live pools from a freshly computed plan, then computes
 * a second live plan. Stored plan artifacts are evidence only and are never
 * accepted as executable input.
 */
export const preparePoolRequirements = async (
  options: PreparePoolRequirementsOptions,
): Promise<Readonly<{ plan: PoolPlanArtifact; preparation: PoolPreparationArtifact }>> => {
  const operations = operationsFor(options.operations);
  const startedAt = operations.now().toISOString();
  const initial = await buildPlan(options, operations);
  await writePlanEvidence(options.artifactDir, initial);
  const actions: PoolPreparationAction[] = [];
  let failure: unknown;

  if (initial.plannedActions.length > 0) {
    try {
      options.signal?.throwIfAborted();
      await options.beforeActions?.(initial);
      options.signal?.throwIfAborted();
    } catch (error) {
      failure = error;
    }
  }

  for (const planned of initial.plannedActions) {
    if (failure) break;
    const actionStartedAt = operations.now().toISOString();
    let actionEvidence = planned;
    let confirmedItems = 0;
    try {
      options.signal?.throwIfAborted();
      if (planned.kind === "generate-input-proof") {
        await operations.generateInputProof(options.env, {
          count: planned.items ?? 0,
          signal: options.signal,
          onProgress: (done, total) =>
            {
              confirmedItems = Math.max(confirmedItems, done);
              options.onProgress?.(`input-proof ${done.toString()}/${total.toString()}`);
            },
        });
      } else if (planned.kind === "create-handles") {
        await operations.createHandles(options.env, {
          flow: planned.flow as HandlePoolFlow,
          count: planned.items ?? 0,
          lanes: options.lanes,
          signal: options.signal,
          onProgress: (done, total) =>
            {
              confirmedItems = Math.max(confirmedItems, done);
              options.onProgress?.(`${planned.flow} ${done.toString()}/${total.toString()}`);
            },
        });
      } else {
        // Re-anchor the horizon immediately before an ACL write: preparation
        // can itself take long enough to make the inspection timestamp stale.
        const requiredValidUntil = requiredDelegationValidUntil(options.scenarios, {
          pauseSec: options.pauseSec ?? 0,
          nowSeconds: BigInt(Math.floor(operations.now().getTime() / 1000)),
        });
        actionEvidence = {
          ...planned,
          requiredValidUntil: requiredValidUntil.toString(),
        };
        await operations.refreshDelegationAcl(options.env, {
          requiredValidUntil,
          signal: options.signal,
          onProgress: options.onProgress,
        });
      }
      options.signal?.throwIfAborted();
      actions.push({
        ...actionEvidence,
        startedAt: actionStartedAt,
        endedAt: operations.now().toISOString(),
        status: "completed",
        ...(planned.items !== undefined ? { actualItems: planned.items } : {}),
      });
    } catch (error) {
      failure = error;
      actions.push({
        ...actionEvidence,
        startedAt: actionStartedAt,
        endedAt: operations.now().toISOString(),
        status: interrupted(error) ? "interrupted" : "failed",
        ...(planned.items !== undefined ? { actualItems: confirmedItems } : {}),
        error: errorText(error),
      });
    }
  }

  let final: PoolPlanArtifact | undefined;
  let finalInspectionError: unknown;
  try {
    final = await buildPlan(options, operations);
  } catch (error) {
    failure ??= error;
    finalInspectionError = error;
  }
  if (!failure && final && !final.ready) {
    failure = new Error("Pool preparation completed, but live re-inspection is not ready.");
  }

  const preparation: PoolPreparationArtifact = {
    version: POOL_PREPARATION_ARTIFACT_VERSION,
    kind: "load-test-pool-preparation",
    startedAt,
    endedAt: operations.now().toISOString(),
    status: failure
      ? interrupted(failure)
        ? "interrupted"
        : "failed"
      : "completed",
    scenarioDigest: initial.scenarioDigest,
    scenarios: initial.scenarios,
    pauseSec: initial.pauseSec,
    environment: initial.environment,
    requested: { ...(options.lanes !== undefined ? { lanes: options.lanes } : {}) },
    initialReady: initial.ready,
    actions,
    finalInspection: final
      ? { status: "completed", observedAt: final.observedAt }
      : { status: "failed", error: errorText(finalInspectionError) },
    ...(final ? { finalReady: final.ready, finalItems: final.items } : {}),
    ...(failure ? { error: errorText(failure) } : {}),
  };

  let persistenceError: unknown;
  try {
    await writeDurableFile(options.artifactDir, "preparation.json", jsonText(preparation));
    await writeDurableFile(
      options.artifactDir,
      "preparation.md",
      preparationMarkdown(preparation),
    );
  } catch (error) {
    persistenceError = error;
  }
  if (failure && persistenceError) {
    throw new AggregateError(
      [failure, persistenceError],
      "Pool preparation failed and its evidence could not be persisted",
      { cause: failure },
    );
  }
  if (failure) throw failure;
  if (persistenceError) throw persistenceError;
  return { plan: final!, preparation };
};
