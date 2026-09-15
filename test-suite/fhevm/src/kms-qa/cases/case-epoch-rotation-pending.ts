/**
 * QA case `epoch-rotation-pending` — both halves of:
 *
 *   Scenario: A new request uses the previous epoch while an epoch rotation is pending
 *     Given the active pair is "C1, E1"
 *     And governance has requested a rotation to epoch "E2"
 *     And the rotation status is "PENDING"
 *     And ProtocolConfig still returns "C1, E1" as the active pair
 *     When the application performs a decryption through the SDK while the status is "PENDING"
 *     And the test captures the request and response extraData
 *     Then the decryption must complete successfully
 *     And the request extraData must decode as version "0x02", context "C1", and epoch "E1"
 *     And the response extraData must be identical to the request extraData
 *     And pending epoch "E2" must not be used by the request
 *
 * The mirror image of `case-epoch-rotation`: that case proves the SDK *follows* a rotation once it
 * activates, this one proves it does not *anticipate* one. Together they pin the SDK to the pair the
 * chain reports, from both sides.
 *
 * ## Why the window is held open, and why that is not cheating
 *
 * The scenario asks for a decryption *while the rotation is Pending*. Left alone that window is
 * about a minute — 65.6s over 14 polls on the run recorded in
 * `qa-kms-context-scenario-1-epoch.md` — and the two probes this case runs inside it take roughly
 * three. The natural window is not merely tight, it is a coin flip, and the container spec reads the
 * active pair in its `before()` hook, so a rotation landing mid-suite fails the test for a reason
 * that has nothing to do with the SDK.
 *
 * So the window is held: one committee member's tx-sender is stopped before the broadcast, which
 * withholds that party's `confirmEpochActivation`. Activation needs *every* signer of the context,
 * so the epoch reshares and then waits. Nothing about the epoch's state is faked — it is Pending for
 * exactly the reason the scenario names, and the case proves the reshare itself completed
 * (`new_kms_epoch.status = completed` on every committee node) so that "Pending" cannot be confused
 * with "stuck". Restoring the node at the end lets the queued confirmation go out and the epoch
 * activate, which is the proof that the hold was the confirmation and nothing else.
 *
 * This is the lever `src/kms-qa/nodes.ts` was written for, and its first call site.
 *
 * ## Proving PENDING when no view exposes it
 *
 * `ProtocolConfig` keeps `epochState` in private storage with no getter. Three independent readings
 * establish the state instead (see `src/kms-qa/pending.ts`): the active pair has not moved, a second
 * lifecycle operation reverts `KmsLifecycleOperationInFlight`, and `isValidEpochForContext(C1, E2)`
 * is still false. The last is read again after activation, where it must be true.
 *
 * ## What this case does NOT cover
 *
 * *"The response extraData must be identical to the request extraData"* — the SDK neither verifies
 * nor exposes the response value. The evidence and the decision to defer are in
 * `test-suite/fhevm/qa-extradata-check.md`, unchanged since scenario 1.
 *
 * ## Disruptiveness
 *
 * Advances the active epoch and does not roll it back. Every container it stops is restarted, by the
 * supervisor's scope and again by the runner's outermost `finally`. Re-up between runs:
 * `fhevm-cli down && fhevm-cli up --scenario five-party-swap-threshold-kms`.
 */
import { PreflightError } from "../../errors";
import { checkConnectorsDbColumn, columnQuery } from "../../kms-connector-db";
import type { QaCase, QaCaseContext } from "../registry";
import { assertTxSendersRunning } from "../nodes";
import {
  assertEpochValidity,
  assertLifecycleOperationInFlight,
  assertQuorumSurvivesStall,
  pickStallParty,
} from "../pending";
import {
  assertPairUnchanged,
  assertRotationConsistency,
  formatKmsId,
  readCommitteeParties,
  readCurrentPair,
  sendDefineNewEpoch,
  waitForActivation,
} from "../protocol-config";

/**
 * Runs the pending-rotation scenario.
 *
 * Sequence: read baseline -> resolve the live committee -> pick and clear a party to stall -> hold
 * its tx-sender down while broadcasting -> establish PENDING from three angles -> prove the previous
 * epoch still serves, on chain and through the SDK -> restore -> watch the withheld confirmation
 * land and the epoch activate.
 */
