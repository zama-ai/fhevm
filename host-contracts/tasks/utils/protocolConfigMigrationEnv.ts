import { getRequiredEnvVar } from './loadVariables';

export const PROTOCOL_CONFIG_MIGRATION_ENV_KEYS = [
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

export type ProtocolConfigMigrationState = {
  migrationContextId: bigint;
  kmsNodes: ProtocolConfigMigrationKmsNode[];
  thresholds: ProtocolConfigMigrationThresholds;
};

export function buildProtocolConfigMigrationStateFromEnv(): ProtocolConfigMigrationState {
  const migrationContextId = BigInt(getRequiredEnvVar('MIGRATION_CONTEXT_ID'));
  const kmsNodes: ProtocolConfigMigrationKmsNode[] = JSON.parse(getRequiredEnvVar('MIGRATION_KMS_NODES'));
  const thresholds: ProtocolConfigMigrationThresholds = JSON.parse(getRequiredEnvVar('MIGRATION_KMS_THRESHOLDS'));
  return { migrationContextId, kmsNodes, thresholds };
}

export function applyProtocolConfigMigrationEnv(env: ProtocolConfigMigrationEnv): void {
  for (const key of PROTOCOL_CONFIG_MIGRATION_ENV_KEYS) {
    process.env[key] = env[key];
  }
}

export function snapshotProtocolConfigMigrationEnv(): ProtocolConfigMigrationEnvSnapshot {
  const snapshot: ProtocolConfigMigrationEnvSnapshot = {};
  for (const key of PROTOCOL_CONFIG_MIGRATION_ENV_KEYS) {
    const value = process.env[key];
    if (value !== undefined) {
      snapshot[key] = value;
    }
  }
  return snapshot;
}

export function restoreProtocolConfigMigrationEnv(snapshot: ProtocolConfigMigrationEnvSnapshot): void {
  for (const key of PROTOCOL_CONFIG_MIGRATION_ENV_KEYS) {
    const value = snapshot[key];
    if (value === undefined) {
      delete process.env[key];
    } else {
      process.env[key] = value;
    }
  }
}
