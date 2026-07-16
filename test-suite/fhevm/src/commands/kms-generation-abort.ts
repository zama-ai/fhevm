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
 *   - Abort immediately after the trigger — the ceremony is the slow path, so the abort always
 *     lands on chain first; waiting for connector ingestion can lose the race to a fast
 *     ceremony.
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
import { kmsConnectorDbName, kmsPartyIds } from "../kms-party";
import { COPROCESSOR_DB_CONTAINER, DEFAULT_POSTGRES_PASSWORD, DEFAULT_POSTGRES_USER, envPath } from "../layout";
import { saveState } from "../state/state";
import type { State } from "../types";
import { readEnvFile, uint256ToId, withHexPrefix } from "../utils/fs";
import { run } from "../utils/process";

/** How long the connector gets to ingest the abort, forward it to KMS Core, and settle. */
const CONNECTOR_SETTLE_TIMEOUT_MS = 240_000;
const CONNECTOR_POLL_MS = 2_000;
/** One listener batch cycle: a KeygenRequest emitted just before the abort may still be ingesting. */
const LISTENER_BATCH_GRACE_MS = 35_000;
/** Bound for a full post-abort keygen/crsgen cycle (trigger -> consensus -> activation). */
const RECOVERY_TIMEOUT_MS = Number(process.env.KMS_ABORT_RECOVERY_TIMEOUT_SECONDS ?? "900") * 1_000;
const RECOVERY_POLL_MS = 5_000;

type Receipt = { status: string; logs: { address: string; topics: string[]; data: string }[] };

/** The little-endian byte order the connector stores U256 ids in (alloy `as_le_slice`). */
export const uint256LeHex = (id: bigint) => {
  const be = uint256ToId(id);
  let le = "";
  for (let i = be.length - 2; i >= 0; i -= 2) {
    le += be.slice(i, i + 2);
  }
  return le;
};

/** Parses the first integer token of cast's `<decimal> [<sci-notation>]` output. */
export const parseUintOutput = (raw: string): bigint => {
  const token = raw
    .replace(/\[[^\]]*\]/g, " ")
    .split(/\s+/)
    .find((candidate) => /^(0x[0-9a-fA-F]+|\d+)$/.test(candidate));
  if (token === undefined) {
    throw new PreflightError(`kms-generation-abort: could not parse a uint from cast output: ${JSON.stringify(raw)}`);
  }
  return BigInt(token);
};

/** First 32-byte word of a log's `data` field (all KMSGeneration events lead with the request id). */
export const firstDataWord = (data: string): bigint => {
  const hex = data.replace(/^0x/, "");
  if (hex.length < 64) {
    throw new PreflightError(`kms-generation-abort: event data too short for a uint256 word: ${data}`);
  }
  return BigInt(`0x${hex.slice(0, 64)}`);
};

/** Extracts the leading uint256 of the receipt event matching `topic0`, or throws. */
export const eventLogWord = (receipt: Receipt, topic0: string, eventName: string): bigint => {
  const log = receipt.logs.find((entry) => entry.topics[0]?.toLowerCase() === topic0.toLowerCase());
  if (!log) {
    throw new PreflightError(
      `kms-generation-abort: transaction receipt has no ${eventName} event (topics seen: ${receipt.logs.map((entry) => entry.topics[0]).join(", ") || "none"})`,
    );
  }
  return firstDataWord(log.data);
};

const keccakCache = new Map<string, string>();

/** `cast keccak`, memoized — the same event/error signatures are asserted several times per run. */
const keccakTopic = async (signature: string) => {
  let hash = keccakCache.get(signature);
  if (hash === undefined) {
    hash = (await run(["cast", "keccak", signature])).stdout.trim();
    keccakCache.set(signature, hash);
  }
  return hash;
};

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
type Owner = { key: string; address: string };
type AbiHashes = Awaited<ReturnType<typeof loadAbiHashes>>;

/** Loads the host-chain owner (the deployer) from the generated host-sc env. */
const loadHostOwner = async (): Promise<Owner> => {
  const env = await readEnvFile(envPath("host-sc"));
  const rawKey = env.DEPLOYER_PRIVATE_KEY;
  if (!rawKey) {
    throw new PreflightError(`kms-generation-abort: no DEPLOYER_PRIVATE_KEY in ${envPath("host-sc")} — cannot act as the KMSGeneration owner`);
  }
  const key = withHexPrefix(rawKey);
  // allowFailure everywhere the key is on the command line: CommandError echoes the full argv.
  const result = await run(["cast", "wallet", "address", "--private-key", key], { allowFailure: true });
  if (result.code !== 0) {
    throw new PreflightError(`kms-generation-abort: could not derive the owner address from DEPLOYER_PRIVATE_KEY: ${(result.stderr || result.stdout).trim()}`);
  }
  return { key, address: result.stdout.trim() };
};

/** Sends an owner transaction to KMSGeneration and returns the parsed receipt. Throws a
 * PreflightError without echoing the command line (it carries the private key). */
