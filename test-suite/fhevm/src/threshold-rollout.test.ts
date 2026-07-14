import { describe, expect, test } from "bun:test";

import { kmsCoreName } from "./kms-party";
import { resolveKmsTopology } from "./scenario/resolve";
import {
  buildKmsUpgradeStep,
  defineKmsUpgradePlan,
  possibleUpgradedNodesInQuorum,
  type KmsUpgradePlanInput,
  upgradeStepsToTest,
} from "./threshold-rollout";

const fourNodePlanInput: KmsUpgradePlanInput = {
  topology: resolveKmsTopology({ mode: "threshold", parties: 4, threshold: 1 }),
  versionByNodeId: { 1: "0.13.3", 2: "0.13.3", 3: "0.13.3", 4: "0.13.3" },
  targetVersion: "0.13.10",
  upgradeOrder: [1, 2, 3, 4],
};
const fourNodePlan = defineKmsUpgradePlan(fourNodePlanInput);

describe("defineKmsUpgradePlan", () => {
  test("derives the serving committee, reconstruction quorum, and canonical node names", () => {
    const plan = defineKmsUpgradePlan(fourNodePlanInput);
    expect(plan).toEqual({
      quorum: 3,
      nodes: [1, 2, 3, 4].map((nodeId) => ({
        nodeId,
        identity: kmsCoreName(nodeId),
        version: "0.13.3",
      })),
      targetVersion: "0.13.10",
      upgradeOrder: [1, 2, 3, 4],
    });
    expect(plan.upgradeOrder).not.toBe(fourNodePlanInput.upgradeOrder);
  });

  test("uses the resolved 3t+1 committee and ignores provisioned spares", () => {
    const plan = defineKmsUpgradePlan({
      topology: resolveKmsTopology({ mode: "threshold", parties: 5, committeeSize: 4, threshold: 1 }),
      versionByNodeId: fourNodePlanInput.versionByNodeId,
      targetVersion: fourNodePlanInput.targetVersion,
      upgradeOrder: fourNodePlanInput.upgradeOrder,
    });
    expect(plan.quorum).toBe(3);
    expect(plan.nodes.map((node) => node.identity)).toEqual([1, 2, 3, 4].map(kmsCoreName));
  });

  test("derives the next valid committee quorum", () => {
    const topology = resolveKmsTopology({ mode: "threshold", parties: 7, threshold: 2 });
    const plan = defineKmsUpgradePlan({
      topology,
      versionByNodeId: Object.fromEntries(Array.from({ length: 7 }, (_, index) => [index + 1, "0.13.3"])),
      targetVersion: "0.13.10",
      upgradeOrder: [1, 2, 3, 4, 5, 6, 7],
    });
    expect(plan.quorum).toBe(5);
  });

  test("rejects centralized and structurally invalid resolved topologies", () => {
    expect(() =>
      defineKmsUpgradePlan({
        ...fourNodePlanInput,
        topology: resolveKmsTopology(undefined),
      }),
    ).toThrow(/threshold topology/);
    expect(() =>
      defineKmsUpgradePlan({
        ...fourNodePlanInput,
        topology: { ...fourNodePlanInput.topology, committeeSize: 5 },
      }),
    ).toThrow(/3\*threshold\+1/);
  });

  test("rejects incomplete and unknown upgrade order entries", () => {
    expect(() => defineKmsUpgradePlan({ ...fourNodePlanInput, upgradeOrder: [1, 2, 3] })).toThrow(
      /every serving KMS node exactly once/,
    );
    expect(() => defineKmsUpgradePlan({ ...fourNodePlanInput, upgradeOrder: [1, 2, 3, 9] })).toThrow(
      /unknown nodeId 9/,
    );
    expect(() => defineKmsUpgradePlan({ ...fourNodePlanInput, upgradeOrder: [1, 2, 3, 3] })).toThrow(
      /duplicate nodeId 3/,
    );
  });

  test("rejects missing, extra, and empty versions", () => {
    expect(() => defineKmsUpgradePlan({ ...fourNodePlanInput, targetVersion: " " })).toThrow(/targetVersion/);
    expect(() =>
      defineKmsUpgradePlan({
        ...fourNodePlanInput,
        versionByNodeId: { 1: "", 2: "0.13.3", 3: "0.13.3", 4: "0.13.3" },
      }),
    ).toThrow(/version/);
    expect(() =>
      defineKmsUpgradePlan({ ...fourNodePlanInput, versionByNodeId: { 1: "0.13.3", 2: "0.13.3", 3: "0.13.3" } }),
    ).toThrow(/missing KMS node 4/);
    expect(() =>
      defineKmsUpgradePlan({ ...fourNodePlanInput, versionByNodeId: { ...fourNodePlanInput.versionByNodeId, 5: "0.13.3" } }),
    ).toThrow(/outside the serving committee/);
  });
});

