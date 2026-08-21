/**
 * The `kms-context-switch` acceptance profile for `fhevm-cli test`.
 *
 * Runs the ProtocolConfig KMS-context lifecycle end to end on a threshold-mode cluster and checks
 * that the KMS reacts to each emitted event. The cluster topology selects one of two variants:
 *
 *   - Same-committee (parties == committeeSize): each context switch reshares to the same
 *     committee, so no new signing keys are needed. Runs steps 1–4 below.
 *   - Node swap (parties > committeeSize, i.e. the cluster has a spare core): the switch drops
 *     one committee node and promotes the spare. Runs steps 1 and 2, skips 3 and 4 (see step 3),
 *     and instead ends by proving the promoted spare serves in the 2t+1 decryption quorum.
 *
 *   1. Context switch (NewKmsContext). Broadcast `defineNewKmsContextAndEpoch`. In the node-swap
 *      variant the dropped node's tx-sender is stopped before the broadcast — a node that cannot
 *      transact must not block its own replacement. Wait until the new context becomes the active
 *      one on chain, then decrypt.
 *   2. Epoch rotation (NewKmsEpoch). Broadcast `defineNewEpochForCurrentKmsContext` (same context,
 *      new epoch). Wait until the new epoch activates, then decrypt again.
 *   3. Destruction (same-committee clusters only). Steps 1 and 2 leave two entries that are still
 *      live but no longer current: the baseline context and the epoch from step 1. Destroy them
 *      with `destroyKmsContext` / `destroyKmsEpoch` and check:
 *        - the current context and epoch cannot be destroyed;
 *        - unknown ids and already-destroyed ids revert;
 *        - `KmsContextDestroyed` / `KmsEpochDestroyed` emitted;
 *        - the destroyed target becomes invalid, and the active context/epoch does not move;
 *        - destroying a context also invalidates its epochs (`isValidEpochForContext` requires a
 *          valid context, so the contract needs no per-epoch write);
 *        - every party's connector forwards `DestroyMpcContext` / `DestroyMpcEpoch` to its core
 *          and invalidates its validation cache;
 *        - the current context/epoch keeps serving.
 *      This step is skipped on a node swap. The retired context's committee is not the current
 *      party set, so the dropped node and the promoted spare cannot decommission it: the KMS
 *      rejects a destroy that would leave a node with no context.
 *   4. Recovery and abort (same-committee clusters only). Two independent checks:
 *      a) A context switch must still work after a destroy. That is, destroying a context must
 *         never leave the system stuck and unable to reshare and activate a new one.
 *      b) A stuck epoch rotation can be aborted and retried. Stop one node's tx-sender and start
 *         a rotation: the new epoch reshares but stays Pending. While it is Pending, any other
 *         lifecycle operation reverts with `KmsLifecycleOperationInFlight`. `destroyKmsEpoch`
 *         aborts the Pending epoch, so the previous epoch is current again. Restart the node and
 *         rotate again: the fresh rotation activates normally.
 *
 * Activation is not automatic. The KMS cores must reshare, and the connectors must submit
 * `confirmKmsContextCreation` / `confirmEpochActivation`, before `getCurrentKmsContextAndEpoch`
 * advances. If that never happens, the ids stay put and the profile fails with the last-read
 * state. This makes the profile also the discovery test for whether the cluster reshares at all.
 * Threshold-only: the switch tasks act on the host ProtocolConfig.
 *
 * App-level checkpoints: every transition must stay invisible to a normal encrypted-input flow,
 * not only to the dedicated user-decryption probe. The input-proof smoke therefore runs at
 * baseline, while a switch is pending (the previous context must keep serving during the
 * reshare), and after each transition.
 *
 * Disruptive and single-run: the profile advances the KMS context/epoch and destroys the retired
 * ones, so it expects a pristine stack. Re-up between runs
 * (`fhevm-cli down && fhevm-cli up --scenario four-party-threshold-kms`).
 */
import { PreflightError } from "../errors";
import { castBool, castCall, resolveKmsGenerationTarget, waitForContainer } from "../flow/readiness";
import { stepComposeTask } from "../flow/runtime-compose";
import { columnQuery, pollConnectors } from "../kms-connector-db";
import { castSend, eventTopicWord, expectRevert, keccakTopic, loadHostOwner, type Owner } from "../kms-onchain";
import { kmsTxSenderName, reconstructionThreshold } from "../kms-party";
import type { State } from "../types";
import {
  type DecryptionRunner,
  partyContainers,
  setRunning,
  waitForContainersStopped,
  waitForPartiesRunning,
  waitForPartiesStopped,
} from "./kms-generation";

