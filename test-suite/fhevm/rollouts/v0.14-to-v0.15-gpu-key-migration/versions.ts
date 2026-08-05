type Env = Record<string, string | undefined>;

const required = (env: Env, name: string): string => {
  const value = env[name]?.trim();
  if (!value) {
    throw new Error(`${name} is required and must be an exact image tag or commit SHA`);
  }
  return value;
};

export const connectorVersionKeys = [
  "CONNECTOR_DB_MIGRATION_VERSION",
  "CONNECTOR_GW_LISTENER_VERSION",
  "CONNECTOR_KMS_WORKER_VERSION",
  "CONNECTOR_TX_SENDER_VERSION",
] as const;

export const coprocessorVersionKeys = [
  "COPROCESSOR_DB_MIGRATION_VERSION",
  "COPROCESSOR_HOST_LISTENER_VERSION",
  "COPROCESSOR_GW_LISTENER_VERSION",
  "COPROCESSOR_TX_SENDER_VERSION",
  "COPROCESSOR_TFHE_WORKER_VERSION",
  "COPROCESSOR_ZKPROOF_WORKER_VERSION",
  "COPROCESSOR_SNS_WORKER_VERSION",
  "COPROCESSOR_CONSENSUS_DETECTOR_VERSION",
  "COPROCESSOR_UPGRADE_CONTROLLER_VERSION",
] as const;

export type MigrationVersions = {
  baseline: Record<string, string>;
  blueTag: string;
};

export const migrationVersions = (env: Env = process.env): MigrationVersions => {
  const releaseTag = env.RFC029_BASELINE_FHEVM_TAG?.trim() || "v0.14.0-7";
  const blueTag = required(env, "RFC029_BLUE_HOTFIX_TAG");
  const kmsCoreTag = env.RFC029_KMS_CORE_TAG?.trim() || "v0.14.0-1";

  return {
    blueTag,
    baseline: {
      GATEWAY_VERSION: releaseTag,
      HOST_VERSION: releaseTag,
      CORE_VERSION: kmsCoreTag,
      ...Object.fromEntries(connectorVersionKeys.map((key) => [key, releaseTag])),
      ...Object.fromEntries(coprocessorVersionKeys.map((key) => [key, blueTag])),
    },
  };
};

export const versionSources = [
  "rollout=v0.14-to-v0.15-gpu-key-migration",
  "migration=same-key-compressed-xof",
];
