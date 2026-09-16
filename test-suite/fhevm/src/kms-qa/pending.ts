/**
 * Observing — and holding open — an in-flight KMS lifecycle operation.
 *
 * The `epoch-rotation-pending` and `context-switch-pending` cases need a transition to sit in Pending
 * for the ~3 minutes two container probes take. Two problems follow from that, and this module owns
 * both — plus a third that only context switches have, covered further down.
 *
 * ## 1. Pending is not readable
 *
 * `ProtocolConfig` stores `epochState[epochCounter]` in private storage and exposes **no** getter
 * for it (`host-contracts/contracts/interfaces/IProtocolConfig.sol` has no `getPending…` /
 * `…Status` view). The state is therefore established indirectly, from three angles that fail for
 * different reasons and so cannot all be wrong at once:
 *
 *   - `getCurrentKmsContextAndEpoch` has not moved — `assertPairUnchanged` in `protocol-config.ts`;
 *   - a second lifecycle operation reverts `KmsLifecycleOperationInFlight` — the gate at
 *     `ProtocolConfig.sol:1083` (`_checkNoKmsLifecycleOperationInFlight`) fires exactly when the
 *     latest context is Pending/Created or the latest epoch is Pending — {@link assertLifecycleOperationInFlight};
 *   - `isValidEpochForContext(C, E)` is still false for the new epoch — {@link assertEpochValidity}.
 *
 * The third is the only *positive* signal of the three: the first two prove "nothing advanced", it
 * proves "this specific epoch exists and is not yet serving".
 *
 * ## 2. Pending is too short to observe
 *
 * Left alone, a rotation activates in roughly a minute (65.6s over 14 polls on the run recorded in
 * `qa-kms-context-scenario-1-epoch.md`), while the user-decryption probe alone takes ~120s. The
 * window is therefore *held*: one committee member's tx-sender is stopped before the broadcast, so
 * its `confirmEpochActivation` can never land and the all-signers quorum is never reached. Its core
 * stays up and reshares normally, which is what makes the hold honest — the epoch is Pending for the
 * reason the scenario describes (awaiting activation), not because the KMS is broken.
 *
 * The stop/restore itself is `NodeSupervisor.withTxSendersStopped`; this module only decides *which*
 * party to stall and proves the choice is safe.
 *
 * A context switch differs on both counts — it has two Pending stages rather than one, and its
 * broadcast returns no receipt to read the new ids from. See {@link CONTEXT_SWITCH_STAGES}.
 */
import { PreflightError } from "../errors";
import { castBool, castCall } from "../flow/readiness";
import {
  callContractAndExpectRevert,
  keccakTopic,
  parseUintOutput,
  type Owner,
  type Receipt,
} from "../kms-onchain";
import { run } from "../utils/process";
import type { CaseEvidence } from "./evidence";
import {
  NEW_KMS_EPOCH_SIGNATURE,
  decodeNewKmsEpoch,
  formatKmsId,
  type NewKmsEpochEvent,
  type ProtocolConfigTarget,
} from "./protocol-config";

/** The revert the ProtocolConfig gate raises while a context or epoch is still settling. */
export const LIFECYCLE_IN_FLIGHT_ERROR = "KmsLifecycleOperationInFlight(uint256,uint256)";

/**
 * Chooses which committee member's tx-sender to stop in order to hold a rotation Pending.
 *
 * Any single member will do — activation needs *every* signer — so the choice is about blast
 * radius, not correctness. The last member is taken because the lowest party ids carry the
 * bootstrap roles in these scenarios (party 1 keeps the bare container names, per `kms-party.ts`),
 * and leaving them alone keeps a failed run easier to reason about.
 *
 * Takes the committee resolved from the chain, never `1..committeeSize`: on a node-swapped stack
 * the serving set is something like `{1,2,3,5}`, and stalling a party that is not in it would not
 * withhold anything — the rotation would activate mid-probe and the case would fail for the wrong
 * reason.
 *
 * Pure; exported for unit testing.
 *
 * @throws PreflightError when the committee is empty, which means the caller resolved nothing.
 */
