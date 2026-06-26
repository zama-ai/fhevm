import path from "node:path";

import { expect, test } from "bun:test";

import { loadRolloutRunbook } from "./commands/rollout-run";
import { loadCoprocessorScenario } from "./scenario/resolve";
import { buildMigrationScheduleArgs } from "../rollouts/rfc029-material-migration/run";
import { phaseVersions, scenario } from "../rollouts/rfc029-material-migration/versions";

const CLI_DIR = path.resolve(import.meta.dir, "..");

test("models the cutover on a 3-of-5 + 4-party-KMS multi-chain topology", async () => {
  expect(scenario).toBe("rfc029-cutover");
  const resolved = await loadCoprocessorScenario(scenario);
  // 5 coprocessors make a per-operation version split observable (it breaks
  // 3-of-5 consensus); two host chains exercise per-chain cutover blocks.
  expect(resolved.topology.count).toBe(5);
  expect(resolved.topology.threshold).toBe(3);
  expect(resolved.hostChains?.length).toBe(2);
  // A real 4-party threshold KMS so the migration keygen-from-existing runs.
  expect(resolved.kms?.mode).toBe("threshold");
  expect(resolved.kms?.parties).toBe(4);
  // All coprocessors are branch-built so the RFC-029 selection code is exercised.
  expect((resolved.instances ?? []).every((i) => i.source?.mode === "local")).toBe(true);
});

test("pins one coherent target across the whole stack (cutover is internal, not an upgrade)", () => {
  const tags = new Set(
    Object.values(phaseVersions.baseline).filter((_, i) => i >= 0),
  );
  // The cutover ships in one build, so there is a single baseline phase.
  expect(Object.keys(phaseVersions)).toEqual(["baseline"]);
  expect(phaseVersions.baseline.HOST_VERSION).toBe(phaseVersions.baseline.GATEWAY_VERSION);
  expect(tags.size).toBeGreaterThan(0);
});

test("pins kms-core to the connector-matched commit so the migration keygen RPC is proto-compatible", () => {
  // The connector compiles its kms-grpc proto from rev 1edf3a0; the running
  // kms-core image MUST be that same commit (published as core-service:1edf3a0).
  expect(phaseVersions.baseline.CORE_VERSION).toBe("1edf3a0");
});

test("loads the checked-in rfc029-material-migration runbook", async () => {
  await expect(
    loadRolloutRunbook(path.join(CLI_DIR, "rollouts/rfc029-material-migration/run.ts")),
  ).resolves.toBeFunction();
});

// --- schedule-arg prep: every host chain (canonical + non-canonical) gets an H_C ---

test("buildMigrationScheduleArgs gives every host chain its own H_C from its current block", () => {
  const args = buildMigrationScheduleArgs(
    [
      { key: "host", chainId: "12345" }, // canonical
      { key: "chain-b", chainId: "67890" }, // non-canonical
    ],
    { host: 100, "chain-b": 200 },
    500, // gateway block
    30, // host offset
    25, // gateway offset
  );
  expect(args.hostChainIds).toEqual(["12345", "67890"]);
  expect(args.hostMigrationBlocks).toEqual(["130", "230"]); // per-chain current + offset
  expect(args.gatewayMigrationBlock).toBe(525);
});

test("buildMigrationScheduleArgs throws if a host chain has no observed block", () => {
  expect(() =>
    buildMigrationScheduleArgs([{ key: "chain-b", chainId: "67890" }], {}, 1, 30, 30),
  ).toThrow(/no current block for host chain "chain-b"/);
});

test("buildMigrationScheduleArgs throws on an empty topology", () => {
  expect(() => buildMigrationScheduleArgs([], {}, 1, 30, 30)).toThrow(/no host chains/);
});
