import { makeEnvHelpers } from './envSnapshot';
import { getRequiredEnvVar } from './loadVariables';

const PROTOCOL_CONFIG_MIGRATION_ENV_KEYS = [
  'MIGRATION_CONTEXT_ID',
  'MIGRATION_KMS_NODES',
  'MIGRATION_KMS_THRESHOLDS',
] as const;

type ProtocolConfigMigrationEnvKey = (typeof PROTOCOL_CONFIG_MIGRATION_ENV_KEYS)[number];
export type ProtocolConfigMigrationEnv = Record<ProtocolConfigMigrationEnvKey, string>;
export type ProtocolConfigMigrationEnvSnapshot = Partial<ProtocolConfigMigrationEnv>;

export type ProtocolConfigMigrationKmsNode = {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
};

export type ProtocolConfigMigrationThresholds = {
  publicDecryption: number;
  userDecryption: number;
  kmsGen: number;
  mpc: number;
};

export function buildProtocolConfigInitializeFromMigrationArgs(): [
  bigint,
  ProtocolConfigMigrationKmsNode[],
  ProtocolConfigMigrationThresholds,
] {
  const migrationContextId = BigInt(getRequiredEnvVar('MIGRATION_CONTEXT_ID'));
  const kmsNodes: ProtocolConfigMigrationKmsNode[] = JSON.parse(getRequiredEnvVar('MIGRATION_KMS_NODES'));
  const thresholds: ProtocolConfigMigrationThresholds = JSON.parse(getRequiredEnvVar('MIGRATION_KMS_THRESHOLDS'));
  return [migrationContextId, kmsNodes, thresholds];
}

export const {
  apply: applyProtocolConfigMigrationEnv,
  snapshot: snapshotProtocolConfigMigrationEnv,
  restore: restoreProtocolConfigMigrationEnv,
} = makeEnvHelpers(PROTOCOL_CONFIG_MIGRATION_ENV_KEYS);
