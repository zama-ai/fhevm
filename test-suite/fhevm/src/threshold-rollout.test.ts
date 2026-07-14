import { describe, expect, test } from "bun:test";

import {
  dangerousPartialUpgradeCounts,
  defineThresholdUpgradePlan,
  quorumCompositionRange,
  thresholdUpgradeCheckpoint,
  type ThresholdUpgradePlan,
} from "./threshold-rollout";

const fourPartyPlan: ThresholdUpgradePlan = {
  quorum: 3,
  parties: [
    { partyId: 1, identity: "kms-core", version: "0.13.3" },
    { partyId: 2, identity: "kms-core-2", version: "0.13.3" },
    { partyId: 3, identity: "kms-core-3", version: "0.13.3" },
    { partyId: 4, identity: "kms-core-4", version: "0.13.3" },
  ],
  targetVersion: "0.13.10",
  upgradeOrder: [1, 2, 3, 4],
};

describe("defineThresholdUpgradePlan", () => {
  test("copies a valid plan", () => {
    const plan = defineThresholdUpgradePlan(fourPartyPlan);
    expect(plan).toEqual(fourPartyPlan);
    expect(plan).not.toBe(fourPartyPlan);
    expect(plan.parties).not.toBe(fourPartyPlan.parties);
    expect(plan.upgradeOrder).not.toBe(fourPartyPlan.upgradeOrder);
  });

  test("rejects invalid quorum and incomplete upgrade order", () => {
    expect(() => defineThresholdUpgradePlan({ ...fourPartyPlan, quorum: 5 })).toThrow(/quorum/);
    expect(() => defineThresholdUpgradePlan({ ...fourPartyPlan, upgradeOrder: [1, 2, 3] })).toThrow(
      /every party exactly once/,
    );
  });

  test("rejects duplicate and unknown party identities", () => {
    expect(() =>
      defineThresholdUpgradePlan({
        ...fourPartyPlan,
        parties: fourPartyPlan.parties.map((party, index) =>
          index === 1 ? { ...party, partyId: 1 } : party,
        ),
      }),
    ).toThrow(/duplicate partyId/);
    expect(() => defineThresholdUpgradePlan({ ...fourPartyPlan, upgradeOrder: [1, 2, 3, 9] })).toThrow(
      /unknown partyId 9/,
    );
    expect(() =>
      defineThresholdUpgradePlan({
        ...fourPartyPlan,
        parties: fourPartyPlan.parties.map((party, index) =>
          index === 1 ? { ...party, identity: "kms-core" } : party,
        ),
      }),
    ).toThrow(/duplicate party identity/);
  });

  test("rejects empty versions and identities", () => {
    expect(() => defineThresholdUpgradePlan({ ...fourPartyPlan, targetVersion: " " })).toThrow(/targetVersion/);
    expect(() =>
      defineThresholdUpgradePlan({
        ...fourPartyPlan,
        parties: fourPartyPlan.parties.map((party, index) =>
          index === 0 ? { ...party, version: "" } : party,
        ),
      }),
    ).toThrow(/version/);
  });
});

describe("quorumCompositionRange", () => {
  test("derives both dangerous half-upgraded compositions for N=4, Q=3", () => {
    expect(quorumCompositionRange(4, 3, 2)).toEqual({
      minimumUpgradedResponders: 1,
      maximumUpgradedResponders: 2,
    });
  });

  test("handles homogeneous endpoints", () => {
    expect(quorumCompositionRange(4, 3, 0)).toEqual({
      minimumUpgradedResponders: 0,
      maximumUpgradedResponders: 0,
    });
    expect(quorumCompositionRange(4, 3, 4)).toEqual({
      minimumUpgradedResponders: 3,
      maximumUpgradedResponders: 3,
    });
  });
});

describe("dangerousPartialUpgradeCounts", () => {
  test("covers every partial state for N=4, Q=3", () => {
    expect(dangerousPartialUpgradeCounts(4, 3)).toEqual([1, 2, 3]);
  });

  test("covers the complete odd majority-quorum middle interval", () => {
    expect(dangerousPartialUpgradeCounts(7, 5)).toEqual([1, 2, 3, 4, 5, 6]);
    expect(dangerousPartialUpgradeCounts(5, 4)).toEqual([1, 2, 3, 4]);
  });
});

describe("thresholdUpgradeCheckpoint", () => {
  test("returns one deterministic quorum for each feasible composition", () => {
    const checkpoint = thresholdUpgradeCheckpoint(fourPartyPlan, 2);

    expect(checkpoint).toMatchObject({
      upgradedParties: 2,
      minimumUpgradedResponders: 1,
      maximumUpgradedResponders: 2,
    });
    expect(checkpoint.representativeQuorums).toEqual([
      {
        pendingResponders: 2,
        upgradedResponders: 1,
        parties: [
          { partyId: 1, identity: "kms-core", version: "0.13.10", upgradeState: "upgraded" },
          { partyId: 3, identity: "kms-core-3", version: "0.13.3", upgradeState: "pending" },
          { partyId: 4, identity: "kms-core-4", version: "0.13.3", upgradeState: "pending" },
        ],
      },
      {
        pendingResponders: 1,
        upgradedResponders: 2,
        parties: [
          { partyId: 1, identity: "kms-core", version: "0.13.10", upgradeState: "upgraded" },
          { partyId: 2, identity: "kms-core-2", version: "0.13.10", upgradeState: "upgraded" },
          { partyId: 3, identity: "kms-core-3", version: "0.13.3", upgradeState: "pending" },
        ],
      },
    ]);
  });

  test("uses the declared upgrade order for membership and receipt-ready identity", () => {
    const checkpoint = thresholdUpgradeCheckpoint({ ...fourPartyPlan, upgradeOrder: [4, 2, 3, 1] }, 1);

    expect(checkpoint.parties).toEqual([
      { partyId: 1, identity: "kms-core", version: "0.13.3", upgradeState: "pending" },
      { partyId: 2, identity: "kms-core-2", version: "0.13.3", upgradeState: "pending" },
      { partyId: 3, identity: "kms-core-3", version: "0.13.3", upgradeState: "pending" },
      { partyId: 4, identity: "kms-core-4", version: "0.13.10", upgradeState: "upgraded" },
    ]);
    expect(checkpoint.representativeQuorums[0].parties.map((party) => party.identity)).toEqual([
      "kms-core-2",
      "kms-core-3",
      "kms-core",
    ]);
  });

  test("reports operation state independently from an already-matching version", () => {
    const checkpoint = thresholdUpgradeCheckpoint(
      {
        ...fourPartyPlan,
        parties: fourPartyPlan.parties.map((party, index) =>
          index === 2 ? { ...party, version: fourPartyPlan.targetVersion } : party,
        ),
      },
      1,
    );

    expect(checkpoint.parties[2]).toEqual({
      partyId: 3,
      identity: "kms-core-3",
      version: "0.13.10",
      upgradeState: "pending",
    });
  });
});
