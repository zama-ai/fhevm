type Env = Record<string, string>;

export const scenario = "four-party-threshold-kms";

const coprocessor = "v0.11.0";
const connector = "v0.12.0";

// Mainnet cutover state on 2026-07-13. Contracts and the relayer had already
// moved; coprocessor services had not. The serving KMS cores were then replaced
// one operator at a time while the connector stayed on v0.12.0.
export const from = {
  RELAYER_VERSION: "v0.11.1",
  RELAYER_MIGRATE_VERSION: "v0.11.0",
  GATEWAY_VERSION: "v0.12.1",
  HOST_VERSION: "v0.12.1",
  CORE_VERSION: "v0.13.3",
  CONNECTOR_DB_MIGRATION_VERSION: connector,
  CONNECTOR_GW_LISTENER_VERSION: connector,
  CONNECTOR_KMS_WORKER_VERSION: connector,
  CONNECTOR_TX_SENDER_VERSION: connector,
  COPROCESSOR_DB_MIGRATION_VERSION: coprocessor,
  COPROCESSOR_HOST_LISTENER_VERSION: coprocessor,
  COPROCESSOR_GW_LISTENER_VERSION: coprocessor,
  COPROCESSOR_TX_SENDER_VERSION: coprocessor,
  COPROCESSOR_TFHE_WORKER_VERSION: coprocessor,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: coprocessor,
  COPROCESSOR_SNS_WORKER_VERSION: coprocessor,
  LISTENER_CORE_VERSION: "v0.13.0",
  TEST_SUITE_VERSION: "v0.13.0",
  RELAYER_SDK_VERSION: "0.4.2",
} satisfies Env;

export const to = { ...from, CORE_VERSION: "v0.13.10" } satisfies Env;

export const versionSources = [
  "rollout=mainnet-v0.11-to-v0.12-kms",
  "incident=2026-07-13-mainnet-kms-rollout",
  "kms-core=v0.13.3->v0.13.10",
  "kms-connector=v0.12.0",
  "relayer-sdk=0.4.2",
];
