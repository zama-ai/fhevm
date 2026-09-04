import { isAddress } from 'ethers';

import { type CanonicalSnapshot, type KmsNode } from '../protocolConfigMirror';
import { makeEnvHelpers } from './envSnapshot';
import { formatError } from './formatError';
import { getRequiredEnvVar } from './loadVariables';

// The context id, epoch id, node set and thresholds become initializeFromCanonical calldata, so the task
// checks them in full. The chain id and block number are provenance, and the task parses them as decimal
// strings. The block hash and the address are provenance that the task only checks for presence, then prints.
const CANONICAL_SNAPSHOT_ENV_KEYS = [
  'CANONICAL_CHAIN_ID',
  'CANONICAL_PROTOCOL_CONFIG_ADDRESS',
  'CANONICAL_BLOCK_NUMBER',
  'CANONICAL_BLOCK_HASH',
  'CANONICAL_KMS_CONTEXT_ID',
  'CANONICAL_EPOCH_ID',
  'CANONICAL_KMS_NODES',
  'CANONICAL_KMS_THRESHOLDS',
] as const;

type CanonicalSnapshotEnvKey = (typeof CANONICAL_SNAPSHOT_ENV_KEYS)[number];
export type CanonicalSnapshotEnv = Record<CanonicalSnapshotEnvKey, string>;

// Every check below names the environment variable it rejected.
function requireDecimal(name: CanonicalSnapshotEnvKey): bigint {
  const raw = getRequiredEnvVar(name);
  if (!/^\d+$/.test(raw)) {
    throw new Error(`"${name}" env variable must be a decimal string, got ${JSON.stringify(raw)}.`);
  }
  return BigInt(raw);
}

// A bare JSON.parse failure names neither the field nor the variable, so name the variable here.
function parseJson(name: CanonicalSnapshotEnvKey): unknown {
  const raw = getRequiredEnvVar(name);
  try {
    return JSON.parse(raw);
  } catch (err) {
    throw new Error(`"${name}" env variable is not valid JSON (${formatError(err)}).`);
  }
}

function requireKmsNodes(): KmsNode[] {
  const nodes = parseJson('CANONICAL_KMS_NODES');
  if (!Array.isArray(nodes) || nodes.length === 0) {
    throw new Error('"CANONICAL_KMS_NODES" env variable must hold a non-empty JSON array.');
  }
  for (const [index, node] of nodes.entries()) {
    // Addresses get baked into the initializeFromCanonical calldata, so reject a malformed one here
    // rather than letting ABI encoding fail with an opaque error.
    for (const field of ['txSenderAddress', 'signerAddress'] as const) {
      if (!isAddress(node?.[field])) {
        throw new Error(
          `"CANONICAL_KMS_NODES" env variable entry ${index} field "${field}" must be a valid address, got ${JSON.stringify(node?.[field])}.`,
        );
      }
    }
    for (const field of ['ipAddress', 'storageUrl'] as const) {
      if (typeof node?.[field] !== 'string') {
        throw new Error(`"CANONICAL_KMS_NODES" env variable entry ${index} field "${field}" is missing.`);
      }
    }
  }
  return nodes as KmsNode[];
}

function requireThresholds(): CanonicalSnapshot['thresholds'] {
  const parsed = parseJson('CANONICAL_KMS_THRESHOLDS');
  if (typeof parsed !== 'object' || parsed === null) {
    throw new Error('"CANONICAL_KMS_THRESHOLDS" env variable must hold a JSON object.');
  }
  const thresholds = parsed as Record<string, unknown>;
  const requireField = (field: keyof CanonicalSnapshot['thresholds']) => {
    const value = thresholds[field];
    if (typeof value !== 'string' || !/^\d+$/.test(value)) {
      throw new Error(
        `"CANONICAL_KMS_THRESHOLDS" env variable field "${field}" must be a decimal string, got ${JSON.stringify(value)}.`,
      );
    }
    return BigInt(value);
  };
  return {
    publicDecryption: requireField('publicDecryption'),
    userDecryption: requireField('userDecryption'),
    kmsGen: requireField('kmsGen'),
    mpc: requireField('mpc'),
  };
}

// Reads the reviewed canonical snapshot from the environment. This is the only way to configure the
// mirror.
export function readCanonicalSnapshotFromEnv(): CanonicalSnapshot {
  // Read in the order the keys are declared, so a wholly unset environment reports the first key.
  return {
    canonicalChainId: requireDecimal('CANONICAL_CHAIN_ID'),
    protocolConfigAddress: getRequiredEnvVar('CANONICAL_PROTOCOL_CONFIG_ADDRESS'),
    blockNumber: Number(requireDecimal('CANONICAL_BLOCK_NUMBER')),
    blockHash: getRequiredEnvVar('CANONICAL_BLOCK_HASH'),
    currentKmsContextId: requireDecimal('CANONICAL_KMS_CONTEXT_ID'),
    currentEpochId: requireDecimal('CANONICAL_EPOCH_ID'),
    kmsNodes: requireKmsNodes(),
    thresholds: requireThresholds(),
  };
}

// Serializes a snapshot into the flat KEY=value map the deploy tasks read, with bigints as decimal
// strings because an environment variable carries a string. task:exportCanonicalProtocolConfig writes
// it. The return type is keyed on the same tuple the tasks read, so a renamed key fails to compile.
export function buildCanonicalSnapshotEnv(snapshot: CanonicalSnapshot): CanonicalSnapshotEnv {
  return {
    CANONICAL_CHAIN_ID: snapshot.canonicalChainId.toString(),
    CANONICAL_PROTOCOL_CONFIG_ADDRESS: snapshot.protocolConfigAddress,
    CANONICAL_BLOCK_NUMBER: String(snapshot.blockNumber),
    CANONICAL_BLOCK_HASH: snapshot.blockHash,
    CANONICAL_KMS_CONTEXT_ID: snapshot.currentKmsContextId.toString(),
    CANONICAL_EPOCH_ID: snapshot.currentEpochId.toString(),
    CANONICAL_KMS_NODES: JSON.stringify(snapshot.kmsNodes),
    CANONICAL_KMS_THRESHOLDS: JSON.stringify({
      publicDecryption: snapshot.thresholds.publicDecryption.toString(),
      userDecryption: snapshot.thresholds.userDecryption.toString(),
      kmsGen: snapshot.thresholds.kmsGen.toString(),
      mpc: snapshot.thresholds.mpc.toString(),
    }),
  };
}

export const {
  apply: applyProtocolConfigCanonicalEnv,
  snapshot: snapshotProtocolConfigCanonicalEnv,
  restore: restoreProtocolConfigCanonicalEnv,
} = makeEnvHelpers(CANONICAL_SNAPSHOT_ENV_KEYS);
