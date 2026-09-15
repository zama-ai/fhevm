/**
 * QA case `context-switch` — the host half of:
 *
 *   Scenario: The SDK uses a new context and epoch
 *     Given the previous active pair is "C1, E1"
 *     And a context switch to "C2, E2" has completed
 *     And context "C2" was registered in the Gateway before activation
 *     And ProtocolConfig returns "C2, E2" as the active pair
 *     When the application performs a decryption through the SDK
 *     And the test captures the request and response extraData
 *     Then the decryption must complete successfully
 *     And the request extraData must decode as version "0x02", context "C2", and epoch "E2"
 *     And the response extraData must be identical to the request extraData
 *     And context "C1" and epoch "E1" must not be used by the new request
 *
 * The sibling of `case-epoch-rotation`, and implemented with the same concessions — the literal ids
 * are read from `ProtocolConfig` rather than carried as constants, and the `Given` clauses are
 * established and *verified* by this case rather than assumed. The rationale is recorded in
 * `test-suite/fhevm/qa-kms-context-scenario-1-context.md`.
 *
 * ## How it differs from the epoch rotation
 *
 * 1. **Both ids move.** A rotation keeps the context and advances the epoch; a switch advances
 *    both. The negative assertion is correspondingly wider: neither `C1` nor `E1` may appear.
 * 2. **No receipt.** `defineNewEpochForCurrentKmsContext` takes no arguments, so the epoch case
 *    sends it with `cast send` and reads `NewKmsEpoch` straight off the receipt. A context switch
 *    carries the whole committee definition, which lives in the contracts task's env file, so it
 *    runs as a compose task and returns nothing. The new context id is therefore derived from the
 *    sequential-allocation invariant (`previous + 1`) and then **verified** against the id the
 *    chain actually activates — the same check `kms-context-switch.ts:222` makes.
 * 3. **The Gateway must accept the context first.** Pre-registration is ordered before activation
 *    so a fresh SDK client can never see a context active on the host that the Gateway would
 *    reject. That ordering is itself one of the scenario's `Given` clauses.
 *
 * ## What is NOT covered
 *
 * *"The response extraData must be identical to the request extraData"* — the SDK neither verifies
 * nor exposes the response value. See `test-suite/fhevm/qa-extradata-check.md`.
 *
 * ## Committee membership is read, not assumed
 *
 * As in the epoch case, the per-party reshare assertion targets the parties that actually serve the
 * new context, resolved from `getKmsSignersForContext`. Here it matters even more: a switch is
 * exactly the operation that can change membership.
 *
 * ## Disruptiveness
 *
 * Advances both the context and the epoch, and does not roll them back. It stops no containers.
 * Re-up between runs: `fhevm-cli down && fhevm-cli up --scenario five-party-swap-threshold-kms`.
 */
import { PreflightError } from "../../errors";
import { checkConnectorsDbColumn, columnQuery } from "../../kms-connector-db";
import type { QaCase, QaCaseContext } from "../registry";
import { assertTxSendersRunning } from "../nodes";
import {
  broadcastContextSwitch,
  formatKmsId,
  preRegisterContextOnGateway,
  readCommitteeParties,
  readCurrentPair,
  waitForActivation,
} from "../protocol-config";

/**
 * Runs the host half of the context-switch scenario.
 *
 * Sequence: read baseline -> broadcast the switch -> pre-register on the Gateway -> wait for
 * activation -> verify the activated context is the pre-registered one -> confirm every serving
 * node reshared -> prove the new pair serves -> hand off to the container spec.
 */
