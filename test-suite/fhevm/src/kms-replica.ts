/**
 * Helpers for the ProtocolConfigReplica of every non-canonical host chain: read the canonical and
 * replica KMS context and epoch, check that each replica matches the canonical state, and replay
 * the signed KMS confirmations onto each replica (`task:buildReplicaReplayCalldata`, then the
 * `submit*` calls).
 */
import { readFile } from "node:fs/promises";
import path from "node:path";

import { PreflightError } from "./errors";
import { runContractTask } from "./flow/contracts";
import { castCall } from "./flow/readiness";
import { defaultHostChain, extraHostChains } from "./flow/topology";
import { castSend, type Owner } from "./kms-onchain";
import { type HostChainRuntime, composePath, envPath } from "./layout";
import type { State } from "./types";
import { hostReachableRpcUrl, readEnvFile, withHexPrefix } from "./utils/fs";
import { run } from "./utils/process";

const FINALITY_TIMEOUT_MS = 120_000;
const FINALITY_POLL_MS = 1_000;

export type ContextAndEpoch = { contextId: bigint; epochId: bigint };

export type SwitchTarget = { rpcUrl: string; configAddress: string; where: string };

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
      `kms-replica: could not parse getCurrentKmsContextAndEpoch output: ${JSON.stringify(raw)}`,
    );
  }
  return { contextId: BigInt(ids[0]), epochId: BigInt(ids[1]) };
};

export const readContextAndEpoch = async (rpcUrl: string, protocolConfig: string): Promise<ContextAndEpoch> =>
  parseContextAndEpoch(await castCall(rpcUrl, protocolConfig, "getCurrentKmsContextAndEpoch()(uint256,uint256)"));

/** Waits for the latest block to finalize and returns that block's timestamp. */
export const waitForLatestFinalized = async (rpcUrl: string): Promise<number> => {
  const url = hostReachableRpcUrl(rpcUrl);
  const castNumber = async (...tag: string[]) =>
    Number((await run(["cast", ...tag, "--rpc-url", url])).stdout.trim());
  const latest = await castNumber("block-number");
  const deadline = Date.now() + FINALITY_TIMEOUT_MS;
  while ((await castNumber("block", "finalized", "--field", "number")) < latest) {
    if (Date.now() >= deadline) {
      throw new PreflightError(`kms-replica: block ${latest} not finalized within ${FINALITY_TIMEOUT_MS / 1000}s`);
    }
    await Bun.sleep(FINALITY_POLL_MS);
  }
  return castNumber("block", String(latest), "--field", "timestamp");
};

/** Reads the calldata of each call that `task:buildReplicaReplayCalldata` prints, in replay order.
 * Exported for unit testing. */
export const parseReplayCalldata = (output: string): string[] =>
  [...output.matchAll(/^\s*calldata:\s*(0x[0-9a-fA-F]+)\s*$/gm)].map((match) => match[1]);

const REPLAY_OUTPUT_FILE = "replica-replay.txt";
const KMS_SIGNERS_SIGNATURE = "getKmsSignersForContext(uint256)(address[])";

type Replica = { chain: HostChainRuntime; rpcUrl: string; address: string };

export const replicasOf = (state: State): Replica[] =>
  extraHostChains(state).map((chain) => ({
    chain,
    rpcUrl: hostReachableRpcUrl(state.discovery!.endpoints.hosts[chain.key].http),
    address: withHexPrefix(state.discovery!.hosts[chain.key].PROTOCOL_CONFIG_CONTRACT_ADDRESS),
  }));

const assertReplicaMatchesCanonical = async (replica: Replica, canonical: SwitchTarget, afterWhat: string) => {
  const expected = await readContextAndEpoch(canonical.rpcUrl, canonical.configAddress);
  const actual = await readContextAndEpoch(replica.rpcUrl, replica.address);
  if (actual.contextId !== expected.contextId || actual.epochId !== expected.epochId) {
    throw new PreflightError(
      `kms-replica: replica on ${replica.chain.key} is at (contextId=${actual.contextId}, epochId=${actual.epochId}) ` +
        `after ${afterWhat}. Canonical is at (contextId=${expected.contextId}, epochId=${expected.epochId})`,
    );
  }
  const [expectedSigners, actualSigners] = await Promise.all([
    castCall(canonical.rpcUrl, canonical.configAddress, KMS_SIGNERS_SIGNATURE, expected.contextId.toString()),
    castCall(replica.rpcUrl, replica.address, KMS_SIGNERS_SIGNATURE, expected.contextId.toString()),
  ]);
  if (actualSigners !== expectedSigners) {
    throw new PreflightError(
      `kms-replica: replica on ${replica.chain.key} lists signers ${actualSigners} for context ${expected.contextId} ` +
        `after ${afterWhat}. Canonical lists ${expectedSigners}`,
    );
  }
};

export const assertReplicasDeployed = async (state: State, canonical: SwitchTarget) => {
  for (const replica of replicasOf(state)) {
    const version = await castCall(replica.rpcUrl, replica.address, "getVersion()(string)");
    if (!version.includes("ProtocolConfigReplica")) {
      throw new PreflightError(
        `kms-replica: ProtocolConfig on ${replica.chain.key} reports version ${version}, not a ProtocolConfigReplica`,
      );
    }
    await assertReplicaMatchesCanonical(replica, canonical, "deploy");
  }
};

/**
 * The replay task reads the canonical events only up to the latest finalized block. Any account may
 * send the calldata, because the replica checks the KMS signatures.
 */
export const syncReplicas = async (state: State, canonical: SwitchTarget, afterWhat: string) => {
  await waitForLatestFinalized(canonical.rpcUrl);
  const canonicalChain = defaultHostChain(state);
  const hostSc = await readEnvFile(envPath("host-sc"));
  // The host pauser is a funded anvil account on every host chain. It is not the ACL owner of the replica.
  const sender: Owner = { key: hostSc.PAUSER_PRIVATE_KEY, address: hostSc.PAUSER_ADDRESS_0 };
  for (const replica of replicasOf(state)) {
    const { chain } = replica;
    console.log(`[kms-replica] replaying ${afterWhat} onto the replica on ${chain.key}…`);
    await runContractTask(
      "host-sc",
      `${chain.sc}-deploy`,
      [
        "npx hardhat task:buildReplicaReplayCalldata",
        `--canonical-rpc-url http://${canonicalChain.node}:${canonicalChain.rpcPort}`,
        `--canonical-protocol-config-address ${canonical.configAddress}`,
        "--use-internal-proxy-address true",
        `> /app/addresses/${REPLAY_OUTPUT_FILE} && cat /app/addresses/${REPLAY_OUTPUT_FILE}`,
      ].join(" "),
      { envComponent: chain.sc, composeFile: composePath(chain.sc) },
    );
    const output = await readFile(path.join(path.dirname(chain.addressesPath), REPLAY_OUTPUT_FILE), "utf8");
    for (const calldata of parseReplayCalldata(output)) {
      await castSend(replica.rpcUrl, replica.address, sender, calldata);
    }
    await assertReplicaMatchesCanonical(replica, canonical, afterWhat);
    console.log(`[kms-replica] replica on ${chain.key} matches canonical after ${afterWhat}`);
  }
};
