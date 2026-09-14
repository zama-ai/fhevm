/**
 * ProtocolConfig I/O for the `kms-context-qa-tests` profile: views, lifecycle transactions, event
 * decoding, activation waits, and the invariants the QA cases assert.
 *
 * Why this exists next to `src/commands/kms-context-switch.ts`, which does similar work: that
 * profile's equivalents (`readContextAndEpoch`, `waitForActivation`, `assertCurrentUnchanged`) are
 * module-private and carry its own log prefixes and error text. It gates CI, so this workstream
 * reimplements rather than reopens it. The duplication is deliberate and reviewed.
 *
 * Every operation here takes a {@link CaseEvidence} so the QA profile's audit trail is produced as
 * a side effect of doing the work, not bolted on afterwards.
 *
 * All ids are the protocol's domain-tagged uint256 values (context ids are tagged `0x07`, epoch ids
 * `0x08`), so they are large numbers, not small counters — never assume they fit in a JS `number`.
 */
import { PreflightError } from "../errors";
import { castCall } from "../flow/readiness";
import { castSend, getEventTopic, keccakTopic, type Owner, type Receipt } from "../kms-onchain";
import { parseContextAndEpoch, type ContextAndEpoch } from "../commands/kms-context-switch";
import type { CaseEvidence } from "./evidence";
import { txEvidenceFields } from "./evidence";

/**
 * Generous bound for a threshold-KMS epoch activation: the cores must complete a full reshare and
 * every committee connector must land `confirmEpochActivation` on chain. Ten minutes comfortably
 * covers a 4-party Test-parameter reshare on an emulated host while still failing a genuinely stuck
 * cluster in bounded time.
 */
export const ACTIVATION_TIMEOUT_MS = 600_000;

/** Poll interval for {@link waitForActivation}; 5s keeps chatter low against a 10-minute bound. */
export const ACTIVATION_POLL_MS = 5_000;

/** The ProtocolConfig deployment a case operates against, plus a human name for error messages. */
export type ProtocolConfigTarget = {
  /** Host-reachable RPC URL, already rewritten by `resolveKmsGenerationTarget`. */
  readonly rpcUrl: string;
  /** Checksummed ProtocolConfig address. */
  readonly address: string;
  /** Where this contract lives, e.g. `host chain "default"`; used verbatim in errors. */
  readonly where: string;
};

/** The `NewKmsEpoch` event, fully decoded. */
export type NewKmsEpochEvent = {
  readonly contextId: bigint;
  readonly epochId: bigint;
  readonly previousContextId: bigint;
  readonly previousEpochId: bigint;
  /** Block the connectors must read previous key/CRS material from. */
  readonly materialBlockNumber: bigint;
};

/** Solidity signature of `NewKmsEpoch`, used to derive its topic0. */
export const NEW_KMS_EPOCH_SIGNATURE = "NewKmsEpoch(uint256,uint256,uint256,uint256,uint256)";

/**
 * Reads the 32-byte word at `index` from a log's non-indexed `data` payload.
 *
 * `firstDataWord` in `src/kms-onchain.ts` only reads word 0, but `NewKmsEpoch` carries three
 * non-indexed arguments (`previousContextId`, `previousEpochId`, `materialBlockNumber`), and this
 * profile needs the second one. Pure; exported for unit testing.
 *
 * @throws PreflightError when `data` is too short to contain the requested word.
 */
export const dataWordAt = (data: string, index: number): bigint => {
  if (!Number.isInteger(index) || index < 0) {
    throw new PreflightError(`data word index must be a non-negative integer, got ${index}`);
  }
  const hex = data.replace(/^0x/, "");
  const start = index * 64;
  if (hex.length < start + 64) {
    throw new PreflightError(
      `event data too short for word ${index}: need ${start + 64} hex chars, have ${hex.length} (${data.slice(0, 80)}…)`,
    );
  }
  return BigInt(`0x${hex.slice(start, start + 64)}`);
};

/**
 * Decodes a `NewKmsEpoch` log out of a transaction receipt.
 *
 * Layout: `kmsContextId` and `epochId` are indexed (topics 1 and 2); `previousContextId`,
 * `previousEpochId` and `materialBlockNumber` are non-indexed data words 0, 1 and 2.
 *
 * Pure apart from the memoised `keccakTopic` shell-out its caller supplies, so it takes the topic
 * as an argument and stays unit-testable. Exported for testing.
 *
 * @throws PreflightError when the receipt carries no matching log.
 */
export const decodeNewKmsEpoch = (receipt: Receipt, topic0: string): NewKmsEpochEvent => {
  const log = receipt.logs.find((entry) => entry.topics[0]?.toLowerCase() === topic0.toLowerCase());
  if (!log) {
    throw new PreflightError(
      `transaction receipt has no NewKmsEpoch event (topics seen: ${
        receipt.logs.map((entry) => entry.topics[0]).join(", ") || "none"
      })`,
    );
  }
  return {
    contextId: getEventTopic(receipt, topic0, 1),
    epochId: getEventTopic(receipt, topic0, 2),
    previousContextId: dataWordAt(log.data, 0),
    previousEpochId: dataWordAt(log.data, 1),
    materialBlockNumber: dataWordAt(log.data, 2),
  };
};

