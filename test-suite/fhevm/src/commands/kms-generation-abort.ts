/**
 * The `kms-generation-abort` acceptance profile for `fhevm-cli test`. It checks the abort flow
 * of FHE key and CRS generation across the KMSGeneration contract, the kms-connector, and KMS
 * Core:
 *
 *   - Abort keygen mid-flight: the contract retires both request ids (`AbortKeygen` event,
 *     `KeyAborted` getter, no consensus), the connector settles its rows, and the active key
 *     is unchanged.
 *   - Abort crsgen mid-flight: the crsgen mirror of the above.
 *   - Recovery: a fresh keygen and crsgen must reach ActivateKey/ActivateCrs with materials
 *     published to MinIO — aborts must unblock the one-request-at-a-time pipeline.
 *   - Reverts: invalid ids, double aborts, aborts of completed requests, and triggers while a
 *     request is in flight.
 *
 * Two rules keep the runs deterministic:
 *   - Sleep TRIGGER_TO_ABORT_SLEEP_MS between the trigger and the abort — long enough for the
 *     connectors to pick the request and register the ceremony on every KMS core, yet below the
 *     ceremony duration so the abort still lands mid-flight.
 *   - Assert connector statuses without assuming a specific phase — depending on where the
 *     abort catches a party, request rows end `aborted` (KMS Core canceled the ceremony),
 *     `completed` (late work recorded on chain without consensus), or `failed` (a threshold
 *     party's ceremony died once its peers aborted theirs), and abort rows end `completed`
 *     or `failed` (nothing left to cancel). What matters: no work stays in flight, nothing
 *     activates, and recovery completes.
 *
 * Disruptive: recovery rotates the active key/CRS. Persisted discovery is re-synced, but
 * running services keep their fetched keyset — run this profile last, or re-up afterwards.
 */
import { PreflightError } from "../errors";
import { castBool, castCall, ensureMaterial, resolveKmsGenerationTarget } from "../flow/readiness";
import { columnQuery, checkConnectorsDbColumn } from "../kms-connector-db";
import {
  castSend,
  eventLogWord,
  callContractAndExpectRevert,
  keccakTopic,
  loadHostOwner,
  type Owner,
  parseUintOutput,
  type Receipt,
} from "../kms-onchain";
import { saveState } from "../state/state";
import type { State } from "../types";
import { uint256ToId } from "../utils/fs";

/**
 * Trigger-to-abort delay keeping the events in different kms-connector gw-listener poll batches.
 * Lower bound: KMS_CONNECTOR_KEY_MANAGEMENT_POLLING (1s, templates/env/.env.kms-connector).
 * Upper bound: ceremony duration (Test params in CI: ~24s crsgen, ~40s keygen preproc) —
 * past that the ceremony is over and the abort reverts AlreadyDone.
 */
const TRIGGER_TO_ABORT_SLEEP_MS = 3_000;
/** A few listener batch cycles: a KeygenRequest emitted alongside the trigger may still be ingesting. */
const LISTENER_BATCH_GRACE_MS = 10_000;
/** Bound for a full post-abort keygen/crsgen cycle (trigger -> consensus -> activation). */
const RECOVERY_TIMEOUT_MS = Number(process.env.KMS_ABORT_RECOVERY_TIMEOUT_SECONDS ?? "900") * 1_000;
const RECOVERY_POLL_MS = 5_000;

/** The event topics the profile asserts on, hashed up front. */
const loadAbiHashes = async () => {
  const [prepKeygenRequest, crsgenRequest, abortKeygen, abortCrsgen] = await Promise.all([
    keccakTopic("PrepKeygenRequest(uint256,uint8,bytes)"),
    keccakTopic("CrsgenRequest(uint256,uint256,uint8,bytes)"),
    keccakTopic("AbortKeygen(uint256)"),
    keccakTopic("AbortCrsgen(uint256)"),
  ]);
  return { topics: { prepKeygenRequest, crsgenRequest, abortKeygen, abortCrsgen } };
};

type Target = ReturnType<typeof resolveKmsGenerationTarget>;
type AbiHashes = Awaited<ReturnType<typeof loadAbiHashes>>;

/** Asserts the shared on-chain terminal state of an aborted request. */
const assertRequestRetiredOnChain = async (target: Target, kind: "keygen" | "crsgen", requestId: bigint) => {
  if (!(await castBool(target.rpcUrl, target.kmsGenerationAddress, "isRequestDone(uint256)(bool)", requestId.toString()))) {
    throw new PreflightError(`kms-generation-abort: ${kind} ${requestId} is not done after the abort`);
  }
  const consensus = await castCall(target.rpcUrl, target.kmsGenerationAddress, "getConsensusTxSenders(uint256)(address[])", requestId.toString());
  if (consensus !== "[]") {
    throw new PreflightError(`kms-generation-abort: aborted ${kind} ${requestId} has consensus tx senders: ${consensus}`);
  }
};

