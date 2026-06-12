import type { Provider } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig } from '../types';
import { formatError } from './utils/formatError';
import { getRequiredEnvVar } from './utils/loadVariables';

export type KmsNode = {
  txSenderAddress: string;
  signerAddress: string;
  ipAddress: string;
  storageUrl: string;
};

export type KmsThresholds = {
  publicDecryption: bigint;
  userDecryption: bigint;
  kmsGen: bigint;
  mpc: bigint;
};

export type CanonicalSnapshot = {
  currentContextId: bigint;
  kmsNodes: KmsNode[];
  thresholds: KmsThresholds;
  canonicalChainId: bigint;
  canonicalBlockTag: number;
};

// Reads the canonical ProtocolConfig's current KMS context, pinned to one block. Shared by the
// secondary mirror deploy and the export task so both seed from the exact same read. Pass blockTag
// to pin to a historical block (the export artifact's blockNumber) so a DAO signer can reproduce a
// snapshot byte-for-byte even after a later context rotation; omit it to read the latest block.
export async function readCanonicalSnapshot(
  hre: HardhatRuntimeEnvironment,
  options: { canonicalProvider: Provider; canonicalProtocolConfigAddress: string; blockTag?: number },
): Promise<CanonicalSnapshot> {
  const { ethers } = hre;
  const { canonicalProvider, canonicalProtocolConfigAddress, blockTag } = options;

  const canonicalProtocolConfigBase = await ethers.getContractAt('ProtocolConfig', canonicalProtocolConfigAddress);
  const canonicalProtocolConfig = canonicalProtocolConfigBase.connect(canonicalProvider) as ProtocolConfig;

  let canonicalVersion: string;
  try {
    canonicalVersion = await canonicalProtocolConfig.getVersion();
  } catch (err) {
    throw new Error(
      `Canonical ProtocolConfig identity check failed: contract at ${canonicalProtocolConfigAddress} does not expose getVersion() (${formatError(err)}).`,
    );
  }
  if (!/^ProtocolConfig v\d/.test(canonicalVersion)) {
    throw new Error(
      `Canonical ProtocolConfig identity check failed: contract at ${canonicalProtocolConfigAddress} reports version "${canonicalVersion}"; expected "ProtocolConfig v<n>...".`,
    );
  }

  let canonicalChainId: bigint;
  let canonicalBlockTag: number;
  try {
    canonicalChainId = (await canonicalProvider.getNetwork()).chainId;
    canonicalBlockTag = blockTag ?? (await canonicalProvider.getBlockNumber());
  } catch (err) {
    throw new Error(`Canonical RPC handshake failed (${formatError(err)}).`);
  }
  const at = { blockTag: canonicalBlockTag };

  const currentContextId: bigint = await canonicalProtocolConfig.getCurrentKmsContextId(at);
  if (currentContextId === 0n) {
    throw new Error(
      `Canonical ProtocolConfig at ${canonicalProtocolConfigAddress} has no active KMS context (currentKmsContextId=0); cannot mirror.`,
    );
  }
  const isCurrentContextValid: boolean = await canonicalProtocolConfig.isValidKmsContext(currentContextId, at);
  if (!isCurrentContextValid) {
    throw new Error(
      `Canonical ProtocolConfig's current context ${currentContextId} is destroyed; cannot mirror a destroyed context.`,
    );
  }

  const [rawNodes, publicDecryption, userDecryption, kmsGen, mpc] = await Promise.all([
    canonicalProtocolConfig.getKmsNodesForContext(currentContextId, at),
    canonicalProtocolConfig.getPublicDecryptionThresholdForContext(currentContextId, at),
    canonicalProtocolConfig.getUserDecryptionThresholdForContext(currentContextId, at),
    canonicalProtocolConfig.getKmsGenThresholdForContext(currentContextId, at),
    canonicalProtocolConfig.getMpcThresholdForContext(currentContextId, at),
  ]);
  const kmsNodes: KmsNode[] = rawNodes.map((node) => ({
    txSenderAddress: node.txSenderAddress,
    signerAddress: node.signerAddress,
    ipAddress: node.ipAddress,
    storageUrl: node.storageUrl,
  }));
  const thresholds: KmsThresholds = { publicDecryption, userDecryption, kmsGen, mpc };

  return { currentContextId, kmsNodes, thresholds, canonicalChainId, canonicalBlockTag };
}

// Upgrades the secondary ProtocolConfig proxy and initializes it from the canonical snapshot. Reuses
// ProtocolConfig.initializeFromMigration (originally the Gateway -> Ethereum migration initializer) to
// land on canonical's currentKmsContextId rather than start a fresh counter.
export async function mirrorProtocolConfigFromCanonical(
  hre: HardhatRuntimeEnvironment,
  options: {
    canonicalProvider: Provider;
    canonicalProtocolConfigAddress: string;
    secondaryProxyAddress: string;
  },
): Promise<CanonicalSnapshot> {
  const { canonicalProvider, canonicalProtocolConfigAddress, secondaryProxyAddress } = options;

  const snapshot = await readCanonicalSnapshot(hre, { canonicalProvider, canonicalProtocolConfigAddress });
  await applyCanonicalSnapshot(hre, { snapshot, secondaryProxyAddress });

  return snapshot;
}