/** Generous bound: a 4-party reshare + per-party on-chain confirmations. */
const ACTIVATION_TIMEOUT_MS = 600_000;
const ACTIVATION_POLL_MS = 5_000;

export type ContextAndEpoch = { contextId: bigint; epochId: bigint };

/** Runs the app-level input-proof smoke; throws when the flow regresses. */
export type SmokeRunner = (label: string) => Promise<void>;

/** Parses `cast call getCurrentKmsContextAndEpoch()(uint256,uint256)` output into bigints. cast prints
 * each return value as `<decimal> [<scientific-notation>]` (one per line for a tuple), so strip the
 * informational `[…]` annotations and take the first two decimal tokens. The ids are large
 * domain-tagged uint256 values (e.g. contextId `0x07…`, epochId `0x08…`), not small counters.
 * Exported for unit testing. */
export const parseContextAndEpoch = (raw: string): ContextAndEpoch => {
  const ids = raw
    .replace(/\[[^\]]*\]/g, " ")
    .split(/\s+/)
    .filter((token) => /^\d+$/.test(token));
  if (ids.length < 2) {
    throw new PreflightError(
      `kms-context-switch: could not parse getCurrentKmsContextAndEpoch output: ${JSON.stringify(raw)}`,
    );
  }
  return { contextId: BigInt(ids[0]), epochId: BigInt(ids[1]) };
};

const readContextAndEpoch = async (rpcUrl: string, protocolConfig: string): Promise<ContextAndEpoch> =>
  parseContextAndEpoch(await castCall(rpcUrl, protocolConfig, "getCurrentKmsContextAndEpoch()(uint256,uint256)"));

/** Polls the on-chain active context/epoch until `reached` holds, or throws with the last-seen
 * state on timeout (the signal that the KMS did not finish resharing/confirming). */
const waitForActivation = async (
  rpcUrl: string,
  protocolConfig: string,
  label: string,
  reached: (current: ContextAndEpoch) => boolean,
): Promise<ContextAndEpoch> => {
  const deadline = Date.now() + ACTIVATION_TIMEOUT_MS;
  let current = await readContextAndEpoch(rpcUrl, protocolConfig);
  while (!reached(current)) {
    if (Date.now() >= deadline) {
      throw new PreflightError(
        `kms-context-switch: ${label} did not activate within ${ACTIVATION_TIMEOUT_MS / 1000}s ` +
          `(last on-chain state: contextId=${current.contextId}, epochId=${current.epochId}). The KMS cores must ` +
          `reshare and the connectors submit confirmKmsContextCreation/confirmEpochActivation for the id to advance — ` +
          `run host-contracts \`task:kmsContextSwitchStatus\` to see which confirmations are outstanding.`,
      );
    }
    await Bun.sleep(ACTIVATION_POLL_MS);
    current = await readContextAndEpoch(rpcUrl, protocolConfig);
  }
  return current;
};

type SwitchTarget = { rpcUrl: string; configAddress: string; where: string };

/** The committee membership change a context switch performs on this topology. With a spare core
 * (parties > committeeSize) the last committee slot(s) are dropped and the spare(s) promoted (e.g.
 * {1,2,3,4} -> {1,2,3,5}); otherwise the committee is unchanged. */
const committeeSwapPlan = (kms: State["scenario"]["kms"]) => {
  const spareCount = kms.parties - kms.committeeSize;
  return {
    isSwap: spareCount > 0,
    continuing: Array.from({ length: kms.committeeSize - spareCount }, (_, i) => i + 1),
    added: Array.from({ length: spareCount }, (_, i) => kms.committeeSize + 1 + i),
    dropped: Array.from({ length: spareCount }, (_, i) => kms.committeeSize - spareCount + 1 + i),
  };
};

/**
 * NewKmsContext step. On a cluster with a spare core this is a genuine node swap: the new context
 * drops one committee node and promotes the spare, so n stays committeeSize. The dropped node's
 * tx-sender is stopped before the broadcast. The switch must then complete on the n − t quorum of
 * the previous committee, without the dropped node's confirmation — a node that cannot transact
 * must not veto its own removal. The rest of the dropped node stays up, because the reshare still
 * needs its core (the KMS core cannot yet reshare around an absent outgoing party); upgrade to a
 * full party stop once it can. Without a spare, this is a same-committee reshare. Activation
 * waits for ALL new-committee signers, so the context id only advances once the spare has
 * reshared and confirmed.
 */
