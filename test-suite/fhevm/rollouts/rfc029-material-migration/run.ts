import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { castCall, resolveKmsGenerationTarget } from "../../src/flow/readiness";
import { phaseVersions, scenario, versionSources } from "./versions";

const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

// RFC-029 key-material versions (mirrors the coprocessor's
// fhevm-engine-common::material_version::MaterialVersion).
export const MATERIAL_VERSION_LEGACY = 0;
export const MATERIAL_VERSION_MIGRATED_V1 = 1;

// FHE params type the scenario generates (Test = 1) — must match the
// migration keygen's paramsType so kms-core re-derives under the same params.
const PARAMS_TYPE_TEST = 1;

// How far ahead of "now" the cutover blocks are scheduled. Large enough that
// the migrated material is published fleet-wide before any chain crosses its
// migration block (else halt-never-substitute would stall the suite), small
// enough that the standard suite's workload actually crosses it.
const HOST_MIGRATION_OFFSET = 30;
const GATEWAY_MIGRATION_OFFSET = 30;

// A fresh migration keygen runs a FULL threshold preprocessing + keygen MPC cycle
// (PrepKeygenRequest -> prepKeygenResponse -> KeygenRequest -> keygenResponse) on the
// 4-party cluster, and EACH on-chain response waits 64 confirmations across multiple
// parties -- so it is much slower than the keygen-only step at boot. Iteration 7
// confirmed the prep phase completes (~3.5min) but the full activation did not finish
// within 25min; budget 45min.
const KEYGEN_ACTIVATION_TIMEOUT_MS = 45 * 60 * 1000;
const KEYGEN_POLL_INTERVAL_MS = 15_000;
// Grace for the host-listener to download + publish v1 onto keys.migrated_xof_keyset
// across the whole fleet before the cutover blocks are crossed.
const PUBLISH_GRACE_MS = 90_000;

// A per-coprocessor view of a single operation's result digest, tagged with
// the material version the coprocessor used to produce it. The cutover phase
// collects one of these per (coprocessor, operation); the assertions below are
// pure over the collection so they can be unit-tested without a cluster.
export type CoprocessorDigest = {
  coprocessor: string;
  operationId: string;
  materialVersion: number;
  digest: string;
};

// The set of material versions a coprocessor reports holding (verified +
// stored), keyed by coprocessor id.
export type HeldMaterial = Record<string, number[]>;

/**
 * Gate before scheduling the cutover: every coprocessor must already hold the
 * target version's material. This is the human-in-the-loop check from the
 * runbook -- confirm the whole fleet ingested `addKeyMaterials` (v1) before the
 * schedule makes anyone switch to it. Throws if any coprocessor is missing it.
 */
export const assertAllCoprocessorsHoldMaterial = (held: HeldMaterial, version: number): void => {
  const coprocessors = Object.keys(held);
  if (coprocessors.length === 0) {
    throw new Error("assertAllCoprocessorsHoldMaterial: no coprocessors reported any material");
  }
  const missing = coprocessors.filter((c) => !held[c]?.includes(version));
  if (missing.length > 0) {
    throw new Error(
      `material version ${version} not held by: ${missing.join(", ")} -- do NOT schedule the cutover until the whole fleet has ingested it`,
    );
  }
};

/**
 * The core consensus assertion: for every operation, every coprocessor must
 * have used the SAME material version AND produced the SAME digest. A split on
 * either is a fleet divergence -- exactly what the deterministic per-operation
 * selection (block < H_C ? v0 : v1, etc.) exists to prevent. Throws on the
 * first divergence found.
 */
export const assertMaterialCutoverConsistent = (digests: CoprocessorDigest[]): void => {
  const byOperation = new Map<string, CoprocessorDigest[]>();
  for (const d of digests) {
    const group = byOperation.get(d.operationId) ?? [];
    group.push(d);
    byOperation.set(d.operationId, group);
  }
  for (const [operationId, group] of byOperation) {
    const versions = new Set(group.map((d) => d.materialVersion));
    if (versions.size > 1) {
      throw new Error(
        `operation ${operationId}: coprocessors disagreed on material version (${[...versions].join(", ")}) -- cutover-block selection diverged across the fleet`,
      );
    }
    const uniqueDigests = new Set(group.map((d) => d.digest));
    if (uniqueDigests.size > 1) {
      throw new Error(
        `operation ${operationId}: coprocessors produced different digests (${[...uniqueDigests].join(", ")}) under material version ${group[0]?.materialVersion}`,
      );
    }
  }
};