export const pickStallParty = (committee: readonly number[]): number => {
  if (!committee.length) {
    throw new PreflightError(
      "kms-context-qa: cannot hold a rotation Pending — the live committee resolved to no parties at all. " +
        "Persisted discovery and the chain disagree about the cluster; re-up the stack.",
    );
  }
  return committee[committee.length - 1]!;
};

/**
 * Returns why the remaining committee could not serve a user decryption with `stalled` held back,
 * or undefined when it can. Pure; exported for unit testing.
 *
 * `threshold` is the number of responses `getUserDecryptionThresholdForContext` requires. Stopping
 * a tx-sender removes that party's ability to land its response on chain, so the case only works
 * when the rest of the committee still reaches it.
 */
export const quorumShortfallAfterStall = (
  committeeSize: number,
  threshold: bigint,
  stalled: number,
): string | undefined => {
  const remaining = BigInt(committeeSize - 1);
  if (remaining >= threshold) return undefined;
  return (
    `stalling party ${stalled} leaves ${remaining} of ${committeeSize} committee member(s) able to respond, but a ` +
    `user decryption needs ${threshold}. Holding the rotation Pending would also break the decryption the scenario ` +
    `requires to succeed, so the two halves cannot both hold on this topology — run this case on a committee with ` +
    `at least ${threshold + 1n} members.`
  );
};

/**
 * Fails fast when withholding `stalled` would also withhold the decryption quorum.
 *
 * Without this the run would stop one node, broadcast, and then spend the probe's full three-minute
 * timeout discovering that the decryption can never complete — reporting a failure that looks like
 * a product bug but is really an unsatisfiable topology.
 */
export const assertQuorumSurvivesStall = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  committee: readonly number[],
  stalled: number,
): Promise<void> => {
  const threshold = await evidence.step(
    "call",
    "read the user-decryption threshold for the active context",
    { contract: target.address, contextId: formatKmsId(contextId) },
    async () =>
      parseUintOutput(
        await castCall(
          target.rpcUrl,
          target.address,
          "getUserDecryptionThresholdForContext(uint256)(uint256)",
          contextId.toString(),
        ),
      ),
  );
  await evidence.step(
    "assert",
    "the decryption quorum survives the stalled party",
    {
      committee: committee.join(","),
      stalledParty: String(stalled),
      remaining: String(committee.length - 1),
      userDecryptionThreshold: threshold.toString(),
    },
    async () => {
      const shortfall = quorumShortfallAfterStall(committee.length, threshold, stalled);
      if (shortfall) throw new PreflightError(`kms-context-qa: ${shortfall}`);
    },
  );
};

/**
 * Proves a lifecycle operation is in flight, by requiring a second one to revert.
 *
 * The probe is an `eth_call`, not a transaction: it asks the contract what *would* happen without
 * changing anything, so it cannot itself disturb the Pending state it is measuring.
 */
export const assertLifecycleOperationInFlight = async (
  target: ProtocolConfigTarget,
  owner: Owner,
  evidence: CaseEvidence,
  why: string,
): Promise<void> => {
  await evidence.step(
    "assert",
    "a second lifecycle operation reverts KmsLifecycleOperationInFlight",
    { contract: target.address, error: LIFECYCLE_IN_FLIGHT_ERROR, why },
    () =>
      callContractAndExpectRevert(
        target.rpcUrl,
        target.address,
        owner,
        `kms-context-qa: ${why}`,
        LIFECYCLE_IN_FLIGHT_ERROR,
        "defineNewEpochForCurrentKmsContext()",
      ),
  );
};

/**
 * Asserts `isValidEpochForContext(contextId, epochId)` reports `expected`.
 *
 * Read twice by the pending case: false while the epoch is Pending, true once it activates. The
 * pair of readings is what turns "the pointer did not move" into "this specific epoch was created,
 * was not serving, and then was" — a claim the negative assertions alone cannot make.
 */
