/**
 * Blue-green `bootstrap`: boot the components that exist before the keys do at a previous
 * release, then upgrade them to the resolved bundle once the operators have ingested the keys.
 */
import type { BlueGreenBootstrap, LocalOverride, OverrideGroup, VersionBundle } from "../types";

export const CONTRACT_VERSION_KEYS = ["GATEWAY_VERSION", "HOST_VERSION"] as const;
export const LISTENER_CORE_VERSION_KEYS = ["LISTENER_CORE_VERSION"] as const;
export const CONNECTOR_VERSION_KEYS = [
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
] as const;
/** Connector services the previous release did not ship; they join with the connector upgrade. */
export const CONNECTOR_OPTIONAL_VERSION_KEYS = ["CONNECTOR_ENDPOINT_VERSION", "CONNECTOR_PROXY_VERSION"] as const;

/** Local overrides that would replace a pinned release image; suspended until the upgrade. */
export const BOOTSTRAP_SUSPENDED_GROUPS: readonly OverrideGroup[] = [
  "gateway-contracts",
  "host-contracts",
  "kms-connector",
  "listener-core",
];

/** The bundle with every bootstrapped component pinned to the release. */
export const bootstrapBootVersions = (bundle: VersionBundle, bootstrap: BlueGreenBootstrap): VersionBundle => {
  const env = { ...bundle.env };
  for (const key of [...CONTRACT_VERSION_KEYS, ...LISTENER_CORE_VERSION_KEYS, ...CONNECTOR_VERSION_KEYS, "CORE_VERSION"]) {
    env[key] = bootstrap.tag;
  }
  for (const key of CONNECTOR_OPTIONAL_VERSION_KEYS) {
    delete env[key];
  }
  return { ...bundle, env, sources: [...bundle.sources, `bootstrap=${bootstrap.tag}`] };
};

export const bootstrapBootOverrides = (overrides: LocalOverride[]) =>
  overrides.filter((override) => !BOOTSTRAP_SUSPENDED_GROUPS.includes(override.group));

/** `env` with the `keys` the target defines taken from it. */
export const bootstrapPhaseEnv = (
  env: Record<string, string>,
  target: Record<string, string>,
  keys: readonly string[],
) => ({
  ...env,
  ...Object.fromEntries(keys.filter((key) => target[key] !== undefined).map((key) => [key, target[key]])),
});

/**
 * Upgrades a proxy from the snapshotted previous sources to the current ones, or exits cleanly
 * when the contract's reinitializer did not move: the task would otherwise revert, and the
 * plans below can stay a list of every upgradeable contract rather than a snapshot of one diff.
 */
export const contractUpgradeCommand = (task: string, contract: string) => {
  const reinitializer = (file: string) =>
    `$(grep -m1 -oE 'REINITIALIZER_VERSION = [0-9]+' ${file} | grep -oE '[0-9]+' || true)`;
  return [
    `prev=${reinitializer(`previous-contracts/${contract}.sol`)}`,
    `next=${reinitializer(`contracts/${contract}.sol`)}`,
    `if [ -n "$prev" ] && [ "$prev" = "$next" ]; then`,
    `  echo "[bootstrap] ${contract}: reinitializer $prev unchanged since the snapshot; nothing to upgrade"; exit 0`,
    "fi",
    [
      `npx hardhat ${task}`,
      `--current-implementation previous-contracts/${contract}.sol:${contract}`,
      `--new-implementation contracts/${contract}.sol:${contract}`,
      "--verify-contract false",
      "--use-internal-proxy-address true",
    ].join(" "),
  ].join("\n");
};

/** Every gateway contract with an upgrade task, consumers before the contract they read. */
export const GATEWAY_CONTRACT_UPGRADES = [
  ["task:upgradeDecryption", "Decryption"],
  ["task:upgradeCiphertextCommits", "CiphertextCommits"],
  ["task:upgradeInputVerification", "InputVerification"],
  ["task:upgradeGatewayConfig", "GatewayConfig"],
  ["task:upgradeKMSGeneration", "KMSGeneration"],
] as const;

/** Deployed on the canonical host chain only. */
export const CANONICAL_HOST_CONTRACT_UPGRADES = [["task:upgradeKMSGeneration", "KMSGeneration"]] as const;

/** Every host contract with an upgrade task that is deployed on each host chain. */
export const HOST_CONTRACT_UPGRADES = [
  ["task:upgradeFHEVMExecutor", "FHEVMExecutor"],
  ["task:upgradeACL", "ACL"],
  ["task:upgradeHCULimit", "HCULimit"],
  ["task:upgradeInputVerifier", "InputVerifier"],
  ["task:upgradeKMSVerifier", "KMSVerifier"],
  ["task:upgradeProtocolConfig", "ProtocolConfig"],
] as const;