const switchKmsContext = async (
  state: State,
  runDecryption: DecryptionRunner,
  runSmoke: SmokeRunner,
  target: SwitchTarget,
  baseline: ContextAndEpoch,
): Promise<ContextAndEpoch> => {
  const { isSwap, continuing, added, dropped } = committeeSwapPlan(state.scenario.kms);
  // Same services as a same-set switch; a swap just points them at the swap-committee env files.
  const hostEnv: Record<string, string> = isSwap ? { HOST_SC_CONTEXT_ENV: "host-sc-swap.env" } : {};
  const gatewayEnv: Record<string, string> = isSwap ? { GATEWAY_SC_CONTEXT_ENV: "gateway-sc-swap.env" } : {};

  if (isSwap) {
    // Stopped before the broadcast and left down: the new committee does not include the node.
    const droppedTxSenders = dropped.map((party) => kmsTxSenderName(party));
    console.log(
      `[kms-context-switch] stopping dropped node(s) ${dropped.join(",")} tx-sender before the switch — a node that cannot confirm must not block its own replacement…`,
    );
    await setRunning(droppedTxSenders, "stop");
    await waitForContainersStopped(droppedTxSenders);
  }

  console.log(
    isSwap
      ? `[kms-context-switch] broadcasting defineNewKmsContextAndEpoch — node swap (drop ${dropped.join(",")} with tx-sender stopped, add ${added.join(",")}, keep ${continuing.join(",")})…`
      : "[kms-context-switch] broadcasting defineNewKmsContextAndEpoch (NewKmsContext, same committee)…",
  );
  await stepComposeTask("host-sc", state, ["host-sc-context-switch"], { noDeps: true, env: hostEnv });
  await waitForContainer("host-sc-context-switch", "complete");

  const pendingContextId = baseline.contextId + 1n;
  console.log(`[kms-context-switch] pre-registering pending context ${pendingContextId} on the gateway…`);
  await stepComposeTask("gateway-sc", state, ["gateway-sc-context-switch"], {
    noDeps: true,
    env: { KMS_CONTEXT_ID: pendingContextId.toString(), ...gatewayEnv },
  });
  await waitForContainer("gateway-sc-context-switch", "complete");

  // While the new context is Pending (the cores are resharing), the previous context keeps
  // serving: app flows and user decryption must work unchanged. The probe is racy by nature — if
  // activation completes mid-probe, the checks must still pass. The gateway was pre-registered
  // with the pending context above, so a fresh SDK client cannot see the new host context before
  // the gateway accepts it.
  await runSmoke("kms-context-switch: input-proof while the switch is pending (previous context serving)");
  if (!(await runDecryption("kms-context-switch: decrypt while the switch is pending (previous context serving)"))) {
    throw new PreflightError(
      "kms-context-switch: user-decryption failed while the context switch was pending — the previous context must keep serving until the new one activates",
    );
  }

  const afterSwitch = await waitForActivation(
    target.rpcUrl,
    target.configAddress,
    isSwap ? "node-swap context" : "context switch (NewKmsContext)",
    (current) => current.contextId > baseline.contextId,
  );
  if (afterSwitch.contextId !== pendingContextId) {
    throw new PreflightError(
      `kms-context-switch: activated unexpected contextId=${afterSwitch.contextId}; expected pre-registered contextId=${pendingContextId}`,
    );
  }
  console.log(
    isSwap
      ? `[kms-context-switch] context switched: contextId ${baseline.contextId} -> ${afterSwitch.contextId} — the spare reshared and confirmed`
      : `[kms-context-switch] context switched: contextId ${baseline.contextId} -> ${afterSwitch.contextId} (epochId=${afterSwitch.epochId})`,
  );

  if (!(await runDecryption(`kms-context-switch: decrypt after context switch (contextId=${afterSwitch.contextId})`))) {
    throw new PreflightError(
      `kms-context-switch: user-decryption failed after the context switch to contextId=${afterSwitch.contextId}`,
    );
  }
  await runSmoke(`kms-context-switch: input-proof after the context switch (contextId=${afterSwitch.contextId})`);
  return afterSwitch;
};