const readActiveIds = async (target: Target) => ({
  keyId: parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, "getActiveKeyId()(uint256)")),
  crsId: parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, "getActiveCrsId()(uint256)")),
});

/** The active id must not move across an abort; when it does, tell apart the aborted request
 * activating (the product failure the abort exists to prevent) from an unrelated id activating
 * (a pending ceremony from an earlier or concurrent run completing mid-flight). */
export const assertActiveIdUnchanged = (kind: "key" | "CRS", baseline: bigint, current: bigint, abortedId: bigint) => {
  if (current === baseline) {
    return;
  }
  if (current === abortedId) {
    throw new PreflightError(
      `kms-generation-abort: the aborted ${kind} ${abortedId} became active — the abort did not prevent activation`,
    );
  }
  throw new PreflightError(
    `kms-generation-abort: active ${kind} changed to an id this run never requested (${baseline} -> ${current}) — a pending ceremony from an earlier or concurrent run completed mid-flight; make sure only one run targets the stack, or re-up for a clean baseline`,
  );
};

/**
 * Aborts an in-flight keygen and proves every layer dropped it: the contract state, the
 * connector DBs (no rows left in flight), and an unchanged active key.
 */
const abortKeygenMidFlight = async (state: State, target: Target, owner: Owner, abi: AbiHashes, paramsType: string, baselineKeyId: bigint) => {
  console.log("[kms-generation-abort] triggering keygen to abort it mid-flight…");
  const trigger = await castSend(target.rpcUrl, target.kmsGenerationAddress, owner,"keygen(uint8)", paramsType);
  // The receipt's PrepKeygenRequest event is the in-flight proof.
  const prepKeygenId = eventLogWord(trigger, abi.topics.prepKeygenRequest, "PrepKeygenRequest");
  const keyId = parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, "getKeyCounter()(uint256)"));
  console.log(`[kms-generation-abort] keygen in flight: prepKeygenId=${prepKeygenId} keyId=${keyId}`);

  // The pipeline is exclusive while the request is in flight.
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"keygen while one is in flight", "KeygenOngoing(uint256)", "keygen(uint8)", paramsType);

  // Let every connector register the ceremony on its KMS core before the abort event exists.
  await Bun.sleep(TRIGGER_TO_ABORT_SLEEP_MS);

  let abortReceipt: Receipt;
  try {
    abortReceipt = await castSend(target.rpcUrl, target.kmsGenerationAddress, owner,"abortKeygen(uint256)", prepKeygenId.toString());
  } catch (error) {
    throw new PreflightError(
      `kms-generation-abort: abortKeygen(${prepKeygenId}) failed — an AbortKeygenAlreadyDone revert means the ceremony completed before the abort landed (stack too fast for a mid-flight abort): ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  if (eventLogWord(abortReceipt, abi.topics.abortKeygen, "AbortKeygen") !== prepKeygenId) {
    throw new PreflightError(`kms-generation-abort: AbortKeygen event does not carry prepKeygenId=${prepKeygenId}`);
  }
  console.log(`[kms-generation-abort] AbortKeygen(${prepKeygenId}) emitted`);

  await assertRequestRetiredOnChain(target, "keygen", prepKeygenId);
  await assertRequestRetiredOnChain(target, "keygen", keyId);
  // No consensus digest was stored, so the key must read as aborted, and abort is terminal.
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"params type of the aborted key", "KeyAborted(uint256)", "getKeyParamsType(uint256)", keyId.toString());
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"double keygen abort", "AbortKeygenAlreadyDone(uint256)", "abortKeygen(uint256)", prepKeygenId.toString());

  // Connector settlement (see the module doc for why the statuses are phase-dependent).
  const parties = state.scenario.kms.parties;
  await checkConnectorsDbColumn(parties, "abort ingested and terminal", columnQuery("abort_keygen_requests", "prep_keygen_id", "status", prepKeygenId), ["completed", "failed"]);
  await checkConnectorsDbColumn(parties, "prep-keygen request terminal", columnQuery("prep_keygen_requests", "prep_keygen_id", "status", prepKeygenId), ["completed", "aborted", "failed"]);
  // A KeygenRequest emitted in the trigger-to-abort window may still be one listener batch away.
  await Bun.sleep(LISTENER_BATCH_GRACE_MS);
  await checkConnectorsDbColumn(parties, "no keygen work left in flight", columnQuery("keygen_requests", "key_id", "status", keyId), ["missing", "completed", "aborted", "failed"]);

  const { keyId: activeKeyId } = await readActiveIds(target);
  assertActiveIdUnchanged("key", baselineKeyId, activeKeyId, keyId);
  console.log("[kms-generation-abort] keygen abort verified across contract, connector, and active key");
};

/** The crsgen mirror of `abortKeygenMidFlight` (single-phase, so no second-event grace). */
const abortCrsgenMidFlight = async (state: State, target: Target, owner: Owner, abi: AbiHashes, paramsType: string, baselineCrsId: bigint) => {
  console.log("[kms-generation-abort] triggering crsgen to abort it mid-flight…");
  const trigger = await castSend(target.rpcUrl, target.kmsGenerationAddress, owner,"crsgenRequest(uint256,uint8)", "2048", paramsType);
  const crsId = eventLogWord(trigger, abi.topics.crsgenRequest, "CrsgenRequest");
  console.log(`[kms-generation-abort] crsgen in flight: crsId=${crsId}`);

  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"crsgen while one is in flight", "CrsgenOngoing(uint256)", "crsgenRequest(uint256,uint8)", "2048", paramsType);

  // Let every connector register the ceremony on its KMS core before the abort event exists.
  await Bun.sleep(TRIGGER_TO_ABORT_SLEEP_MS);

  let abortReceipt: Receipt;
  try {
    abortReceipt = await castSend(target.rpcUrl, target.kmsGenerationAddress, owner,"abortCrsgen(uint256)", crsId.toString());
  } catch (error) {
    throw new PreflightError(
      `kms-generation-abort: abortCrsgen(${crsId}) failed — an AbortCrsgenAlreadyDone revert means the ceremony completed before the abort landed: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  if (eventLogWord(abortReceipt, abi.topics.abortCrsgen, "AbortCrsgen") !== crsId) {
    throw new PreflightError(`kms-generation-abort: AbortCrsgen event does not carry crsId=${crsId}`);
  }
  console.log(`[kms-generation-abort] AbortCrsgen(${crsId}) emitted`);

  await assertRequestRetiredOnChain(target, "crsgen", crsId);
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"params type of the aborted CRS", "CrsAborted(uint256)", "getCrsParamsType(uint256)", crsId.toString());
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"double crsgen abort", "AbortCrsgenAlreadyDone(uint256)", "abortCrsgen(uint256)", crsId.toString());

  const parties = state.scenario.kms.parties;
  await checkConnectorsDbColumn(parties, "abort ingested and terminal", columnQuery("abort_crsgen_requests", "crs_id", "status", crsId), ["completed", "failed"]);
  await checkConnectorsDbColumn(parties, "crsgen request terminal", columnQuery("crsgen_requests", "crs_id", "status", crsId), ["completed", "aborted", "failed"]);

  const { crsId: activeCrsId } = await readActiveIds(target);
  assertActiveIdUnchanged("CRS", baselineCrsId, activeCrsId, crsId);
  console.log("[kms-generation-abort] crsgen abort verified across contract, connector, and active CRS");
};

