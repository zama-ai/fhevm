type Env = Record<string, string>;

// RFC-029 one-time compressed-key migration rollout. Single boot, no
// version ladder: the "phases" are protocol actions (baseline -> migration
// keygen + publication -> cutover scheduling -> boundary-straddling
// traffic), all on one coherent component set.
//
// The migration surface only exists on components built from the RFC-029
// branch, so every component under test defaults to a branch-built tag,
// overridable per component through the environment (RFC029_VERSION for
// the whole set). KMS core needs keygen-from-existing
// (KeySetConfig::UseExisting + CompressedAll + copy_compressed_key_to_original),
// implemented as of v0.13.20.
const featureVersion = process.env.RFC029_VERSION ?? "latest";
const coreVersion = process.env.RFC029_CORE_VERSION ?? "v0.13.20";
const relayerSdkVersion = "0.4.2";

export const scenario = "multi-chain";

export const versions = {
  RELAYER_VERSION: featureVersion,
  RELAYER_MIGRATE_VERSION: featureVersion,
  GATEWAY_VERSION: featureVersion,
  HOST_VERSION: featureVersion,
  CORE_VERSION: coreVersion,
  CONNECTOR_DB_MIGRATION_VERSION: featureVersion,
  CONNECTOR_GW_LISTENER_VERSION: featureVersion,
  CONNECTOR_KMS_WORKER_VERSION: featureVersion,
  CONNECTOR_TX_SENDER_VERSION: featureVersion,
  COPROCESSOR_DB_MIGRATION_VERSION: featureVersion,
  COPROCESSOR_HOST_LISTENER_VERSION: featureVersion,
  COPROCESSOR_GW_LISTENER_VERSION: featureVersion,
  COPROCESSOR_TX_SENDER_VERSION: featureVersion,
  COPROCESSOR_TFHE_WORKER_VERSION: featureVersion,
  COPROCESSOR_ZKPROOF_WORKER_VERSION: featureVersion,
  COPROCESSOR_SNS_WORKER_VERSION: featureVersion,
  LISTENER_CORE_VERSION: featureVersion,
  TEST_SUITE_VERSION: featureVersion,
  RELAYER_SDK_VERSION: relayerSdkVersion,
} satisfies Env;

export const versionSources = [
  "rollout=rfc029-compressed-key-cutover",
  `feature=${featureVersion}`,
  `kms-core=${coreVersion} (keygen-from-existing)`,
  "follows=RFC-029 (tech-spec#478 as amended by tech-spec#485)",
];
