import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { phaseVersions, scenario, versionSources } from "./versions";

const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

// RFC-029 key-material versions (mirrors the coprocessor's
// fhevm-engine-common::material_version::MaterialVersion).
export const MATERIAL_VERSION_LEGACY = 0;
export const MATERIAL_VERSION_MIGRATED_V1 = 1;

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

export default async function run(ctx: RolloutRunContext) {
  const baselineLock = await ctx.writeVersionLock("00-baseline", {
    versions: phaseVersions.baseline,
    sources: versionSources,
  });

  // Inert smoke: boot the multi-chain stack on the RFC-029 build and run the
  // standard suite. With no schedule published, the coprocessor's material
  // selection is inert (always v0), so a green run here is the integration-level
  // proof that the foundation changes (migration + selection module) don't alter
  // today's behavior.
  logPhase("00 baseline: boot the multi-chain stack (L1 + Polygon stand-in) and run the standard suite");
  await ctx.up({ lockFile: baselineLock, scenario, overrides: [{ group: "test-suite" }] });
  await ctx.test("rollout-standard", { parallel: false });

  // The cutover phase below is intentionally NOT wired here (fhevm-internal#1568):
  //   1. publish v1 (addKeyMaterials stand-in) -> assertAllCoprocessorsHoldMaterial(MIGRATED_V1)
  //   2. publish the schedule ([{L1,H_C},{chain-b,H_C}], G)
  //   3. drive workload past every H_C and G, collect per-(coprocessor,op)
  //      digests -> assertMaterialCutoverConsistent
  // It depends on (a) the version-partitioned-batch worker wiring, (b) a
  // RolloutRunContext helper to seed material + schedule into the coprocessor DB
  // (or the real addKeyMaterials/schedule tasks once they land), (c) real v1
  // CompressedXofKeySet bytes, and (d) a >=2-coprocessor topology for the
  // divergence assertion to be meaningful. The assertions it will use
  // (assertAllCoprocessorsHoldMaterial / assertMaterialCutoverConsistent) are
  // implemented and unit-tested above so they're ready for that wiring PR.
}
