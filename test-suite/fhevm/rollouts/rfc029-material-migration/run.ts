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

// How far ahead of "now" the cutover blocks are scheduled, as a BLOCK offset per
// timeline. Selection is a deterministic function of the per-operation anchoring
// block (host-chain block for compute, gateway block for input verification), NOT
// of wall-clock time -- so each timeline cuts over independently at its own block
// and the cutovers need NOT coincide in time across chains. With unequal block
// times (e.g. a 2s chain vs a 12s chain) "+30 blocks" is simply a different
// wall-clock lead on each, which is fine: the only ordering constraint is that the
// migrated material is published fleet-wide before any chain reaches its block
// (else halt-never-substitute stalls). The offset is large enough for that, small
// enough that the standard suite's workload still crosses it.
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

// How the cutover's zero-divergence property is actually verified: the scenario runs
// 5 coprocessors at threshold 3, and the gateway/host consensus cross-checks their
// per-operation result digests. If any coprocessor selected a different material version
// (or produced different bytes) around a migration block, the digests split and 3-of-5
// consensus cannot form, so the operation fails and the standard suite (phase 03) goes RED.
// A green phase-03 run is therefore the fleet-wide zero-divergence proof; there is no
// separate hand-rolled per-coprocessor digest collection in this rollout (it would
// duplicate, less reliably, what consensus already enforces).

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

/** Reads a uint256 view (e.g. getActiveKeyId / getKeyMaterialVersion) off KMSGeneration via `cast call`. */
const readKmsGenerationUint = async (ctx: RolloutRunContext, sig: string, ...args: string[]): Promise<bigint> => {
  const { rpcUrl, kmsGenerationAddress } = resolveKmsGenerationTarget(await ctx.readState());
  const raw = await castCall(rpcUrl, kmsGenerationAddress, sig, ...args);
  return BigInt(raw.split(/\s+/)[0] ?? "0");
};

const activeKeyId = (ctx: RolloutRunContext): Promise<bigint> => readKmsGenerationUint(ctx, "getActiveKeyId()(uint256)");

/**
 * Polls until the migration keygen `keyId` reaches consensus (isRequestDone). Option 2 publishes by
 * reading the migration keygen's digests on-chain (getKeyMaterials), so the keygen MUST have completed
 * before publish; otherwise addKeyMaterials has nothing to bind to. Shares the keygen timeout budget.
 */
const waitForKeygenDone = async (ctx: RolloutRunContext, keyId: bigint): Promise<void> => {
  const { rpcUrl, kmsGenerationAddress } = resolveKmsGenerationTarget(await ctx.readState());
  const started = Date.now();
  const deadline = started + KEYGEN_ACTIVATION_TIMEOUT_MS;
  while (Date.now() < deadline) {
    const raw = await castCall(rpcUrl, kmsGenerationAddress, "isRequestDone(uint256)(bool)", keyId.toString());
    const elapsed = Math.round((Date.now() - started) / 1000);
    if (raw.trim().startsWith("true")) {
      console.log(`[rollout] migration keygen ${keyId} reached consensus after ${elapsed}s`);
      return;
    }
    console.log(`[rollout] waiting for migration keygen ${keyId} consensus... ${elapsed}s elapsed`);
    await sleep(KEYGEN_POLL_INTERVAL_MS);
  }
  throw new Error(`migration keygen ${keyId} did not reach consensus within ${KEYGEN_ACTIVATION_TIMEOUT_MS / 1000}s`);
};

/**
 * Polls until the migrated material (version 1) is published under `keyId`. RFC-029 is
 * publish-not-activate: governance's addKeyMaterials emits KeyMaterialAdded under the existing key
 * (without advancing activeKeyId), which sets keyMaterialVersion[keyId] = 1. Logs progress so the
 * (slow) keygen + publish cycle is observable.
 */