/**
 * Proves the promoted spare holds a working reshared key. Stop `t` of the continuing members, so
 * the live committee is exactly the 2t+1 quorum and that quorum must include the spare. Then
 * decrypt: it cannot succeed without the spare's share. Restarts the stopped members afterwards.
 * Runs last, on an otherwise healthy cluster.
 */
const proveSpareInQuorum = async (state: State, runDecryption: DecryptionRunner) => {
  const { kms } = state.scenario;
  const { continuing, added } = committeeSwapPlan(kms);
  const reconstruct = reconstructionThreshold(kms.threshold);
  const forced = continuing.slice(continuing.length - kms.threshold);
  const forcedContainers = forced.flatMap((party) => partyContainers(party));
  console.log(
    `[kms-context-switch] stopping ${forced.join(",")} so spare(s) ${added.join(",")} are required for the ${reconstruct}/${kms.committeeSize} (2t+1) quorum…`,
  );
  await setRunning(forcedContainers, "stop");
  try {
    await waitForPartiesStopped(forced);
    if (!(await runDecryption(`kms-context-switch: decrypt with spare(s) ${added.join(",")} in the 2t+1 quorum`))) {
      throw new PreflightError(
        `kms-context-switch: decryption failed with spare(s) ${added.join(",")} forced into the quorum — the reshared key did not work`,
      );
    }
  } finally {
    await setRunning(forcedContainers, "start");
    await waitForPartiesRunning(forced);
  }
};

type DestroyTarget = { rpcUrl: string; protocolConfig: string; owner: Owner };
type DestroyIds = {
  oldContextId: bigint;
  // The epoch that belongs to the retired context (its baseline epoch). Destroying the context
  // decommissions this epoch in KMS Core: DestroyMpcContext returns its id, and the connector
  // then marks it invalid in the epoch cache. So it must turn invalid without a separate destroy.
  contextEpochId: bigint;
  oldEpochId: bigint;
  current: ContextAndEpoch;
};

// A deterministic address that is never the ACL owner, used to prove destroy is owner-gated. Only
// the address is used (an eth_call revert probe needs no signature), so the key is irrelevant.
const NON_OWNER: Owner = { key: "", address: "0x000000000000000000000000000000000000dEaD" };

/** Asserts the on-chain active pointer did not move — destroying retired material must never
 * change what `getCurrentKmsContextAndEpoch` resolves to. */
const assertCurrentUnchanged = async (
  rpcUrl: string,
  protocolConfig: string,
  current: ContextAndEpoch,
  afterWhat: string,
): Promise<void> => {
  const now = await readContextAndEpoch(rpcUrl, protocolConfig);
  if (now.contextId !== current.contextId || now.epochId !== current.epochId) {
    throw new PreflightError(
      `kms-context-switch: current (contextId, epochId) moved from (${current.contextId}, ${current.epochId}) to ` +
        `(${now.contextId}, ${now.epochId}) after ${afterWhat} — destroying retired material must not move the active pointer`,
    );
  }
};

/**
 * Context/epoch destruction. The switch and the rotation leave two entries that are retired but
 * still live: the baseline context (replaced by the switch) and the switch's epoch (replaced by
 * the rotation, still under the current context). This destroys both and checks that every layer
 * retires them:
 *   - reverts: only the ACL owner may destroy; the current context and epoch cannot be destroyed;
 *     unknown ids revert; a destroyed context/epoch cannot be destroyed twice;
 *   - on-chain: `KmsContextDestroyed` / `KmsEpochDestroyed` fire, the target becomes invalid, and
 *     the active context/epoch does not move;
 *   - connector + KMS Core: every party forwards `DestroyMpcContext` / `DestroyMpcEpoch` to its
 *     core (its destroy request ends `completed`) and marks the target invalid in its validation
 *     cache. A context destroy also invalidates the context's epoch, whose id DestroyMpcContext
 *     returns;
 *   - app: the current context/epoch keeps serving user decryption and the input-proof flow.
 */