export type MigrationScheduleArgs = {
  hostChainIds: string[];
  hostMigrationBlocks: string[];
  gatewayMigrationBlock: number;
};

/**
 * Builds the scheduleKeyMaterialMigration arguments from each host chain's
 * current block height. Pure (heights are passed in) so the per-chain H_C and
 * gateway G computation is unit-testable without a running stack. EVERY host
 * chain in the topology gets its own H_C (parallel arrays), so the cutover is
 * scheduled on the canonical chain AND every non-canonical chain -- the
 * two-host-chain migration is genuinely covered, not just the canonical one.
 */
export const buildMigrationScheduleArgs = (
  hostChains: { key: string; chainId: string | number }[],
  currentBlockByChainKey: Record<string, number>,
  gatewayBlock: number,
  hostOffset: number,
  gatewayOffset: number,
): MigrationScheduleArgs => {
  if (hostChains.length === 0) {
    throw new Error("buildMigrationScheduleArgs: topology has no host chains");
  }
  const hostChainIds: string[] = [];
  const hostMigrationBlocks: string[] = [];
  for (const chain of hostChains) {
    const current = currentBlockByChainKey[chain.key];
    if (current === undefined) {
      throw new Error(`buildMigrationScheduleArgs: no current block for host chain "${chain.key}"`);
    }
    hostChainIds.push(String(chain.chainId));
    hostMigrationBlocks.push(String(current + hostOffset));
  }
  return { hostChainIds, hostMigrationBlocks, gatewayMigrationBlock: gatewayBlock + gatewayOffset };
};

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/** Reads the latest block number off an EVM JSON-RPC endpoint (dependency-free). */
const blockNumber = async (rpcUrl: string): Promise<number> => {
  const response = await fetch(rpcUrl, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "eth_blockNumber", params: [] }),
  });
  const payload = (await response.json()) as { result?: string };
  return payload.result ? Number(BigInt(payload.result)) : 0;
};

/** Reads getActiveKeyId() off the KMSGeneration contract via `cast call`. */
const activeKeyId = async (ctx: RolloutRunContext): Promise<bigint> => {
  const { rpcUrl, kmsGenerationAddress } = resolveKmsGenerationTarget(await ctx.readState());
  const raw = await castCall(rpcUrl, kmsGenerationAddress, "getActiveKeyId()(uint256)");
  return BigInt(raw.split(/\s+/)[0] ?? "0");
};

/** Polls until the active key id advances past `previous` (a fresh keygen finalized),
 * logging progress each tick so a slow MPC cycle is observable in the rollout log. */
const waitForActiveKeyAdvance = async (ctx: RolloutRunContext, previous: bigint): Promise<bigint> => {
  const started = Date.now();
  const deadline = started + KEYGEN_ACTIVATION_TIMEOUT_MS;
  while (Date.now() < deadline) {
    const current = await activeKeyId(ctx);
    const elapsed = Math.round((Date.now() - started) / 1000);
    if (current > previous) {
      console.log(`[rollout] migration keygen finalized after ${elapsed}s (activeKeyId ${previous} -> ${current})`);
      return current;
    }
    console.log(`[rollout] waiting for migration keygen... ${elapsed}s elapsed, activeKeyId still ${previous}`);
    await sleep(KEYGEN_POLL_INTERVAL_MS);
  }
  throw new Error(
    `migration keygen did not finalize within ${KEYGEN_ACTIVATION_TIMEOUT_MS / 1000}s (activeKeyId still ${previous})`,
  );
};

