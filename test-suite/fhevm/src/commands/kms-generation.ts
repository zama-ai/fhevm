/**
 * The `kms-generation` acceptance profile for `fhevm-cli test`.
 *
 * It turns the implicit "the boot didn't fail" signal into named, on-chain assertions about the
 * FHE key and CRS generation flows (implemented by the KMSGeneration contract), then proves the
 * 2t+1 quorum property is real by varying the live-party count around it. (KMS-context
 * guarantees — epoch activation, cross-context replay, resharing — are deferred to RFC-005 and
 * are not modeled here.)
 */
import { PreflightError } from "../errors";
import { castCall, dockerInspect, probeBootstrap, resolveKmsGenerationTarget, waitForContainer } from "../flow/readiness";
import { kmsConnectorPrefix, kmsCoreName, kmsTxSenderName, reconstructionThreshold } from "../kms-party";
import type { State } from "../types";
import { withHexPrefix } from "../utils/fs";
import { run } from "../utils/process";

/** Runs a user-decryption probe; resolves true when it decrypts, false when it does not. */
export type DecryptionRunner = (label: string, opts?: { expectFailure?: boolean }) => Promise<boolean>;

/** Every container that belongs to one KMS party (its core + its connector tier). */
export const partyContainers = (party: number) => {
  const connector = kmsConnectorPrefix(party);
  return [kmsCoreName(party), `${connector}-gw-listener`, `${connector}-kms-worker`, kmsTxSenderName(party)];
};

/**
 * How many parties to stop to probe the 2t+1 quorum, given an N-party / threshold-t cluster.
 * `stopForTolerance` leaves exactly `reconstruct` parties live (must still decrypt);
 * `stopForFloor` leaves `reconstruct - 1` live (must NOT decrypt).
 */
export const quorumPlan = (parties: number, threshold: number) => {
  const reconstruct = reconstructionThreshold(threshold); // 2t+1
  return {
    reconstruct,
    stopForTolerance: parties - reconstruct,
    stopForFloor: parties - (reconstruct - 1),
  };
};

/** Stops or starts each container, tolerating already-stopped / missing ones. */
export const setRunning = async (containers: string[], action: "start" | "stop") => {
  for (const container of containers) {
    await run(["docker", action, container], { allowFailure: true });
  }
};

/** Polls one container until docker reports it is not running (docker stop exits 137/143,
 * so waitForContainer — which treats any non-zero exit as a crash — cannot be used here). */
const waitForContainerStopped = async (container: string) => {
  for (let attempt = 0; attempt <= 30; attempt += 1) {
    const [inspect] = await dockerInspect(container);
    if (!inspect || inspect.State.Status !== "running") {
      return;
    }
    await Bun.sleep(1_000);
  }
  throw new PreflightError(
    `kms-generation: ${container} is still running after docker stop — the quorum probe would count a live party as stopped`,
  );
};

/** Confirms every listed container is genuinely down before a verdict is read. `setRunning`
 * tolerates stop failures for idempotency, so without this check a silently no-op'd stop would
 * probe with too many live containers and misdiagnose the scenario. */
export const waitForContainersStopped = async (containers: string[]) => {
  for (const container of containers) {
    await waitForContainerStopped(container);
  }
};

/** Confirms every stopped party is genuinely down before a quorum verdict is read. */
export const waitForPartiesStopped = async (parties: number[]) => {
  for (const party of parties) {
    await waitForContainersStopped(partyContainers(party));
  }
};

/** Asserts the on-chain key/CRS generation state (KMSGeneration contract) and returns the
 * verified topology numbers. */
export const auditKmsGeneration = async (state: State) => {
  // kms-generation audits the on-chain COMMITTEE (the serving MPC group), which is committeeSize —
  // not the cluster size (`parties`), which may include idle spares.
  const { threshold, fheParams, committeeSize } = state.scenario.kms;
  if (committeeSize !== 3 * threshold + 1) {
    throw new PreflightError(
      `kms-generation: committee ${committeeSize} must equal 3*threshold+1 (threshold=${threshold})`,
    );
  }
  // Re-reads activeKeyId/activeCrsId on chain, confirms they are set, match discovery, and that
  // the key + CRS materials are published. Returns null until keygen/crsgen has finalized.
  const probe = await probeBootstrap(state);
  if (!probe) {
    throw new PreflightError(
      "kms-generation: activeKeyId/activeCrsId are not set on chain — secure threshold-mode keygen/crsgen did not finalize",
    );
  }

  const { rpcUrl, kmsGenerationAddress, configAddress, where } = resolveKmsGenerationTarget(state);
  if (!configAddress) {
    throw new PreflightError(
      `kms-generation: no ProtocolConfig/GatewayConfig address on ${where} — cannot read the registered KMS context`,
    );
  }

  // KMSGeneration must have run with the params type the scenario generated (Test=1, Default=0):
  // proves the on-chain keygen/crsgen used the intended FHE parameters, not just that ids exist.
  const expectedParams = fheParams === "Test" ? "1" : "0";
  const keyParams = await castCall(rpcUrl, kmsGenerationAddress, "getKeyParamsType(uint256)(uint8)", withHexPrefix(probe.actualFheKeyId));
  const crsParams = await castCall(rpcUrl, kmsGenerationAddress, "getCrsParamsType(uint256)(uint8)", withHexPrefix(probe.actualCrsKeyId));
  if (keyParams !== expectedParams || crsParams !== expectedParams) {
    throw new PreflightError(
      `kms-generation: on-chain params type mismatch — expected ${fheParams}(${expectedParams}), got key=${keyParams} crs=${crsParams}`,
    );
  }

  // ProtocolConfig must register exactly one distinct signer per party (the verifying addresses
  // derived from the threshold DKG). Reading getKmsSigners() on chain is the authoritative
  // registration check — unlike scraping KMS-core logs, it confirms the deploy wired N signers.
  const onChainSigners = (await castCall(rpcUrl, configAddress, "getKmsSigners()(address[])")).match(/0x[0-9a-fA-F]{40}/g) ?? [];
  const distinct = new Set(onChainSigners.map((address) => address.toLowerCase()));
  if (distinct.size !== committeeSize) {
    throw new PreflightError(
      `kms-generation: ProtocolConfig registered ${distinct.size} distinct KMS signer(s), expected ${committeeSize} (${[...distinct].join(", ")})`,
    );
  }

  const reconstruct = reconstructionThreshold(threshold);
  console.log(
    `[kms-generation] on-chain audit OK: activeKeyId=0x${probe.actualFheKeyId} activeCrsId=0x${probe.actualCrsKeyId} paramsType=${fheParams}(${expectedParams})`,
  );
  console.log(
    `[kms-generation] ${committeeSize}-node committee, t=${threshold}, reconstruction=${reconstruct} (2t+1), ${distinct.size} signers registered in ProtocolConfig on ${where}`,
  );
  // Return committeeSize as the serving-party count so the quorum probe stops committee nodes only.
  return { parties: committeeSize, threshold, reconstruct };
};

