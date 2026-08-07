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

export const kmsOperatorVersionKeys = ["CORE_VERSION", ...connectorVersionKeys] as const;

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
  baselineTag: string;
  blueTag: string;
};

const requiredVersion = (versions: Record<string, string>, key: string): string => {
  const value = versions[key]?.trim();
  if (!value) throw new Error(`${key} is missing from the resolved target lock`);
  return value;
};

export const migrationPhaseVersions = (
  baseline: Record<string, string>,
  target: Record<string, string>,
  blueTag: string,
) => {
  const contract: Record<string, string> = {
    ...baseline,
    HOST_VERSION: requiredVersion(target, "HOST_VERSION"),
  };
  const connector: Record<string, string> = {
    ...contract,
    CORE_VERSION: requiredVersion(target, "CORE_VERSION"),
    ...Object.fromEntries(connectorVersionKeys.map((key) => [key, requiredVersion(target, key)])),
  };
  const blue: Record<string, string> = {
    ...connector,
    ...Object.fromEntries(coprocessorVersionKeys.map((key) => [key, blueTag])),
  };
  return { blue, contract, connector };
};

export const migrationVersions = (env: Env = process.env): MigrationVersions => {
  const releaseTag = env.RFC029_BASELINE_FHEVM_TAG?.trim() || "v0.14.0-10";
  // v0.14.0-10 did not publish a host-contracts image; that component remained on v0.14.0-9.
  const hostTag = env.RFC029_BASELINE_HOST_TAG?.trim() || "v0.14.0-9";
  const requestedBlueTag = required(env, "RFC029_BLUE_TAG");
  if (!/^(?:v0\.15\.0-\d+|[0-9a-f]{7}|[0-9a-f]{40})$/i.test(requestedBlueTag)) {
    throw new Error("RFC029_BLUE_TAG must be a v0.15.0-N release tag or an exact commit SHA");
  }
  const blueTag = /^[0-9a-f]{40}$/i.test(requestedBlueTag)
    ? requestedBlueTag.slice(0, 7)
    : requestedBlueTag;
  const kmsCoreTag = env.RFC029_KMS_CORE_TAG?.trim() || "v0.14.0-1";

  return {
    baselineTag: releaseTag,
    blueTag,
    baseline: {
      GATEWAY_VERSION: releaseTag,
      HOST_VERSION: hostTag,
      CORE_VERSION: kmsCoreTag,
      ...Object.fromEntries(connectorVersionKeys.map((key) => [key, releaseTag])),
      ...Object.fromEntries(coprocessorVersionKeys.map((key) => [key, releaseTag])),
    },
  };
};

export const versionSources = [
  "rollout=v0.15-to-v0.15.1-gpu-key-migration",
  "migration=same-key-compressed-xof",
];