export default async function run(ctx: RolloutRunContext) {
  const baselineLock = await ctx.writeVersionLock("00-baseline", {
    versions: phaseVersions.baseline,
    sources: versionSources,
  });

  // 00 baseline -- boot the 5-coprocessor + 4-party-KMS multi-chain stack on the
  // RFC-029 branch build and run the standard suite. With no schedule published,
  // material selection is inert (always v0), so a green run here is the
  // integration-level proof that the foundation changes don't alter today's
  // behavior (DONE-WHEN: no-schedule behavior identical to today).
  logPhase("00 baseline: boot 5-coprocessor + 4-party-KMS multi-chain stack, run the standard suite (no schedule = inert v0)");
  // The coprocessor is branch-built via the scenario's local instances. The
  // connector (v3 extraData -> keygen-from-existing) and host-contracts (the
  // keygen(bytes) / addKeyMaterials / scheduleKeyMaterialMigration surface) must
  // ALSO be branch-built -- registry images lack the RFC-029 surface. (Do NOT
  // add { group: "coprocessor" }: the scenario already owns coprocessor source.)
  await ctx.up({
    lockFile: baselineLock,
    scenario,
    overrides: [{ group: "test-suite" }, { group: "kms-connector" }, { group: "host-contracts" }],
  });
  await ctx.test("rollout-standard", { parallel: false });

  // 01 publish -- drive a real migration keygen-from-existing and publish its
  // result as v1 under the (now-active) migrated key. "Governance publishes"
  // variant: the migration keygen advances activeKeyId; we then re-publish the
  // KMS-consensus digests/urls as material version 1 via addKeyMaterials.
  logPhase("01 publish: trigger migration keygen-from-existing, then publish v1 material");
  const state = await ctx.readState();
  const keyBeforeMigration = await activeKeyId(ctx);
  await ctx.runHostContractTask(`npx hardhat task:triggerMigrationKeygen --params-type ${PARAMS_TYPE_TEST}`);
  const migratedKeyId = await waitForActiveKeyAdvance(ctx, keyBeforeMigration);
  console.log(`[rollout] migration keygen finalized: activeKeyId ${keyBeforeMigration} -> ${migratedKeyId}`);
  await ctx.runHostContractTask(`npx hardhat task:publishMigratedKeyMaterials --material-version ${MATERIAL_VERSION_MIGRATED_V1}`);
  // Let the host-listener download the migrated keyset + publish it onto
  // keys.migrated_xof_keyset across the fleet before anyone crosses a cutover block.
  console.log(`[rollout] waiting ${PUBLISH_GRACE_MS / 1000}s for v1 material to publish fleet-wide`);
  await sleep(PUBLISH_GRACE_MS);

  // 02 schedule -- publish the per-chain (H_C) + gateway (G) cutover blocks, set
  // a small distance ahead of each chain's current height so the standard suite
  // crosses them mid-run.
  logPhase("02 schedule: publish the per-chain + gateway material-version cutover blocks");
  const hostChains = state.scenario.hostChains;
  const currentBlockByChainKey: Record<string, number> = {};
  for (const chain of hostChains) {
    const httpUrl = state.discovery?.endpoints.hosts[chain.key]?.http;
    if (!httpUrl) {
      throw new Error(`no RPC endpoint discovered for host chain "${chain.key}"`);
    }
    currentBlockByChainKey[chain.key] = await blockNumber(httpUrl);
  }
  const gatewayHttp = state.discovery?.endpoints.gateway.http;
  if (!gatewayHttp) {
    throw new Error("no gateway RPC endpoint discovered");
  }
  const { hostChainIds, hostMigrationBlocks, gatewayMigrationBlock } = buildMigrationScheduleArgs(
    hostChains,
    currentBlockByChainKey,
    await blockNumber(gatewayHttp),
    HOST_MIGRATION_OFFSET,
    GATEWAY_MIGRATION_OFFSET,
  );

  await ctx.runHostContractTask(
    `npx hardhat task:scheduleKeyMaterialMigration ` +
      `--host-chain-ids ${hostChainIds.join(",")} ` +
      `--host-migration-blocks ${hostMigrationBlocks.join(",")} ` +
      `--gateway-migration-block ${gatewayMigrationBlock} ` +
      `--material-version ${MATERIAL_VERSION_MIGRATED_V1}`,
  );
  console.log(
    `[rollout] cutover scheduled: chains=${hostChainIds.join(",")} H_C=${hostMigrationBlocks.join(",")} G=${gatewayMigrationBlock}`,
  );

  // 03 cross -- run workload across the cutover on BOTH host chains. rollout-standard
  // exercises the canonical chain (its H_C) and the gateway (G); multi-chain-isolation
  // additionally transacts on the non-canonical chain-b, so its H_C is genuinely
  // crossed too. Each coprocessor switches v0 -> v1 deterministically per chain
  // (block < H_C ? v0 : v1). On a 3-of-5 fleet, any per-operation material-version
  // split breaks consensus and turns this red -- so green is the zero-divergence
  // proof through EVERY host_migration_block and the gateway_migration_block.
  logPhase("03 cross: run workload across the cutover on both host chains (3-of-5 consensus = zero-divergence detector)");
  await ctx.test("rollout-standard", { parallel: false });
  await ctx.test("multi-chain-isolation", { parallel: false });
}
