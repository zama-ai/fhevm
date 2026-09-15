/**
 * QA case `context-switch-pending` — both halves of:
 *
 *   Scenario: A new request uses the previous context and epoch while a context switch is pending
 *     Given the active pair is "C1, E1"
 *     And governance has requested a switch to "C2, E2"
 *     And context "C2" was pre-registered in the Gateway
 *     And the switch status is "PENDING"
 *     And ProtocolConfig still returns "C1, E1" as the active pair
 *     When the application performs a decryption through the SDK while the status is "PENDING"
 *     And the test captures the request and response extraData
 *     Then the decryption must complete successfully
 *     And the request extraData must decode as version "0x02", context "C1", and epoch "E1"
 *     And the response extraData must be identical to the request extraData
 *     And pending context "C2" and pending epoch "E2" must not be used by the request
 *
 * The context-switch sibling of `case-epoch-rotation-pending`, and the fourth corner of the matrix
 * the profile is filling in: `epoch-rotation` and `context-switch` prove the SDK *follows* a
 * transition once it activates; the two `-pending` cases prove it does not *anticipate* one. Here
 * both ids are at stake, so the negative clause is twice as wide.
 *
 * ## Which "Pending" this is, and why it is the second one
 *
 * A context switch has two Pending stages, not one. `defineNewKmsContextAndEpoch` stores `C2` as
 * Pending and stops; the epoch is allocated later, inside `confirmKmsContextCreation`, once every
 * node of the new context has confirmed (`ProtocolConfig.sol:383-388`). So at stage 1 there is no
 * `E2` at all — the scenario's "switch to C2, E2" and "pending epoch E2" can only describe stage 2,
 * where the context is Created, the epoch is Pending, and the cores are resharing.
 *
 * Holding stage 1 would be simpler — stop a tx-sender before the broadcast and the creation quorum
 * never forms — but it would answer a different question, and the clause about `E2` would be
 * untestable rather than tested. This case therefore lets the creation quorum complete, watches for
 * the `NewKmsEpoch` that quorum emits, and only then withholds the *activation* confirmation.
 *
 * ## No receipt, so the ids come from the chain's logs
 *
 * `defineNewEpochForCurrentKmsContext` is a `cast send` whose receipt carries `NewKmsEpoch`. A
 * switch is not: it carries the whole committee definition, which lives in the contracts task's env
 * file, so it runs as a compose task and returns nothing. `case-context-switch` works around this by
 * predicting `C2 = C1 + 1` and verifying it only after activation — which is too late here, because
 * this case must name both ids *while* they are pending.
 *
 * `waitForNewKmsEpochEvent` recovers the event from a `cast logs` query instead. That yields `E2`
 * authoritatively and verifies the `C2` prediction at the same moment, before a single assertion is
 * built on top of either.
 *
 * ## The margin, stated plainly
 *
 * Between observing `NewKmsEpoch` and stopping the tx-sender there is a real, if wide, gap: the
 * cores' reshare, tens of seconds, against a one-second poll and a container stop of a few hundred
 * milliseconds. It is not a race in any practical sense, but it is not a lock either — so the case
 * re-reads the active pair immediately after the stop. A switch that activated inside that gap
 * fails loudly there rather than turning into a vacuous pass.
 *
 * ## The pending context's membership is not readable
 *
 * Both membership views — `getKmsSignersForContext` and `getKmsNodesForContext` — are gated on the
 * context being valid and revert `InvalidKmsContext(uint256)` until it activates. There is no third
 * view. So while the switch is held, the case cannot ask the chain which nodes are resharing; it
 * requires the reshare of the set the chain *will* name, `C1`'s committee, and then verifies after
 * activation that `C2` has the same membership. The assumption is deferred, not permanent.
 *
 * ## Why the stalled party is a serving one
 *
 * `host-sc.env` defines five KMS nodes but sets `NUM_KMS_NODES=4`, so the new context is built from
 * parties 1-4 — the same set that serves `C1`. The spare (party 5) is in neither, so stalling it
 * would withhold nothing and the switch would activate mid-probe. The stalled party must therefore
 * come from the committee that is also serving the decryption, exactly as in the epoch case, and
 * `assertQuorumSurvivesStall` is what keeps the two requirements from silently colliding.
 *
 * ## What this case does NOT cover
 *
 * *"The response extraData must be identical to the request extraData"* — the SDK neither verifies
 * nor exposes the response value. See `test-suite/fhevm/qa-extradata-check.md`.
 *
 * ## Disruptiveness
 *
 * Advances both the context and the epoch, and does not roll them back. Every container it stops is
 * restarted, by the supervisor's scope and again by the runner's outermost `finally`. Re-up between
 * runs: `fhevm-cli down && fhevm-cli up --scenario five-party-swap-threshold-kms`.
 */