const destroyContextAndEpoch = async (
  state: State,
  target: DestroyTarget,
  ids: DestroyIds,
  runDecryption: DecryptionRunner,
  runSmoke: SmokeRunner,
): Promise<void> => {
  const { rpcUrl, protocolConfig, owner } = target;
  const { oldContextId, contextEpochId, oldEpochId, current } = ids;
  const parties = state.scenario.kms.parties;
  const [contextDestroyedTopic, epochDestroyedTopic] = await Promise.all([
    keccakTopic("KmsContextDestroyed(uint256)"),
    keccakTopic("KmsEpochDestroyed(uint256)"),
  ]);

  // Check the reverts first, while everything is still live. Destroying the current context/epoch
  // would strand live decryptions, and unknown ids were never issued.
  const unknownId = (1n << 255n).toString();
  await expectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy the current KMS context",
    "LatestActiveKmsContextCannotBeDestroyed(uint256)",
    "destroyKmsContext(uint256)", current.contextId.toString(),
  );
  await expectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy the current KMS epoch",
    "LatestActiveKmsEpochCannotBeDestroyed(uint256)",
    "destroyKmsEpoch(uint256)", current.epochId.toString(),
  );
  await expectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an unknown KMS context",
    "InvalidKmsContext(uint256)",
    "destroyKmsContext(uint256)", unknownId,
  );
  await expectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an unknown KMS epoch",
    "InvalidKmsEpoch(uint256)",
    "destroyKmsEpoch(uint256)", unknownId,
  );
  // Owner-gated: a non-owner is rejected by onlyACLOwner before any target check, so a valid
  // destroyable id still reverts with the access-control error rather than the target-state one.
  await expectRevert(
    rpcUrl, protocolConfig, NON_OWNER,
    "destroy a KMS context as a non-owner",
    "NotHostOwner(address)",
    "destroyKmsContext(uint256)", oldContextId.toString(),
  );
  await expectRevert(
    rpcUrl, protocolConfig, NON_OWNER,
    "destroy a KMS epoch as a non-owner",
    "NotHostOwner(address)",
    "destroyKmsEpoch(uint256)", oldEpochId.toString(),
  );
  console.log(
    "[kms-context-switch] destroy reverts OK (non-owner, current context/epoch, and unknown ids are all rejected)",
  );

  // Destroy the retired context.
  if (!(await castBool(rpcUrl, protocolConfig, "isValidKmsContext(uint256)(bool)", oldContextId.toString()))) {
    throw new PreflightError(
      `kms-context-switch: retired context ${oldContextId} is not valid before destroy — nothing to prove the destroy transition against`,
    );
  }
  console.log(`[kms-context-switch] destroying retired context ${oldContextId}…`);
  const contextReceipt = await castSend(rpcUrl, protocolConfig, owner, "destroyKmsContext(uint256)", oldContextId.toString());
  if (eventTopicWord(contextReceipt, contextDestroyedTopic, 1, "KmsContextDestroyed") !== oldContextId) {
    throw new PreflightError(`kms-context-switch: KmsContextDestroyed event does not carry contextId=${oldContextId}`);
  }
  if (await castBool(rpcUrl, protocolConfig, "isValidKmsContext(uint256)(bool)", oldContextId.toString())) {
    throw new PreflightError(`kms-context-switch: context ${oldContextId} still reads valid after destroy`);
  }
  await assertCurrentUnchanged(rpcUrl, protocolConfig, current, "destroying the retired context");
  await pollConnectors(
    parties, "context-destroy forwarded to KMS Core",
    columnQuery("kms_context_destroyed", "context_id", "status", oldContextId), ["completed"],
  );
  await pollConnectors(
    parties, "destroyed context invalidated in the validation cache",
    // Postgres renders boolean::text as 'true'/'false' (not the 't'/'f' psql shows for the raw type).
    columnQuery("kms_context", "id", "is_valid", oldContextId), ["false"],
  );
  // DestroyMpcContext returns the epoch ids it decommissioned, and the connector invalidates
  // them. The retired context's epoch must therefore turn invalid from the context
  // destroy alone, with no separate destroyKmsEpoch call.
  await pollConnectors(
    parties, "destroyed context cascaded to its epoch in the validation cache",
    columnQuery("kms_epoch", "id", "is_valid", contextEpochId), ["false"],
  );
  await expectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an already-destroyed context",
    "InvalidKmsContext(uint256)",
    "destroyKmsContext(uint256)", oldContextId.toString(),
  );
  console.log(`[kms-context-switch] retired context ${oldContextId} destroyed across contract, connector, and KMS Core`);

  // Destroy the retired epoch (still under the current context, superseded by the rotation).
  if (
    !(await castBool(
      rpcUrl, protocolConfig, "isValidEpochForContext(uint256,uint256)(bool)",
      current.contextId.toString(), oldEpochId.toString(),
    ))
  ) {
    throw new PreflightError(
      `kms-context-switch: retired epoch ${oldEpochId} is not valid under context ${current.contextId} before destroy`,
    );
  }
  console.log(`[kms-context-switch] destroying retired epoch ${oldEpochId}…`);
  const epochReceipt = await castSend(rpcUrl, protocolConfig, owner, "destroyKmsEpoch(uint256)", oldEpochId.toString());
  if (eventTopicWord(epochReceipt, epochDestroyedTopic, 1, "KmsEpochDestroyed") !== oldEpochId) {
    throw new PreflightError(`kms-context-switch: KmsEpochDestroyed event does not carry epochId=${oldEpochId}`);
  }
  if (
    await castBool(
      rpcUrl, protocolConfig, "isValidEpochForContext(uint256,uint256)(bool)",
      current.contextId.toString(), oldEpochId.toString(),
    )
  ) {
    throw new PreflightError(`kms-context-switch: epoch ${oldEpochId} still reads valid after destroy`);
  }
  await assertCurrentUnchanged(rpcUrl, protocolConfig, current, "destroying the retired epoch");
  await pollConnectors(
    parties, "epoch-destroy forwarded to KMS Core",
    columnQuery("kms_epoch_destroyed", "epoch_id", "status", oldEpochId), ["completed"],
  );
  await pollConnectors(
    parties, "destroyed epoch invalidated in the validation cache",
    columnQuery("kms_epoch", "id", "is_valid", oldEpochId), ["false"],
  );
  await expectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an already-destroyed epoch",
    "InvalidKmsEpoch(uint256)",
    "destroyKmsEpoch(uint256)", oldEpochId.toString(),
  );
  console.log(`[kms-context-switch] retired epoch ${oldEpochId} destroyed across contract, connector, and KMS Core`);

  // The active context/epoch must keep serving after the retired material is gone.
  if (
    !(await runDecryption(
      `kms-context-switch: decrypt after destroying retired context ${oldContextId} and epoch ${oldEpochId}`,
    ))
  ) {
    throw new PreflightError(
      "kms-context-switch: user-decryption failed after destroying the retired context/epoch — the current context/epoch must keep serving",
    );
  }
  await runSmoke(`kms-context-switch: input-proof after destroying retired context ${oldContextId} and epoch ${oldEpochId}`);
};

