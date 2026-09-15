/**
 * Observing — and holding open — an in-flight KMS lifecycle operation.
 *
 * The `epoch-rotation-pending` case needs a rotation to sit in Pending for the ~3 minutes two
 * container probes take. Two problems follow from that, and this module owns both.
 *
 * ## 1. Pending is not readable
 *
 * `ProtocolConfig` stores `epochState[epochCounter]` in private storage and exposes **no** getter
 * for it (`host-contracts/contracts/interfaces/IProtocolConfig.sol` has no `getPending…` /
 * `…Status` view). The state is therefore established indirectly, from three angles that fail for
 * different reasons and so cannot all be wrong at once:
 *
 *   - `getCurrentKmsContextAndEpoch` has not moved — `assertPairUnchanged` in `protocol-config.ts`;
 *   - a second lifecycle operation reverts `KmsLifecycleOperationInFlight` — the gate at
 *     `ProtocolConfig.sol:1083` (`_checkNoKmsLifecycleOperationInFlight`) fires exactly when the
 *     latest context is Pending/Created or the latest epoch is Pending — {@link assertLifecycleOperationInFlight};
 *   - `isValidEpochForContext(C, E)` is still false for the new epoch — {@link assertEpochValidity}.
 *
 * The third is the only *positive* signal of the three: the first two prove "nothing advanced", it
 * proves "this specific epoch exists and is not yet serving".
 *
 * ## 2. Pending is too short to observe
 *
 * Left alone, a rotation activates in roughly a minute (65.6s over 14 polls on the run recorded in
 * `qa-kms-context-scenario-1-epoch.md`), while the user-decryption probe alone takes ~120s. The
 * window is therefore *held*: one committee member's tx-sender is stopped before the broadcast, so
 * its `confirmEpochActivation` can never land and the all-signers quorum is never reached. Its core
 * stays up and reshares normally, which is what makes the hold honest — the epoch is Pending for the
 * reason the scenario describes (awaiting activation), not because the KMS is broken.
 *
 * The stop/restore itself is {@link NodeSupervisor.withTxSendersStopped}; this module only decides
 * *which* party to stall and proves the choice is safe.
 */
import { PreflightError } from "../errors";
import { castBool, castCall } from "../flow/readiness";
import { callContractAndExpectRevert, parseUintOutput, type Owner } from "../kms-onchain";
import type { CaseEvidence } from "./evidence";
import { formatKmsId, type ProtocolConfigTarget } from "./protocol-config";

/** The revert the ProtocolConfig gate raises while a context or epoch is still settling. */
export const LIFECYCLE_IN_FLIGHT_ERROR = "KmsLifecycleOperationInFlight(uint256,uint256)";

/**
 * Chooses which committee member's tx-sender to stop in order to hold a rotation Pending.
 *
 * Any single member will do — activation needs *every* signer — so the choice is about blast
 * radius, not correctness. The last member is taken because the lowest party ids carry the
 * bootstrap roles in these scenarios (party 1 keeps the bare container names, per `kms-party.ts`),
 * and leaving them alone keeps a failed run easier to reason about.
 *
 * Takes the committee resolved from the chain, never `1..committeeSize`: on a node-swapped stack
 * the serving set is something like `{1,2,3,5}`, and stalling a party that is not in it would not
 * withhold anything — the rotation would activate mid-probe and the case would fail for the wrong
 * reason.
 *
 * Pure; exported for unit testing.
 *
 * @throws PreflightError when the committee is empty, which means the caller resolved nothing.
 */
export const pickStallParty = (committee: readonly number[]): number => {
  if (!committee.length) {
    throw new PreflightError(
      "kms-context-qa: cannot hold a rotation Pending — the live committee resolved to no parties at all. " +
        "Persisted discovery and the chain disagree about the cluster; re-up the stack.",
    );
  }
  return committee[committee.length - 1]!;
};

/**
 * Returns why the remaining committee could not serve a user decryption with `stalled` held back,
 * or undefined when it can. Pure; exported for unit testing.
 *
 * `threshold` is the number of responses `getUserDecryptionThresholdForContext` requires. Stopping
 * a tx-sender removes that party's ability to land its response on chain, so the case only works
 * when the rest of the committee still reaches it.
 */