describe("possibleUpgradedNodesInQuorum", () => {
  test("returns both possible mixes after two of four upgrade steps complete", () => {
    expect(possibleUpgradedNodesInQuorum(4, 3, 2)).toEqual({
      minUpgradedNodeCountInQuorum: 1,
      maxUpgradedNodeCountInQuorum: 2,
    });
  });

  test("handles the steps before the first upgrade and after the last upgrade", () => {
    expect(possibleUpgradedNodesInQuorum(4, 3, 0)).toEqual({
      minUpgradedNodeCountInQuorum: 0,
      maxUpgradedNodeCountInQuorum: 0,
    });
    expect(possibleUpgradedNodesInQuorum(4, 3, 4)).toEqual({
      minUpgradedNodeCountInQuorum: 3,
      maxUpgradedNodeCountInQuorum: 3,
    });
  });
});

describe("upgradeStepsToTest", () => {
  test("tests every partial step for four nodes with a quorum of three", () => {
    expect(upgradeStepsToTest(4, 3)).toEqual([1, 2, 3]);
  });

  test("tests every partial step when the quorum is a large majority", () => {
    expect(upgradeStepsToTest(7, 5)).toEqual([1, 2, 3, 4, 5, 6]);
    expect(upgradeStepsToTest(5, 4)).toEqual([1, 2, 3, 4]);
  });
});

describe("buildKmsUpgradeStep", () => {
  test("returns one deterministic quorum for each feasible mix", () => {
    const step = buildKmsUpgradeStep(fourNodePlan, 2);

    expect(step).toMatchObject({
      completedUpgradeSteps: 2,
      minUpgradedNodeCountInQuorum: 1,
      maxUpgradedNodeCountInQuorum: 2,
    });
    expect(step.quorumsToTest).toEqual([
      {
        notYetUpgradedNodeCount: 2,
        upgradedNodeCount: 1,
        nodes: [
          { nodeId: 1, identity: kmsCoreName(1), version: "0.13.10", upgradeState: "upgraded" },
          { nodeId: 3, identity: kmsCoreName(3), version: "0.13.3", upgradeState: "not-upgraded" },
          { nodeId: 4, identity: kmsCoreName(4), version: "0.13.3", upgradeState: "not-upgraded" },
        ],
      },
      {
        notYetUpgradedNodeCount: 1,
        upgradedNodeCount: 2,
        nodes: [
          { nodeId: 1, identity: kmsCoreName(1), version: "0.13.10", upgradeState: "upgraded" },
          { nodeId: 2, identity: kmsCoreName(2), version: "0.13.10", upgradeState: "upgraded" },
          { nodeId: 3, identity: kmsCoreName(3), version: "0.13.3", upgradeState: "not-upgraded" },
        ],
      },
    ]);
  });

  test("uses the declared upgrade order for membership and receipt-ready identity", () => {
    const plan = defineKmsUpgradePlan({ ...fourNodePlanInput, upgradeOrder: [4, 2, 3, 1] });
    const step = buildKmsUpgradeStep(plan, 1);

    expect(step.nodes).toEqual([
      { nodeId: 1, identity: kmsCoreName(1), version: "0.13.3", upgradeState: "not-upgraded" },
      { nodeId: 2, identity: kmsCoreName(2), version: "0.13.3", upgradeState: "not-upgraded" },
      { nodeId: 3, identity: kmsCoreName(3), version: "0.13.3", upgradeState: "not-upgraded" },
      { nodeId: 4, identity: kmsCoreName(4), version: "0.13.10", upgradeState: "upgraded" },
    ]);
    expect(step.quorumsToTest[0].nodes.map((node) => node.identity)).toEqual([2, 3, 1].map(kmsCoreName));
  });

  test("reports operation state independently from an already-matching planned version", () => {
    const plan = defineKmsUpgradePlan({
      ...fourNodePlanInput,
      versionByNodeId: { ...fourNodePlanInput.versionByNodeId, 3: fourNodePlanInput.targetVersion },
    });
    const step = buildKmsUpgradeStep(plan, 1);

    expect(step.nodes[2]).toEqual({
      nodeId: 3,
      identity: kmsCoreName(3),
      version: "0.13.10",
      upgradeState: "not-upgraded",
    });
    expect(step.quorumsToTest[0]).toMatchObject({
      notYetUpgradedNodeCount: 3,
      upgradedNodeCount: 0,
    });
    expect(step.quorumsToTest[0].nodes.map((node) => node.version)).toEqual([
      "0.13.3",
      "0.13.10",
      "0.13.3",
    ]);
  });
});
