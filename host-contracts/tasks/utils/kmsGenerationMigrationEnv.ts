import { makeEnvHelpers } from './envSnapshot';
import { getRequiredEnvVar } from './loadVariables';

const KMS_GENERATION_MIGRATION_ENV_KEYS = [
  'MIGRATION_PREP_KEYGEN_COUNTER',
  'MIGRATION_KEY_COUNTER',
  'MIGRATION_CRS_COUNTER',
  'MIGRATION_ACTIVE_KEY_ID',
  'MIGRATION_ACTIVE_CRS_ID',
  'MIGRATION_ACTIVE_PREP_KEYGEN_ID',
  'MIGRATION_ACTIVE_KEY_DIGESTS',
  'MIGRATION_ACTIVE_CRS_DIGEST',
  'MIGRATION_KEY_CONSENSUS_TX_SENDERS',
  'MIGRATION_KEY_CONSENSUS_DIGEST',
  'MIGRATION_CRS_CONSENSUS_TX_SENDERS',
  'MIGRATION_CRS_CONSENSUS_DIGEST',
  'MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS',
  'MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST',
  'MIGRATION_CRS_MAX_BIT_LENGTH',
  'MIGRATION_PREP_KEYGEN_PARAMS_TYPE',
  'MIGRATION_CRS_PARAMS_TYPE',
  'MIGRATION_CONTEXT_ID',
] as const;

type KmsGenerationMigrationEnvKey = (typeof KMS_GENERATION_MIGRATION_ENV_KEYS)[number];
export type KmsGenerationMigrationEnv = Record<KmsGenerationMigrationEnvKey, string>;
export type KmsGenerationMigrationEnvSnapshot = Partial<KmsGenerationMigrationEnv>;
export type KmsGenerationMigrationState = {
  prepKeygenCounter: bigint;
  keyCounter: bigint;
  crsCounter: bigint;
  activeKeyId: bigint;
  activeCrsId: bigint;
  activePrepKeygenId: bigint;
  activeKeyDigests: Array<{ keyType: number; digest: string }>;
  activeCrsDigest: string;
  keyConsensusTxSenders: string[];
  keyConsensusDigest: string;
  crsConsensusTxSenders: string[];
  crsConsensusDigest: string;
  prepKeygenConsensusTxSenders: string[];
  prepKeygenConsensusDigest: string;
  crsMaxBitLength: bigint;
  prepKeygenParamsType: number;
  crsParamsType: number;
  contextId: bigint;
};

function parseAddressList(envVarName: KmsGenerationMigrationEnvKey): string[] {
  const raw = getRequiredEnvVar(envVarName);
  return raw.split(',').map((address) => address.trim());
}

export function buildKMSGenerationInitializeFromMigrationArgs(): [KmsGenerationMigrationState] {
  return [
    {
      prepKeygenCounter: BigInt(getRequiredEnvVar('MIGRATION_PREP_KEYGEN_COUNTER')),
      keyCounter: BigInt(getRequiredEnvVar('MIGRATION_KEY_COUNTER')),
      crsCounter: BigInt(getRequiredEnvVar('MIGRATION_CRS_COUNTER')),
      activeKeyId: BigInt(getRequiredEnvVar('MIGRATION_ACTIVE_KEY_ID')),
      activeCrsId: BigInt(getRequiredEnvVar('MIGRATION_ACTIVE_CRS_ID')),
      activePrepKeygenId: BigInt(getRequiredEnvVar('MIGRATION_ACTIVE_PREP_KEYGEN_ID')),
      activeKeyDigests: JSON.parse(getRequiredEnvVar('MIGRATION_ACTIVE_KEY_DIGESTS')),
      activeCrsDigest: getRequiredEnvVar('MIGRATION_ACTIVE_CRS_DIGEST'),
      keyConsensusTxSenders: parseAddressList('MIGRATION_KEY_CONSENSUS_TX_SENDERS'),
      keyConsensusDigest: getRequiredEnvVar('MIGRATION_KEY_CONSENSUS_DIGEST'),
      crsConsensusTxSenders: parseAddressList('MIGRATION_CRS_CONSENSUS_TX_SENDERS'),
      crsConsensusDigest: getRequiredEnvVar('MIGRATION_CRS_CONSENSUS_DIGEST'),
      prepKeygenConsensusTxSenders: parseAddressList('MIGRATION_PREP_KEYGEN_CONSENSUS_TX_SENDERS'),
      prepKeygenConsensusDigest: getRequiredEnvVar('MIGRATION_PREP_KEYGEN_CONSENSUS_DIGEST'),
      crsMaxBitLength: BigInt(getRequiredEnvVar('MIGRATION_CRS_MAX_BIT_LENGTH')),
      prepKeygenParamsType: +getRequiredEnvVar('MIGRATION_PREP_KEYGEN_PARAMS_TYPE'),
      crsParamsType: +getRequiredEnvVar('MIGRATION_CRS_PARAMS_TYPE'),
      contextId: BigInt(getRequiredEnvVar('MIGRATION_CONTEXT_ID')),
    },
  ];
}

export const {
  apply: applyKmsGenerationMigrationEnv,
  snapshot: snapshotKmsGenerationMigrationEnv,
  restore: restoreKmsGenerationMigrationEnv,
} = makeEnvHelpers(KMS_GENERATION_MIGRATION_ENV_KEYS);
