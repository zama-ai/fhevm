import { getAddress, isAddress, isHexString } from 'ethers';
import { readFileSync } from 'fs';

export type KmsNode = {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
};

export type ThresholdName = 'publicDecryption' | 'userDecryption' | 'kmsGen' | 'mpc';

export type ProtocolConfigMigrationState = {
  contextId: string;
  nodes: KmsNode[];
  signers: string[];
  thresholds: Record<ThresholdName, string>;
};

export type KmsGenerationMigrationStateSnapshot = {
  prepKeygenCounter: string;
  keyCounter: string;
  crsCounter: string;
  activeKeyId: string;
  activeCrsId: string;
  activePrepKeygenId: string;
  activeKeyDigests: Array<{ keyType: string; digest: string }>;
  activeCrsDigest: string;
  keyConsensusTxSenders: string[];
  keyConsensusDigest: string;
  crsConsensusTxSenders: string[];
  crsConsensusDigest: string;
  prepKeygenConsensusTxSenders: string[];
  prepKeygenConsensusDigest: string;
  crsMaxBitLength: string;
  prepKeygenParamsType: string;
  crsParamsType: string;
  contextId: string;
};

export function loadProtocolConfigMigrationState(path: string): ProtocolConfigMigrationState {
  const migrationState = JSON.parse(readFileSync(path, 'utf8')) as Record<string, unknown>;
  const nodes = normalizeNodes(migrationState.kmsNodes, 'migration-state.kmsNodes');
  const thresholds = normalizeThresholds(migrationState.thresholds, 'migration-state.thresholds');

  return {
    contextId: normalizeUint(migrationState.contextId, 'migration-state.contextId'),
    nodes,
    // signerAddresses are already validated+checksummed by normalizeNodes; just sort.
    signers: nodes.map((node) => node.signerAddress).sort(),
    thresholds,
  };
}

export function loadKmsGenerationMigrationState(path: string): KmsGenerationMigrationStateSnapshot {
  const migrationState = JSON.parse(readFileSync(path, 'utf8')) as Record<string, unknown>;
  const snapshot = normalizeKmsGenerationMigrationState(
    migrationState.hostKmsGenerationMigrationState,
    'migration-state.hostKmsGenerationMigrationState',
  );
  if (migrationState.contextId !== undefined) {
    assertSame(
      'migration-state.hostKmsGenerationMigrationState.contextId',
      snapshot.contextId,
      normalizeUint(migrationState.contextId, 'migration-state.contextId'),
    );
  }
  return snapshot;
}

export function normalizeThresholds(value: unknown, path: string): Record<ThresholdName, string> {
  if (typeof value !== 'object' || value === null) {
    throw new Error(`${path} must be an object`);
  }
  const thresholds = value as Record<ThresholdName, unknown>;
  const threshold = (name: ThresholdName) => normalizeUint(thresholds[name], `${path}.${name}`);

  return {
    publicDecryption: threshold('publicDecryption'),
    userDecryption: threshold('userDecryption'),
    kmsGen: threshold('kmsGen'),
    mpc: threshold('mpc'),
  };
}

function normalizeAddressSequence(addresses: unknown, path: string): string[] {
  return requiredArray(addresses, path).map((address, index) => normalizeAddress(address, `${path}[${index}]`));
}

export function normalizeAddresses(addresses: unknown, path: string): string[] {
  return normalizeAddressSequence(addresses, path).sort();
}

export function normalizeNodes(nodes: unknown, path: string): KmsNode[] {
  return requiredArray(nodes, path)
    .map((node, index) => normalizeNode(node, `${path}[${index}]`))
    .sort(sortNodes);
}

export function normalizeKmsGenerationMigrationState(
  value: unknown,
  path: string,
): KmsGenerationMigrationStateSnapshot {
  const state = normalizeKmsGenerationStateObject(value, path);

  return {
    prepKeygenCounter: normalizeUint(state.prepKeygenCounter, `${path}.prepKeygenCounter`),
    keyCounter: normalizeUint(state.keyCounter, `${path}.keyCounter`),
    crsCounter: normalizeUint(state.crsCounter, `${path}.crsCounter`),
    activeKeyId: normalizeUint(state.activeKeyId, `${path}.activeKeyId`),
    activeCrsId: normalizeUint(state.activeCrsId, `${path}.activeCrsId`),
    activePrepKeygenId: normalizeUint(state.activePrepKeygenId, `${path}.activePrepKeygenId`),
    activeKeyDigests: normalizeKeyDigests(state.activeKeyDigests, `${path}.activeKeyDigests`),
    activeCrsDigest: normalizeBytes(state.activeCrsDigest, `${path}.activeCrsDigest`),
    keyConsensusTxSenders: normalizeAddressSequence(state.keyConsensusTxSenders, `${path}.keyConsensusTxSenders`),
    keyConsensusDigest: normalizeBytes(state.keyConsensusDigest, `${path}.keyConsensusDigest`),
    crsConsensusTxSenders: normalizeAddressSequence(state.crsConsensusTxSenders, `${path}.crsConsensusTxSenders`),
    crsConsensusDigest: normalizeBytes(state.crsConsensusDigest, `${path}.crsConsensusDigest`),
    prepKeygenConsensusTxSenders: normalizeAddressSequence(
      state.prepKeygenConsensusTxSenders,
      `${path}.prepKeygenConsensusTxSenders`,
    ),
    prepKeygenConsensusDigest: normalizeBytes(state.prepKeygenConsensusDigest, `${path}.prepKeygenConsensusDigest`),
    crsMaxBitLength: normalizeUint(state.crsMaxBitLength, `${path}.crsMaxBitLength`),
    prepKeygenParamsType: normalizeUint(state.prepKeygenParamsType, `${path}.prepKeygenParamsType`),
    crsParamsType: normalizeUint(state.crsParamsType, `${path}.crsParamsType`),
    contextId: normalizeUint(state.contextId, `${path}.contextId`),
  };
}