/**
 * Parses the address array `cast call` prints for an `address[]` return, e.g.
 * `[0xAbc…, 0xDef…]`. Pure; exported for unit testing.
 */
export const parseAddressList = (raw: string): string[] =>
  (raw.match(/0x[0-9a-fA-F]{40}/g) ?? []).map((address) => address.toLowerCase());

/**
 * Resolves which KMS parties form the live committee of `contextId`.
 *
 * The committee is NOT simply `1..committeeSize`. A node-swap context switch drops a party and
 * promotes a spare, so on any stack that has switched, the serving set is something like
 * `{1,2,3,5}`. Assuming the initial committee makes a per-party assertion fail against a dropped
 * node that is behaving perfectly correctly — it no longer holds the context's material, so its
 * core legitimately reports a failed reshare.
 *
 * `state.discovery.kmsSigners` is the signer address of each party, indexed by party number, so
 * intersecting it with `getKmsSignersForContext` yields the live party ids.
 *
 * @param kmsSigners Signer addresses indexed by party (element 0 is party 1).
 * @throws PreflightError when the contract reports a signer that no known party owns, which would
 *         mean discovery and the chain disagree about the cluster.
 */
export const readCommitteeParties = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  kmsSigners: readonly string[],
): Promise<number[]> => {
  const raw = await evidence.step(
    "call",
    "read live committee signers for the active context",
    { contract: target.address, contextId: contextId.toString() },
    () =>
      castCall(target.rpcUrl, target.address, "getKmsSignersForContext(uint256)(address[])", contextId.toString()),
  );
  const live = new Set(parseAddressList(raw));
  const byAddress = new Map(kmsSigners.map((address, index) => [address.toLowerCase(), index + 1]));

  const unknown = [...live].filter((address) => !byAddress.has(address));
  if (unknown.length) {
    throw new PreflightError(
      `kms-context-qa: the active context's committee includes signer(s) ${unknown.join(", ")} that no discovered ` +
        `party owns (known parties: ${kmsSigners.length}). Persisted discovery and the chain disagree — re-up the stack.`,
    );
  }

  const parties = [...live].map((address) => byAddress.get(address)!).sort((a, b) => a - b);
  const dropped = kmsSigners.map((_, index) => index + 1).filter((party) => !parties.includes(party));
  evidence.note("note", "live committee resolved from the chain", {
    contextId: contextId.toString(),
    committee: parties.join(","),
    notInCommittee: dropped.join(",") || "(none)",
  });
  return parties;
};

/** Formats a context/epoch pair for logs and error messages. */
export const formatPair = (pair: ContextAndEpoch): string =>
  `contextId=${pair.contextId} epochId=${pair.epochId}`;

/** Reads the currently active `(context, epoch)` pair. */
export const readCurrentPair = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  label = "read active context/epoch",
): Promise<ContextAndEpoch> => {
  const pair = await evidence.step("call", label, { contract: target.address }, async () =>
    parseContextAndEpoch(
      await castCall(target.rpcUrl, target.address, "getCurrentKmsContextAndEpoch()(uint256,uint256)"),
    ),
  );
  evidence.note("note", `${label}: result`, {
    contextId: pair.contextId.toString(),
    epochId: pair.epochId.toString(),
  });
  return pair;
};

/**
 * Broadcasts `defineNewEpochForCurrentKmsContext` and returns the receipt with the decoded
 * `NewKmsEpoch` event.
 *
 * Sent directly with `cast send` rather than through the `host-sc-epoch-rotation` compose task that
 * `kms-context-switch` uses, because the receipt is the only place the event — and therefore the
 * authoritative `previousEpochId` — is available without a historical log query.
 */
export const sendDefineNewEpoch = async (
  target: ProtocolConfigTarget,
  owner: Owner,
  evidence: CaseEvidence,
): Promise<{ receipt: Receipt; event: NewKmsEpochEvent }> => {
  const topic0 = await keccakTopic(NEW_KMS_EPOCH_SIGNATURE);
  const receipt = await evidence.step(
    "tx",
    "defineNewEpochForCurrentKmsContext",
    { contract: target.address, from: owner.address },
    () => castSend(target.rpcUrl, target.address, owner, "defineNewEpochForCurrentKmsContext()"),
  );
  evidence.note("tx", "defineNewEpochForCurrentKmsContext: receipt", txEvidenceFields(receipt));

  const event = decodeNewKmsEpoch(receipt, topic0);
  evidence.note("event", "NewKmsEpoch", {
    topic0,
    contextId: event.contextId.toString(),
    epochId: event.epochId.toString(),
    previousContextId: event.previousContextId.toString(),
    previousEpochId: event.previousEpochId.toString(),
    materialBlockNumber: event.materialBlockNumber.toString(),
  });
  return { receipt, event };
};

