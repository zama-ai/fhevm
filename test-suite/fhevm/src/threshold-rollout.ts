import { kmsCoreName, kmsPartyIds, reconstructionThreshold } from "./kms-party";
import type { ResolvedKmsTopology } from "./types";

export type KmsNode = {
  nodeId: number;
  identity: string;
  /** Planned version at this upgrade step. The executor records the observed runtime version. */
  version: string;
};

export type KmsUpgradePlan = {
  quorum: number;
  nodes: readonly KmsNode[];
  targetVersion: string;
  upgradeOrder: readonly number[];
};

export type KmsUpgradePlanInput = {
  topology: ResolvedKmsTopology;
  versionByNodeId: Readonly<Record<number, string>>;
  targetVersion: string;
  upgradeOrder: readonly number[];
};

export type KmsNodeAtUpgradeStep = KmsNode & {
  /** Whether this rollout executed the node's upgrade step. Independent from `version`. */
  upgradeState: "not-upgraded" | "upgraded";
};

export type KmsQuorumToTest = {
  /** Counts upgrade operations; read `nodes[].version` for the planned version mix. */
  notYetUpgradedNodeCount: number;
  upgradedNodeCount: number;
  nodes: readonly KmsNodeAtUpgradeStep[];
};

export type KmsUpgradeStep = {
  completedUpgradeSteps: number;
  minUpgradedNodeCountInQuorum: number;
  maxUpgradedNodeCountInQuorum: number;
  nodes: readonly KmsNodeAtUpgradeStep[];
  quorumsToTest: readonly KmsQuorumToTest[];
};

const assertIntegerInRange = (name: string, value: number, minimum: number, maximum: number) => {
  if (!Number.isInteger(value) || value < minimum || value > maximum) {
    throw new Error(`${name} must be an integer between ${minimum} and ${maximum}, received ${value}`);
  }
};

const assertNonEmpty = (name: string, value: string) => {
  if (value.trim().length === 0) {
    throw new Error(`${name} must not be empty`);
  }
};

/**
 * Builds a KMS upgrade plan from the resolved serving committee. Node ids and
 * identities use the suite's canonical KMS topology and naming helpers. Spare
 * cores outside `committeeSize` are not part of this progressive-upgrade plan.
 * The derived 2t+1 quorum is the exact size of each minimal quorum generated;
 * larger response sets are separate executor coverage.
 */
export const defineKmsUpgradePlan = (input: KmsUpgradePlanInput): KmsUpgradePlan => {
  const { topology } = input;
  if (topology.mode !== "threshold") {
    throw new Error("KMS progressive rollout requires a resolved threshold topology");
  }
  if (topology.committeeSize !== 3 * topology.threshold + 1) {
    throw new Error(
      `KMS committee must equal 3*threshold+1; received committeeSize=${topology.committeeSize}, threshold=${topology.threshold}`,
    );
  }
  assertIntegerInRange("topology.committeeSize", topology.committeeSize, 4, topology.parties);
  assertNonEmpty("targetVersion", input.targetVersion);

  const nodeIds = kmsPartyIds(topology.committeeSize);
  const expectedNodeIds = new Set(nodeIds);
  for (const key of Object.keys(input.versionByNodeId)) {
    const nodeId = Number(key);
    if (!Number.isInteger(nodeId) || !expectedNodeIds.has(nodeId)) {
      throw new Error(`versionByNodeId contains node ${JSON.stringify(key)} outside the serving committee`);
    }
  }
  const nodes = nodeIds.map((nodeId): KmsNode => {
    const version = input.versionByNodeId[nodeId];
    if (version === undefined) {
      throw new Error(`versionByNodeId is missing KMS node ${nodeId}`);
    }
    assertNonEmpty(`versionByNodeId[${nodeId}]`, version);
    return { nodeId, identity: kmsCoreName(nodeId), version };
  });

  if (input.upgradeOrder.length !== nodes.length) {
    throw new Error(
      `upgradeOrder must contain every serving KMS node exactly once; expected ${nodes.length} entries, received ${input.upgradeOrder.length}`,
    );
  }
  const orderedIds = new Set<number>();
  for (const nodeId of input.upgradeOrder) {
    if (!expectedNodeIds.has(nodeId)) {
      throw new Error(`upgradeOrder contains unknown nodeId ${nodeId}`);
    }
    if (orderedIds.has(nodeId)) {
      throw new Error(`upgradeOrder contains duplicate nodeId ${nodeId}`);
    }
    orderedIds.add(nodeId);
  }

  return {
    quorum: reconstructionThreshold(topology.threshold),
    nodes,
    targetVersion: input.targetVersion,
    upgradeOrder: [...input.upgradeOrder],
  };
};