import { PreflightError } from "../../errors";
import { checkConnectorsDbColumn, columnQuery } from "../../kms-connector-db";
import type { QaCase, QaCaseContext } from "../registry";
import { assertTxSendersRunning } from "../nodes";
import {
  assertContextValidity,
  assertEpochValidity,
  assertLifecycleOperationInFlight,
  assertQuorumSurvivesStall,
  pickStallParty,
  readBlockNumber,
  waitForNewKmsEpochEvent,
} from "../pending";
import {
  assertPairUnchanged,
  broadcastContextSwitch,
  formatKmsId,
  preRegisterContextOnGateway,
  readCommitteeParties,
  readCurrentPair,
  waitForActivation,
} from "../protocol-config";

/**
 * Runs the pending-switch scenario.
 *
 * Sequence: read baseline -> pick and clear a party to stall -> broadcast the switch -> let the
 * creation quorum allocate E2 and read it from the logs -> withhold the activation confirmation ->
 * pre-register on the Gateway -> establish PENDING from four angles -> prove C1/E1 still serve, on
 * chain and through the SDK -> restore -> watch the withheld confirmation land and the pair
 * activate.
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
      "kms-context-qa/context-switch-pending: no KMS signers in persisted discovery — cannot resolve which parties " +
        "serve the active context. The stack must have completed the discover step.",
    );
  }
  const committee = await readCommitteeParties(target, evidence, baseline.contextId, kmsSigners);

  // Every provisioned party, not just the serving committee: the new context's node set comes from
  // the contracts task's env file, and a node of the NEW context whose tx-sender is already down
  // would hold the switch at stage 1, where the epoch this case needs is never allocated.
  await assertTxSendersRunning(
    Array.from({ length: state.scenario.kms.parties }, (_, index) => index + 1),
    evidence,
    "A context switch needs every node of the new context to confirm its creation before the epoch even exists.",
  );

  const stalledParty = pickStallParty(committee);
  evidence.note("note", "party chosen to withhold the activation confirmation", {
    stalledParty: String(stalledParty),
    committee: committee.join(","),
  });

  // The stalled party serves C1 as well as belonging to C2, so withholding its confirmation also
  // costs it a decryption response. Check the rest still reach the threshold, in seconds rather
  // than after the probe's three-minute timeout.
  await assertQuorumSurvivesStall(target, evidence, baseline.contextId, committee, stalledParty);

  await runSmoke(
    `kms-context-qa/context-switch-pending: input-proof at baseline (contextId=${baseline.contextId})`,
  );

  // Bound the log query to what follows: older NewKmsEpoch logs exist under previous contexts on
  // any stack that has rotated before.
  const fromBlock = await evidence.step("call", "read the host block height before the switch", {}, () =>
    readBlockNumber(target.rpcUrl),
  );

  // Context ids are allocated sequentially by `_storeNextKmsContext`, so the pending id is knowable
  // before the chain reports it. A prediction — verified below against the event itself, not merely
  // after activation.
  const pendingContextId = baseline.contextId + 1n;
  evidence.note("note", "pending context id predicted from sequential allocation", {
    baselineContextId: formatKmsId(baseline.contextId),
    pendingContextId: formatKmsId(pendingContextId),
  });

  // When: governance defines the new context. Stage 1 — C2 is Pending and no epoch exists yet.
  await broadcastContextSwitch(state, evidence);

  // Stage 2: the creation quorum forms and allocates E2. This is the event that tells us the id,
  // and it is also the moment after which the activation confirmation becomes withholdable.
  const event = await waitForNewKmsEpochEvent(target, evidence, pendingContextId, fromBlock);

  await evidence.step(
    "assert",
    "the event confirms the predicted context and supersedes the baseline pair",
    {
      predictedContextId: formatKmsId(pendingContextId),
      eventContextId: formatKmsId(event.contextId),
      eventEpochId: formatKmsId(event.epochId),
      eventPreviousContextId: formatKmsId(event.previousContextId),
      eventPreviousEpochId: formatKmsId(event.previousEpochId),
    },
    async () => {
      if (event.contextId !== pendingContextId) {
        throw new PreflightError(
          `kms-context-qa/context-switch-pending: NewKmsEpoch names context ${event.contextId}, but the sequential ` +
            `allocation predicted ${pendingContextId}. Another lifecycle operation ran concurrently, so every ` +
            `downstream assertion would be made against an ambiguous baseline — rerun on a quiet cluster.`,
        );
      }
      if (event.previousContextId !== baseline.contextId || event.previousEpochId !== baseline.epochId) {
        throw new PreflightError(
          `kms-context-qa/context-switch-pending: NewKmsEpoch reports it supersedes ` +
            `(${event.previousContextId}, ${event.previousEpochId}) but the pair read before the broadcast was ` +
            `(${baseline.contextId}, ${baseline.epochId}).`,
        );
      }
      if (event.epochId === baseline.epochId) {
        throw new PreflightError(
          `kms-context-qa/context-switch-pending: the switch reused epoch ${event.epochId}; a new context must open ` +
            `a new epoch.`,
        );
      }
    },
  );

  await nodes.withTxSendersStopped(
    [stalledParty],
    `withhold party ${stalledParty}'s activation confirmation`,
    evidence,
    async () => {
      // Given, reading 1 of 4, and the first thing checked after the stop: the switch must not have
      // activated in the gap between observing the event and withholding the confirmation.
      await assertPairUnchanged(
        target,
        evidence,
        baseline,
        "withholding the activation confirmation (the switch must not have activated in the gap)",
      );

      // Given: context C2 was pre-registered in the Gateway. Ordered before activation — which is
      // now held — so a fresh SDK client can never observe a context active on the host that the
      // Gateway would reject.
      await preRegisterContextOnGateway(state, evidence, pendingContextId);

      // Given, reading 2 of 4: a second lifecycle operation must be refused while one is settling.
      await assertLifecycleOperationInFlight(
        target,
        owner,
        evidence,
        "a context switch is in flight, so a second lifecycle operation must be refused",
      );

      // Given, readings 3 and 4 — the positive ones: both ids exist and neither is serving yet.
      await assertContextValidity(
        target,
        evidence,
        pendingContextId,
        false,
        "a context awaiting epoch activation is not yet valid",
      );
      await assertEpochValidity(
        target,
        evidence,
        pendingContextId,
        event.epochId,
        false,
        "a Pending epoch is not yet valid for its context",
      );

      // Pending must mean "awaiting confirmation", not "the KMS never finished".
      //
      // Which nodes to require it of is the awkward part: the resharing set belongs to the NEW
      // context, and the chain will not disclose it while that context is pending. Both membership
      // views — getKmsSignersForContext and getKmsNodesForContext — are gated on validity and
      // revert InvalidKmsContext(uint256) until activation. There is no third view.
      //
      // So the check runs against the set the chain *will* disclose, C1's serving committee, and
      // the assumption that C2 has the same membership is verified after activation rather than
      // assumed permanently — see the assertion following the hold.
      evidence.note("note", "resharing set used while the switch is pending", {
        serving: committee.join(","),
        why: "the pending context's membership is not readable (getKmsSignersForContext reverts InvalidKmsContext)",
        verifiedAfter: "activation",
      });

      await evidence.step(
        "assert",
        "every node of the serving committee completed the reshare while the confirmation is withheld",
        { parties: committee.join(","), epochId: formatKmsId(event.epochId) },
        async () =>
          checkConnectorsDbColumn(
            committee,
            `epoch ${event.epochId} reshare completed (activation still withheld)`,
            columnQuery("new_kms_epoch", "epoch_id", "status", event.epochId),
            ["completed"],
          ),
      );

      // Then: the decryption must complete successfully — under C1/E1, the pair still active.
      const decrypted = await evidence.step(
        "probe",
        "user-decryption while the context switch is pending",
        {
          activeContextId: formatKmsId(baseline.contextId),
          activeEpochId: formatKmsId(baseline.epochId),
          pendingContextId: formatKmsId(pendingContextId),
          pendingEpochId: formatKmsId(event.epochId),
        },
        () =>
          runDecryption(
            `kms-context-qa/context-switch-pending: decrypt while the switch to contextId=${pendingContextId} is pending`,
          ),
      );
      if (!decrypted) {
        throw new PreflightError(
          `kms-context-qa/context-switch-pending: user-decryption failed while the switch to context ` +
            `${pendingContextId} was pending. The active pair is still (${baseline.contextId}, ` +
            `${baseline.epochId}), so the previous context must keep serving until the new one activates — a ` +
            `Pending switch must not interrupt traffic. Check the kms-core and kms-connector-*-kms-worker logs.`,
        );
      }

      // The one place the full uint256 ids are printed: these are what the container asserts and
      // what you paste into `cast`. Everywhere else the evidence uses the short `ctx#n` form.
      evidence.note("note", "raw ids handed to the container-side extraData check", {
        activeContext: baseline.contextId.toString(),
        activeEpoch: baseline.epochId.toString(),
        forbiddenPendingContext: pendingContextId.toString(),
        forbiddenPendingEpoch: event.epochId.toString(),
        expectedExtraDataVersion: "0x02",
      });

      // Then: the permit must carry C1 and E1, and neither pending id. Both forbidden ids are in
      // the future — the SDK must not read ahead of activation on either axis.
      await evidence.step(
        "probe",
        "SDK embeds the still-active pair, not the pending one, in the permit extraData",
        {
          contextId: formatKmsId(baseline.contextId),
          epochId: formatKmsId(baseline.epochId),
          forbiddenContextId: formatKmsId(pendingContextId),
          forbiddenEpochId: formatKmsId(event.epochId),
        },
        () =>
          runExtraDataCheck(
            `kms-context-qa/context-switch-pending: extraData still carries contextId=${baseline.contextId}`,
            {
              contextId: baseline.contextId,
              epochId: baseline.epochId,
              forbiddenContextId: pendingContextId,
              forbiddenEpochId: event.epochId,
            },
          ),
      );

      // The probes take about three minutes. Re-read rather than assume the hold survived them.
      await assertPairUnchanged(target, evidence, baseline, "the probes run inside the pending window");
    },
  );

  // Leaving the scope restarted the tx-sender. Its confirmation was queued, not lost, so activation
  // should follow quickly — and that it follows at all is the proof that the hold was the withheld
  // confirmation rather than anything wrong with the switch.
  const activated = await waitForActivation(
    target,
    evidence,
    `context switch to (${formatKmsId(pendingContextId)}, ${formatKmsId(event.epochId)}) once the withheld ` +
      `confirmation is restored`,
    (current) => current.contextId === pendingContextId && current.epochId === event.epochId,
  );

  await assertContextValidity(
    target,
    evidence,
    activated.contextId,
    true,
    "the activated context is valid",
  );

  // Now — and only now — the chain will name the new context's members. Resolving them here closes
  // the one assumption the hold had to make: the reshare assertion ran against C1's committee
  // because C2's membership was unreadable, and this is where that becomes a verified claim rather
  // than an assumption. A switch that changed membership would make that assertion unsound, so it
  // is reported as such instead of being quietly tolerated.
  const newContextParties = await readCommitteeParties(target, evidence, activated.contextId, kmsSigners);
  await evidence.step(
    "assert",
    "the activated context has the membership the pending-window reshare check assumed",
    { serving: committee.join(","), activated: newContextParties.join(",") },
    async () => {
      if (newContextParties.join(",") !== committee.join(",")) {
        throw new PreflightError(
          `kms-context-qa/context-switch-pending: the switch changed the node set from ${committee.join(",")} to ` +
            `${newContextParties.join(",")}. The reshare check inside the pending window necessarily ran against ` +
            `the former, because a pending context's membership is not readable on chain — so that check was made ` +
            `against the wrong set and this run's evidence for it cannot be trusted. Run this case against a ` +
            `same-committee switch (the default host-sc.env), not a node swap.`,
        );
      }
    },
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
    `[kms-context-qa] context-switch-pending complete: the pair (${baseline.contextId}, ${baseline.epochId}) kept ` +
      `serving for the whole time (${pendingContextId}, ${event.epochId}) was pending, and the permit carried ` +
      `neither. Releasing party ${stalledParty}'s confirmation then activated both. The response-extraData echo ` +
      `remains uncovered — the SDK neither verifies nor exposes it (see test-suite/fhevm/qa-extradata-check.md).`,
  );
};

/** Registry entry for the `context-switch-pending` case. */
export const contextSwitchPendingCase: QaCase = {
  id: "context-switch-pending",
  title: "SDK keeps using the previous context and epoch while a switch is Pending (both halves)",
  proves:
    "a context switch whose activation confirmation is withheld reaches its second Pending stage — context Created, " +
    "epoch Pending — without moving the active pair, the previous context keeps serving a real user decryption " +
    "throughout, the SDK embeds that previous pair and neither pending id in the permit extraData, and releasing " +
    "the confirmation activates both",
  requirements: {
    mode: "threshold",
    // The stalled party belongs to both the new context and the committee serving the old one, so
    // the remaining members must still reach the user-decryption threshold. See the epoch-pending
    // case for why 4 rather than 3; the exact check runs at case time against the contract.
    minCommitteeSize: 4,
  },
  mutatesLifecycle: true,
  run,
};