const waitForMaterialPublished = async (ctx: RolloutRunContext, keyId: bigint): Promise<void> => {
  const started = Date.now();
  const deadline = started + KEYGEN_ACTIVATION_TIMEOUT_MS;
  while (Date.now() < deadline) {
    const version = await readKmsGenerationUint(ctx, "getKeyMaterialVersion(uint256)(uint256)", keyId.toString());
    const elapsed = Math.round((Date.now() - started) / 1000);
    if (version === BigInt(MATERIAL_VERSION_MIGRATED_V1)) {
      console.log(`[rollout] migrated material v1 published under key ${keyId} after ${elapsed}s`);
      return;
    }
    console.log(`[rollout] waiting for migration keygen + publish... ${elapsed}s elapsed, keyMaterialVersion(${keyId})=${version}`);
    await sleep(KEYGEN_POLL_INTERVAL_MS);
  }
  throw new Error(
    `migration keygen + publish did not finalize within ${KEYGEN_ACTIVATION_TIMEOUT_MS / 1000}s (keyMaterialVersion(${keyId}) never reached ${MATERIAL_VERSION_MIGRATED_V1})`,
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
  // connector (typed MigrationKeygenRequest -> keygen-from-existing) and
  // host-contracts (the migrationKeygen / addKeyMaterials / scheduleKeyMaterialMigration
  // surface) must ALSO be branch-built -- registry images lack the RFC-029 surface. (Do NOT
  // add { group: "coprocessor" }: the scenario already owns coprocessor source.)
  await ctx.up({
    lockFile: baselineLock,
    scenario,
    overrides: [{ group: "test-suite" }, { group: "kms-connector" }, { group: "host-contracts" }],
  });
  await ctx.test("rollout-standard", { parallel: false });

  // 01 publish -- drive a real migration keygen-from-existing, then have GOVERNANCE publish the
  // migrated material under the ACTIVE key via addKeyMaterials. RFC-029 publish-not-activate: the
  // keygen re-derives the active key's material in migrated (CompressedXofKeySet) form (its extraData
  // is the standard v2 context+epoch, so it reaches KMS consensus normally and its digests land
  // on-chain). Governance then publishes those digests UNDER the existing key (KeyMaterialAdded, no
  // activeKeyId move, no KMS on-chain signature -- the "DAO drives the cutover" model). We then wait
  // for keyMaterialVersion(activeKey)=1 and the host-listener to download v1 into
  // keys.migrated_xof_keyset fleet-wide.
  logPhase("01 publish: migration keygen-from-existing -> governance addKeyMaterials under the unchanged active key");
  const state = await ctx.readState();
  const migratedKeyId = await activeKeyId(ctx); // the key being migrated; stays active throughout
  await ctx.runHostContractTask(`npx hardhat task:triggerMigrationKeygen --params-type ${PARAMS_TYPE_TEST}`);
  // The migration keygen is the only keygen in flight, so its (throwaway) key id is the current key
  // counter. Bind publish to exactly this id.
  const migrationKeyId = await readKmsGenerationUint(ctx, "getKeyCounter()(uint256)");
  // Wait for the migration keygen to reach consensus so its KMS-attested digests are stored on-chain,
  // then have GOVERNANCE publish them under the existing key (addKeyMaterials binds to migrationKeyId).
  await waitForKeygenDone(ctx, migrationKeyId);
  await ctx.runHostContractTask(
    `npx hardhat task:publishMigratedKeyMaterials --migration-key-id ${migrationKeyId} --key-id ${migratedKeyId}`,
  );
  await waitForMaterialPublished(ctx, migratedKeyId);
  const stillActive = await activeKeyId(ctx);
  if (stillActive !== migratedKeyId) {
    throw new Error(`publish-not-activate violated: activeKeyId moved ${migratedKeyId} -> ${stillActive}`);
  }
  // Let the host-listener download the migrated keyset + publish it onto
  // keys.migrated_xof_keyset across the fleet before anyone crosses a cutover block.
  console.log(`[rollout] activeKeyId held at ${migratedKeyId}; waiting ${PUBLISH_GRACE_MS / 1000}s for v1 download fleet-wide`);
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
      `--gateway-migration-block ${gatewayMigrationBlock}`,
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