/** Waits for restarted party containers to be running again before probing recovery. */
export const waitForPartiesRunning = async (parties: number[]) => {
  for (const party of parties) {
    for (const container of partyContainers(party)) {
      await waitForContainer(container, "running");
    }
  }
  // Give the cores a short grace window to re-establish their MPC peer sessions.
  await Bun.sleep(15_000);
};

/**
 * `kms-generation`: audit the on-chain key/CRS generation state of a threshold-mode cluster,
 * then prove the decryption quorum is genuinely 2t+1 by decrypting with a tolerated party loss,
 * confirming it cannot decrypt below quorum, and confirming it recovers once the parties return.
 */
export const runKmsGenerationProfile = async (state: State, runDecryption: DecryptionRunner) => {
  if (state.scenario.kms.mode !== "threshold") {
    throw new PreflightError(
      "kms-generation requires a threshold-mode KMS cluster; rerun `fhevm-cli up --scenario four-party-threshold-kms`",
    );
  }
  const { parties, reconstruct } = await auditKmsGeneration(state);
  const { stopForTolerance, stopForFloor } = quorumPlan(parties, state.scenario.kms.threshold);

  if (!(await runDecryption("kms-generation: baseline decryption (all parties up)"))) {
    throw new PreflightError("kms-generation: baseline user-decryption failed with all parties up");
  }

  // Stop parties from the top down so party 1 (the bare kms-core) stays up.
  const stopOrder = Array.from({ length: parties }, (_, index) => parties - index);
  const stopped: number[] = [];
  try {
    // Tolerance: drop to exactly the 2t+1 quorum — decryption must still succeed.
    for (const party of stopOrder.slice(0, stopForTolerance)) {
      await setRunning(partyContainers(party), "stop");
      stopped.push(party);
    }
    await waitForPartiesStopped(stopped);
    console.log(`[kms-generation] stopped ${stopForTolerance} part(y/ies); ${reconstruct} of ${parties} live (== 2t+1)`);
    if (!(await runDecryption(`kms-generation: decrypt with ${reconstruct}/${parties} live (== 2t+1)`))) {
      throw new PreflightError(
        `kms-generation: decryption failed at the 2t+1 quorum (${reconstruct}/${parties}) — it should tolerate losing ${stopForTolerance} part(y/ies)`,
      );
    }

    // Floor: drop one more, below quorum — decryption must NOT succeed.
    for (const party of stopOrder.slice(stopForTolerance, stopForFloor)) {
      await setRunning(partyContainers(party), "stop");
      stopped.push(party);
    }
    await waitForPartiesStopped(stopped);
    console.log(`[kms-generation] stopped ${stopForFloor} part(y/ies); ${reconstruct - 1} of ${parties} live (< 2t+1)`);
    if (await runDecryption(`kms-generation: attempt decrypt with ${reconstruct - 1}/${parties} live (< 2t+1)`, { expectFailure: true })) {
      throw new PreflightError(
        `kms-generation: user-decryption SUCCEEDED with only ${reconstruct - 1}/${parties} parties — the 2t+1 quorum is not enforced`,
      );
    }
    console.log("[kms-generation] confirmed: no decryption is possible below the 2t+1 quorum");
  } finally {
    for (const party of [...stopped].reverse()) {
      await setRunning(partyContainers(party), "start");
    }
  }

  await waitForPartiesRunning(stopped);
  if (!(await runDecryption("kms-generation: recovery decryption (all parties restarted)"))) {
    throw new PreflightError("kms-generation: decryption did not recover after restarting the stopped parties");
  }
  console.log("[kms-generation] PASS — threshold KMSGeneration verified and the 2t+1 decryption quorum holds");
};
