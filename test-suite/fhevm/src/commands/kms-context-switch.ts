/**
 * The `kms-context-switch` acceptance profile for `fhevm-cli test`.
 *
 * Runs the full ProtocolConfig KMS-context lifecycle end to end on one threshold-mode cluster and
 * checks that the KMS reacts to each emitted event. It requires the `five-party-swap-threshold-kms`
 * scenario: 5 cores, an initial 4-node committee {1,2,3,4} (t = 1), and one spare core (node 5)
 * that boots idle. The spare makes the final node-swap step possible; every earlier step runs on
 * the unchanged {1,2,3,4} committee.
 *
 *   1. Same-committee context switch (NewKmsContext). Broadcast `defineNewKmsContextAndEpoch` with
 *      the same committee, so no new signing keys are needed. Wait until the new context becomes
 *      the active one on chain, then decrypt.
 *   2. Epoch rotation (NewKmsEpoch). Broadcast `defineNewEpochForCurrentKmsContext` (same context,
 *      new epoch). Wait until the new epoch activates, then decrypt again.
 *   3. Destruction. Steps 1 and 2 leave two entries that are still live but no longer current: the
 *      baseline context and the epoch from step 1. Destroy them with `destroyKmsContext` /
 *      `destroyKmsEpoch` and check:
 *        - the current context and epoch cannot be destroyed;
 *        - unknown ids and already-destroyed ids revert;
 *        - `KmsContextDestroyed` / `KmsEpochDestroyed` emitted;
 *        - the destroyed target becomes invalid, and the active context/epoch does not move;
 *        - destroying a context also invalidates its epochs (`isValidEpochForContext` requires a
 *          valid context, so the contract needs no per-epoch write);
 *        - every party's connector forwards `DestroyMpcContext` / `DestroyMpcEpoch` to its core
 *          and invalidates the destroyed target in its validation cache;
 *        - the spare never held the material, so its core accepts the context destroy (nothing to
 *          delete) but rejects the epoch destroy, and that request ends `failed`. Both outcomes
 *          are benign, and the spare still invalidates its cache rows;
 *        - the context-to-epoch cascade reaches committee caches only: it is driven by the epoch
 *          ids each core returns, and the spare's core returns none. Its stale epoch row is
 *          harmless — on-chain epoch validity derives from context validity;
 *        - the current context/epoch keeps serving.
 *   4. Recovery and abort. Two independent checks:
 *      a) A context switch must still work after a destroy. That is, destroying a context must
 *         never leave the system stuck and unable to reshare and activate a new one.
 *      b) A stuck epoch rotation can be aborted and retried. Stop one node's tx-sender and start
 *         a rotation: the new epoch reshares but stays Pending. While it is Pending, any other
 *         lifecycle operation reverts with `KmsLifecycleOperationInFlight`. `destroyKmsEpoch`
 *         aborts the Pending epoch, so the previous epoch is current again. Restart the node and
 *         rotate again: the fresh rotation activates normally.
 *   5. Node swap (NewKmsContext with a membership change). Broadcast the switch with the swap
 *      committee {1,2,3,5}: node 4 is dropped and the spare is promoted. Node 4's tx-sender is
 *      stopped before the broadcast — a node that cannot transact must not block its own
 *      replacement. After activation, stop `t` continuing members so the live committee is exactly
 *      the 2t+1 quorum including the spare, then decrypt: this proves the spare holds a working
 *      reshared key. Running the swap last also proves a switch still works after every destroy
 *      and abort above.
 *
 * Activation is not automatic. The KMS cores must reshare, and the connectors must submit
 * `confirmKmsContextCreation` / `confirmEpochActivation`, before `getCurrentKmsContextAndEpoch`
 * advances. If that never happens, the ids stay put and the profile fails with the last-read
 * state. This makes the profile also the discovery test for whether the cluster reshares at all.
 * All lifecycle transactions target the ProtocolConfig contract on the host chain.
 *
 * App-level checkpoints: every transition must stay invisible to a normal encrypted-input flow,
 * not only to the dedicated user-decryption probe. The input-proof smoke therefore runs at
 * baseline, while a switch is pending (the previous context must keep serving during the
 * reshare), and after each transition.
 *
 * Disruptive and single-run: the profile advances the KMS context/epoch and destroys the retired
 * ones, so it expects a pristine stack. Re-up between runs
 * (`fhevm-cli down && fhevm-cli up --scenario five-party-swap-threshold-kms`).
 */