// Upgrades the secondary ProtocolConfig proxy and initializes it from an already-read snapshot —
// either freshly read (mirrorProtocolConfigFromCanonical) or parsed from a reviewed export artifact.
export async function applyCanonicalSnapshot(
  hre: HardhatRuntimeEnvironment,
  options: { snapshot: CanonicalSnapshot; secondaryProxyAddress: string },
): Promise<void> {
  const { ethers, upgrades } = hre;
  const { snapshot, secondaryProxyAddress } = options;
  const { currentContextId, kmsNodes, thresholds, canonicalChainId, canonicalBlockTag } = snapshot;
  console.log(
    `Mirroring ProtocolConfig from canonical chain ${canonicalChainId} at block ${canonicalBlockTag}: contextId=${currentContextId}, kmsNodes=${kmsNodes.length}.`,
  );

  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);
  const currentImplementation = await ethers.getContractFactory('EmptyUUPSProxy', deployer);
  const newImplem = await ethers.getContractFactory('ProtocolConfig', deployer);
  const proxy = await upgrades.forceImport(secondaryProxyAddress, currentImplementation);

  await upgrades.upgradeProxy(proxy, newImplem, {
    call: {
      fn: 'initializeFromMigration',
      args: [currentContextId, kmsNodes, thresholds],
    },
  });
}

// The JSON shape written by task:exportCanonicalProtocolConfig. Bigints are serialized as strings
// so DAO signers can diff artifacts as plain text.
export type CanonicalSnapshotArtifact = {
  canonicalChainId: string;
  blockNumber: number;
  protocolConfigAddress: string;
  currentKmsContextId: string;
  kmsNodes: KmsNode[];
  thresholds: { publicDecryption: string; userDecryption: string; kmsGen: string; mpc: string };
};

export function buildSnapshotArtifact(
  snapshot: CanonicalSnapshot,
  protocolConfigAddress: string,
): CanonicalSnapshotArtifact {
  return {
    canonicalChainId: snapshot.canonicalChainId.toString(),
    blockNumber: snapshot.canonicalBlockTag,
    protocolConfigAddress,
    currentKmsContextId: snapshot.currentContextId.toString(),
    kmsNodes: snapshot.kmsNodes,
    thresholds: {
      publicDecryption: snapshot.thresholds.publicDecryption.toString(),
      userDecryption: snapshot.thresholds.userDecryption.toString(),
      kmsGen: snapshot.thresholds.kmsGen.toString(),
      mpc: snapshot.thresholds.mpc.toString(),
    },
  };
}

export function parseSnapshotArtifact(raw: string): {
  snapshot: CanonicalSnapshot;
  protocolConfigAddress: string;
} {
  let artifact: CanonicalSnapshotArtifact;
  try {
    artifact = JSON.parse(raw);
  } catch (err) {
    throw new Error(`Snapshot artifact is not valid JSON (${formatError(err)}).`);
  }

  const requireBigint = (field: string, value: unknown): bigint => {
    if (typeof value !== 'string' || !/^\d+$/.test(value)) {
      throw new Error(`Snapshot artifact field "${field}" must be a decimal string, got ${JSON.stringify(value)}.`);
    }
    return BigInt(value);
  };

  if (typeof artifact.blockNumber !== 'number') {
    throw new Error(
      `Snapshot artifact field "blockNumber" must be a number, got ${JSON.stringify(artifact.blockNumber)}.`,
    );
  }
  if (typeof artifact.protocolConfigAddress !== 'string' || artifact.protocolConfigAddress.length === 0) {
    throw new Error('Snapshot artifact field "protocolConfigAddress" is missing.');
  }
  if (!Array.isArray(artifact.kmsNodes) || artifact.kmsNodes.length === 0) {
    throw new Error('Snapshot artifact field "kmsNodes" must be a non-empty array.');
  }
  for (const [index, node] of artifact.kmsNodes.entries()) {
    for (const field of ['txSenderAddress', 'signerAddress', 'ipAddress', 'storageUrl'] as const) {
      if (typeof node?.[field] !== 'string') {
        throw new Error(`Snapshot artifact field "kmsNodes[${index}].${field}" is missing.`);
      }
    }
  }

  const snapshot: CanonicalSnapshot = {
    currentContextId: requireBigint('currentKmsContextId', artifact.currentKmsContextId),
    kmsNodes: artifact.kmsNodes,
    thresholds: {
      publicDecryption: requireBigint('thresholds.publicDecryption', artifact.thresholds?.publicDecryption),
      userDecryption: requireBigint('thresholds.userDecryption', artifact.thresholds?.userDecryption),
      kmsGen: requireBigint('thresholds.kmsGen', artifact.thresholds?.kmsGen),
      mpc: requireBigint('thresholds.mpc', artifact.thresholds?.mpc),
    },
    canonicalChainId: requireBigint('canonicalChainId', artifact.canonicalChainId),
    canonicalBlockTag: artifact.blockNumber,
  };

  return { snapshot, protocolConfigAddress: artifact.protocolConfigAddress };
}
