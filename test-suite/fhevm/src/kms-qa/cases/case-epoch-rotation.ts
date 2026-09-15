/**
 * QA case `epoch-rotation` — both halves of:
 *
 *   Feature: Normal decryption after a new epoch becomes active
 *
 *     Scenario: The SDK uses the currently active epoch
 *       Given ProtocolConfig reports an active pair "(C, E)"
 *       And the NewKmsEpoch event for "E" names a previous epoch "E_prev" distinct from "E"
 *       When the application performs a decryption through the SDK
 *       And the test captures the request and response extraData
 *       Then the decryption must complete successfully
 *       And the request extraData must decode as version "0x02", context "C", and epoch "E"
 *       And the response extraData must be identical to the request extraData
 *       And the decoded epoch must not equal "E_prev"
 *
 * This is the amended, self-describing form of the original scenario: rather than having the
 * orchestrator inject literal ids into the container, the test reads them from ProtocolConfig. That
 * keeps the two halves decoupled and lets the container-side spec stand on its own.
 *
 * ## What this case covers, and what it does not
 *
 * It establishes and *verifies* the `Given`: it reads the baseline pair, triggers the rotation,
 * waits until the new epoch is genuinely active on chain, and captures `E_prev` from two
 * independent sources. It then proves the rotated pair actually serves, by running the existing
 * user-decryption and input-proof probes.
 *
 * It then drives the container-side spec (`KMS context extraData`, in
 * `test-suite/e2e/test/kmsContextExtraData/`), which signs a decryption permit through the SDK and
 * asserts the embedded `extraData` decodes as v2 with the pair that is now active — closing the
 * scenario's `Then` clauses about the request extraData.
 *
 * **One clause remains uncovered**: *"the response extraData must be identical to the request
 * extraData"*. The SDK neither verifies nor exposes the response value — the comparison code exists
 * but is commented out, and `equalsKmsExtraData` has no production call sites. The evidence and the
 * decision to defer are recorded in `test-suite/fhevm/qa-extradata-check.md`.
 *
 * ## Why the wait is the real test
 *
 * Activation is not automatic. `defineNewEpochForCurrentKmsContext` only opens a *Pending* epoch;
 * the KMS cores must reshare and every committee connector must submit `confirmEpochActivation`
 * before `getCurrentKmsContextAndEpoch` advances. A timeout is a genuine product signal that the
 * cluster did not converge, which makes this case a discovery test for whether the cluster
 * reshares at all.
 *
 * ## Committee membership is read, not assumed
 *
 * The per-party reshare assertion targets the parties that actually serve the active context,
 * resolved from `getKmsSignersForContext` and the persisted signer discovery — never `1..committeeSize`.
 * A node-swap context switch drops a party and promotes a spare, so a stack that has switched serves
 * e.g. `{1,2,3,5}`. The dropped party no longer holds the context's material and its core reports a
 * failed reshare, which is correct behaviour; asserting against the initial committee would blame it.
 *
 * ## Disruptiveness
 *
 * The case advances the active epoch and does not roll it back. It stops no containers. Re-up the
 * stack between runs: `fhevm-cli down && fhevm-cli up --scenario five-party-swap-threshold-kms`.
 */
import { PreflightError } from "../../errors";
import { checkConnectorsDbColumn, columnQuery } from "../../kms-connector-db";
import type { QaCase, QaCaseContext } from "../registry";
import {
  assertRotationConsistency,
  formatKmsId,
  readCommitteeParties,
  readCurrentPair,
  sendDefineNewEpoch,
  waitForActivation,
} from "../protocol-config";

/**
 * Runs the host half of the epoch-rotation scenario.
 *
 * Sequence: read baseline -> broadcast -> decode and cross-check the event -> wait for activation
 * -> confirm every committee node reshared -> prove the new pair serves.
 */
