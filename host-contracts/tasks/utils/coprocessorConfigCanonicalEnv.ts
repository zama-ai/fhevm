import { isAddress } from 'ethers';

import { type CanonicalCoprocessorConfigSnapshot, type Coprocessor } from '../coprocessorConfigMirror';
import { makeEnvHelpers } from './envSnapshot';
import { formatError } from './formatError';
import { getRequiredEnvVar } from './loadVariables';

// The context id, coprocessor set and threshold become initializeFromCanonical calldata, so the task
// checks them in full. The chain id and block number are provenance, parsed as decimal strings. The
// block hash and the address are provenance that the task only checks for presence, then prints.
// The keys differ from the ProtocolConfig snapshot's, so both snapshots can be loaded together.
const CANONICAL_COPROCESSOR_CONFIG_SNAPSHOT_ENV_KEYS = [
  'CANONICAL_COPROCESSOR_CONFIG_CHAIN_ID',
  'CANONICAL_COPROCESSOR_CONFIG_ADDRESS',
  'CANONICAL_COPROCESSOR_CONFIG_BLOCK_NUMBER',
  'CANONICAL_COPROCESSOR_CONFIG_BLOCK_HASH',
  'CANONICAL_COPROCESSOR_CONTEXT_ID',
  'CANONICAL_COPROCESSORS',
  'CANONICAL_COPROCESSOR_MAJORITY_THRESHOLD',
] as const;

type CanonicalCoprocessorConfigSnapshotEnvKey = (typeof CANONICAL_COPROCESSOR_CONFIG_SNAPSHOT_ENV_KEYS)[number];
export type CanonicalCoprocessorConfigSnapshotEnv = Record<CanonicalCoprocessorConfigSnapshotEnvKey, string>;

function requireDecimal(name: CanonicalCoprocessorConfigSnapshotEnvKey): bigint {
  const raw = getRequiredEnvVar(name);
  if (!/^\d+$/.test(raw)) {
    throw new Error(`"${name}" env variable must be a decimal string, got ${JSON.stringify(raw)}.`);
  }
  return BigInt(raw);
}

function requireCoprocessors(): Coprocessor[] {
  const raw = getRequiredEnvVar('CANONICAL_COPROCESSORS');
  let coprocessors: unknown;
  try {
    coprocessors = JSON.parse(raw);
  } catch (err) {
    throw new Error(`"CANONICAL_COPROCESSORS" env variable is not valid JSON (${formatError(err)}).`);
  }
  if (!Array.isArray(coprocessors) || coprocessors.length === 0) {
    throw new Error('"CANONICAL_COPROCESSORS" env variable must hold a non-empty JSON array.');
  }
  for (const [index, coprocessor] of coprocessors.entries()) {
    for (const field of ['txSenderAddress', 'signerAddress'] as const) {
      if (!isAddress(coprocessor?.[field])) {
        throw new Error(
          `"CANONICAL_COPROCESSORS" env variable entry ${index} field "${field}" must be a valid address, got ${JSON.stringify(coprocessor?.[field])}.`,
        );
      }
    }
    for (const field of ['storageUrl', 'prfEndpointUrl'] as const) {
      if (typeof coprocessor?.[field] !== 'string') {
        throw new Error(`"CANONICAL_COPROCESSORS" env variable entry ${index} field "${field}" is missing.`);
      }
    }
  }
  return coprocessors as Coprocessor[];
}

// Reads the reviewed canonical CoprocessorConfig snapshot from the environment. This is the only way to
// configure the replica deployment.
export function readCanonicalCoprocessorConfigSnapshotFromEnv(): CanonicalCoprocessorConfigSnapshot {
  // Read in the order the keys are declared, so a wholly unset environment reports the first key.
  return {
    canonicalChainId: requireDecimal('CANONICAL_COPROCESSOR_CONFIG_CHAIN_ID'),
    coprocessorConfigAddress: getRequiredEnvVar('CANONICAL_COPROCESSOR_CONFIG_ADDRESS'),
    blockNumber: Number(requireDecimal('CANONICAL_COPROCESSOR_CONFIG_BLOCK_NUMBER')),
    blockHash: getRequiredEnvVar('CANONICAL_COPROCESSOR_CONFIG_BLOCK_HASH'),
    currentCoprocessorContextId: requireDecimal('CANONICAL_COPROCESSOR_CONTEXT_ID'),
    coprocessors: requireCoprocessors(),
    majorityThreshold: requireDecimal('CANONICAL_COPROCESSOR_MAJORITY_THRESHOLD'),
  };
}

// Serializes a snapshot into the flat KEY=value map the deploy tasks read, with bigints as decimal
// strings. task:exportCanonicalCoprocessorConfig writes it.
export function buildCanonicalCoprocessorConfigSnapshotEnv(
  snapshot: CanonicalCoprocessorConfigSnapshot,
): CanonicalCoprocessorConfigSnapshotEnv {
  return {
    CANONICAL_COPROCESSOR_CONFIG_CHAIN_ID: snapshot.canonicalChainId.toString(),
    CANONICAL_COPROCESSOR_CONFIG_ADDRESS: snapshot.coprocessorConfigAddress,
    CANONICAL_COPROCESSOR_CONFIG_BLOCK_NUMBER: String(snapshot.blockNumber),
    CANONICAL_COPROCESSOR_CONFIG_BLOCK_HASH: snapshot.blockHash,
    CANONICAL_COPROCESSOR_CONTEXT_ID: snapshot.currentCoprocessorContextId.toString(),
    CANONICAL_COPROCESSORS: JSON.stringify(snapshot.coprocessors),
    CANONICAL_COPROCESSOR_MAJORITY_THRESHOLD: snapshot.majorityThreshold.toString(),
  };
}

export const {
  apply: applyCoprocessorConfigCanonicalEnv,
  snapshot: snapshotCoprocessorConfigCanonicalEnv,
  restore: restoreCoprocessorConfigCanonicalEnv,
} = makeEnvHelpers(CANONICAL_COPROCESSOR_CONFIG_SNAPSHOT_ENV_KEYS);