const castSend = async (target: Target, owner: Owner, signature: string, ...args: string[]): Promise<Receipt> => {
  const result = await run(
    ["cast", "send", target.kmsGenerationAddress, signature, ...args, "--rpc-url", target.rpcUrl, "--private-key", owner.key, "--json"],
    { allowFailure: true },
  );
  if (result.code !== 0) {
    throw new PreflightError(
      `kms-generation-abort: cast send ${signature} [${args.join(", ")}] failed: ${(result.stderr || result.stdout).trim().slice(0, 400)}`,
    );
  }
  try {
    return JSON.parse(result.stdout) as Receipt;
  } catch {
    throw new PreflightError(`kms-generation-abort: cast send ${signature} returned a non-JSON receipt: ${result.stdout.trim().slice(0, 200)}`);
  }
};

/** Asserts an eth_call from the owner reverts with the given custom error. */
const expectRevert = async (target: Target, owner: Owner, label: string, errorSignature: string, callSignature: string, ...args: string[]) => {
  const result = await run(
    ["cast", "call", target.kmsGenerationAddress, callSignature, ...args, "--from", owner.address, "--rpc-url", target.rpcUrl],
    { allowFailure: true },
  );
  if (result.code === 0) {
    throw new PreflightError(`kms-generation-abort: ${label}: expected ${errorSignature} revert, but the call succeeded`);
  }
  // Match the 4-byte selector in the revert data; also accept cast decoding the error by name.
  const selector = (await keccakTopic(errorSignature)).slice(2, 10);
  const errorName = errorSignature.split("(")[0];
  const output = `${result.stdout}\n${result.stderr}`;
  if (!output.includes(selector) && !output.includes(errorName)) {
    throw new PreflightError(
      `kms-generation-abort: ${label}: reverted, but not with ${errorSignature} (selector ${selector}): ${output.trim().slice(0, 300)}`,
    );
  }
};

const connectorDbRuntime = () => ({
  container: process.env.POSTGRES_CONTAINER ?? COPROCESSOR_DB_CONTAINER,
  user: process.env.POSTGRES_USER ?? DEFAULT_POSTGRES_USER,
  password: process.env.POSTGRES_PASSWORD ?? DEFAULT_POSTGRES_PASSWORD,
});

/** Runs a scalar query against one party's kms-connector database. */
const connectorQuery = async (dbName: string, sql: string) => {
  const db = connectorDbRuntime();
  const result = await run([
    "docker", "exec", "-e", `PGPASSWORD=${db.password}`, db.container,
    "psql", "-U", db.user, "-d", dbName, "-t", "-A", "-c", sql,
  ]);
  return result.stdout.trim();
};

const byteaLiteral = (id: bigint) => `decode('${uint256LeHex(id)}','hex')`;

/** A row's status as text, or 'missing' when the row does not exist. */
const statusQuery = (table: string, idColumn: string, id: bigint) =>
  `SELECT COALESCE((SELECT status::text FROM ${table} WHERE ${idColumn} = ${byteaLiteral(id)}), 'missing')`;

/** Polls one query in every party's connector DB until it returns one of `want` everywhere. */
const pollConnectors = async (parties: number, label: string, sql: string, want: string[]) => {
  const finals: string[] = [];
  for (const party of kmsPartyIds(parties)) {
    const dbName = kmsConnectorDbName(party);
    const deadline = Date.now() + CONNECTOR_SETTLE_TIMEOUT_MS;
    let last = await connectorQuery(dbName, sql);
    while (!want.includes(last)) {
      if (Date.now() >= deadline) {
        throw new PreflightError(
          `kms-generation-abort: ${label}: db "${dbName}" returned ${JSON.stringify(last)} (wanted one of ${want.join("/")}) after ${CONNECTOR_SETTLE_TIMEOUT_MS / 1000}s — query: ${sql}`,
        );
      }
      await Bun.sleep(CONNECTOR_POLL_MS);
      last = await connectorQuery(dbName, sql);
    }
    finals.push(last);
  }
  console.log(`[kms-generation-abort] connector check OK on ${parties} db(s): ${label} -> ${finals.join(", ")}`);
};

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
  const trigger = await castSend(target, owner, "keygen(uint8)", paramsType);
  // The receipt's PrepKeygenRequest event is the in-flight proof.
  const prepKeygenId = eventLogWord(trigger, abi.topics.prepKeygenRequest, "PrepKeygenRequest");
  const keyId = parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, "getKeyCounter()(uint256)"));
  console.log(`[kms-generation-abort] keygen in flight: prepKeygenId=${prepKeygenId} keyId=${keyId}`);

  // The pipeline is exclusive while the request is in flight.
  await expectRevert(target, owner, "keygen while one is in flight", "KeygenOngoing(uint256)", "keygen(uint8)", paramsType);

  let abortReceipt: Receipt;
  try {
    abortReceipt = await castSend(target, owner, "abortKeygen(uint256)", prepKeygenId.toString());
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
  await expectRevert(target, owner, "params type of the aborted key", "KeyAborted(uint256)", "getKeyParamsType(uint256)", keyId.toString());
  await expectRevert(target, owner, "double keygen abort", "AbortKeygenAlreadyDone(uint256)", "abortKeygen(uint256)", prepKeygenId.toString());

  // Connector settlement (see the module doc for why the statuses are phase-dependent).
  const parties = state.scenario.kms.parties;
  await pollConnectors(parties, "abort ingested and terminal", statusQuery("abort_keygen_requests", "prep_keygen_id", prepKeygenId), ["completed", "failed"]);
  await pollConnectors(parties, "prep-keygen request terminal", statusQuery("prep_keygen_requests", "prep_keygen_id", prepKeygenId), ["completed", "aborted", "failed"]);
  // A KeygenRequest emitted in the trigger-to-abort window may still be one listener batch away.
  await Bun.sleep(LISTENER_BATCH_GRACE_MS);
  await pollConnectors(parties, "no keygen work left in flight", statusQuery("keygen_requests", "key_id", keyId), ["missing", "completed", "aborted", "failed"]);

  const { keyId: activeKeyId } = await readActiveIds(target);
  assertActiveIdUnchanged("key", baselineKeyId, activeKeyId, keyId);
  console.log("[kms-generation-abort] keygen abort verified across contract, connector, and active key");
};