const run = async (ctx: QaCaseContext): Promise<void> => {
  const { target, owner, evidence, runDecryption, runSmoke, runExtraDataCheck } = ctx;

  evidence.note("note", "target", {
    protocolConfig: target.address,
    rpcUrl: target.rpcUrl,
    where: target.where,
    owner: owner.address,
  });

  // Given: the pair that is active before anything happens. This is E_prev as observed by a client.
  const baseline = await readCurrentPair(target, evidence, "baseline active pair");

  // Which parties actually serve this context. Never assume 1..committeeSize: a node-swap switch
  // drops a party and promotes a spare, so a stack that has switched serves e.g. {1,2,3,5}. A
  // dropped party no longer holds the context's material and legitimately fails the reshare, so
  // asserting against the initial committee would blame a node that is behaving correctly.
  const kmsSigners = ctx.state.discovery?.kmsSigners ?? [];
  if (!kmsSigners.length) {
    throw new PreflightError(
      "kms-context-qa/epoch-rotation: no KMS signers in persisted discovery — cannot resolve which parties serve " +
        "the active context. The stack must have completed the discover step.",
    );
  }
  const committee = await readCommitteeParties(target, evidence, baseline.contextId, kmsSigners);

  // A smoke at baseline, so any later failure is attributable to the transition that precedes it
  // rather than to a cluster that was already unhealthy.
  await runSmoke(`kms-context-qa/epoch-rotation: input-proof at baseline (epochId=${baseline.epochId})`);

  // When: governance rotates the epoch under the same context.
  const { event } = await sendDefineNewEpoch(target, owner, evidence);

  // The contract's own view of what was superseded must agree with what we read. A mismatch means
  // a concurrent lifecycle operation, which would make every downstream assertion ambiguous.
  await evidence.step(
    "assert",
    "rotation is consistent with the baseline",
    {
      baselineContextId: formatKmsId(baseline.contextId),
      baselineEpochId: formatKmsId(baseline.epochId),
      eventPreviousEpochId: formatKmsId(event.previousEpochId),
      eventEpochId: formatKmsId(event.epochId),
    },
    async () => assertRotationConsistency(baseline, event),
  );

  // Then: the new epoch must actually become active. We know the exact target id from the event, so
  // we assert equality rather than merely "greater than".
  const activated = await waitForActivation(
    target,
    evidence,
    `epoch rotation to epochId=${event.epochId}`,
    (current) => current.contextId === baseline.contextId && current.epochId === event.epochId,
  );

  // Pointer movement alone does not prove the committee did the work. The connector DB records each
  // party's reshare result, so require every committee node to have completed it.
  await evidence.step(
    "assert",
    "every committee node completed the epoch reshare",
    { parties: committee.join(","), epochId: formatKmsId(event.epochId) },
    async () =>
      checkConnectorsDbColumn(
        committee,
        `epoch ${event.epochId} reshare completed`,
        columnQuery("new_kms_epoch", "epoch_id", "status", event.epochId),
        ["completed"],
      ),
  );

  // The rotated pair must serve a normal application flow, not just exist on chain.
  await runSmoke(`kms-context-qa/epoch-rotation: input-proof after rotation (epochId=${activated.epochId})`);

  const decrypted = await evidence.step(
    "probe",
    "user-decryption under the rotated epoch",
    { contextId: formatKmsId(activated.contextId), epochId: formatKmsId(activated.epochId) },
    () =>
      runDecryption(
        `kms-context-qa/epoch-rotation: decrypt after rotation (epochId=${activated.epochId})`,
      ),
  );
  if (!decrypted) {
    throw new PreflightError(
      `kms-context-qa/epoch-rotation: user-decryption failed after the epoch activated ` +
        `(contextId=${activated.contextId}, epochId=${activated.epochId}). The rotation completed on chain, so the ` +
        `new epoch's reshared key material is not serving — check the kms-core and kms-connector-*-kms-worker logs.`,
    );
  }

  // The one place the full uint256 ids are printed: these are what the container asserts and
  // what you paste into `cast`. Everywhere else the evidence uses the short `ctx#n` form.
  evidence.note("note", "raw ids handed to the container-side extraData check", {
    context: activated.contextId.toString(),
    epoch: activated.epochId.toString(),
    previousEpoch: baseline.epochId.toString(),
    expectedExtraDataVersion: "0x02",
  });

  // The client-side half of the scenario: the SDK must embed the pair that is now active in the
  // extraData of the permit it signs. The spec reads the active pair itself and also cross-checks
  // it against the values injected here, so a rotation landing between the two reads is caught.
  await evidence.step(
    "probe",
    "SDK embeds the active (context, epoch) in the permit extraData",
    {
      contextId: formatKmsId(activated.contextId),
      epochId: formatKmsId(activated.epochId),
      previousEpochId: formatKmsId(baseline.epochId),
    },
    () =>
      runExtraDataCheck(
        `kms-context-qa/epoch-rotation: extraData carries epochId=${activated.epochId}`,
        {
          contextId: activated.contextId,
          epochId: activated.epochId,
          previousEpochId: baseline.epochId,
        },
      ),
  );

  console.log(
    `[kms-context-qa] epoch-rotation complete: context=${activated.contextId} epoch=${activated.epochId} ` +
      `previousEpoch=${baseline.epochId}. The response-extraData echo remains uncovered — the SDK neither ` +
      `verifies nor exposes it (see test-suite/fhevm/qa-extradata-check.md).`,
  );
};

/** Registry entry for the `epoch-rotation` case. */
export const epochRotationCase: QaCase = {
  id: "epoch-rotation",
  title: "SDK uses the currently active epoch after a same-context rotation (both halves)",
  proves:
    "a same-context epoch rotation activates on chain, every committee node completes the reshare, " +
    "the rotated (context, epoch) pair serves both an input-proof flow and a user decryption, and " +
    "the SDK embeds the rotated epoch — not the superseded one — in the permit extraData",
  requirements: {
    mode: "threshold",
    // A same-context rotation needs the committee only; no spare is involved. The profile-level
    // preflight still pins the scenario, but the case states its own honest minimum so a future
    // lighter topology needs no change here.
    minCommitteeSize: 1,
  },
  mutatesLifecycle: true,
  run,
};
