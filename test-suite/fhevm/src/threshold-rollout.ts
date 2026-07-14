export type ThresholdParty = {
  partyId: number;
  identity: string;
  version: string;
};

export type ThresholdUpgradePlan = {
  quorum: number;
  parties: readonly ThresholdParty[];
  targetVersion: string;
  upgradeOrder: readonly number[];
};

export type ThresholdQuorumParty = ThresholdParty & {
  upgradeState: "pending" | "upgraded";
};

export type ThresholdQuorum = {
  pendingResponders: number;
  upgradedResponders: number;
  parties: readonly ThresholdQuorumParty[];
};

export type ThresholdUpgradeCheckpoint = {
  upgradedParties: number;
  minimumUpgradedResponders: number;
  maximumUpgradedResponders: number;
  parties: readonly ThresholdQuorumParty[];
  representativeQuorums: readonly ThresholdQuorum[];
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
 * Validates and copies a threshold upgrade plan. `quorum` is the minimum number
 * of responding parties required to reconstruct a result.
 */
export const defineThresholdUpgradePlan = (plan: ThresholdUpgradePlan): ThresholdUpgradePlan => {
  if (plan.parties.length < 2) {
    throw new Error(`threshold rollout requires at least 2 parties, received ${plan.parties.length}`);
  }
  assertIntegerInRange("quorum", plan.quorum, 1, plan.parties.length);
  assertNonEmpty("targetVersion", plan.targetVersion);

  const partyIds = new Set<number>();
  const identities = new Set<string>();
  for (const [index, party] of plan.parties.entries()) {
    assertIntegerInRange(`parties[${index}].partyId`, party.partyId, 1, Number.MAX_SAFE_INTEGER);
    assertNonEmpty(`parties[${index}].identity`, party.identity);
    assertNonEmpty(`parties[${index}].version`, party.version);
    if (partyIds.has(party.partyId)) {
      throw new Error(`duplicate partyId ${party.partyId}`);
    }
    if (identities.has(party.identity)) {
      throw new Error(`duplicate party identity ${JSON.stringify(party.identity)}`);
    }
    partyIds.add(party.partyId);
    identities.add(party.identity);
  }

  if (plan.upgradeOrder.length !== plan.parties.length) {
    throw new Error(
      `upgradeOrder must contain every party exactly once; expected ${plan.parties.length} entries, received ${plan.upgradeOrder.length}`,
    );
  }
  const orderedIds = new Set<number>();
  for (const partyId of plan.upgradeOrder) {
    if (!partyIds.has(partyId)) {
      throw new Error(`upgradeOrder contains unknown partyId ${partyId}`);
    }
    if (orderedIds.has(partyId)) {
      throw new Error(`upgradeOrder contains duplicate partyId ${partyId}`);
    }
    orderedIds.add(partyId);
  }

  return {
    quorum: plan.quorum,
    parties: plan.parties.map((party) => ({ ...party })),
    targetVersion: plan.targetVersion,
    upgradeOrder: [...plan.upgradeOrder],
  };
};

/** Returns the feasible number of already-upgraded responders in a quorum. */
export const quorumCompositionRange = (
  parties: number,
  quorum: number,
  upgradedParties: number,
): { minimumUpgradedResponders: number; maximumUpgradedResponders: number } => {
  assertIntegerInRange("parties", parties, 1, Number.MAX_SAFE_INTEGER);
  assertIntegerInRange("quorum", quorum, 1, parties);
  assertIntegerInRange("upgradedParties", upgradedParties, 0, parties);
  return {
    minimumUpgradedResponders: Math.max(0, quorum - (parties - upgradedParties)),
    maximumUpgradedResponders: Math.min(quorum, upgradedParties),
  };
};

/**
 * Checkpoints for the standard progressive-upgrade lane: every mixed state
 * through half, the complete forced-mixed interval, cohort quorum-gain/loss
 * boundaries, and the last mixed state.
 * Homogeneous endpoints are intentionally excluded because they are not partial
 * upgrades and should already be covered by the rollout's baseline/final checks.
 */
export const dangerousPartialUpgradeCounts = (parties: number, quorum: number): readonly number[] => {
  assertIntegerInRange("parties", parties, 2, Number.MAX_SAFE_INTEGER);
  assertIntegerInRange("quorum", quorum, 1, parties);

  const counts = new Set<number>();
  for (let upgraded = 1; upgraded <= Math.floor(parties / 2); upgraded += 1) {
    counts.add(upgraded);
  }
  for (let upgraded = parties - quorum + 1; upgraded <= quorum; upgraded += 1) {
    counts.add(upgraded);
  }
  counts.add(quorum); // The upgraded cohort first has enough parties to form a quorum.
  counts.add(parties - quorum + 1); // The pending cohort first cannot form a quorum alone.
  counts.add(parties - 1);

  return [...counts].filter((count) => count > 0 && count < parties).sort((a, b) => a - b);
};

/**
 * Builds one deterministic quorum for every feasible pending/upgraded composition.
 * It plans exact membership but does not stop, isolate, or restore any containers.
 */
export const thresholdUpgradeCheckpoint = (
  uncheckedPlan: ThresholdUpgradePlan,
  upgradedParties: number,
): ThresholdUpgradeCheckpoint => {
  const plan = defineThresholdUpgradePlan(uncheckedPlan);
  assertIntegerInRange("upgradedParties", upgradedParties, 0, plan.parties.length);

  const upgradedIds = new Set(plan.upgradeOrder.slice(0, upgradedParties));
  const parties = plan.parties.map((party): ThresholdQuorumParty =>
    upgradedIds.has(party.partyId)
      ? { ...party, version: plan.targetVersion, upgradeState: "upgraded" }
      : { ...party, upgradeState: "pending" },
  );
  const upgraded = plan.upgradeOrder
    .slice(0, upgradedParties)
    .map((partyId) => parties.find((party) => party.partyId === partyId)!);
  const pending = plan.upgradeOrder
    .slice(upgradedParties)
    .map((partyId) => parties.find((party) => party.partyId === partyId)!);
  const range = quorumCompositionRange(plan.parties.length, plan.quorum, upgradedParties);
  const representativeQuorums: ThresholdQuorum[] = [];

  for (
    let upgradedResponders = range.minimumUpgradedResponders;
    upgradedResponders <= range.maximumUpgradedResponders;
    upgradedResponders += 1
  ) {
    const pendingResponders = plan.quorum - upgradedResponders;
    representativeQuorums.push({
      pendingResponders,
      upgradedResponders,
      parties: [...upgraded.slice(0, upgradedResponders), ...pending.slice(0, pendingResponders)],
    });
  }

  return {
    upgradedParties,
    ...range,
    parties,
    representativeQuorums,
  };
};