export const assertEpochValidity = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  epochId: bigint,
  expected: boolean,
  why: string,
): Promise<void> => {
  const actual = await evidence.step(
    "call",
    `isValidEpochForContext is ${expected} (${why})`,
    { contract: target.address, contextId: formatKmsId(contextId), epochId: formatKmsId(epochId) },
    () =>
      castBool(
        target.rpcUrl,
        target.address,
        "isValidEpochForContext(uint256,uint256)(bool)",
        contextId.toString(),
        epochId.toString(),
      ),
  );
  evidence.note("note", `isValidEpochForContext: result`, {
    contextId: formatKmsId(contextId),
    epochId: formatKmsId(epochId),
    valid: String(actual),
    expected: String(expected),
  });
  if (actual !== expected) {
    throw new PreflightError(
      `kms-context-qa: expected isValidEpochForContext(${contextId}, ${epochId}) to be ${expected} — ${why} — ` +
        `but the contract reports ${actual} on ${target.where}.`,
    );
  }
};

/* -------------------------------------------------------------------------------------------- *
 * Context switches: a second Pending stage, and no receipt to read it from
 * -------------------------------------------------------------------------------------------- */

/**
 * A context switch passes through **two** distinct Pending stages, and only the second one matches
 * the QA scenario's wording.
 *
 * `defineNewKmsContextAndEpoch` stores the new context as `Pending` and stops there. The epoch is
 * not allocated yet: `_createPendingEpoch` runs inside `confirmKmsContextCreation`, only once
 * `_hasContextCreationQuorum` holds — every node of the new context has confirmed
 * (`ProtocolConfig.sol:383-388`). So:
 *
 *   - **stage 1** — context `Pending`, no epoch exists at all;
 *   - **stage 2** — context `Created`, epoch `Pending`, `NewKmsEpoch` emitted, cores resharing.
 *
 * A scenario that names a pending `E2` can only mean stage 2. Reaching it means letting the
 * creation quorum complete and withholding the *activation* confirmation instead, which in turn
 * means knowing `E2` — and unlike an epoch rotation, a switch is broadcast through a compose task
 * that returns no receipt. {@link waitForNewKmsEpochEvent} recovers the event from the chain's logs
 * instead, which is also what makes the predicted context id verifiable while the switch is still
 * pending rather than only after it activates.
 */
export const CONTEXT_SWITCH_STAGES = 2;

/** Poll interval while waiting for the creation quorum to allocate the new epoch. */
export const EPOCH_EVENT_POLL_MS = 1_000;

/**
 * Bound for the creation quorum. Every node of the new context must land one confirmation, which is
 * a handful of transactions on an idle chain; three minutes is generous enough to absorb a busy
 * host while still failing a genuinely stuck cluster quickly.
 */
export const EPOCH_EVENT_TIMEOUT_MS = 180_000;

/** Reads the host chain's current block height. Used to bound a log query to what follows it. */
export const readBlockNumber = async (rpcUrl: string): Promise<bigint> => {
  const result = await run(["cast", "block-number", "--rpc-url", rpcUrl]);
  return BigInt(result.stdout.trim());
};

/**
 * Queries `topic0` logs from `fromBlock` onwards and returns them shaped as a {@link Receipt}, so
 * the existing {@link decodeNewKmsEpoch} can consume them unchanged.
 *
 * `cast logs --json` emits the same `{address, topics, data}` objects a receipt carries, so the
 * wrapper is structural rather than a conversion. The synthetic `status` is never read.
 */
export const castLogsAsReceipt = async (
  rpcUrl: string,
  address: string,
  topic0: string,
  fromBlock: bigint,
): Promise<Receipt> => {
  const result = await run([
    "cast",
    "logs",
    "--from-block",
    fromBlock.toString(),
    "--to-block",
    "latest",
    "--address",
    address,
    topic0,
    "--rpc-url",
    rpcUrl,
    "--json",
  ]);
  const raw = result.stdout.trim();
  const logs = raw ? (JSON.parse(raw) as Receipt["logs"]) : [];
  return { status: "0x1", logs } as Receipt;
};

