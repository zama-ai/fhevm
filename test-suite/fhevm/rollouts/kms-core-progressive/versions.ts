type Env = Record<string, string>;

export const scenario = "four-party-threshold-kms";

const release = "v0.13.0";

// Keep the generic example within one compatible fhEVM release family. The
// v0.13.10 core belongs to the v0.12 stack and is covered by the incident
// runbook; v0.13.20 -> v0.13.22 is a KMS-only patch rollout on fhEVM v0.13.
export const from = {
  RELAYER_VERSION: release,
  RELAYER_MIGRATE_VERSION: release,
  GATEWAY_VERSION: release,
  HOST_VERSION: release,
  CORE_VERSION: "v0.13.20",
  CONNECTOR_DB_MIGRATION_VERSION: release,
  CONNECTOR_GW_LISTENER_VERSION: release,
  CONNECTOR_KMS_WORKER_VERSION: release,
  CONNECTOR_TX_SENDER_VERSION: release,
  COPROCESSOR_DB_MIGRATION_VERSION: release,
  COPROCESSOR_HOST_LISTENER_VERSION: release,
  COPROCESSOR_GW_LISTENER_VERSION: release,
  COPROCESSOR_TX_SENDER_VERSION: release,
  COPROCESSOR_TFHE_WORKER_VERSION: release,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: release,
  COPROCESSOR_SNS_WORKER_VERSION: release,
  LISTENER_CORE_VERSION: release,
  TEST_SUITE_VERSION: release,
  RELAYER_SDK_VERSION: "0.4.2",
} satisfies Env;

export const to = { ...from, CORE_VERSION: "v0.13.22" } satisfies Env;

export const versionSources = [
  "rollout=kms-core-progressive",
  "kms-core=v0.13.20->v0.13.22",
  "other-components=v0.13.0",
  "relayer-sdk=0.4.2",
];
