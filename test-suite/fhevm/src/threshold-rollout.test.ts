import { describe, expect, test } from "bun:test";

import {
  buildKmsUpgradeStep,
  defineKmsUpgradePlan,
  possibleUpgradedNodesInQuorum,
  type KmsUpgradePlan,
  upgradeStepsToTest,
} from "./threshold-rollout";

const fourNodePlan: KmsUpgradePlan = {
  quorum: 3,
  nodes: [
    { nodeId: 1, identity: "kms-core", version: "0.13.3" },
    { nodeId: 2, identity: "kms-core-2", version: "0.13.3" },
    { nodeId: 3, identity: "kms-core-3", version: "0.13.3" },
    { nodeId: 4, identity: "kms-core-4", version: "0.13.3" },
  ],
  targetVersion: "0.13.10",
  upgradeOrder: [1, 2, 3, 4],
};

describe("defineKmsUpgradePlan", () => {
  test("copies a valid plan", () => {
    const plan = defineKmsUpgradePlan(fourNodePlan);
    expect(plan).toEqual(fourNodePlan);
    expect(plan).not.toBe(fourNodePlan);
    expect(plan.nodes).not.toBe(fourNodePlan.nodes);
    expect(plan.upgradeOrder).not.toBe(fourNodePlan.upgradeOrder);
  });

  test("rejects invalid quorum and incomplete upgrade order", () => {
    expect(() => defineKmsUpgradePlan({ ...fourNodePlan, quorum: 5 })).toThrow(/quorum/);
    expect(() => defineKmsUpgradePlan({ ...fourNodePlan, upgradeOrder: [1, 2, 3] })).toThrow(
      /every KMS node exactly once/,
    );
  });

  test("rejects duplicate IDs, unknown IDs, and duplicate identities", () => {
    expect(() =>
      defineKmsUpgradePlan({
        ...fourNodePlan,
        nodes: fourNodePlan.nodes.map((node, index) =>
          index === 1 ? { ...node, nodeId: 1 } : node,
        ),
      }),
    ).toThrow(/duplicate nodeId/);
    expect(() => defineKmsUpgradePlan({ ...fourNodePlan, upgradeOrder: [1, 2, 3, 9] })).toThrow(
      /unknown nodeId 9/,
    );
    expect(() =>
      defineKmsUpgradePlan({
        ...fourNodePlan,
        nodes: fourNodePlan.nodes.map((node, index) =>
          index === 1 ? { ...node, identity: "kms-core" } : node,
        ),
      }),
    ).toThrow(/duplicate KMS node identity/);
  });

  test("rejects empty versions and identities", () => {
    expect(() => defineKmsUpgradePlan({ ...fourNodePlan, targetVersion: " " })).toThrow(/targetVersion/);
    expect(() =>
      defineKmsUpgradePlan({
        ...fourNodePlan,
        nodes: fourNodePlan.nodes.map((node, index) =>
          index === 0 ? { ...node, version: "" } : node,
        ),
      }),
    ).toThrow(/version/);
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
          { nodeId: 1, identity: "kms-core", version: "0.13.10", upgradeState: "upgraded" },
          { nodeId: 3, identity: "kms-core-3", version: "0.13.3", upgradeState: "not-upgraded" },
          { nodeId: 4, identity: "kms-core-4", version: "0.13.3", upgradeState: "not-upgraded" },
        ],
      },
      {
        notYetUpgradedNodeCount: 1,
        upgradedNodeCount: 2,
        nodes: [
          { nodeId: 1, identity: "kms-core", version: "0.13.10", upgradeState: "upgraded" },
          { nodeId: 2, identity: "kms-core-2", version: "0.13.10", upgradeState: "upgraded" },
          { nodeId: 3, identity: "kms-core-3", version: "0.13.3", upgradeState: "not-upgraded" },
        ],
      },
    ]);
  });

  test("uses the declared upgrade order for membership and receipt-ready identity", () => {
    const step = buildKmsUpgradeStep({ ...fourNodePlan, upgradeOrder: [4, 2, 3, 1] }, 1);

    expect(step.nodes).toEqual([
      { nodeId: 1, identity: "kms-core", version: "0.13.3", upgradeState: "not-upgraded" },
      { nodeId: 2, identity: "kms-core-2", version: "0.13.3", upgradeState: "not-upgraded" },
      { nodeId: 3, identity: "kms-core-3", version: "0.13.3", upgradeState: "not-upgraded" },
      { nodeId: 4, identity: "kms-core-4", version: "0.13.10", upgradeState: "upgraded" },
    ]);
    expect(step.quorumsToTest[0].nodes.map((node) => node.identity)).toEqual([
      "kms-core-2",
      "kms-core-3",
      "kms-core",
    ]);
  });

  test("reports operation state independently from an already-matching version", () => {
    const step = buildKmsUpgradeStep(
      {
        ...fourNodePlan,
        nodes: fourNodePlan.nodes.map((node, index) =>
          index === 2 ? { ...node, version: fourNodePlan.targetVersion } : node,
        ),
      },
      1,
    );

    expect(step.nodes[2]).toEqual({
      nodeId: 3,
      identity: "kms-core-3",
      version: "0.13.10",
      upgradeState: "not-upgraded",
    });
  });
});