const run = async (ctx: QaCaseContext): Promise<void> => {
  const { state, target, owner, nodes, evidence, runDecryption, runSmoke, runExtraDataCheck } = ctx;

  evidence.note("note", "target", {
    protocolConfig: target.address,
    rpcUrl: target.rpcUrl,
    where: target.where,
    owner: owner.address,
  });

  // Given: the pair active before anything happens — C1 and E1, the pair that must keep serving.
  const baseline = await readCurrentPair(target, evidence, "baseline active pair");

  const kmsSigners = state.discovery?.kmsSigners ?? [];
  if (!kmsSigners.length) {
    throw new PreflightError(
      "kms-context-qa/epoch-rotation-pending: no KMS signers in persisted discovery — cannot resolve which parties " +
        "serve the active context. The stack must have completed the discover step.",
    );
  }
  const committee = await readCommitteeParties(target, evidence, baseline.contextId, kmsSigners);

  // Everyone must be up *before* we take one down deliberately. A stack that has run
  // kms-context-switch's node swap already has a tx-sender stopped; starting from there, the final
  // activation could never reach quorum even after this case restores its own node, and the run
  // would end in a 600s timeout that blames the wrong thing.
  await assertTxSendersRunning(
    committee,
    evidence,
    "This case withholds exactly one committee confirmation on purpose; a second one already missing would make the " +
      "rotation unactivatable even after the hold is released.",
  );

  const stalledParty = pickStallParty(committee);
  evidence.note("note", "party chosen to withhold the activation confirmation", {
    stalledParty: String(stalledParty),
    committee: committee.join(","),
  });

  // The scenario needs the decryption to SUCCEED while the rotation is pending. Stopping a tx-sender
  // costs that party its response, so check the rest of the committee still reaches the threshold —
  // in seconds, rather than after the probe's three-minute timeout.
  await assertQuorumSurvivesStall(target, evidence, baseline.contextId, committee, stalledParty);

  // Baseline smoke, before anything is withheld, so a failure inside the held window is attributable
  // to the window rather than to a cluster that was already unhealthy.
  await runSmoke(
    `kms-context-qa/epoch-rotation-pending: input-proof at baseline (epochId=${baseline.epochId})`,
  );

  const pendingEpochId = await nodes.withTxSendersStopped(
    [stalledParty],
    `withhold party ${stalledParty}'s activation confirmation`,
    evidence,
    async () => {
      // When: governance requests the rotation. With one confirmation withheld it opens a Pending
      // epoch and stays there.
      const { event } = await sendDefineNewEpoch(target, owner, evidence);

      await evidence.step(
        "assert",
        "the pending rotation is consistent with the baseline",
        {
          baselineContextId: formatKmsId(baseline.contextId),
          baselineEpochId: formatKmsId(baseline.epochId),
          eventPreviousEpochId: formatKmsId(event.previousEpochId),
          eventEpochId: formatKmsId(event.epochId),
        },
        async () => assertRotationConsistency(baseline, event),
      );

      // Given, reading 1 of 3: ProtocolConfig still returns C1, E1.
      await assertPairUnchanged(target, evidence, baseline, "broadcasting the rotation with a confirmation withheld");

      // Given, reading 2 of 3: the rotation status is PENDING. No view says so, but the contract's
      // own gate does — `_checkNoKmsLifecycleOperationInFlight` reverts exactly while the latest
      // epoch is Pending.
      await assertLifecycleOperationInFlight(
        target,
        owner,
        evidence,
        "a rotation is in flight, so a second lifecycle operation must be refused",
      );

      // Given, reading 3 of 3, and the only positive one: E2 exists but is not yet serving.
      await assertEpochValidity(
        target,
        evidence,
        baseline.contextId,
        event.epochId,
        false,
        "a Pending epoch is not yet valid for its context",
      );

      // Pending must mean "awaiting confirmation", not "the KMS never finished". The connector DB
      // records each party's reshare result independently of whether its confirmation went out, so
      // requiring `completed` here is what separates a held rotation from a broken one — and it
      // settles the cluster before the probes, so they measure the SDK and not a reshare in flight.
      await evidence.step(
        "assert",
        "every committee node completed the reshare while the confirmation is withheld",
        { parties: committee.join(","), epochId: formatKmsId(event.epochId) },
        async () =>
          checkConnectorsDbColumn(
            committee,
            `epoch ${event.epochId} reshare completed (activation still withheld)`,
            columnQuery("new_kms_epoch", "epoch_id", "status", event.epochId),
            ["completed"],
          ),
      );

      // Then: the decryption must complete successfully — under E1, the epoch that is still active.
      const decrypted = await evidence.step(
        "probe",
        "user-decryption while the rotation is pending",
        {
          contextId: formatKmsId(baseline.contextId),
          activeEpochId: formatKmsId(baseline.epochId),
          pendingEpochId: formatKmsId(event.epochId),
        },
        () =>
          runDecryption(
            `kms-context-qa/epoch-rotation-pending: decrypt while the rotation to epochId=${event.epochId} is pending`,
          ),
      );
      if (!decrypted) {
        throw new PreflightError(
          `kms-context-qa/epoch-rotation-pending: user-decryption failed while the rotation to epoch ` +
            `${event.epochId} was pending. The active pair is still (${baseline.contextId}, ${baseline.epochId}), so ` +
            `the previous epoch must keep serving until the new one activates — a Pending rotation must not ` +
            `interrupt traffic. Check the kms-core and kms-connector-*-kms-worker logs.`,
        );
      }

      // The one place the full uint256 ids are printed: these are what the container asserts and
      // what you paste into `cast`. Everywhere else the evidence uses the short `ctx#n` form.
      evidence.note("note", "raw ids handed to the container-side extraData check", {
        context: baseline.contextId.toString(),
        activeEpoch: baseline.epochId.toString(),
        forbiddenPendingEpoch: event.epochId.toString(),
        expectedExtraDataVersion: "0x02",
      });

      // Then: the permit must carry E1, and must NOT carry the pending E2. Unlike the other cases
      // the forbidden id is in the future — the SDK must not read ahead of activation.
      await evidence.step(
        "probe",
        "SDK embeds the still-active epoch, not the pending one, in the permit extraData",
        {
          contextId: formatKmsId(baseline.contextId),
          epochId: formatKmsId(baseline.epochId),
          forbiddenEpochId: formatKmsId(event.epochId),
        },
        () =>
          runExtraDataCheck(
            `kms-context-qa/epoch-rotation-pending: extraData still carries epochId=${baseline.epochId}`,
            {
              contextId: baseline.contextId,
              epochId: baseline.epochId,
              forbiddenEpochId: event.epochId,
            },
          ),
      );

      // The probes take about three minutes. Re-read rather than assume the hold survived them.
      await assertPairUnchanged(target, evidence, baseline, "the probes run inside the pending window");

      return event.epochId;
    },
  );

  // Leaving the scope restarted the tx-sender. Its confirmation was queued, not lost, so activation
  // should follow quickly — and that it follows at all is the proof that the hold was the withheld
  // confirmation rather than anything wrong with the rotation.
  const activated = await waitForActivation(
    target,
    evidence,
    `epoch rotation to epochId=${pendingEpochId} once the withheld confirmation is restored`,
    (current) => current.contextId === baseline.contextId && current.epochId === pendingEpochId,
  );

  await assertEpochValidity(
    target,
    evidence,
    activated.contextId,
    activated.epochId,
    true,
    "the activated epoch is valid for its context",
  );

  console.log(
    `[kms-context-qa] epoch-rotation-pending complete: the pair (${baseline.contextId}, ${baseline.epochId}) kept ` +
      `serving for the whole time epoch ${pendingEpochId} was Pending, and the permit never carried it. Releasing ` +
      `party ${stalledParty}'s confirmation then activated it. The response-extraData echo remains uncovered — the ` +
      `SDK neither verifies nor exposes it (see test-suite/fhevm/qa-extradata-check.md).`,
  );
};

/** Registry entry for the `epoch-rotation-pending` case. */
export const epochRotationPendingCase: QaCase = {
  id: "epoch-rotation-pending",
  title: "SDK keeps using the previous epoch while a rotation is Pending (both halves)",
  proves:
    "a rotation whose activation confirmation is withheld stays Pending without moving the active pair, the " +
    "previous epoch keeps serving a real user decryption throughout, the SDK embeds that previous epoch — never " +
    "the pending one — in the permit extraData, and releasing the confirmation activates the epoch",
  requirements: {
    mode: "threshold",
    // One member is stalled and the rest must still reach the user-decryption threshold. The live
    // run measured that threshold at 3 on a 4-member committee, so a committee of 3 would leave 2
    // and could not serve the decryption this scenario requires to succeed — 4 is the real floor.
    // The exact check against the contract's own threshold still runs at case time, because the
    // relationship between committee size and threshold is the contract's to define, not ours.
    minCommitteeSize: 4,
  },
  mutatesLifecycle: true,
  run,
};