/**
 * Picks the `NewKmsEpoch` log belonging to `contextId` out of a log page, or undefined when none is
 * there yet.
 *
 * Filtering on the context matters and is not belt-and-braces: on a stack that has rotated epochs
 * before, older `NewKmsEpoch` logs exist under previous contexts, and a block lower bound alone
 * would not exclude a concurrent rotation. The **last** match wins, so a re-org replay or a
 * duplicated page yields the most recent one.
 *
 * Pure; exported for unit testing.
 */
export const selectNewKmsEpochLog = (
  logs: Receipt["logs"],
  topic0: string,
  contextId: bigint,
): Receipt["logs"][number] | undefined => {
  const matches = logs.filter((log) => {
    if (log.topics[0]?.toLowerCase() !== topic0.toLowerCase()) return false;
    const indexedContext = log.topics[1];
    if (indexedContext === undefined) return false;
    try {
      return BigInt(indexedContext) === contextId;
    } catch {
      return false;
    }
  });
  return matches[matches.length - 1];
};

/**
 * Waits until the creation quorum allocates the new context's first epoch, and returns the decoded
 * `NewKmsEpoch`.
 *
 * This is the moment a switch crosses from stage 1 to stage 2. The caller withholds the activation
 * confirmation immediately afterwards; the margin is the cores' reshare, which is tens of seconds,
 * against a poll interval of one second and a container stop of a few hundred milliseconds.
 *
 * Filtering on `contextId` matters: on a stack that has rotated epochs before, older `NewKmsEpoch`
 * logs exist under the previous context, and `fromBlock` alone would not exclude a rotation racing
 * this switch.
 *
 * @throws PreflightError naming the likely cause when the quorum never forms — which is what a
 *         stopped tx-sender on a node of the new context looks like.
 */
export const waitForNewKmsEpochEvent = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  fromBlock: bigint,
): Promise<NewKmsEpochEvent> => {
  const topic0 = await keccakTopic(NEW_KMS_EPOCH_SIGNATURE);
  const deadline = Date.now() + EPOCH_EVENT_TIMEOUT_MS;

  const event = await evidence.step(
    "wait",
    "the creation quorum allocates the new context's first epoch (NewKmsEpoch)",
    {
      contract: target.address,
      contextId: formatKmsId(contextId),
      fromBlock: fromBlock.toString(),
      timeoutMs: String(EPOCH_EVENT_TIMEOUT_MS),
    },
    async () => {
      for (;;) {
        const receipt = await castLogsAsReceipt(target.rpcUrl, target.address, topic0, fromBlock);
        const mine = selectNewKmsEpochLog(receipt.logs, topic0, contextId);
        if (mine) {
          return decodeNewKmsEpoch({ status: "0x1", logs: [mine] } as Receipt, topic0);
        }
        if (Date.now() >= deadline) {
          throw new PreflightError(
            `kms-context-qa: no NewKmsEpoch for context ${contextId} appeared within ` +
              `${EPOCH_EVENT_TIMEOUT_MS / 1000}s on ${target.where}. The epoch is allocated only once every node of ` +
              `the new context has confirmed its creation (_hasContextCreationQuorum), so a node whose tx-sender is ` +
              `down holds the switch at stage 1 forever — check the kms-connector-*-tx-sender containers.`,
          );
        }
        await Bun.sleep(EPOCH_EVENT_POLL_MS);
      }
    },
  );

  evidence.note("event", "NewKmsEpoch (recovered from chain logs, not a receipt)", {
    topic0,
    contextId: formatKmsId(event.contextId),
    epochId: formatKmsId(event.epochId),
    previousContextId: formatKmsId(event.previousContextId),
    previousEpochId: formatKmsId(event.previousEpochId),
    materialBlockNumber: event.materialBlockNumber.toString(),
  });
  return event;
};

/**
 * Asserts `isValidKmsContext(contextId)` reports `expected`.
 *
 * The context-level counterpart of {@link assertEpochValidity}. A context in stage 1 or stage 2 is
 * stored and live but not yet valid; only activation makes it valid. Read once during the hold and
 * once after, so the case makes a claim about `C2` specifically rather than only about what did not
 * move.
 */