/** Polls one on-chain active id until it equals `want`, or throws with the last-seen value. */
const waitForActivation = async (target: Target, label: string, getter: string, want: bigint) => {
  const deadline = Date.now() + RECOVERY_TIMEOUT_MS;
  let current = parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, getter));
  while (current !== want) {
    if (Date.now() >= deadline) {
      throw new PreflightError(
        `kms-generation-abort: ${label} did not activate within ${RECOVERY_TIMEOUT_MS / 1000}s (wanted ${want}, last on-chain value ${current}) — the abort did not unblock the pipeline, or the ceremony stalled`,
      );
    }
    await Bun.sleep(RECOVERY_POLL_MS);
    current = parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, getter));
  }
};

/**
 * Recovery: a fresh keygen and crsgen after the aborts must run to full consensus, publish
 * materials, and — being completed — refuse any late abort.
 */
const recoverAfterAborts = async (state: State, target: Target, owner: Owner, abi: AbiHashes, paramsType: string) => {
  const minioBase = `${state.discovery!.endpoints.minioExternal}/kms-public/${state.discovery!.minioKeyPrefix ?? "PUB"}`;

  console.log("[kms-generation-abort] recovery: triggering a fresh keygen (must not revert KeygenOngoing)…");
  const keygenTrigger = await castSend(target.rpcUrl, target.kmsGenerationAddress, owner,"keygen(uint8)", paramsType);
  const prepKeygenId = eventLogWord(keygenTrigger, abi.topics.prepKeygenRequest, "PrepKeygenRequest");
  const keyId = parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, "getKeyCounter()(uint256)"));
  await waitForActivation(target, "recovery keygen", "getActiveKeyId()(uint256)", keyId);
  await ensureMaterial(`${minioBase}/PublicKey/${uint256ToId(keyId)}`);
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"abort of a completed keygen", "AbortKeygenAlreadyDone(uint256)", "abortKeygen(uint256)", prepKeygenId.toString());
  console.log(`[kms-generation-abort] recovery keygen activated: keyId=${keyId}, materials published`);

  console.log("[kms-generation-abort] recovery: triggering a fresh crsgen (must not revert CrsgenOngoing)…");
  const crsgenTrigger = await castSend(target.rpcUrl, target.kmsGenerationAddress, owner,"crsgenRequest(uint256,uint8)", "2048", paramsType);
  const crsId = eventLogWord(crsgenTrigger, abi.topics.crsgenRequest, "CrsgenRequest");
  await waitForActivation(target, "recovery crsgen", "getActiveCrsId()(uint256)", crsId);
  await ensureMaterial(`${minioBase}/CRS/${uint256ToId(crsId)}`);
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"abort of a completed crsgen", "AbortCrsgenAlreadyDone(uint256)", "abortCrsgen(uint256)", crsId.toString());
  console.log(`[kms-generation-abort] recovery crsgen activated: crsId=${crsId}, materials published`);

  return { keyId, crsId };
};

