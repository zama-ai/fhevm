import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { phaseVersions, scenario, versionSources } from "./versions";

export type RolloutEnv = Record<string, string>;

const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

// RFC-029 key-material versions (mirrors the coprocessor's
// fhevm-engine-common::material_version::MaterialVersion).
export const MATERIAL_VERSION_LEGACY = 0;
export const MATERIAL_VERSION_MIGRATED_V1 = 1;

// A per-coprocessor view of a single operation's result digest, tagged with
// the material version the coprocessor used to produce it. The rollout
// collects one of these per (coprocessor, operation) by reading each
// coprocessor's output; the assertions below are pure over the collection so
// they can be unit-tested without a cluster.
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

// ---------------------------------------------------------------------------
// Integration points NOT yet wired (need either a coprocessor-DB exec helper on
// RolloutRunContext, or the on-chain KMSGeneration.addKeyMaterials /
// scheduleKeyMaterialVersion1Migration functions, which are out of scope for
// fhevm-internal#1568's first cut). These stubs document the exact governance
// actions the rollout stands in for, and throw if invoked before wiring so a
// half-wired rollout fails loudly instead of silently passing.
// ---------------------------------------------------------------------------

const NOT_WIRED = (what: string) =>
  new Error(
    `[rfc029] ${what} is not wired yet (fhevm-internal#1568). First cut seeds material + schedule directly into the coprocessor DB ` +
      `("coprocessor-and-kms-db"); this needs a RolloutRunContext.execCoprocessorSql helper (or the real addKeyMaterials/schedule tasks once they land).`,
  );

// Governance action 2 (stand-in): publish the migrated v1 material so every
// coprocessor ingests it under the same keyId. First cut: seed the v1
// CompressedXofKeySet bytes into each coprocessor's keys table.
const publishMigratedMaterial = async (_ctx: RolloutRunContext): Promise<void> => {
  throw NOT_WIRED("publishMigratedMaterial (addKeyMaterials)");
};

// Governance action 3 (stand-in): publish the cutover schedule -- per host
// chain H_C and the gateway G. First cut: INSERT into
// material_version_host_schedule / material_version_gateway_schedule.
const publishCutoverSchedule = async (
  _ctx: RolloutRunContext,
  _schedule: { hostChainCutovers: { chainKey: string; hostChainId: number; targetBlock: number }[]; gatewayCutover: number },
): Promise<void> => {
  throw NOT_WIRED("publishCutoverSchedule (scheduleKeyMaterialVersion1Migration)");
};

// Drive workload on both host chains past their H_C and on the gateway past G,
// then collect each coprocessor's per-operation digest + material version.
const crossCutoverAndCollectDigests = async (_ctx: RolloutRunContext): Promise<CoprocessorDigest[]> => {
  throw NOT_WIRED("crossCutoverAndCollectDigests");
};

const collectHeldMaterial = async (_ctx: RolloutRunContext): Promise<HeldMaterial> => {
  throw NOT_WIRED("collectHeldMaterial");
};

export default async function run(ctx: RolloutRunContext) {
  const baselineLock = await ctx.writeVersionLock("00-baseline", {
    versions: phaseVersions.baseline,
    sources: versionSources,
  });

  logPhase("00 baseline: boot the multi-chain stack (L1 + Polygon stand-in) on the RFC-029 build");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await ctx.test("rollout-standard", { parallel: false });

  // Pre-cutover: the whole fleet is on legacy material and agrees on it. With
  // no schedule published, the coprocessor's selection is inert (always v0), so
  // this is just today's behavior.
  logPhase("01 pre-cutover: assert every coprocessor holds and uses legacy material (inert default)");
  const heldBefore = await collectHeldMaterial(ctx);
  assertAllCoprocessorsHoldMaterial(heldBefore, MATERIAL_VERSION_LEGACY);

  // Governance action 2: publish v1, then GATE on the whole fleet holding it
  // before any schedule exists (the human-in-the-loop check).
  logPhase("02 publish v1 material (addKeyMaterials stand-in), then gate on fleet-wide ingestion");
  await publishMigratedMaterial(ctx);
  const heldAfterPublish = await collectHeldMaterial(ctx);
  assertAllCoprocessorsHoldMaterial(heldAfterPublish, MATERIAL_VERSION_MIGRATED_V1);

  // Governance action 3: publish the per-chain + gateway cutover schedule. Only
  // now does any coprocessor start selecting v1, and only for operations at or
  // past the cutover block.
  logPhase("03 publish the cutover schedule: per-chain H_C on L1 + chain-b, and gateway G");
  await publishCutoverSchedule(ctx, {
    hostChainCutovers: [
      { chainKey: "chain-a", hostChainId: 0, targetBlock: 0 },
      { chainKey: "chain-b", hostChainId: 0, targetBlock: 0 },
    ],
    gatewayCutover: 0,
  });

  // Cross the cutover on both host chains and the gateway, then assert the
  // fleet never diverged: same material version and same digest per operation,
  // before, across, and after each boundary.
  logPhase("04 cross H_C on both chains and G on the gateway; assert zero cross-coprocessor divergence");
  const digests = await crossCutoverAndCollectDigests(ctx);
  assertMaterialCutoverConsistent(digests);

  await ctx.test("rollout-standard", { parallel: false });
}
