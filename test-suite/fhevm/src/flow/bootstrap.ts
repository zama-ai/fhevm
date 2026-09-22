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
  for (const key of [...CONTRACT_VERSION_KEYS, ...LISTENER_CORE_VERSION_KEYS, ...CONNECTOR_VERSION_KEYS]) {
    env[key] = bootstrap.tag;
  }
  for (const key of CONNECTOR_OPTIONAL_VERSION_KEYS) {
    delete env[key];
  }
  env.CORE_VERSION = bootstrap.coreVersion;
  return { ...bundle, env, sources: [...bundle.sources, `bootstrap=${bootstrap.tag}`] };
};

export const bootstrapBootOverrides = (overrides: LocalOverride[]) =>
  overrides.filter((override) => !BOOTSTRAP_SUSPENDED_GROUPS.includes(override.group));

/** `env` with `keys` taken from the target; a key the target lacks is dropped. */
export const bootstrapPhaseEnv = (
  env: Record<string, string>,
  target: Record<string, string>,
  keys: readonly string[],
) => {
  const next = { ...env };
  for (const key of keys) {
    if (target[key] === undefined) {
      delete next[key];
    } else {
      next[key] = target[key];
    }
  }
  return next;
};

export const contractUpgradeCommand = (task: string, contract: string) =>
  [
    `npx hardhat ${task}`,
    `--current-implementation previous-contracts/${contract}.sol:${contract}`,
    `--new-implementation contracts/${contract}.sol:${contract}`,
    "--verify-contract false",
    "--use-internal-proxy-address true",
  ].join(" ");

/** Contracts whose reinitializer moved since the 0.14 release, in dependency order. */
export const GATEWAY_CONTRACT_UPGRADES = [
  // Consumers of the removed priority-coprocessor getters go before GatewayConfig removes them.
  ["task:upgradeDecryption", "Decryption"],
  ["task:upgradeCiphertextCommits", "CiphertextCommits"],
  ["task:upgradeInputVerification", "InputVerification"],
  ["task:upgradeGatewayConfig", "GatewayConfig"],
] as const;

/** Deployed on the canonical host chain only. */
export const CANONICAL_HOST_CONTRACT_UPGRADES = [["task:upgradeKMSGeneration", "KMSGeneration"]] as const;

/** Deployed on every host chain. */
export const HOST_CONTRACT_UPGRADES = [
  ["task:upgradeFHEVMExecutor", "FHEVMExecutor"],
  ["task:upgradeProtocolConfig", "ProtocolConfig"],
] as const;
