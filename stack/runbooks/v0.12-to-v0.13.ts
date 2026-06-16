// EXEMPLAR — v0.12 -> v0.13 upgrade runbook.
// Shape mirrors test-suite/fhevm/rollouts/v0.12-to-v0.13/run.ts but is expressed
// over the new Stack API instead of RolloutRunContext.  The ordering is the
// contractually-required upgrade sequence; do not reorder without updating the
// companion RFC.
import type { Stack } from "../lib/stack";

export default async (s: Stack): Promise<void> => {
  // ── 00 baseline ──────────────────────────────────────────────────────────
  // Boot v0.12.5 with a two-of-three coprocessor scenario.  The test-suite
  // image is already pinned to the target tag in the MANIFEST so it exercises
  // the v0.13 harness against the old stack from the first test gate.
  await s.up({ scenario: "two-of-three" });
  await s.test("rollout-standard");

  // ── 01 contracts ─────────────────────────────────────────────────────────
  // Preserve the current contract artefacts as "previous-contracts" so upgrade
  // tasks can diff old vs new ABIs, then activate the v0.13 contract sources.
  await s.snapshotContracts("gateway");
  await s.snapshotContracts("host");

  // Gateway chain first — GatewayConfig must be view-only before exporting KMS
  // migration state; KMSGeneration upgrade follows once GatewayConfig is frozen.
  await s.gatewayTask(
    "npx hardhat compile && npx hardhat task:upgradeGatewayConfig" +
      " --current-implementation previous-contracts/GatewayConfig.sol:GatewayConfig" +
      " --new-implementation contracts/GatewayConfig.sol:GatewayConfig" +
      " --verify-contract false --use-internal-proxy-address true",
  );
  await s.gatewayTask(
    "npx hardhat compile && npx hardhat task:upgradeKMSGeneration" +
      " --current-implementation previous-contracts/KMSGeneration.sol:KMSGeneration" +
      " --new-implementation contracts/KMSGeneration.sol:KMSGeneration" +
      " --verify-contract false --use-internal-proxy-address true",
  );

  // Export gateway KMS migration state (node list + thresholds) before the
  // host chain migration tasks consume it.  The sc-deploy Job writes output
  // into the sc-addresses ConfigMap; refreshDiscovery propagates new addresses.
  await s.gatewayTask(
    "npx hardhat compile && npx hardhat task:exportKmsMigrationState" +
      " --kms-generation-proxy $KMS_GENERATION_ADDRESS" +
      " --gateway-config-proxy $GATEWAY_CONFIG_ADDRESS" +
      " --output /app/addresses/kms-migration-state.json",
  );

  // Deploy new host proxies, then run migration tasks in dependency order:
  // ProtocolConfig and KMSGeneration (from gateway state), KMSVerifier (consumes
  // new KMS context), HCULimit (must precede FHEVMExecutor), then the rest.
  await s.hostTask("npx hardhat task:deployEmptyProxiesProtocolConfigKMSGeneration");
  await s.hostTask("npx hardhat task:deployProtocolConfigFromMigration");
  await s.hostTask("npx hardhat task:deployKMSGenerationFromMigration");

  // Addresses written by the migration deploy tasks must propagate to runtime
  // env (ACL_CONTRACT_ADDRESS, KMS_GENERATION_ADDRESS, etc.) before any
  // services that read them are restarted.
  await s.refreshDiscovery();

  await s.hostTask(
    "npx hardhat compile && npx hardhat task:upgradeKMSVerifier" +
      " --current-implementation previous-contracts/KMSVerifier.sol:KMSVerifier" +
      " --new-implementation contracts/KMSVerifier.sol:KMSVerifier" +
      " --verify-contract false --use-internal-proxy-address true",
  );
  await s.hostTask(
    "npx hardhat compile && npx hardhat task:upgradeHCULimit" +
      " --current-implementation previous-contracts/HCULimit.sol:HCULimit" +
      " --new-implementation contracts/HCULimit.sol:HCULimit" +
      " --verify-contract false --use-internal-proxy-address true",
  );
  await s.hostTask(
    "npx hardhat compile && npx hardhat task:upgradeFHEVMExecutor" +
      " --current-implementation previous-contracts/FHEVMExecutor.sol:FHEVMExecutor" +
      " --new-implementation contracts/FHEVMExecutor.sol:FHEVMExecutor" +
      " --verify-contract false --use-internal-proxy-address true",
  );
  await s.hostTask(
    "npx hardhat compile && npx hardhat task:upgradeACL" +
      " --current-implementation previous-contracts/ACL.sol:ACL" +
      " --new-implementation contracts/ACL.sol:ACL" +
      " --verify-contract false --use-internal-proxy-address true",
  );

  await s.refreshDiscovery();
  await s.test("rollout-standard");

  // ── 02 relayer ───────────────────────────────────────────────────────────
  // Relayer must be on v0.13 before the KMS connector starts writing versioned
  // extraData; upgrading it here ensures no in-flight transactions are
  // misinterpreted by the old relayer path.
  await s.upgrade("relayer");
  await s.test("rollout-standard");

  // ── 03 kms ───────────────────────────────────────────────────────────────
  // kms-core (external) and kms-connector (chart) are upgraded together so
  // their protocol version stays in lock-step.
  await s.upgrade("kms");
  await s.test("rollout-standard");

  // ── 04 listener-core ─────────────────────────────────────────────────────
  // listener-core ships a breaking change in how coprocessor listeners
  // subscribe to events.  Old coprocessors do not consume listener-core, so
  // there is no test gate between listener-core and coprocessor — the
  // compatibility boundary is the coprocessor upgrade itself.
  await s.upgrade("listener-core");

  // ── 05 coprocessor (LAST) ─────────────────────────────────────────────────
  // Coprocessor is last because it is the heaviest state-bearing component.
  // Upgrading it last minimises the window during which new contract ABIs are
  // live but old coprocessor logic processes them.
  await s.upgrade("coprocessor");
  await s.test("rollout-standard");
};