/**
 * Polls the active pair until `reached` holds, then returns the observed pair.
 *
 * Activation is never automatic: the KMS cores must reshare and each committee connector must
 * submit `confirmEpochActivation` before the contract advances. A timeout here is therefore a real
 * product signal — the cluster did not converge — not a flaky test, so the error reports the last
 * observed state and how to investigate.
 */
export const waitForActivation = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  label: string,
  reached: (current: ContextAndEpoch) => boolean,
): Promise<ContextAndEpoch> => {
  const deadline = Date.now() + ACTIVATION_TIMEOUT_MS;
  let polls = 0;

  return evidence.step(
    "wait",
    label,
    { timeoutMs: String(ACTIVATION_TIMEOUT_MS), pollMs: String(ACTIVATION_POLL_MS) },
    async () => {
      let current = parseContextAndEpoch(
        await castCall(target.rpcUrl, target.address, "getCurrentKmsContextAndEpoch()(uint256,uint256)"),
      );
      polls += 1;
      while (!reached(current)) {
        if (Date.now() >= deadline) {
          throw new PreflightError(
            `kms-context-qa: ${label} did not complete within ${ACTIVATION_TIMEOUT_MS / 1000}s on ${target.where} ` +
              `(last on-chain state: ${formatPair(current)}, after ${polls} poll(s)). The KMS cores must reshare and ` +
              `every committee connector must submit confirmEpochActivation before the id advances — check the ` +
              `kms-core and kms-connector-*-tx-sender logs, and run host-contracts \`task:kmsContextSwitchStatus\` ` +
              `to see which confirmations are outstanding.`,
          );
        }
        await Bun.sleep(ACTIVATION_POLL_MS);
        current = parseContextAndEpoch(
          await castCall(target.rpcUrl, target.address, "getCurrentKmsContextAndEpoch()(uint256,uint256)"),
        );
        polls += 1;
      }
      evidence.note("note", `${label}: activated`, {
        polls: String(polls),
        contextId: current.contextId.toString(),
        epochId: current.epochId.toString(),
      });
      return current;
    },
  );
};

/**
 * Asserts the rotation's own consistency before anything is built on top of it.
 *
 * Two independent sources describe the epoch that was superseded: the pair read before the
 * broadcast, and the `previousEpochId` the contract itself put in the event. They must agree. When
 * they do not, something else drove a lifecycle operation between the read and the broadcast, and
 * every downstream assertion would be made against an ambiguous baseline — so fail here, loudly,
 * rather than report a misleading result.
 *
 * Also checks the two invariants the contract guarantees for a same-context rotation: the context
 * does not change, and `_createPendingEpoch` allocates the next sequential id.
 *
 * Pure; exported for unit testing.
 *
 * @throws PreflightError naming the specific invariant that failed.
 */
export const assertRotationConsistency = (baseline: ContextAndEpoch, event: NewKmsEpochEvent): void => {
  if (event.previousEpochId !== baseline.epochId) {
    throw new PreflightError(
      `kms-context-qa: NewKmsEpoch reports previousEpochId=${event.previousEpochId} but the pair read before the ` +
        `broadcast was epochId=${baseline.epochId}. Another lifecycle operation ran concurrently, so the baseline ` +
        `is ambiguous — re-up the stack and rerun on a quiet cluster.`,
    );
  }
  if (event.previousContextId !== baseline.contextId) {
    throw new PreflightError(
      `kms-context-qa: NewKmsEpoch reports previousContextId=${event.previousContextId} but the active context was ` +
        `${baseline.contextId}. A same-context rotation must not change the context.`,
    );
  }
  if (event.contextId !== baseline.contextId) {
    throw new PreflightError(
      `kms-context-qa: NewKmsEpoch opened epoch ${event.epochId} under context ${event.contextId}, but the active ` +
        `context is ${baseline.contextId}. defineNewEpochForCurrentKmsContext must reuse the active context.`,
    );
  }
  if (event.epochId !== baseline.epochId + 1n) {
    throw new PreflightError(
      `kms-context-qa: expected the rotation to open epoch ${baseline.epochId + 1n} (ids are allocated by ` +
        `++epochCounter), got ${event.epochId}.`,
    );
  }
};

/**
 * Asserts the active pair still matches `expected`.
 *
 * Used to prove a transition did not move the pointer. `afterWhat` names the action just performed
 * and is quoted verbatim in the failure message.
 */
export const assertPairUnchanged = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  expected: ContextAndEpoch,
  afterWhat: string,
): Promise<void> => {
  const current = await readCurrentPair(target, evidence, `re-read active pair after ${afterWhat}`);
  if (current.contextId !== expected.contextId || current.epochId !== expected.epochId) {
    throw new PreflightError(
      `kms-context-qa: the active context/epoch moved after ${afterWhat} — expected ${formatPair(expected)}, ` +
        `got ${formatPair(current)}.`,
    );
  }
};