import { PreflightError } from "../errors";
import { castBool, castCall, resolveKmsGenerationTarget, waitForContainer } from "../flow/readiness";
import { stepComposeTask } from "../flow/runtime-compose";
import { columnQuery, checkConnectorsDbColumn } from "../kms-connector-db";
import { castSend, getEventTopic, callContractAndExpectRevert, keccakTopic, loadHostOwner, type Owner } from "../kms-onchain";
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
    const isActivationTimedOut = Date.now() >= deadline;
    if (isActivationTimedOut) {
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

/** Node roles on this topology. The initial on-chain committee is parties 1..committeeSize; cores
 * beyond it are spares. The node-swap step drops the last committee slot(s) and promotes the
 * spare(s), e.g. {1,2,3,4} -> {1,2,3,5}. */
const committeePlan = (kms: State["scenario"]["kms"]) => {
  const spareCount = kms.parties - kms.committeeSize;
  return {
    committee: Array.from({ length: kms.committeeSize }, (_, i) => i + 1),
    spares: Array.from({ length: spareCount }, (_, i) => kms.committeeSize + 1 + i),
    continuing: Array.from({ length: kms.committeeSize - spareCount }, (_, i) => i + 1),
    dropped: Array.from({ length: spareCount }, (_, i) => kms.committeeSize - spareCount + 1 + i),
  };
};

/**
 * NewKmsContext step, in one of two modes.
 * Same-committee (swap = false): the new context keeps the committee, so this is a pure reshare.
 * Node swap (swap = true): the new context drops one committee node and promotes the spare, so n
 * stays committeeSize. The dropped node's tx-sender is stopped before the broadcast. The switch
 * must then complete on the n − t quorum of the previous committee, without the dropped node's
 * confirmation — a node that cannot transact must not veto its own removal. The rest of the
 * dropped node stays up, because the reshare still needs its core (the KMS core cannot yet
 * reshare around an absent outgoing party); upgrade to a full party stop once it can.
 * In both modes activation waits for ALL new-committee signers, so the context id only advances
 * once every new-committee member has reshared and confirmed.
 */
const switchKmsContext = async (
  state: State,
  runDecryption: DecryptionRunner,
  runSmoke: SmokeRunner,
  target: SwitchTarget,
  baseline: ContextAndEpoch,
  swap: boolean,
): Promise<ContextAndEpoch> => {
  const { continuing, spares, dropped } = committeePlan(state.scenario.kms);
  // Same services in both modes; a swap just points them at the swap-committee env files.
  const hostEnv: Record<string, string> = swap ? { HOST_SC_CONTEXT_ENV: "host-sc-swap.env" } : {};
  const gatewayEnv: Record<string, string> = swap ? { GATEWAY_SC_CONTEXT_ENV: "gateway-sc-swap.env" } : {};

  if (swap) {
    // Stopped before the broadcast and left down: the new committee does not include the node.
    const droppedTxSenders = dropped.map((party) => kmsTxSenderName(party));
    console.log(
      `[kms-context-switch] stopping dropped node(s) ${dropped.join(",")} tx-sender before the switch — a node that cannot confirm must not block its own replacement…`,
    );
    await setRunning(droppedTxSenders, "stop");
    await waitForContainersStopped(droppedTxSenders);
  }

  console.log(
    swap
      ? `[kms-context-switch] broadcasting defineNewKmsContextAndEpoch — node swap (drop ${dropped.join(",")} with tx-sender stopped, add ${spares.join(",")}, keep ${continuing.join(",")})…`
      : "[kms-context-switch] broadcasting defineNewKmsContextAndEpoch - same committee… (no node changes)",
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

  const decryptionSuccessOnPendingSwitch = await runDecryption("kms-context-switch: decrypt while the switch is pending (previous context serving)");

  if (!decryptionSuccessOnPendingSwitch) {
    throw new PreflightError(
      "kms-context-switch: user-decryption failed while the context switch was pending — the previous context must keep serving until the new one activates",
    );
  }

  const afterSwitch = await waitForActivation(
    target.rpcUrl,
    target.configAddress,
    swap ? "node-swap context" : "same committee context",
    (current) => current.contextId > baseline.contextId,
  );
  if (afterSwitch.contextId !== pendingContextId) {
    throw new PreflightError(
      `kms-context-switch: activated unexpected contextId=${afterSwitch.contextId}; expected pre-registered contextId=${pendingContextId}`,
    );
  }
  console.log(`[kms-context-switch] context switched: contextId ${baseline.contextId} -> ${afterSwitch.contextId} (epochId=${afterSwitch.epochId})`);

  const decryptionSuccessAfterSwitch = await runDecryption(`kms-context-switch: decrypt after the context switch (contextId=${afterSwitch.contextId})`);
  if (!decryptionSuccessAfterSwitch) {
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
  const { continuing, spares } = committeePlan(kms);
  const reconstruct = reconstructionThreshold(kms.threshold);
  const forced = continuing.slice(continuing.length - kms.threshold);
  const forcedContainers = forced.flatMap((party) => partyContainers(party));
  console.log(
    `[kms-context-switch] stopping ${forced.join(",")} so spare(s) ${spares.join(",")} are required for the ${reconstruct}/${kms.committeeSize} (2t+1) quorum…`,
  );
  await setRunning(forcedContainers, "stop");
  try {
    await waitForPartiesStopped(forced);
    const decryptionSuccessWithSpareInQuorum = await runDecryption(
      `kms-context-switch: decrypt with spare(s) ${spares.join(",")} in the 2t+1 quorum`,
    );
    if (!decryptionSuccessWithSpareInQuorum) {
      throw new PreflightError(
        `kms-context-switch: decryption failed with spare(s) ${spares.join(",")} forced into the quorum — the reshared key did not work`,
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
 *     core and marks the destroyed target invalid in its validation cache (invalidation follows
 *     the chain event, not the core outcome). The spare never held the material: its core acks
 *     the context destroy trivially but rejects the epoch destroy, so that request ends `failed`
 *     — benign either way. A context destroy also cascades to the context's epoch, whose id
 *     DestroyMpcContext returns — on committee parties only: the spare's core returns no epoch
 *     ids, so its cache row for that epoch stays valid. That stale row is harmless: on-chain
 *     epoch validity derives from context validity;
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
  const { committee, spares } = committeePlan(state.scenario.kms);
  const [contextDestroyedTopic, epochDestroyedTopic] = await Promise.all([
    keccakTopic("KmsContextDestroyed(uint256)"),
    keccakTopic("KmsEpochDestroyed(uint256)"),
  ]);

  // Check the reverts first, while everything is still live. Destroying the current context/epoch
  // would strand live decryptions, and unknown ids were never issued.
  const unknownId = (1n << 255n).toString();
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy the current KMS context",
    "LatestActiveKmsContextCannotBeDestroyed(uint256)",
    "destroyKmsContext(uint256)", current.contextId.toString(),
  );
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy the current KMS epoch",
    "LatestActiveKmsEpochCannotBeDestroyed(uint256)",
    "destroyKmsEpoch(uint256)", current.epochId.toString(),
  );
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an unknown KMS context",
    "InvalidKmsContext(uint256)",
    "destroyKmsContext(uint256)", unknownId,
  );
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an unknown KMS epoch",
    "InvalidKmsEpoch(uint256)",
    "destroyKmsEpoch(uint256)", unknownId,
  );
  // Owner-gated: a non-owner is rejected by onlyACLOwner before any target check, so a valid
  // destroyable id still reverts with the access-control error rather than the target-state one.
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, NON_OWNER,
    "destroy a KMS context as a non-owner",
    "NotHostOwner(address)",
    "destroyKmsContext(uint256)", oldContextId.toString(),
  );
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, NON_OWNER,
    "destroy a KMS epoch as a non-owner",
    "NotHostOwner(address)",
    "destroyKmsEpoch(uint256)", oldEpochId.toString(),
  );
  console.log(
    "[kms-context-switch] destroy reverts OK (non-owner, current context/epoch, and unknown ids are all rejected)",
  );

  // Destroy the retired context.
  const isOldContextValid = await castBool(rpcUrl, protocolConfig, "isValidKmsContext(uint256)(bool)", oldContextId.toString());
  if (!isOldContextValid) {
    throw new PreflightError(
      `kms-context-switch: retired context ${oldContextId} is not valid before destroy — nothing to prove the destroy transition against`,
    );
  }

  console.log(`[kms-context-switch] destroying retired context ${oldContextId}…`);
  const contextReceipt = await castSend(rpcUrl, protocolConfig, owner, "destroyKmsContext(uint256)", oldContextId.toString());
  const destroyedContextIdFromEvent = getEventTopic(contextReceipt, contextDestroyedTopic, 1);
  if (destroyedContextIdFromEvent !== oldContextId) {
    throw new PreflightError(`kms-context-switch: KmsContextDestroyed event does not carry contextId=${oldContextId}`);
  }

  const isOldContextStillValid = await castBool(rpcUrl, protocolConfig, "isValidKmsContext(uint256)(bool)", oldContextId.toString());
  if (isOldContextStillValid) {
    throw new PreflightError(`kms-context-switch: context ${oldContextId} still reads valid after destroy`);
  }

  await assertCurrentUnchanged(rpcUrl, protocolConfig, current, "destroying the retired context");
  // Every party completes the destroy, the spare included: its core never held the context, so
  // it has nothing to delete and completes trivially.
  await checkConnectorsDbColumn(
    parties, "context-destroy forwarded to KMS Core",
    columnQuery("kms_context_destroyed", "context_id", "status", oldContextId), ["completed"],
  );
  await checkConnectorsDbColumn(
    parties, "destroyed context invalidated in the validation cache",
    // Postgres renders boolean::text as 'true'/'false' (not the 't'/'f' psql shows for the raw type).
    columnQuery("kms_context", "id", "is_valid", oldContextId), ["false"],
  );
  // DestroyMpcContext returns the epoch ids it decommissioned, and the connector invalidates
  // them. The retired context's epoch must therefore turn invalid from the context
  // destroy alone, with no separate destroyKmsEpoch call. Committee only: the spare's core held
  // no epochs, so it returns no epoch ids and no cascade happens there (checked right after).
  await checkConnectorsDbColumn(
    committee, "destroyed context cascaded to its epoch in the validation cache",
    columnQuery("kms_epoch", "id", "is_valid", contextEpochId), ["false"],
  );
  // The spare's row for that epoch stays valid — harmless, since on-chain epoch validity derives
  // from context validity. Checked after the committee cascade settles, so this is not a race.
  await checkConnectorsDbColumn(
    spares, "no cascade on the spare (its core returned no epoch ids)",
    columnQuery("kms_epoch", "id", "is_valid", contextEpochId), ["true"],
  );
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an already-destroyed context",
    "InvalidKmsContext(uint256)",
    "destroyKmsContext(uint256)", oldContextId.toString(),
  );
  console.log(`[kms-context-switch] retired context ${oldContextId} destroyed across contract, connector, and KMS Core`);

  // Destroy the retired epoch (still under the current context, superseded by the rotation).
  const isOldEpochValid = await castBool(
    rpcUrl, protocolConfig, "isValidEpochForContext(uint256,uint256)(bool)",
    current.contextId.toString(), oldEpochId.toString(),
  );
  if (!isOldEpochValid) {
    throw new PreflightError(
      `kms-context-switch: retired epoch ${oldEpochId} is not valid under context ${current.contextId} before destroy`,
    );
  }
  console.log(`[kms-context-switch] destroying retired epoch ${oldEpochId}…`);
  const epochReceipt = await castSend(rpcUrl, protocolConfig, owner, "destroyKmsEpoch(uint256)", oldEpochId.toString());
  const destroyedEpochIdFromEvent = getEventTopic(epochReceipt, epochDestroyedTopic, 1);
  if (destroyedEpochIdFromEvent !== oldEpochId) {
    throw new PreflightError(`kms-context-switch: KmsEpochDestroyed event does not carry epochId=${oldEpochId}`);
  }
  const isOldEpochStillValid = await castBool(
    rpcUrl, protocolConfig, "isValidEpochForContext(uint256,uint256)(bool)",
    current.contextId.toString(), oldEpochId.toString(),
  );
  if (isOldEpochStillValid) {
    throw new PreflightError(`kms-context-switch: epoch ${oldEpochId} still reads valid after destroy`);
  }
  await assertCurrentUnchanged(rpcUrl, protocolConfig, current, "destroying the retired epoch");
  await checkConnectorsDbColumn(
    committee, "epoch-destroy forwarded to KMS Core",
    columnQuery("kms_epoch_destroyed", "epoch_id", "status", oldEpochId), ["completed"],
  );
  // Unlike a context destroy (which the spare's core acks trivially), the core rejects a destroy
  // for an epoch it never held, so the spare's request ends failed. Benign: it had nothing to
  // delete, and its cache row still turns invalid below — invalidation follows the chain event,
  // not the core outcome.
  await checkConnectorsDbColumn(
    spares, "spare's epoch-destroy ends failed (never held the epoch)",
    columnQuery("kms_epoch_destroyed", "epoch_id", "status", oldEpochId), ["failed"],
  );
  await checkConnectorsDbColumn(
    parties, "destroyed epoch invalidated in the validation cache",
    columnQuery("kms_epoch", "id", "is_valid", oldEpochId), ["false"],
  );
  await callContractAndExpectRevert(
    rpcUrl, protocolConfig, owner,
    "destroy an already-destroyed epoch",
    "InvalidKmsEpoch(uint256)",
    "destroyKmsEpoch(uint256)", oldEpochId.toString(),
  );
  console.log(`[kms-context-switch] retired epoch ${oldEpochId} destroyed across contract, connector, and KMS Core`);

  // The active context/epoch must keep serving after the retired material is gone.
  const decryptionSuccessAfterDestroy = await runDecryption(
    `kms-context-switch: decrypt after destroying retired context ${oldContextId} and epoch ${oldEpochId}`,
  );
  if (!decryptionSuccessAfterDestroy) {
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
 * node is back, a fresh rotation activates normally. Returns the state after the recovery
 * rotation.
 */
const abortStuckRotation = async (
  state: State,
  target: SwitchTarget,
  owner: Owner,
  baseline: ContextAndEpoch,
  runDecryption: DecryptionRunner,
): Promise<ContextAndEpoch> => {
  const { rpcUrl, configAddress } = target;
  const { committee, spares } = committeePlan(state.scenario.kms);
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
    const pendingEpochId = getEventTopic(rotationReceipt, newKmsEpochTopic, 2);
    const expectedPendingEpochId = baseline.epochId + 1n;
    if (pendingEpochId !== expectedPendingEpochId) {
      throw new PreflightError(
        `kms-context-switch: expected stalled rotation to open epoch ${expectedPendingEpochId}, got ${pendingEpochId}`,
      );
    }

    // Single in-flight: a new lifecycle operation must revert while this epoch is Pending. This
    // also proves the epoch really is Pending, which no view exposes directly.
    await callContractAndExpectRevert(
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
    // Committee only: the spare is not in this committee, so its core does not reshare here.
    await checkConnectorsDbColumn(
      committee, "stalled rotation finished resharing (ready to abort)",
      columnQuery("new_kms_epoch", "epoch_id", "status", pendingEpochId), ["completed"],
    );

    // Abort: destroy the Pending epoch (allowed because its context is still Active).
    console.log(`[kms-context-switch] aborting the stuck rotation — destroyKmsEpoch(${pendingEpochId})…`);
    const destroyReceipt = await castSend(rpcUrl, configAddress, owner, "destroyKmsEpoch(uint256)", pendingEpochId.toString());
    const abortedEpochIdFromEvent = getEventTopic(destroyReceipt, epochDestroyedTopic, 1);
    if (abortedEpochIdFromEvent !== pendingEpochId) {
      throw new PreflightError(`kms-context-switch: KmsEpochDestroyed event does not carry epochId=${pendingEpochId}`);
    }
    await checkConnectorsDbColumn(
      committee, "aborted epoch-destroy forwarded to KMS Core",
      columnQuery("kms_epoch_destroyed", "epoch_id", "status", pendingEpochId), ["completed"],
    );
    // The spare never reshared this Pending epoch, so its core rejects the destroy (see the same
    // asymmetry in destroyContextAndEpoch).
    await checkConnectorsDbColumn(
      spares, "spare's aborted epoch-destroy ends failed (never held the epoch)",
      columnQuery("kms_epoch_destroyed", "epoch_id", "status", pendingEpochId), ["failed"],
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
  const decryptionSuccessAfterRecovery = await runDecryption(
    `kms-context-switch: decrypt after abort + recovery (epochId=${recovered.epochId})`,
  );
  if (!decryptionSuccessAfterRecovery) {
    throw new PreflightError(
      "kms-context-switch: user-decryption failed after aborting the stuck rotation and recovering",
    );
  }
  return recovered;
};

export const runKmsContextSwitchProfile = async (
  state: State,
  runDecryption: DecryptionRunner,
  runSmoke: SmokeRunner,
) => {
  if (state.scenario.kms.mode !== "threshold") {
    throw new PreflightError(
      "kms-context-switch requires a threshold-mode KMS cluster; rerun `fhevm-cli up --scenario five-party-swap-threshold-kms`",
    );
  }
  // The node-swap step needs a spare core, and the swap env files only exist when the cluster has
  // one — on a spare-less cluster the profile would only fail later, at the swap broadcast.
  if (state.scenario.kms.parties <= state.scenario.kms.committeeSize) {
    throw new PreflightError(
      `kms-context-switch requires a spare core for the node-swap step (parties > committeeSize; ` +
        `got parties=${state.scenario.kms.parties}, committeeSize=${state.scenario.kms.committeeSize}); ` +
        "rerun `fhevm-cli up --scenario five-party-swap-threshold-kms`",
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

  // 1) Same-committee context switch (NewKmsContext).
  const afterSwitch = await switchKmsContext(
    state, runDecryption, runSmoke,
    { rpcUrl, configAddress, where },
    baseline, false,
  );

  // 2) Same-committee key resharing (NewKmsEpoch). Then probes the new epoch ID activates.
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
  const decryptionSuccessAfterEpoch = await runDecryption(`kms-context-switch: decrypt after epoch rotation (epochId=${afterEpoch.epochId})`);
  if (!decryptionSuccessAfterEpoch) {
    throw new PreflightError(
      `kms-context-switch: user-decryption failed after the epoch rotation to epochId=${afterEpoch.epochId}`,
    );
  }
  await runSmoke(`kms-context-switch: input-proof after the epoch rotation (epochId=${afterEpoch.epochId})`);

  // 3) Destruction: retire the baseline context and the superseded epoch across every layer.
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

  // 4a) A context switch must still work after a destroy. The connector reads the previous
  //     key/CRS material via getCrsMaterials, which resolves the context that material was
  //     generated under — the context just destroyed. Check the switch still reshares and
  //     activates end to end.
  console.log(
    "[kms-context-switch] broadcasting a second switch after the destroy (a destroyed context must not stall the next switch)…",
  );
  const afterDestroySwitch = await switchKmsContext(
    state, runDecryption, runSmoke,
    { rpcUrl, configAddress, where },
    afterEpoch, false,
  );
  console.log(
    `[kms-context-switch] post-destroy switch activated: contextId ${afterEpoch.contextId} -> ${afterDestroySwitch.contextId} (epochId=${afterDestroySwitch.epochId})`,
  );

  // 4b) Abort a stuck epoch rotation. Check that the single-in-flight guard reverts a second
  //     operation, that destroying the Pending epoch aborts the rotation, and that a fresh
  //     rotation then activates.
  const recovered = await abortStuckRotation(
    state, { rpcUrl, configAddress, where }, owner, afterDestroySwitch, runDecryption,
  );

  // 5) Node swap: drop a committee node, promote the spare. Running it after every destroy and
  //    abort above also proves the lifecycle never left the system unable to switch.
  await switchKmsContext(
    state, runDecryption, runSmoke,
    { rpcUrl, configAddress, where },
    recovered, true,
  );

  // 5b) Prove the promoted spare actually holds a working reshared key.
  await proveSpareInQuorum(state, runDecryption);

  console.log(
    "[kms-context-switch] PASS — same-committee switch and epoch rotation activated on chain; the retired " +
      "context and epoch were destroyed across contract, connector, and KMS Core (the spare's benign " +
      "outcomes and the committee-only cascade asserted); a further switch after the destroy still reshared and activated; a stuck rotation " +
      "was aborted (single-in-flight revert + Pending-epoch destroy) and recovered; the node swap activated " +
      "and the promoted spare serves the 2t+1 quorum; user-decryption works under each transition; and the " +
      "input-proof app flow held at every checkpoint",
  );
};