/**
 * Aborts a stuck epoch rotation and recovers from it. Stopping one committee node's tx-sender
 * blocks that node's activation confirmation, and activation needs every signer. The new epoch
 * therefore reshares but stays Pending under the still-Active context. This checks three things:
 * while the rotation is Pending, any new lifecycle operation reverts with
 * KmsLifecycleOperationInFlight; destroying the Pending epoch aborts the rotation; and once the
 * node is back, a fresh rotation activates normally. Same-committee clusters only.
 */
const abortStuckRotation = async (
  state: State,
  target: SwitchTarget,
  owner: Owner,
  baseline: ContextAndEpoch,
  runDecryption: DecryptionRunner,
): Promise<void> => {
  const { rpcUrl, configAddress } = target;
  const parties = state.scenario.kms.parties;
  // A committee member: stopping its tx-sender blocks its on-chain confirmation, so activation can
  // never reach the all-signers quorum. Its core stays up, so the reshare itself still completes.
  const stalledParty = state.scenario.kms.committeeSize;
  const stalledTxSender = kmsTxSenderName(stalledParty);
  const [newKmsEpochTopic, epochDestroyedTopic] = await Promise.all([
    keccakTopic("NewKmsEpoch(uint256,uint256,uint256,uint256,uint256)"),
    keccakTopic("KmsEpochDestroyed(uint256)"),
  ]);

  console.log(`[kms-context-switch] stopping node ${stalledParty} tx-sender to stall an epoch rotation in Pending…`);
  await setRunning([stalledTxSender], "stop");
  await waitForContainersStopped([stalledTxSender]);

  try {
    console.log("[kms-context-switch] broadcasting defineNewEpochForCurrentKmsContext with a node down (rotation sticks Pending)…");
    const rotationReceipt = await castSend(rpcUrl, configAddress, owner, "defineNewEpochForCurrentKmsContext()");
    const pendingEpochId = eventTopicWord(rotationReceipt, newKmsEpochTopic, 2, "NewKmsEpoch");
    const expectedPendingEpochId = baseline.epochId + 1n;
    if (pendingEpochId !== expectedPendingEpochId) {
      throw new PreflightError(
        `kms-context-switch: expected stalled rotation to open epoch ${expectedPendingEpochId}, got ${pendingEpochId}`,
      );
    }

    // Single in-flight: a new lifecycle operation must revert while this epoch is Pending. This
    // also proves the epoch really is Pending, which no view exposes directly.
    await expectRevert(
      rpcUrl, configAddress, owner,
      "open a second lifecycle op while a rotation is in flight",
      "KmsLifecycleOperationInFlight(uint256,uint256)",
      "defineNewEpochForCurrentKmsContext()",
    );
    await assertCurrentUnchanged(rpcUrl, configAddress, baseline, "stalling the epoch rotation");
    console.log(
      `[kms-context-switch] rotation stuck Pending (epoch ${pendingEpochId}); a second lifecycle op reverts KmsLifecycleOperationInFlight and the active pointer held`,
    );

    // A stuck rotation has reshared but never activated (the stopped node blocks the final
    // confirmation). Wait until every party finishes the reshare before aborting; otherwise
    // destroyKmsEpoch races the reshare and a core has nothing to delete.
    // `new_kms_epoch.status = completed` is exactly that signal: a DB trigger sets it when the
    // core's epoch result lands, regardless of whether the activation confirmation went out.
    await pollConnectors(
      parties, "stalled rotation finished resharing (ready to abort)",
      columnQuery("new_kms_epoch", "epoch_id", "status", pendingEpochId), ["completed"],
    );

    // Abort: destroy the Pending epoch (allowed because its context is still Active).
    console.log(`[kms-context-switch] aborting the stuck rotation — destroyKmsEpoch(${pendingEpochId})…`);
    const destroyReceipt = await castSend(rpcUrl, configAddress, owner, "destroyKmsEpoch(uint256)", pendingEpochId.toString());
    if (eventTopicWord(destroyReceipt, epochDestroyedTopic, 1, "KmsEpochDestroyed") !== pendingEpochId) {
      throw new PreflightError(`kms-context-switch: KmsEpochDestroyed event does not carry epochId=${pendingEpochId}`);
    }
    await pollConnectors(
      parties, "aborted epoch-destroy forwarded to KMS Core",
      columnQuery("kms_epoch_destroyed", "epoch_id", "status", pendingEpochId), ["completed"],
    );
    console.log(`[kms-context-switch] stuck rotation aborted: Pending epoch ${pendingEpochId} destroyed`);
  } finally {
    // Restore the node so the recovery rotation can reach the activation quorum.
    await setRunning([stalledTxSender], "start");
    await waitForPartiesRunning([stalledParty]);
  }

  // Recovery: with the stalled node back and no in-flight op, a fresh rotation reshares and activates.
  console.log("[kms-context-switch] recovery: broadcasting defineNewEpochForCurrentKmsContext after the abort…");
  await castSend(rpcUrl, configAddress, owner, "defineNewEpochForCurrentKmsContext()");
  const recovered = await waitForActivation(
    rpcUrl, configAddress, "recovery epoch rotation after abort",
    (current) => current.contextId === baseline.contextId && current.epochId > baseline.epochId,
  );
  console.log(`[kms-context-switch] recovery rotation activated: epochId ${baseline.epochId} -> ${recovered.epochId}`);
  if (!(await runDecryption(`kms-context-switch: decrypt after abort + recovery (epochId=${recovered.epochId})`))) {
    throw new PreflightError(
      "kms-context-switch: user-decryption failed after aborting the stuck rotation and recovering",
    );
  }
};

