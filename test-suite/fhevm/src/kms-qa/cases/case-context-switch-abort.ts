/**
 * QA case `context-switch-abort-and-retry` — **two QA scenarios, merged into one case.**
 *
 * They were proposed separately, and the second turned out to be the literal continuation of the
 * first: everything it lists as `Given` is what the first one *leaves behind*, and its `When`/`Then`
 * were already the first one's recovery step. Implementing them apart would have meant a second case
 * that re-created, from a clean stack, exactly the state the first case ends in — paying six minutes
 * to reach a starting line the other one crosses on its way past.
 *
 * So they are one case. The seam is the state itself, not any setup code.
 *
 *   Scenario A: Governance aborts a context switch that does not reach quorum
 *     Given the active pair is "C1, E1"
 *     And governance has requested a switch to "C2, E2"
 *     And context "C2" was pre-registered in the Gateway
 *     And the confirmations required to create "C2" have been withheld
 *     And the switch status is "PENDING"
 *     And ProtocolConfig still returns "C1, E1" as the active pair
 *     When governance executes destroyKmsContext for "C2"
 *     Then the status must indicate that the switch was aborted
 *     And ProtocolConfig must continue returning "C1, E1" as the active pair
 *
 *   Scenario B: Governance retries a switch after cancelling a context pre-registered in the Gateway
 *     Given the active pair is "C1, E1"
 *     And the switch to "C2, E2" was aborted by destroyKmsContext
 *     And "C2" remains registered in the Gateway
 *     And ProtocolConfig still returns "C1, E1" as the active pair
 *     And the status indicates no pending transition
 *     When governance requests a new switch to "C3, E3"
 *     And context "C3" is registered in the Gateway
 *     And all required confirmations for "C3, E3" complete with compatible results
 *     Then "C3, E3" must become the active pair
 *
 * Scenario B's five `Given` clauses map one-to-one onto assertions scenario A already makes — the
 * abort, the orphaned Gateway registration, the unchanged pair, and the reopened gate. Nothing has to
 * be set up for B; it starts where A stops.
 *
 * ## What the merge added, beyond concatenation
 *
 * Scenario B asks for confirmations that complete *"with compatible results"*. Activation alone does
 * not say that: it proves every signer voted, not that the work behind the votes agreed. The recovery
 * step therefore also requires `new_kms_epoch.status = completed` on every serving node — the
 * connector's independent record of each reshare. That assertion exists because B was merged in; A
 * did not need it.
 *
 * ## What is already covered, and what this case is actually for
 *
 * The contract-level claims above are **already proven twice**, and this case does not exist to
 * prove them a third time:
 *
 *   - `host-contracts/test/protocolConfig/protocolConfig.t.sol` —
 *     `test_destroyPendingContextClearsPairedEpoch` asserts the event, both validity views and the
 *     unchanged pair, in Foundry;
 *   - `host-contracts/test/tasks/kmsContext.ts:230` — *"broadcasts the destruction of a non-current
 *     PENDING context"* asserts `aborted == true` / `abortReason == "context-destroyed"` through the
 *     status task.
 *
 * What neither can reach, because neither has a cluster or a second chain:
 *
 *   1. **Withholding real confirmations.** The unit tests get a Pending context because no node ever
 *      confirms; here a real committee is up and one node's tx-sender is stopped on purpose, so the
 *      quorum is withheld rather than absent.
 *   2. **The Gateway divergence.** `C2` is pre-registered on the Gateway, as the scenario requires.
 *      The Gateway keeps its own registry and its own owner-gated `destroyKmsContext`
 *      (`GatewayConfig.sol:327`), so aborting on the host does NOT clean it up: afterwards the
 *      Gateway still holds a context the host destroyed and will never activate. No unit test spans
 *      both chains, so nothing has ever observed this. The case asserts it as a canary — if
 *      cross-chain cleanup is ever added, this fails and forces a conscious update.
 *   3. **That the abort does not disturb service.** `C1, E1` must still decrypt afterwards.
 *   4. **That the gate really reopened**, end to end: a fresh switch must activate on a live cluster,
 *      not merely be accepted by an `eth_call`.
 *
 * ## Stage 1, and why this is the sibling of `context-switch-pending`
 *
 * A context switch has two Pending stages (see `src/kms-qa/pending.ts`). `context-switch-pending`
 * deliberately targets stage 2 — context Created, epoch Pending — because its scenario names a
 * pending `E2`. This one is stage 1: the tx-sender is stopped **before** the broadcast, so
 * `_hasContextCreationQuorum` never holds, `_createPendingEpoch` never runs, and there is no `E2` at
 * all. The case proves that rather than assuming it, by requiring that no `NewKmsEpoch` was emitted.
 *
 * That is also why the scenario's "switch to C2, E2" is implemented as a switch to `C2` only.
 *
 * ## How "the status indicates the switch was aborted" is asserted
 *
 * No view exposes it, and the status task that does has no compose service here. The observable
 * meaning is used instead: the in-flight gate reopens. Before the destroy a second lifecycle
 * operation reverts `KmsLifecycleOperationInFlight`; after it, the same call succeeds — which is
 * exactly what the contract documents, destroyed entries counting as `None`.
 *
 * ## Disruptiveness
 *
 * Destroys a context and, in its recovery step, advances the context and epoch — but it assumes
 * nothing about where it starts: the baseline pair and the committee are read from the chain, and the
 * ids come from sequential allocation. It needs every tx-sender running (it withholds one, and checks
 * this itself) and no lifecycle operation in flight — both of which are the state it leaves behind,
 * so consecutive runs work with no `down`/`up` in between. Verified: a run started on the previous
 * run's output (ctx#3/epoch#2 -> aborted ctx#4 -> recovered to ctx#5) passed.
 */
