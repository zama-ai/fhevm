type Env = Record<string, string | undefined>;

export const connectorVersionKeys = [
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
] as const;

export const relayerVersionKeys = ["RELAYER_VERSION", "RELAYER_MIGRATE_VERSION"] as const;

export const listenerCoreVersionKeys = ["LISTENER_CORE_VERSION"] as const;

export const coprocessorVersionKeys = [
  "COPROCESSOR_DB_MIGRATION_VERSION",
  "COPROCESSOR_HOST_LISTENER_VERSION",
  "COPROCESSOR_GW_LISTENER_VERSION",
  "COPROCESSOR_TX_SENDER_VERSION",
  "COPROCESSOR_TFHE_WORKER_VERSION",
  "COPROCESSOR_ZKPROOF_WORKER_VERSION",
  "COPROCESSOR_SNS_WORKER_VERSION",
] as const;

export const optionalCoprocessorVersionKeys = [
  "COPROCESSOR_CONSENSUS_DETECTOR_VERSION",
  "COPROCESSOR_UPGRADE_CONTROLLER_VERSION",
] as const;

export type MigrationVersions = {
  baseline: Record<string, string>;
  baselineTag: string;
};

const DEFAULT_TARGET_FHEVM_SHA = "6430345d834d78af47b7bc625d21cee2031b07f7";

const requiredVersion = (versions: Record<string, string>, key: string): string => {
  const value = versions[key]?.trim();
  if (!value) throw new Error(`${key} is missing from the resolved target lock`);
  return value;
};

export const migrationPhaseVersions = (
  baseline: Record<string, string>,
  target: Record<string, string>,
) => {
  const contract: Record<string, string> = {
    ...baseline,
    GATEWAY_VERSION: requiredVersion(target, "GATEWAY_VERSION"),
    HOST_VERSION: requiredVersion(target, "HOST_VERSION"),
  };
  const relayer: Record<string, string> = {
    ...contract,
    ...Object.fromEntries(relayerVersionKeys.map((key) => [key, requiredVersion(target, key)])),
  };
  const connector: Record<string, string> = {
    ...relayer,
    CORE_VERSION: requiredVersion(target, "CORE_VERSION"),
    ...Object.fromEntries(connectorVersionKeys.map((key) => [key, requiredVersion(target, key)])),
  };
  const listenerCore: Record<string, string> = {
    ...connector,
    ...Object.fromEntries(listenerCoreVersionKeys.map((key) => [key, requiredVersion(target, key)])),
  };
  return { contract, relayer, connector, listenerCore };
};

export const migrationVersions = (env: Env = process.env): MigrationVersions => {
  const releaseTag = env.RFC029_BASELINE_FHEVM_TAG?.trim() || "v0.14.1";
  const hostTag = env.RFC029_BASELINE_HOST_TAG?.trim() || releaseTag;
  const kmsCoreTag = env.RFC029_KMS_CORE_TAG?.trim() || "v0.14.0-1";
  const relayerTag = env.RFC029_BASELINE_RELAYER_TAG?.trim() || releaseTag;
  return {
    baselineTag: releaseTag,
    baseline: {
      GATEWAY_VERSION: releaseTag,
      HOST_VERSION: hostTag,
      CORE_VERSION: kmsCoreTag,
      RELAYER_VERSION: relayerTag,
      RELAYER_MIGRATE_VERSION: relayerTag,
      LISTENER_CORE_VERSION: releaseTag,
      ...Object.fromEntries(connectorVersionKeys.map((key) => [key, releaseTag])),
      ...Object.fromEntries(coprocessorVersionKeys.map((key) => [key, releaseTag])),
    },
  };
};

export const migrationTargetSha = (env: Env = process.env) =>
  env.RFC029_TARGET_FHEVM_SHA?.trim() || DEFAULT_TARGET_FHEVM_SHA;

export const migrationBaselineVersions = (
  target: Record<string, string>,
  baseline: Record<string, string>,
) => {
  const versions = { ...target, ...baseline };
  for (const key of optionalCoprocessorVersionKeys) delete versions[key];
  return versions;
};

export const versionSources = [
  "rollout=v0.14-to-v0.15-gpu-key-migration",
  "migration=same-key-compressed-xof",
];