export const assertContextValidity = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  expected: boolean,
  why: string,
): Promise<void> => {
  const actual = await evidence.step(
    "call",
    `isValidKmsContext is ${expected} (${why})`,
    { contract: target.address, contextId: formatKmsId(contextId) },
    () => castBool(target.rpcUrl, target.address, "isValidKmsContext(uint256)(bool)", contextId.toString()),
  );
  evidence.note("note", "isValidKmsContext: result", {
    contextId: formatKmsId(contextId),
    valid: String(actual),
    expected: String(expected),
  });
  if (actual !== expected) {
    throw new PreflightError(
      `kms-context-qa: expected isValidKmsContext(${contextId}) to be ${expected} — ${why} — but the contract ` +
        `reports ${actual} on ${target.where}.`,
    );
  }
};

/* -------------------------------------------------------------------------------------------- *
 * Aborting: the inverse readings
 * -------------------------------------------------------------------------------------------- */

/**
 * Asserts the creation quorum has NOT allocated an epoch for `contextId`.
 *
 * This is what separates a switch held at **stage 1** — context Pending, no epoch — from one held at
 * stage 2. A one-shot query, not a wait: the claim is that nothing is there *now*, and the caller has
 * already established why (a node of the new context cannot confirm).
 *
 * Without it, "the confirmations required to create C2 have been withheld" would be an assumption
 * about container state rather than an observation about the chain.
 */
export const assertNoNewKmsEpochEvent = async (
  target: ProtocolConfigTarget,
  evidence: CaseEvidence,
  contextId: bigint,
  fromBlock: bigint,
  why: string,
): Promise<void> => {
  const topic0 = await keccakTopic(NEW_KMS_EPOCH_SIGNATURE);
  await evidence.step(
    "assert",
    "no epoch was allocated for the pending context (the creation quorum never formed)",
    { contract: target.address, contextId: formatKmsId(contextId), fromBlock: fromBlock.toString(), why },
    async () => {
      const receipt = await castLogsAsReceipt(target.rpcUrl, target.address, topic0, fromBlock);
      const found = selectNewKmsEpochLog(receipt.logs, topic0, contextId);
      if (found) {
        throw new PreflightError(
          `kms-context-qa: a NewKmsEpoch was emitted for context ${contextId}, so its creation quorum DID form and ` +
            `the switch is at stage 2, not stage 1. ${why} Either the withheld node confirmed anyway, or the new ` +
            `context does not include it — check which parties the contracts task's env file defines.`,
        );
      }
    },
  );
};

/**
 * Asserts a lifecycle operation is NO LONGER in flight — the inverse of
 * {@link assertLifecycleOperationInFlight}.
 *
 * This is the observable meaning of "the switch was aborted". `_checkNoKmsLifecycleOperationInFlight`
 * treats a destroyed entry as `None`, so destroying a Pending context reopens the gate
 * (`ProtocolConfig.sol:1083`, and the comment on the destroy paths). Probed with an `eth_call`, which
 * asks what *would* happen without opening an operation of its own.
 */
export const assertLifecycleGateOpen = async (
  target: ProtocolConfigTarget,
  owner: Owner,
  evidence: CaseEvidence,
  why: string,
): Promise<void> => {
  await evidence.step(
    "assert",
    "a new lifecycle operation is allowed again (the in-flight gate reopened)",
    { contract: target.address, why },
    async () => {
      const result = await run(
        [
          "cast",
          "call",
          target.address,
          "defineNewEpochForCurrentKmsContext()",
          "--from",
          owner.address,
          "--rpc-url",
          target.rpcUrl,
        ],
        { allowFailure: true },
      );
      if (result.code !== 0) {
        const output = `${result.stdout}\n${result.stderr}`.trim();
        throw new PreflightError(
          `kms-context-qa: a new lifecycle operation is still refused after the abort — ${why} The destroy should ` +
            `have cleared the in-flight state (destroyed entries count as None), so the switch was not actually ` +
            `aborted: ${output.slice(0, 300)}`,
        );
      }
    },
  );
};