/** The crsgen mirror of `abortKeygenMidFlight` (single-phase, so no second-event grace). */
const abortCrsgenMidFlight = async (state: State, target: Target, owner: Owner, abi: AbiHashes, paramsType: string, baselineCrsId: bigint) => {
  console.log("[kms-generation-abort] triggering crsgen to abort it mid-flight…");
  const trigger = await castSend(target, owner, "crsgenRequest(uint256,uint8)", "2048", paramsType);
  const crsId = eventLogWord(trigger, abi.topics.crsgenRequest, "CrsgenRequest");
  console.log(`[kms-generation-abort] crsgen in flight: crsId=${crsId}`);

  await expectRevert(target, owner, "crsgen while one is in flight", "CrsgenOngoing(uint256)", "crsgenRequest(uint256,uint8)", "2048", paramsType);

  let abortReceipt: Receipt;
  try {
    abortReceipt = await castSend(target, owner, "abortCrsgen(uint256)", crsId.toString());
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
  await expectRevert(target, owner, "params type of the aborted CRS", "CrsAborted(uint256)", "getCrsParamsType(uint256)", crsId.toString());
  await expectRevert(target, owner, "double crsgen abort", "AbortCrsgenAlreadyDone(uint256)", "abortCrsgen(uint256)", crsId.toString());

  const parties = state.scenario.kms.parties;
  await pollConnectors(parties, "abort ingested and terminal", statusQuery("abort_crsgen_requests", "crs_id", crsId), ["completed", "failed"]);
  await pollConnectors(parties, "crsgen request terminal", statusQuery("crsgen_requests", "crs_id", crsId), ["completed", "aborted", "failed"]);

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
  const keygenTrigger = await castSend(target, owner, "keygen(uint8)", paramsType);
  const prepKeygenId = eventLogWord(keygenTrigger, abi.topics.prepKeygenRequest, "PrepKeygenRequest");
  const keyId = parseUintOutput(await castCall(target.rpcUrl, target.kmsGenerationAddress, "getKeyCounter()(uint256)"));
  await waitForActivation(target, "recovery keygen", "getActiveKeyId()(uint256)", keyId);
  await ensureMaterial(`${minioBase}/PublicKey/${uint256ToId(keyId)}`);
  await expectRevert(target, owner, "abort of a completed keygen", "AbortKeygenAlreadyDone(uint256)", "abortKeygen(uint256)", prepKeygenId.toString());
  console.log(`[kms-generation-abort] recovery keygen activated: keyId=${keyId}, materials published`);

  console.log("[kms-generation-abort] recovery: triggering a fresh crsgen (must not revert CrsgenOngoing)…");
  const crsgenTrigger = await castSend(target, owner, "crsgenRequest(uint256,uint8)", "2048", paramsType);
  const crsId = eventLogWord(crsgenTrigger, abi.topics.crsgenRequest, "CrsgenRequest");
  await waitForActivation(target, "recovery crsgen", "getActiveCrsId()(uint256)", crsId);
  await ensureMaterial(`${minioBase}/CRS/${uint256ToId(crsId)}`);
  await expectRevert(target, owner, "abort of a completed crsgen", "AbortCrsgenAlreadyDone(uint256)", "abortCrsgen(uint256)", crsId.toString());
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
  await expectRevert(target, owner, "abortKeygen(0)", "AbortKeygenInvalidId(uint256)", "abortKeygen(uint256)", "0");
  await expectRevert(target, owner, "abortKeygen(unknown id)", "AbortKeygenInvalidId(uint256)", "abortKeygen(uint256)", (1n << 255n).toString());
  await expectRevert(target, owner, "abortCrsgen(0)", "AbortCrsgenInvalidId(uint256)", "abortCrsgen(uint256)", "0");
  await expectRevert(target, owner, "abortCrsgen(unknown id)", "AbortCrsgenInvalidId(uint256)", "abortCrsgen(uint256)", (1n << 255n).toString());
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