/** Returns how many upgraded nodes can be present in a quorum at this upgrade step. */
export const possibleUpgradedNodesInQuorum = (
  totalNodes: number,
  quorum: number,
  completedUpgradeSteps: number,
): { minUpgradedNodeCountInQuorum: number; maxUpgradedNodeCountInQuorum: number } => {
  assertIntegerInRange("totalNodes", totalNodes, 1, Number.MAX_SAFE_INTEGER);
  assertIntegerInRange("quorum", quorum, 1, totalNodes);
  assertIntegerInRange("completedUpgradeSteps", completedUpgradeSteps, 0, totalNodes);
  return {
    minUpgradedNodeCountInQuorum: Math.max(0, quorum - (totalNodes - completedUpgradeSteps)),
    maxUpgradedNodeCountInQuorum: Math.min(quorum, completedUpgradeSteps),
  };
};

/**
 * Returns the completed KMS node upgrade steps at which the standard rollout must
 * test. It covers every step through half, every partial step from the point where
 * not-yet-upgraded nodes lose a standalone quorum through the point where upgraded
 * nodes gain one, and the final step before every upgrade is complete. The baseline
 * and final checks cover zero completed upgrades and every upgrade completed.
 */
export const upgradeStepsToTest = (totalNodes: number, quorum: number): readonly number[] => {
  assertIntegerInRange("totalNodes", totalNodes, 2, Number.MAX_SAFE_INTEGER);
  assertIntegerInRange("quorum", quorum, 1, totalNodes);

  const counts = new Set<number>();
  for (let upgraded = 1; upgraded <= Math.floor(totalNodes / 2); upgraded += 1) {
    counts.add(upgraded);
  }
  for (let upgraded = totalNodes - quorum + 1; upgraded <= quorum; upgraded += 1) {
    counts.add(upgraded);
  }
  counts.add(totalNodes - 1);

  return [...counts].filter((count) => count > 0 && count < totalNodes).sort((a, b) => a - b);
};

/**
 * Builds one deterministic quorum for every feasible mix of upgraded and not-yet-upgraded nodes.
 * It plans exact membership but does not stop, isolate, or restore any containers.
 */
export const buildKmsUpgradeStep = (
  plan: KmsUpgradePlan,
  completedUpgradeSteps: number,
): KmsUpgradeStep => {
  assertIntegerInRange("completedUpgradeSteps", completedUpgradeSteps, 0, plan.nodes.length);

  const upgradedIds = new Set(plan.upgradeOrder.slice(0, completedUpgradeSteps));
  const nodes = plan.nodes.map((node): KmsNodeAtUpgradeStep =>
    upgradedIds.has(node.nodeId)
      ? { ...node, version: plan.targetVersion, upgradeState: "upgraded" }
      : { ...node, upgradeState: "not-upgraded" },
  );
  const upgraded = plan.upgradeOrder
    .slice(0, completedUpgradeSteps)
    .map((nodeId) => nodes.find((node) => node.nodeId === nodeId)!);
  const notYetUpgraded = plan.upgradeOrder
    .slice(completedUpgradeSteps)
    .map((nodeId) => nodes.find((node) => node.nodeId === nodeId)!);
  const range = possibleUpgradedNodesInQuorum(plan.nodes.length, plan.quorum, completedUpgradeSteps);
  const quorumsToTest: KmsQuorumToTest[] = [];

  for (
    let upgradedNodeCount = range.minUpgradedNodeCountInQuorum;
    upgradedNodeCount <= range.maxUpgradedNodeCountInQuorum;
    upgradedNodeCount += 1
  ) {
    const notYetUpgradedNodeCount = plan.quorum - upgradedNodeCount;
    quorumsToTest.push({
      notYetUpgradedNodeCount,
      upgradedNodeCount,
      nodes: [...upgraded.slice(0, upgradedNodeCount), ...notYetUpgraded.slice(0, notYetUpgradedNodeCount)],
    });
  }

  return {
    completedUpgradeSteps,
    ...range,
    nodes,
    quorumsToTest,
  };
};
