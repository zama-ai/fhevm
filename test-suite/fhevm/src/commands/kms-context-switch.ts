/**
 * The `kms-context-switch` acceptance profile for `fhevm-cli test`.
 *
 * Drives RFC-005's two ProtocolConfig lifecycle flows end to end on a threshold-mode cluster and
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
import type { State } from "../types";
import type { DecryptionRunner } from "./kms-generation";

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

  // 1) NewKmsContext: broadcast the (same-committee) context switch, then prove it activates.
  console.log("[kms-context-switch] broadcasting defineNewKmsContextAndEpoch (NewKmsContext)…");
  await stepComposeTask("host-sc", state, ["host-sc-context-switch"], { noDeps: true });
  await waitForContainer("host-sc-context-switch", "complete");
  const afterSwitch = await waitForActivation(
    rpcUrl,
    configAddress,
    "context switch (NewKmsContext)",
    (current) => current.contextId > baseline.contextId,
  );
  console.log(
    `[kms-context-switch] context switched: contextId ${baseline.contextId} -> ${afterSwitch.contextId} (epochId=${afterSwitch.epochId})`,
  );
  if (!(await runDecryption(`kms-context-switch: decrypt after context switch (contextId=${afterSwitch.contextId})`))) {
    throw new PreflightError(
      `kms-context-switch: user-decryption failed after the context switch to contextId=${afterSwitch.contextId}`,
    );
  }

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

  console.log(
    "[kms-context-switch] PASS — NewKmsContext and NewKmsEpoch both activated on chain and user-decryption works under each",
  );
};
