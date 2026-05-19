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

export type ProtocolConfigMigrationKmsNodeParams = ProtocolConfigMigrationKmsNode & {
  partyId: number;
  mpcIdentity: string;
  caCert: string;
  storagePrefix: string;
};

export type ProtocolConfigMigrationThresholds = {
  publicDecryption: number;
  userDecryption: number;
  kmsGen: number;
  mpc: number;
};

export function buildProtocolConfigInitializeFromMigrationArgs(): [
  bigint,
  ProtocolConfigMigrationKmsNodeParams[],
  ProtocolConfigMigrationThresholds,
] {
  const migrationContextId = BigInt(getRequiredEnvVar('MIGRATION_CONTEXT_ID'));
  // MIGRATION_KMS_NODES may carry only the four stored KmsNode fields (gateway reconstruction reads
  // on-chain storage, which does not retain MPC metadata), so synthesize the missing params.
  const rawNodes: Array<ProtocolConfigMigrationKmsNode & Partial<ProtocolConfigMigrationKmsNodeParams>> = JSON.parse(
    getRequiredEnvVar('MIGRATION_KMS_NODES'),
  );
  const kmsNodeParams: ProtocolConfigMigrationKmsNodeParams[] = rawNodes.map((node, idx) => ({
    txSenderAddress: node.txSenderAddress,
    signerAddress: node.signerAddress,
    ipAddress: node.ipAddress,
    storageUrl: node.storageUrl,
    partyId: node.partyId ?? idx,
    mpcIdentity: node.mpcIdentity ?? node.ipAddress,
    caCert: node.caCert ?? '0x',
    storagePrefix: node.storagePrefix ?? '',
  }));
  const thresholds: ProtocolConfigMigrationThresholds = JSON.parse(getRequiredEnvVar('MIGRATION_KMS_THRESHOLDS'));
  return [migrationContextId, kmsNodeParams, thresholds];
}

export const {
  apply: applyProtocolConfigMigrationEnv,
  snapshot: snapshotProtocolConfigMigrationEnv,
  restore: restoreProtocolConfigMigrationEnv,
} = makeEnvHelpers(PROTOCOL_CONFIG_MIGRATION_ENV_KEYS);