export function assertSame(name: string, actual: unknown, expected: unknown): void {
  const actualText = JSON.stringify(actual);
  const expectedText = JSON.stringify(expected);
  if (actualText !== expectedText) {
    throw new Error(`${name} mismatch:\n  actual:   ${actualText}\n  expected: ${expectedText}`);
  }
}

function requiredArray(value: unknown, path: string): unknown[] {
  if (!Array.isArray(value)) {
    throw new Error(`${path} must be an array`);
  }
  return value;
}

export function normalizeAddress(value: unknown, path: string): string {
  if (typeof value !== 'string' || !isAddress(value)) {
    throw new Error(`${path} must be an EVM address`);
  }
  return getAddress(value);
}

function normalizeNode(value: unknown, path: string): KmsNode {
  if (typeof value !== 'object' || value === null) {
    throw new Error(`${path} must be a KMS node object`);
  }
  const node = value as Record<string, unknown>;

  return {
    txSenderAddress: normalizeAddress(node.txSenderAddress ?? node['0'], `${path}.txSenderAddress`),
    signerAddress: normalizeAddress(node.signerAddress ?? node['1'], `${path}.signerAddress`),
    ipAddress: requireString(node.ipAddress ?? node['2'], `${path}.ipAddress`),
    storageUrl: requireString(node.storageUrl ?? node['3'], `${path}.storageUrl`),
  };
}

function normalizeKmsGenerationStateObject(
  value: unknown,
  path: string,
): Record<keyof KmsGenerationMigrationStateSnapshot, unknown> {
  const keys: Array<keyof KmsGenerationMigrationStateSnapshot> = [
    'prepKeygenCounter',
    'keyCounter',
    'crsCounter',
    'activeKeyId',
    'activeCrsId',
    'activePrepKeygenId',
    'activeKeyDigests',
    'activeCrsDigest',
    'keyConsensusTxSenders',
    'keyConsensusDigest',
    'crsConsensusTxSenders',
    'crsConsensusDigest',
    'prepKeygenConsensusTxSenders',
    'prepKeygenConsensusDigest',
    'crsMaxBitLength',
    'prepKeygenParamsType',
    'crsParamsType',
    'contextId',
  ];

  if (Array.isArray(value)) {
    if (value.length !== keys.length) {
      throw new Error(`${path} must have ${keys.length} entries; got ${value.length}`);
    }
    return Object.fromEntries(keys.map((key, index) => [key, value[index]])) as Record<
      keyof KmsGenerationMigrationStateSnapshot,
      unknown
    >;
  }

  if (typeof value !== 'object' || value === null) {
    throw new Error(`${path} must be a KMSGeneration migration state object or tuple`);
  }
  return value as Record<keyof KmsGenerationMigrationStateSnapshot, unknown>;
}

function normalizeKeyDigests(value: unknown, path: string): Array<{ keyType: string; digest: string }> {
  return requiredArray(value, path).map((digest, index) => normalizeKeyDigest(digest, `${path}[${index}]`));
}

function normalizeKeyDigest(value: unknown, path: string): { keyType: string; digest: string } {
  if (Array.isArray(value)) {
    if (value.length !== 2) {
      throw new Error(`${path} must be a key digest tuple [keyType, digest]`);
    }
    return {
      keyType: normalizeUint(value[0], `${path}.keyType`),
      digest: normalizeBytes(value[1], `${path}.digest`),
    };
  }
  if (typeof value !== 'object' || value === null) {
    throw new Error(`${path} must be a key digest object or tuple`);
  }
  const digest = value as Record<string, unknown>;
  return {
    keyType: normalizeUint(digest.keyType ?? digest['0'], `${path}.keyType`),
    digest: normalizeBytes(digest.digest ?? digest['1'], `${path}.digest`),
  };
}

function normalizeUint(value: unknown, path: string): string {
  if (typeof value !== 'string' && typeof value !== 'number' && typeof value !== 'bigint') {
    throw new Error(`${path} must be a string, number, or bigint`);
  }
  return BigInt(value).toString();
}

function normalizeBytes(value: unknown, path: string): string {
  if (typeof value !== 'string' || !isHexString(value)) {
    throw new Error(`${path} must be a hex string`);
  }
  return value.toLowerCase();
}

function requireString(value: unknown, path: string): string {
  if (typeof value !== 'string') {
    throw new Error(`${path} must be a string`);
  }
  return value;
}

function sortNodes(a: KmsNode, b: KmsNode): number {
  return a.txSenderAddress.localeCompare(b.txSenderAddress);
}