const run = async (ctx: QaCaseContext): Promise<void> => {
  const { state, target, owner, evidence, runDecryption, runSmoke, runExtraDataCheck } = ctx;

  evidence.note("note", "target", {
    protocolConfig: target.address,
    rpcUrl: target.rpcUrl,
    where: target.where,
    owner: owner.address,
  });

  // A switch needs every node of the NEW context to confirm on chain. The new context's node set
  // comes from the contracts task's env file, which lists every provisioned party — so check them
  // all, not just the current committee. Without this the run would discover a stopped tx-sender
  // only after the 600s activation wait expired.
  await assertTxSendersRunning(
    Array.from({ length: state.scenario.kms.parties }, (_, index) => index + 1),
    evidence,
    "A context switch requires all nodes of the new context to submit confirmKmsContextCreation.",
  );

  // Given: the pair active before the switch — C1 and E1.
  const baseline = await readCurrentPair(target, evidence, "baseline active pair");

  const kmsSigners = state.discovery?.kmsSigners ?? [];
  if (!kmsSigners.length) {
    throw new PreflightError(
      "kms-context-qa/context-switch: no KMS signers in persisted discovery — cannot resolve which parties serve " +
        "the active context. The stack must have completed the discover step.",
    );
  }
  const committeeBefore = await readCommitteeParties(target, evidence, baseline.contextId, kmsSigners);

  await runSmoke(
    `kms-context-qa/context-switch: input-proof at baseline (contextId=${baseline.contextId})`,
  );

  // Context ids are allocated sequentially by `_storeNextKmsContext`, so the pending id is
  // knowable before the chain reports it. This is a prediction, verified below — never trusted.
  const pendingContextId = baseline.contextId + 1n;
  evidence.note("note", "pending context id predicted from sequential allocation", {
    baselineContextId: formatKmsId(baseline.contextId),
    pendingContextId: formatKmsId(pendingContextId),
  });

  // When: governance defines the new context. The committee comes from the contracts task's env
  // file, NOT from current chain state — so on a stack that has been node-swapped this restores the
  // env's committee rather than preserving the serving one. The case reports what actually happened
  // (see the committee note below) instead of assuming either.
  await broadcastContextSwitch(state, evidence);

  // The Gateway must accept the context before it activates on the host — one of the scenario's
  // Given clauses, and the reason a fresh SDK client never observes an unregistered context.
  await preRegisterContextOnGateway(state, evidence, pendingContextId);

  // Then: the new pair must activate. Both ids move, so the wait targets the predicted context.
  const activated = await waitForActivation(
    target,
    evidence,
    `context switch to contextId=${pendingContextId}`,
    (current) => current.contextId === pendingContextId && current.epochId > baseline.epochId,
  );

  await evidence.step(
    "assert",
    "the activated context is the one pre-registered on the gateway",
    {
      activatedContextId: formatKmsId(activated.contextId),
      preRegisteredContextId: formatKmsId(pendingContextId),
    },
    async () => {
      if (activated.contextId !== pendingContextId) {
        throw new PreflightError(
          `kms-context-qa/context-switch: activated contextId=${activated.contextId} but the gateway was ` +
            `pre-registered with ${pendingContextId}. A client would present a context the gateway rejects.`,
        );
      }
      if (activated.epochId === baseline.epochId) {
        throw new PreflightError(
          `kms-context-qa/context-switch: the context advanced to ${activated.contextId} but the epoch stayed at ` +
            `${baseline.epochId}. A switch must open a new epoch under the new context.`,
        );
      }
    },
  );

  // Membership can change across a switch, so resolve the serving set again rather than reusing
  // the pre-switch committee.
  const committeeAfter = await readCommitteeParties(target, evidence, activated.contextId, kmsSigners);
  evidence.note("note", "committee across the switch", {
    before: committeeBefore.join(","),
    after: committeeAfter.join(","),
    changed: String(committeeBefore.join(",") !== committeeAfter.join(",")),
  });

  await evidence.step(
    "assert",
    "every serving node completed the new epoch reshare",
    { parties: committeeAfter.join(","), epochId: formatKmsId(activated.epochId) },
    async () =>
      checkConnectorsDbColumn(
        committeeAfter,
        `epoch ${activated.epochId} reshare completed`,
        columnQuery("new_kms_epoch", "epoch_id", "status", activated.epochId),
        ["completed"],
      ),
  );

  await runSmoke(
    `kms-context-qa/context-switch: input-proof after the switch (contextId=${activated.contextId})`,
  );

  const decrypted = await evidence.step(
    "probe",
    "user-decryption under the new context",
    { contextId: formatKmsId(activated.contextId), epochId: formatKmsId(activated.epochId) },
    () =>
      runDecryption(
        `kms-context-qa/context-switch: decrypt after the switch (contextId=${activated.contextId})`,
      ),
  );
  if (!decrypted) {
    throw new PreflightError(
      `kms-context-qa/context-switch: user-decryption failed after the switch activated ` +
        `(contextId=${activated.contextId}, epochId=${activated.epochId}). The switch completed on chain, so the ` +
        `new context's reshared key material is not serving — check the kms-core and ` +
        `kms-connector-*-kms-worker logs.`,
    );
  }

  // The one place the full uint256 ids are printed: these are what the container asserts and
  // what you paste into `cast`. Everywhere else the evidence uses the short `ctx#n` form.
  evidence.note("note", "raw ids handed to the container-side extraData check", {
    context: activated.contextId.toString(),
    epoch: activated.epochId.toString(),
    previousContext: baseline.contextId.toString(),
    previousEpoch: baseline.epochId.toString(),
    expectedExtraDataVersion: "0x02",
  });

  // The client-side half: the SDK must embed the NEW pair, and neither C1 nor E1.
  await evidence.step(
    "probe",
    "SDK embeds the new (context, epoch) in the permit extraData",
    {
      contextId: formatKmsId(activated.contextId),
      epochId: formatKmsId(activated.epochId),
      previousContextId: formatKmsId(baseline.contextId),
      previousEpochId: formatKmsId(baseline.epochId),
    },
    () =>
      runExtraDataCheck(
        `kms-context-qa/context-switch: extraData carries contextId=${activated.contextId}`,
        {
          contextId: activated.contextId,
          epochId: activated.epochId,
          previousContextId: baseline.contextId,
          previousEpochId: baseline.epochId,
        },
      ),
  );

  console.log(
    `[kms-context-qa] context-switch complete: context ${baseline.contextId} -> ${activated.contextId}, ` +
      `epoch ${baseline.epochId} -> ${activated.epochId}. The response-extraData echo remains uncovered — the SDK ` +
      `neither verifies nor exposes it (see test-suite/fhevm/qa-extradata-check.md).`,
  );
};

/** Registry entry for the `context-switch` case. */
export const contextSwitchCase: QaCase = {
  id: "context-switch",
  title: "SDK uses the new context and epoch after a context switch (both halves)",
  proves:
    "a context switch activates the pre-registered context on chain, every serving node completes " +
    "the reshare, the new (context, epoch) pair serves both an input-proof flow and a user " +
    "decryption, and the SDK embeds that new pair — not the superseded one — in the permit extraData",
  requirements: {
    mode: "threshold",
    // A same-committee switch needs the committee only. The profile-level preflight still pins the
    // scenario; this states the case's own honest minimum.
    minCommitteeSize: 1,
  },
  mutatesLifecycle: true,
  run,
};