export const quorumShortfallAfterStall = (
  committeeSize: number,
  threshold: bigint,
  stalled: number,
): string | undefined => {
  const remaining = BigInt(committeeSize - 1);
  if (remaining >= threshold) return undefined;
  return (
    `stalling party ${stalled} leaves ${remaining} of ${committeeSize} committee member(s) able to respond, but a ` +
    `user decryption needs ${threshold}. Holding the rotation Pending would also break the decryption the scenario ` +
    `requires to succeed, so the two halves cannot both hold on this topology — run this case on a committee with ` +
    `at least ${threshold + 1n} members.`
  );
};

/**
 * Fails fast when withholding `stalled` would also withhold the decryption quorum.
 *
 * Without this the run would stop one node, broadcast, and then spend the probe's full three-minute
 * timeout discovering that the decryption can never complete — reporting a failure that looks like
 * a product bug but is really an unsatisfiable topology.
 */
export const assertQuorumSurvivesStall = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  committee: readonly number[],
  stalled: number,
): Promise<void> => {
  const threshold = await evidence.step(
    "call",
    "read the user-decryption threshold for the active context",
    { contract: target.address, contextId: formatKmsId(contextId) },
    async () =>
      parseUintOutput(
        await castCall(
          target.rpcUrl,
          target.address,
          "getUserDecryptionThresholdForContext(uint256)(uint256)",
          contextId.toString(),
        ),
      ),
  );
  await evidence.step(
    "assert",
    "the decryption quorum survives the stalled party",
    {
      committee: committee.join(","),
      stalledParty: String(stalled),
      remaining: String(committee.length - 1),
      userDecryptionThreshold: threshold.toString(),
    },
    async () => {
      const shortfall = quorumShortfallAfterStall(committee.length, threshold, stalled);
      if (shortfall) throw new PreflightError(`kms-context-qa: ${shortfall}`);
    },
  );
};

/**
 * Proves a lifecycle operation is in flight, by requiring a second one to revert.
 *
 * The probe is an `eth_call`, not a transaction: it asks the contract what *would* happen without
 * changing anything, so it cannot itself disturb the Pending state it is measuring.
 */
export const assertLifecycleOperationInFlight = async (
  target: ProtocolConfigTarget,
  owner: Owner,
  evidence: CaseEvidence,
  why: string,
): Promise<void> => {
  await evidence.step(
    "assert",
    "a second lifecycle operation reverts KmsLifecycleOperationInFlight",
    { contract: target.address, error: LIFECYCLE_IN_FLIGHT_ERROR, why },
    () =>
      callContractAndExpectRevert(
        target.rpcUrl,
        target.address,
        owner,
        `kms-context-qa: ${why}`,
        LIFECYCLE_IN_FLIGHT_ERROR,
        "defineNewEpochForCurrentKmsContext()",
      ),
  );
};

/**
 * Asserts `isValidEpochForContext(contextId, epochId)` reports `expected`.
 *
 * Read twice by the pending case: false while the epoch is Pending, true once it activates. The
 * pair of readings is what turns "the pointer did not move" into "this specific epoch was created,
 * was not serving, and then was" — a claim the negative assertions alone cannot make.
 */
export const assertEpochValidity = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  epochId: bigint,
  expected: boolean,
  why: string,
): Promise<void> => {
  const actual = await evidence.step(
    "call",
    `isValidEpochForContext is ${expected} (${why})`,
    { contract: target.address, contextId: formatKmsId(contextId), epochId: formatKmsId(epochId) },
    () =>
      castBool(
        target.rpcUrl,
        target.address,
        "isValidEpochForContext(uint256,uint256)(bool)",
        contextId.toString(),
        epochId.toString(),
      ),
  );
  evidence.note("note", `isValidEpochForContext: result`, {
    contextId: formatKmsId(contextId),
    epochId: formatKmsId(epochId),
    valid: String(actual),
    expected: String(expected),
  });
  if (actual !== expected) {
    throw new PreflightError(
      `kms-context-qa: expected isValidEpochForContext(${contextId}, ${epochId}) to be ${expected} — ${why} — ` +
        `but the contract reports ${actual} on ${target.where}.`,
    );
  }
};