/** Runs the abort acceptance flow; see the module doc for the phases. */
export const runKmsGenerationAbortProfile = async (state: State) => {
  const target = resolveKmsGenerationTarget(state);
  const owner = await loadHostOwner();
  const abi = await loadAbiHashes();
  const paramsType = state.scenario.kms.fheParams === "Test" ? "1" : "0";

  // Direct reads, not probeBootstrap: this profile rotates the active ids itself, so persisted
  // discovery may legitimately lag (e.g. after an earlier interrupted run) — the end-of-run
  // re-sync below heals it. Bootstrap must still have finalized: the triggers rely on the
  // previous request being done.
  const baseline = await readActiveIds(target);
  if (baseline.keyId === 0n || baseline.crsId === 0n) {
    throw new PreflightError("kms-generation-abort: bootstrap keygen/crsgen has not finalized — nothing safe to abort against");
  }
  console.log(
    `[kms-generation-abort] baseline on ${target.where}: activeKeyId=${baseline.keyId} activeCrsId=${baseline.crsId} (owner ${owner.address})`,
  );

  // Revert checks first: ids that were never assigned to a request are rejected outright.
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"abortKeygen(0)", "AbortKeygenInvalidId(uint256)", "abortKeygen(uint256)", "0");
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"abortKeygen(unknown id)", "AbortKeygenInvalidId(uint256)", "abortKeygen(uint256)", (1n << 255n).toString());
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"abortCrsgen(0)", "AbortCrsgenInvalidId(uint256)", "abortCrsgen(uint256)", "0");
  await callContractAndExpectRevert(target.rpcUrl, target.kmsGenerationAddress, owner,"abortCrsgen(unknown id)", "AbortCrsgenInvalidId(uint256)", "abortCrsgen(uint256)", (1n << 255n).toString());
  console.log("[kms-generation-abort] invalid-id reverts OK");

  await abortKeygenMidFlight(state, target, owner, abi, paramsType, baseline.keyId);
  await abortCrsgenMidFlight(state, target, owner, abi, paramsType, baseline.crsId);
  const active = await recoverAfterAborts(state, target, owner, abi, paramsType);

  // The recovery rotated the active ids: re-sync persisted discovery so later bootstrap
  // probes (probeBootstrap drift check) stay coherent with the chain.
  const discovery = state.discovery!;
  discovery.fheKeyId = uint256ToId(active.keyId);
  discovery.actualFheKeyId = uint256ToId(active.keyId);
  discovery.crsKeyId = uint256ToId(active.crsId);
  discovery.actualCrsKeyId = uint256ToId(active.crsId);
  await saveState(state);
  console.log(
    `[kms-generation-abort] PASS — aborts retired both ceremonies across contract, connector, and KMS Core, and the pipeline recovered to a fresh key/CRS (discovery re-synced to keyId=${active.keyId}, crsId=${active.crsId}; running services keep their fetched keyset — re-up before key-sensitive profiles)`,
  );
};