export const runKmsContextSwitchProfile = async (
  state: State,
  runDecryption: DecryptionRunner,
  runSmoke: SmokeRunner,
) => {
  if (state.scenario.kms.mode !== "threshold") {
    throw new PreflightError(
      "kms-context-switch requires a threshold-mode KMS cluster; rerun `fhevm-cli up --scenario four-party-threshold-kms`",
    );
  }
  const { rpcUrl, configAddress, where } = resolveKmsGenerationTarget(state);
  if (!configAddress) {
    throw new PreflightError(
      `kms-context-switch: no ProtocolConfig address on ${where} — cannot read or switch the KMS context`,
    );
  }

  const baseline = await readContextAndEpoch(rpcUrl, configAddress);
  console.log(`[kms-context-switch] baseline on ${where}: contextId=${baseline.contextId} epochId=${baseline.epochId}`);

  // Baseline app smoke first, so a later failure is attributable to the transition it follows.
  await runSmoke("kms-context-switch: input-proof at baseline (before any switch)");

  // 1) NewKmsContext: a node swap when the cluster has a spare core, a same-committee reshare otherwise.
  const afterSwitch = await switchKmsContext(state, runDecryption, runSmoke, { rpcUrl, configAddress, where }, baseline);

  // 2) NewKmsEpoch: epoch rotation under the (now active) context, then prove it activates.
  console.log("[kms-context-switch] broadcasting defineNewEpochForCurrentKmsContext (NewKmsEpoch)…");
  await stepComposeTask("host-sc", state, ["host-sc-epoch-rotation"], { noDeps: true });
  await waitForContainer("host-sc-epoch-rotation", "complete");
  const afterEpoch = await waitForActivation(
    rpcUrl,
    configAddress,
    "epoch rotation (NewKmsEpoch)",
    (current) => current.contextId === afterSwitch.contextId && current.epochId > afterSwitch.epochId,
  );
  console.log(
    `[kms-context-switch] epoch rotated: epochId ${afterSwitch.epochId} -> ${afterEpoch.epochId} (contextId=${afterEpoch.contextId})`,
  );
  if (!(await runDecryption(`kms-context-switch: decrypt after epoch rotation (epochId=${afterEpoch.epochId})`))) {
    throw new PreflightError(
      `kms-context-switch: user-decryption failed after the epoch rotation to epochId=${afterEpoch.epochId}`,
    );
  }
  await runSmoke(`kms-context-switch: input-proof after the epoch rotation (epochId=${afterEpoch.epochId})`);

  const isSwap = committeeSwapPlan(state.scenario.kms).isSwap;

  // 3) Destruction (same-committee clusters only). Destroying the retired context/epoch expects
  //    every party to decommission it. In a node swap the retired context's committee is not the
  //    current party set, so the dropped node and the promoted spare legitimately cannot: the KMS
  //    core rejects a destroy that would leave a node with no context/epoch, and their connector
  //    rows end `failed` instead of `completed`. Destruction is therefore exercised on the
  //    non-swap scenario; the swap scenario's job is the node-swap switch itself.
  if (!isSwap) {
    const owner = await loadHostOwner();
    await destroyContextAndEpoch(
      state,
      { rpcUrl, protocolConfig: configAddress, owner },
      {
        oldContextId: baseline.contextId,
        contextEpochId: baseline.epochId,
        oldEpochId: afterSwitch.epochId,
        current: afterEpoch,
      },
      runDecryption,
      runSmoke,
    );

    // 3b) A context switch must still work after a destroy. The connector reads the previous
    //     key/CRS material via getCrsMaterials, which resolves the context that material was
    //     generated under — the context just destroyed. Check the switch still reshares and
    //     activates end to end.
    console.log(
      "[kms-context-switch] broadcasting a second switch after the destroy (a destroyed context must not stall the next switch)…",
    );
    const afterDestroySwitch = await switchKmsContext(
      state,
      runDecryption,
      runSmoke,
      { rpcUrl, configAddress, where },
      afterEpoch,
    );
    console.log(
      `[kms-context-switch] post-destroy switch activated: contextId ${afterEpoch.contextId} -> ${afterDestroySwitch.contextId} (epochId=${afterDestroySwitch.epochId})`,
    );

    // 3c) Abort a stuck epoch rotation. Check that the single-in-flight guard reverts a second
    //     operation, that destroying the Pending epoch aborts the rotation, and that a fresh
    //     rotation then activates.
    await abortStuckRotation(state, { rpcUrl, configAddress, where }, owner, afterDestroySwitch, runDecryption);
  }

  // 4) Node swap only: prove the promoted spare actually holds a working reshared key. Runs last,
  //    so the earlier steps see a healthy cluster.
  if (isSwap) {
    await proveSpareInQuorum(state, runDecryption);
  }

  console.log(
    isSwap
      ? "[kms-context-switch] PASS — NewKmsContext (node swap) and NewKmsEpoch both activated on chain, the promoted spare serves the 2t+1 quorum, user-decryption works under each transition, and the input-proof app flow held at every checkpoint"
      : "[kms-context-switch] PASS — NewKmsContext and NewKmsEpoch both activated on chain, the retired context and epoch were destroyed across contract, connector, and KMS Core, a further context switch after the destroy still reshared and activated, a stuck rotation was aborted (single-in-flight revert + Pending-epoch destroy) and recovered, user-decryption works under each transition, and the input-proof app flow held at every checkpoint",
  );
};