/**
 * Asserts the GATEWAY's view of a context's validity.
 *
 * The Gateway keeps its own registry and its own `destroyKmsContext`, owner-gated and independent of
 * the host's (`gateway-contracts/contracts/GatewayConfig.sol:327`). Aborting a switch on the host
 * therefore does not clean up the Gateway, and this is the only place that observes it.
 *
 * @param rpcUrl Host-reachable Gateway RPC.
 * @param address GatewayConfig address.
 */
export const assertGatewayContextValidity = async (
  rpcUrl: string,
  address: string,
  evidence: CaseEvidence,
  contextId: bigint,
  expected: boolean,
  why: string,
): Promise<void> => {
  const actual = await evidence.step(
    "call",
    `gateway isValidKmsContext is ${expected} (${why})`,
    { contract: address, contextId: formatKmsId(contextId) },
    () => castBool(rpcUrl, address, "isValidKmsContext(uint256)(bool)", contextId.toString()),
  );
  evidence.note("note", "gateway isValidKmsContext: result", {
    contextId: formatKmsId(contextId),
    valid: String(actual),
    expected: String(expected),
  });
  if (actual !== expected) {
    throw new PreflightError(
      `kms-context-qa: expected the GATEWAY to report isValidKmsContext(${contextId}) as ${expected} — ${why} — ` +
        `but it reports ${actual}. The Gateway has its own destroyKmsContext, independent of the host's; if this ` +
        `changed, cross-chain cleanup was added and this case's divergence note needs updating.`,
    );
  }
};

/**
 * Asserts where each chain's "current KMS context" pointer sits.
 *
 * The two are independent. The host rolls its pointer back when a pending context is destroyed; the
 * Gateway's only moves forward, because `updateKmsContext` advances it unconditionally and nothing
 * ever lowers it (`GatewayConfig.sol:294-309` — the sole guards are non-zero and strictly
 * increasing). After an aborted switch they therefore disagree, and the Gateway points at a context
 * the host no longer has.
 *
 * That matters beyond bookkeeping: `Decryption._extractContextId` resolves an empty or `0x00`
 * extraData through the Gateway's pointer, so while the two disagree an unqualified request binds to
 * a context the host cannot serve. Recorded in `qa/scenario_tb_checked/ghost.md`.
 *
 * Asserting both in one step keeps the divergence legible as a single line of evidence rather than
 * two readings a reader has to correlate.
 */
export const assertContextPointers = async (
  target: ProtocolConfigTarget,
  gatewayRpcUrl: string,
  gatewayConfigAddress: string,
  evidence: CaseEvidence,
  expectedHost: bigint,
  expectedGateway: bigint,
  why: string,
): Promise<void> => {
  const [host, gateway] = await evidence.step(
    "call",
    "read the current-context pointer on both chains",
    { host: target.address, gateway: gatewayConfigAddress, why },
    async () =>
      Promise.all([
        castCall(target.rpcUrl, target.address, "getCurrentKmsContextId()(uint256)").then(parseUintOutput),
        castCall(gatewayRpcUrl, gatewayConfigAddress, "getCurrentKmsContextId()(uint256)").then(parseUintOutput),
      ]),
  );
  evidence.note("note", "current-context pointers", {
    host: formatKmsId(host!),
    gateway: formatKmsId(gateway!),
    diverged: String(host !== gateway),
    expectedHost: formatKmsId(expectedHost),
    expectedGateway: formatKmsId(expectedGateway),
  });
  if (host !== expectedHost || gateway !== expectedGateway) {
    throw new PreflightError(
      `kms-context-qa: expected the current-context pointers to be host=${expectedHost} gateway=${expectedGateway} ` +
        `— ${why} — but they are host=${host} gateway=${gateway}. The Gateway's pointer only ever advances; if this ` +
        `changed, cross-chain synchronisation was added and qa/scenario_tb_checked/ghost.md needs revisiting.`,
    );
  }
};