import { PreflightError } from "../../errors";
import { checkConnectorsDbColumn, columnQuery } from "../../kms-connector-db";
import { castSend, keccakTopic, getEventTopic } from "../../kms-onchain";
import type { QaCase, QaCaseContext } from "../registry";
import { assertTxSendersRunning } from "../nodes";
import {
  assertContextValidity,
  assertGatewayContextValidity,
  assertLifecycleGateOpen,
  assertLifecycleOperationInFlight,
  assertNoNewKmsEpochEvent,
  assertQuorumSurvivesStall,
  pickStallParty,
  readBlockNumber,
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
import { txEvidenceFields } from "../evidence";

/** Solidity signature of the event `destroyKmsContext` emits. */
const KMS_CONTEXT_DESTROYED_SIGNATURE = "KmsContextDestroyed(uint256)";

const run = async (ctx: QaCaseContext): Promise<void> => {
  const { state, target, owner, nodes, evidence, runDecryption, runSmoke } = ctx;

  evidence.note("note", "target", {
    protocolConfig: target.address,
    rpcUrl: target.rpcUrl,
    where: target.where,
    owner: owner.address,
  });

  // The Gateway is read directly, to observe what the abort does NOT do to it.
  const gatewayRpcUrl = state.discovery?.endpoints.gateway.http;
  const gatewayConfigAddress = state.discovery?.gateway.GATEWAY_CONFIG_ADDRESS;
  if (!gatewayRpcUrl || !gatewayConfigAddress) {
    throw new PreflightError(
      "kms-context-qa/context-switch-abort: no Gateway RPC or GatewayConfig address in persisted discovery — the " +
        "case pre-registers the pending context there and then observes that the host-side abort leaves it behind.",
    );
  }

  // Given: the pair active before anything happens — the pair that must survive the whole case.
  const baseline = await readCurrentPair(target, evidence, "baseline active pair");

  const kmsSigners = state.discovery?.kmsSigners ?? [];
  if (!kmsSigners.length) {
    throw new PreflightError(
      "kms-context-qa/context-switch-abort: no KMS signers in persisted discovery — cannot resolve which parties " +
        "serve the active context. The stack must have completed the discover step.",
    );
  }
  const committee = await readCommitteeParties(target, evidence, baseline.contextId, kmsSigners);

  await assertTxSendersRunning(
    Array.from({ length: state.scenario.kms.parties }, (_, index) => index + 1),
    evidence,
    "This case withholds exactly one creation confirmation on purpose; a second one already missing would make the " +
      "recovery switch at the end unactivatable too.",
  );

  const stalledParty = pickStallParty(committee);
  evidence.note("note", "party chosen to withhold the creation confirmation", {
    stalledParty: String(stalledParty),
    committee: committee.join(","),
  });

  // The abort must not cost the previous pair its ability to serve, and the case asserts exactly
  // that below — so the remaining committee has to reach the decryption threshold.
  await assertQuorumSurvivesStall(target, evidence, baseline.contextId, committee, stalledParty);

  await runSmoke(`kms-context-qa/context-switch-abort: input-proof at baseline (contextId=${baseline.contextId})`);

  const fromBlock = await evidence.step("call", "read the host block height before the switch", {}, () =>
    readBlockNumber(target.rpcUrl),
  );

  const pendingContextId = baseline.contextId + 1n;
  evidence.note("note", "pending context id predicted from sequential allocation", {
    baselineContextId: formatKmsId(baseline.contextId),
    pendingContextId: formatKmsId(pendingContextId),
  });

  const destroyTopic = await keccakTopic(KMS_CONTEXT_DESTROYED_SIGNATURE);

  await nodes.withTxSendersStopped(
    [stalledParty],
    `withhold party ${stalledParty}'s creation confirmation`,
    evidence,
    async () => {
      // When: governance defines the new context with one node unable to transact. Unlike
      // `context-switch-pending`, the stop happens BEFORE the broadcast, so the creation quorum
      // never forms and the switch stays at stage 1.
      await broadcastContextSwitch(state, evidence);

      // Given: context C2 was pre-registered in the Gateway. Doing it here, on a switch that will be
      // aborted, is what sets up the divergence observed after the destroy.
      await preRegisterContextOnGateway(state, evidence, pendingContextId);

      // Given, reading 1 of 4: the active pair did not move.
      await assertPairUnchanged(target, evidence, baseline, "broadcasting the switch with a creation confirmation withheld");

      // Given, reading 2 of 4: an operation is in flight — the switch is Pending.
      await assertLifecycleOperationInFlight(
        target,
        owner,
        evidence,
        "a context switch is in flight, so a second lifecycle operation must be refused",
      );

      // Given, reading 3 of 4: the context exists but is not serving.
      await assertContextValidity(
        target,
        evidence,
        pendingContextId,
        false,
        "a context awaiting its creation quorum is not yet valid",
      );

      // Given, reading 4 of 4, and the one that pins this to STAGE 1: no epoch was ever allocated,
      // because `_createPendingEpoch` runs only once the creation quorum holds.
      await assertNoNewKmsEpochEvent(
        target,
        evidence,
        pendingContextId,
        fromBlock,
        "The withheld confirmation should have prevented the creation quorum entirely.",
      );

      // When: governance aborts.
      const receipt = await evidence.step(
        "tx",
        `destroyKmsContext(${formatKmsId(pendingContextId)})`,
        { contract: target.address, from: owner.address, contextId: formatKmsId(pendingContextId) },
        () => castSend(target.rpcUrl, target.address, owner, "destroyKmsContext(uint256)", pendingContextId.toString()),
      );
      evidence.note("tx", "destroyKmsContext: receipt", txEvidenceFields(receipt));

      await evidence.step(
        "assert",
        "the destroy names the pending context",
        { contextId: formatKmsId(pendingContextId), topic0: destroyTopic },
        async () => {
          const destroyed = getEventTopic(receipt, destroyTopic, 1);
          if (destroyed !== pendingContextId) {
            throw new PreflightError(
              `kms-context-qa/context-switch-abort: KmsContextDestroyed names ${destroyed}, expected ` +
                `${pendingContextId}.`,
            );
          }
        },
      );

      // Then: ProtocolConfig must continue returning the original pair.
      await assertPairUnchanged(target, evidence, baseline, "destroying the pending context");

      // Then: the status must indicate the switch was aborted. No view says so; the gate reopening
      // is what that means, and it is the inverse of the reading taken before the destroy.
      await assertLifecycleGateOpen(
        target,
        owner,
        evidence,
        "governance destroyed the pending context, which clears the in-flight state.",
      );

      await assertContextValidity(
        target,
        evidence,
        pendingContextId,
        false,
        "a destroyed context is never valid",
      );

      // The cross-chain observation no unit test can make: the Gateway still holds the context the
      // host just destroyed, because its registry and its destroy are its own. Asserted as a canary
      // rather than merely noted, so adding cross-chain cleanup fails here and gets noticed.
      await assertGatewayContextValidity(
        gatewayRpcUrl,
        gatewayConfigAddress,
        evidence,
        pendingContextId,
        true,
        "the host-side abort does not reach the Gateway, which has its own destroyKmsContext",
      );
    },
  );

  // The abort must not have cost the previous pair its ability to serve.
  const decrypted = await evidence.step(
    "probe",
    "user-decryption under the original pair after the abort",
    { contextId: formatKmsId(baseline.contextId), epochId: formatKmsId(baseline.epochId) },
    () => runDecryption(`kms-context-qa/context-switch-abort: decrypt after aborting the switch`),
  );
  if (!decrypted) {
    throw new PreflightError(
      `kms-context-qa/context-switch-abort: user-decryption failed after the abort. The active pair never moved ` +
        `from (${baseline.contextId}, ${baseline.epochId}), so aborting a switch that never took effect must leave ` +
        `service untouched — check the kms-core and kms-connector-*-kms-worker logs.`,
    );
  }

  // Recovery: the gate reopening is only a claim until a real switch goes through it. With the node
  // restored and no in-flight operation, a fresh switch must activate normally.
  const recoveryContextId = pendingContextId + 1n;
  evidence.note("note", "recovery switch predicted from sequential allocation", {
    abortedContextId: formatKmsId(pendingContextId),
    recoveryContextId: formatKmsId(recoveryContextId),
  });
  await broadcastContextSwitch(state, evidence);
  await preRegisterContextOnGateway(state, evidence, recoveryContextId);
  const activated = await waitForActivation(
    target,
    evidence,
    `recovery context switch to ${formatKmsId(recoveryContextId)} after the abort`,
    (current) => current.contextId === recoveryContextId && current.epochId > baseline.epochId,
  );

  // "All required confirmations complete with compatible results": activation already implies every
  // signer confirmed — `confirmEpochActivation` needs full quorum — but that is the on-chain vote,
  // not the work behind it. The connector DB records each node's reshare result independently, so
  // requiring `completed` on every serving node states the "compatible results" clause outright
  // instead of leaving it implied by the pointer having moved.
  const recoveryParties = await readCommitteeParties(target, evidence, activated.contextId, kmsSigners);
  await evidence.step(
    "assert",
    "every node of the recovered context completed the reshare",
    { parties: recoveryParties.join(","), epochId: formatKmsId(activated.epochId) },
    async () =>
      checkConnectorsDbColumn(
        recoveryParties,
        `epoch ${activated.epochId} reshare completed (recovery after the abort)`,
        columnQuery("new_kms_epoch", "epoch_id", "status", activated.epochId),
        ["completed"],
      ),
  );

  console.log(
    `[kms-context-qa] context-switch-abort complete: the switch to ${pendingContextId} was held before its ` +
      `creation quorum, never allocated an epoch, and was destroyed by governance — leaving ` +
      `(${baseline.contextId}, ${baseline.epochId}) active and serving throughout. The Gateway still holds ` +
      `${pendingContextId}: its registry is its own. A recovery switch then activated ` +
      `(${activated.contextId}, ${activated.epochId}), proving the in-flight gate really reopened.`,
  );
};

/** Registry entry for the `context-switch-abort` case. */
export const contextSwitchAbortCase: QaCase = {
  id: "context-switch-abort-and-retry",
  title: "Governance aborts a switch that never reached quorum, then retries it successfully (two merged scenarios)",
  proves:
    "a context switch whose creation confirmation is withheld never allocates an epoch, destroyKmsContext aborts " +
    "it without moving the active pair, the previous pair keeps serving a real user decryption throughout, the " +
    "Gateway is left still holding the context the host destroyed — its registry being independent — and a fresh " +
    "switch requested afterwards registers on the Gateway, reshares on every node and becomes the active pair, " +
    "proving the in-flight gate reopened",
  requirements: {
    mode: "threshold",
    // One node of the new context is stalled while the old committee must still serve a decryption.
    // See the epoch-pending case for why 4 rather than 3.
    minCommitteeSize: 4,
  },
  mutatesLifecycle: true,
  run,
};
