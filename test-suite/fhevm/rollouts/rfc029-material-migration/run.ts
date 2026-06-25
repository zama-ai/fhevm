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

// Migration keygen-from-existing on a 4-party cluster is ~360s (measured); poll
// generously before giving up.
const KEYGEN_ACTIVATION_TIMEOUT_MS = 12 * 60 * 1000;
const KEYGEN_POLL_INTERVAL_MS = 10_000;
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

/** Polls until the active key id advances past `previous` (a fresh keygen finalized). */
const waitForActiveKeyAdvance = async (ctx: RolloutRunContext, previous: bigint): Promise<bigint> => {
  const deadline = Date.now() + KEYGEN_ACTIVATION_TIMEOUT_MS;
  while (Date.now() < deadline) {
    const current = await activeKeyId(ctx);
    if (current > previous) {
      return current;
    }
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
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await ctx.test("rollout-standard", { parallel: false });

  // 01 publish -- drive a real migration keygen-from-existing and publish its
  // result as v1 under the (now-active) migrated key. "Governance publishes"
  // variant: the migration keygen advances activeKeyId; we then re-publish the
  // KMS-consensus digests/urls as material version 1 via addKeyMaterials.
  logPhase("01 publish: trigger migration keygen-from-existing, then publish v1 material");
  const state = await ctx.readState();
  const keyBeforeMigration = await activeKeyId(ctx);
  await ctx.runHostContractTask(`task:triggerMigrationKeygen --params-type ${PARAMS_TYPE_TEST}`);
  const migratedKeyId = await waitForActiveKeyAdvance(ctx, keyBeforeMigration);
  console.log(`[rollout] migration keygen finalized: activeKeyId ${keyBeforeMigration} -> ${migratedKeyId}`);
  await ctx.runHostContractTask(`task:publishMigratedKeyMaterials --material-version ${MATERIAL_VERSION_MIGRATED_V1}`);
  // Let the host-listener download the migrated keyset + publish it onto
  // keys.migrated_xof_keyset across the fleet before anyone crosses a cutover block.
  console.log(`[rollout] waiting ${PUBLISH_GRACE_MS / 1000}s for v1 material to publish fleet-wide`);
  await sleep(PUBLISH_GRACE_MS);

  // 02 schedule -- publish the per-chain (H_C) + gateway (G) cutover blocks, set
  // a small distance ahead of each chain's current height so the standard suite
  // crosses them mid-run.
  logPhase("02 schedule: publish the per-chain + gateway material-version cutover blocks");
  const hostChains = state.scenario.hostChains;
  const hostChainIds: string[] = [];
  const hostMigrationBlocks: string[] = [];
  for (const chain of hostChains) {
    const httpUrl = state.discovery?.endpoints.hosts[chain.key]?.http;
    if (!httpUrl) {
      throw new Error(`no RPC endpoint discovered for host chain "${chain.key}"`);
    }
    const current = await blockNumber(httpUrl);
    hostChainIds.push(String(chain.chainId));
    hostMigrationBlocks.push(String(current + HOST_MIGRATION_OFFSET));
  }
  const gatewayHttp = state.discovery?.endpoints.gateway.http;
  if (!gatewayHttp) {
    throw new Error("no gateway RPC endpoint discovered");
  }
  const gatewayMigrationBlock = (await blockNumber(gatewayHttp)) + GATEWAY_MIGRATION_OFFSET;

  await ctx.runHostContractTask(
    `task:scheduleKeyMaterialMigration ` +
      `--host-chain-ids ${hostChainIds.join(",")} ` +
      `--host-migration-blocks ${hostMigrationBlocks.join(",")} ` +
      `--gateway-migration-block ${gatewayMigrationBlock} ` +
      `--material-version ${MATERIAL_VERSION_MIGRATED_V1}`,
  );
  console.log(
    `[rollout] cutover scheduled: chains=${hostChainIds.join(",")} H_C=${hostMigrationBlocks.join(",")} G=${gatewayMigrationBlock}`,
  );

  // 03 cross -- run the standard suite again. Its workload advances every chain
  // past its migration block and the gateway past G, so each coprocessor switches
  // v0 -> v1 deterministically (block < H_C ? v0 : v1). On a 3-of-5 fleet, any
  // per-operation material-version split breaks consensus and turns this red --
  // so a green run IS the zero-divergence proof through the cutover blocks.
  logPhase("03 cross: run the standard suite across the cutover (3-of-5 consensus = zero-divergence detector)");
  await ctx.test("rollout-standard", { parallel: false });
}
