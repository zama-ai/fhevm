/**
 * The `kms-context-switch` acceptance profile for `fhevm-cli test`.
 *
 * Drives the two ProtocolConfig KMS-context lifecycle flows end to end on a threshold-mode cluster and
 * proves the KMS reacts to the emitted events:
 *   1. NewKmsContext — broadcast `defineNewKmsContextAndEpoch` (same committee, so no new signing
 *      keys are needed), wait for the new context to become the on-chain active one, then decrypt.
 *   2. NewKmsEpoch — broadcast `defineNewEpochForCurrentKmsContext` (same context, new epoch), wait
 *      for the new epoch to activate, then decrypt again.
 *
 * Activation is not automatic: the KMS cores must reshare and the connectors must submit
 * `confirmKmsContextCreation` / `confirmEpochActivation` for `getCurrentKmsContextAndEpoch` to
 * advance. If that never happens the ids stay put and the profile fails with the last-read state —
 * so this doubles as the discovery test for whether the cluster reshares at all. Threshold-only:
 * the switch tasks act on the host ProtocolConfig.
 */
import { PreflightError } from "../errors";
import { castCall, resolveKmsGenerationTarget, waitForContainer } from "../flow/readiness";
import { stepComposeTask } from "../flow/runtime-compose";
import { reconstructionThreshold } from "../kms-party";
import type { State } from "../types";
import {
  type DecryptionRunner,
  partyContainers,
  setRunning,
  waitForPartiesRunning,
  waitForPartiesStopped,
} from "./kms-generation";

/** Generous bound: a 4-party reshare + per-party on-chain confirmations. */
const ACTIVATION_TIMEOUT_MS = 600_000;
const ACTIVATION_POLL_MS = 5_000;

export type ContextAndEpoch = { contextId: bigint; epochId: bigint };

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
 * drops a committee node and promotes the spare, keeping n = committeeSize so the threshold stays
 * valid. The dropped node reshares as Set 1 (outgoing), the spare as Set 2 (incoming), the rest as
 * both. Without a spare it is a same-committee reshare. Activation already gates on the spare:
 * it requires ALL new-committee signers to confirm, so the context id only advances once the
 * spare has reshared and submitted confirmKmsContextCreation.
 */
const switchKmsContext = async (
  state: State,
  runDecryption: DecryptionRunner,
  target: SwitchTarget,
  baseline: ContextAndEpoch,
): Promise<ContextAndEpoch> => {
  const { isSwap, continuing, added, dropped } = committeeSwapPlan(state.scenario.kms);
  // Same services as a same-set switch; a swap just points them at the swap-committee env files.
  const hostEnv: Record<string, string> = isSwap ? { HOST_SC_CONTEXT_ENV: "host-sc-swap.env" } : {};
  const gatewayEnv: Record<string, string> = isSwap ? { GATEWAY_SC_CONTEXT_ENV: "gateway-sc-swap.env" } : {};

  console.log(
    isSwap
      ? `[kms-context-switch] broadcasting defineNewKmsContextAndEpoch — node swap (drop ${dropped.join(",")}, add ${added.join(",")}, keep ${continuing.join(",")})…`
      : "[kms-context-switch] broadcasting defineNewKmsContextAndEpoch (NewKmsContext, same committee)…",
  );
  await stepComposeTask("host-sc", state, ["host-sc-context-switch"], { noDeps: true, env: hostEnv });
  await waitForContainer("host-sc-context-switch", "complete");
  const afterSwitch = await waitForActivation(
    target.rpcUrl,
    target.configAddress,
    isSwap ? "node-swap context" : "context switch (NewKmsContext)",
    (current) => current.contextId > baseline.contextId,
  );
  console.log(
    isSwap
      ? `[kms-context-switch] context switched: contextId ${baseline.contextId} -> ${afterSwitch.contextId} — the spare reshared and confirmed`
      : `[kms-context-switch] context switched: contextId ${baseline.contextId} -> ${afterSwitch.contextId} (epochId=${afterSwitch.epochId})`,
  );

  // The gateway keeps its own KMS-context registry; register the new committee on GatewayConfig so its
  // Decryption contract accepts decryptions tagged with it (and, for a swap, the spare's signatures
  // while rejecting the dropped node's) — else it reverts InvalidKmsContext.
  console.log(`[kms-context-switch] registering context ${afterSwitch.contextId} on the gateway (updateKmsContext)…`);
  await stepComposeTask("gateway-sc", state, ["gateway-sc-context-switch"], {
    noDeps: true,
    env: { KMS_CONTEXT_ID: afterSwitch.contextId.toString(), ...gatewayEnv },
  });
  await waitForContainer("gateway-sc-context-switch", "complete");

  if (!(await runDecryption(`kms-context-switch: decrypt after context switch (contextId=${afterSwitch.contextId})`))) {
    throw new PreflightError(
      `kms-context-switch: user-decryption failed after the context switch to contextId=${afterSwitch.contextId}`,
    );
  }
  return afterSwitch;
};

/**
 * Conclusive proof a swapped-in spare holds a working reshared key: drop the top `t` continuing
 * members so the live committee is exactly the 2t+1 quorum INCLUDING the spare, then decrypt — it
 * must use the spare. Restores the stopped members afterwards. Runs last, on an otherwise healthy
 * cluster.
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

export const runKmsContextSwitchProfile = async (state: State, runDecryption: DecryptionRunner) => {
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

  // 1) NewKmsContext: a node swap when the cluster has a spare core, a same-committee reshare otherwise.
  const afterSwitch = await switchKmsContext(state, runDecryption, { rpcUrl, configAddress, where }, baseline);

  // 2) NewKmsEpoch: same-set epoch rotation under the (now active) context, then prove it activates.
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

  // 3) Node swap only: prove the promoted spare actually holds a working reshared key (runs last so
  //    the earlier steps see a healthy cluster and the stopped member is restored at the end).
  if (committeeSwapPlan(state.scenario.kms).isSwap) {
    await proveSpareInQuorum(state, runDecryption);
  }

  console.log(
    "[kms-context-switch] PASS — NewKmsContext and NewKmsEpoch both activated on chain and user-decryption works under each",
  );
};
