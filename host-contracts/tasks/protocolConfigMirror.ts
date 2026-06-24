import { type Provider, isAddress } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig } from '../types';
import { assertContractMatchesVersionPrefix } from './utils/contractVersion';
import { formatError } from './utils/formatError';
import { type UpgradeProposal, buildUpgradeProposal } from './utils/upgradeProposal';

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
  protocolConfigAddress: string;
  currentKmsContextId: bigint;
  kmsNodes: KmsNode[];
  thresholds: KmsThresholds;
  canonicalChainId: bigint;
  blockNumber: number;
  blockHash: string;
};

// Reads the canonical ProtocolConfig's current KMS context, pinned to one block. Shared by
// task:exportCanonicalProtocolConfig and task:deployProtocolConfigFromCanonical's live-read mode so
// both seed from the exact same read. Pass blockNumber to pin to a historical block (the export
// artifact's blockNumber) so a DAO signer can reproduce a snapshot byte-for-byte even after a later
// context rotation; omit it to read the latest finalized block.
export async function readCanonicalSnapshot(
  hre: HardhatRuntimeEnvironment,
  options: { canonicalProvider: Provider; canonicalProtocolConfigAddress: string; blockNumber?: number },
): Promise<CanonicalSnapshot> {
  const { ethers } = hre;
  const { canonicalProvider, canonicalProtocolConfigAddress } = options;

  // Handshake before the identity check so a dead or mistyped RPC URL is reported as an RPC
  // problem, not as a contract identity failure. Pin to the finalized block when no explicit block
  // is requested: a finalized block can't be reorged out, so the exported artifact stays
  // reproducible. We resolve the full block (not just its number) to capture the hash too, which
  // uniquely identifies the read's state across reorgs — a height alone is ambiguous.
  let canonicalChainId: bigint;
  let blockTag: number | 'finalized';
  let block: Awaited<ReturnType<Provider['getBlock']>>;
  try {
    canonicalChainId = (await canonicalProvider.getNetwork()).chainId;
    blockTag = options.blockNumber ?? 'finalized';
    block = await canonicalProvider.getBlock(blockTag);
  } catch (err) {
    throw new Error(`Canonical RPC handshake failed (${formatError(err)}).`);
  }
  if (block === null || block.hash === null) {
    throw new Error(`Canonical RPC returned no finalized block for "${blockTag}".`);
  }
  const blockNumber = block.number;
  const blockHash = block.hash;
  const at = { blockTag: blockNumber };

  // Reuse the shared version-prefix check, pointed at the canonical provider rather than the local
  // network so the identity check runs against the remote ProtocolConfig.
  await assertContractMatchesVersionPrefix(hre, canonicalProtocolConfigAddress, 'ProtocolConfig', canonicalProvider);

  const canonicalProtocolConfigBase = await ethers.getContractAt('ProtocolConfig', canonicalProtocolConfigAddress);
  const canonicalProtocolConfig = canonicalProtocolConfigBase.connect(canonicalProvider) as ProtocolConfig;

  const currentKmsContextId: bigint = await canonicalProtocolConfig.getCurrentKmsContextId(at);
  if (currentKmsContextId === 0n) {
    throw new Error(
      `Canonical ProtocolConfig at ${canonicalProtocolConfigAddress} has no active KMS context (currentKmsContextId=0); cannot mirror.`,
    );
  }
  const isCurrentContextValid: boolean = await canonicalProtocolConfig.isValidKmsContext(currentKmsContextId, at);
  if (!isCurrentContextValid) {
    throw new Error(
      `Canonical ProtocolConfig's current context ${currentKmsContextId} is destroyed; cannot mirror a destroyed context.`,
    );
  }

  const [rawNodes, publicDecryption, userDecryption, kmsGen, mpc] = await Promise.all([
    canonicalProtocolConfig.getKmsNodesForContext(currentKmsContextId, at),
    canonicalProtocolConfig.getPublicDecryptionThresholdForContext(currentKmsContextId, at),
    canonicalProtocolConfig.getUserDecryptionThresholdForContext(currentKmsContextId, at),
    canonicalProtocolConfig.getKmsGenThresholdForContext(currentKmsContextId, at),
    canonicalProtocolConfig.getMpcThresholdForContext(currentKmsContextId, at),
  ]);
  const kmsNodes: KmsNode[] = rawNodes.map((node) => ({
    txSenderAddress: node.txSenderAddress,
    signerAddress: node.signerAddress,
    ipAddress: node.ipAddress,
    storageUrl: node.storageUrl,
  }));
  const thresholds: KmsThresholds = { publicDecryption, userDecryption, kmsGen, mpc };

  return {
    protocolConfigAddress: canonicalProtocolConfigAddress,
    currentKmsContextId,
    kmsNodes,
    thresholds,
    canonicalChainId,
    blockNumber,
    blockHash,
  };
}

// Builds the upgrade for a secondary ProtocolConfig proxy from a snapshot — freshly read via
// readCanonicalSnapshot or parsed from a reviewed export artifact: deploys the implementation and
// returns the upgradeToAndCall(mirrorKmsContext(... args ...)) payload. The DAO path prints it for
// signers; the direct (devnet) path executes the very same payload with the deployer key
// (executeUpgradeProposal). Mirror initialization lands the replica on canonical's currentKmsContextId
// rather than starting a fresh counter.
export async function buildCanonicalUpgradeProposal(
  hre: HardhatRuntimeEnvironment,
  options: { snapshot: CanonicalSnapshot; proxyAddress: string },
): Promise<UpgradeProposal> {
  const { snapshot, proxyAddress } = options;
  console.log(
    `Mirroring ProtocolConfig from canonical chain ${snapshot.canonicalChainId} at block ${snapshot.blockNumber} (${snapshot.blockHash}): contextId=${snapshot.currentKmsContextId}, kmsNodes=${snapshot.kmsNodes.length}.`,
  );

  // mirrorKmsContext takes KmsNodeParams (txSender/signer/ip/storageUrl plus MPC metadata:
  // partyId, mpcIdentity, caCert, storagePrefix). Only the first four are persisted in the on-chain
  // KmsNode struct, so the MPC metadata can't be read back from canonical and isn't part of the
  // mirrored state. _storeKmsContext neither stores nor validates those fields, so we fill them with
  // deterministic placeholders; the replica's stored node set still matches canonical exactly.
  const kmsNodeParams = snapshot.kmsNodes.map((node, index) => ({
    ...node,
    partyId: index,
    mpcIdentity: '',
    caCert: '0x',
    storagePrefix: '',
  }));

  return buildUpgradeProposal(hre, {
    proxyAddress,
    contractName: 'ProtocolConfig',
    innerFunctionName: 'mirrorKmsContext',
    decodedArgs: [snapshot.currentKmsContextId, kmsNodeParams, snapshot.thresholds, '', []],
  });
}

// The JSON shape written by task:exportCanonicalProtocolConfig: the snapshot fields plus the
// canonical contract address, with bigints serialized as strings so DAO signers can diff artifacts
// as plain text.
export type CanonicalSnapshotArtifact = {
  canonicalChainId: string;
  blockNumber: number;
  blockHash: string;
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
    blockNumber: snapshot.blockNumber,
    blockHash: snapshot.blockHash,
    protocolConfigAddress,
    currentKmsContextId: snapshot.currentKmsContextId.toString(),
    kmsNodes: snapshot.kmsNodes,
    thresholds: {
      publicDecryption: snapshot.thresholds.publicDecryption.toString(),
      userDecryption: snapshot.thresholds.userDecryption.toString(),
      kmsGen: snapshot.thresholds.kmsGen.toString(),
      mpc: snapshot.thresholds.mpc.toString(),
    },
  };
}

export function parseSnapshotArtifact(raw: string): CanonicalSnapshot {
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
  if (typeof artifact.blockHash !== 'string' || !/^0x[0-9a-fA-F]{64}$/.test(artifact.blockHash)) {
    throw new Error(
      `Snapshot artifact field "blockHash" must be a 32-byte hex string, got ${JSON.stringify(artifact.blockHash)}.`,
    );
  }
  if (!isAddress(artifact.protocolConfigAddress)) {
    throw new Error(
      `Snapshot artifact field "protocolConfigAddress" must be a valid address, got ${JSON.stringify(artifact.protocolConfigAddress)}.`,
    );
  }
  if (!Array.isArray(artifact.kmsNodes) || artifact.kmsNodes.length === 0) {
    throw new Error('Snapshot artifact field "kmsNodes" must be a non-empty array.');
  }
  for (const [index, node] of artifact.kmsNodes.entries()) {
    // Addresses get baked into the initializeFromMigration calldata, so reject a malformed one in
    // a hand-reviewed artifact here rather than letting ABI encoding fail with an opaque error.
    for (const field of ['txSenderAddress', 'signerAddress'] as const) {
      if (!isAddress(node?.[field])) {
        throw new Error(
          `Snapshot artifact field "kmsNodes[${index}].${field}" must be a valid address, got ${JSON.stringify(node?.[field])}.`,
        );
      }
    }
    for (const field of ['ipAddress', 'storageUrl'] as const) {
      if (typeof node?.[field] !== 'string') {
        throw new Error(`Snapshot artifact field "kmsNodes[${index}].${field}" is missing.`);
      }
    }
  }

  return {
    protocolConfigAddress: artifact.protocolConfigAddress,
    currentKmsContextId: requireBigint('currentKmsContextId', artifact.currentKmsContextId),
    kmsNodes: artifact.kmsNodes,
    thresholds: {
      publicDecryption: requireBigint('thresholds.publicDecryption', artifact.thresholds?.publicDecryption),
      userDecryption: requireBigint('thresholds.userDecryption', artifact.thresholds?.userDecryption),
      kmsGen: requireBigint('thresholds.kmsGen', artifact.thresholds?.kmsGen),
      mpc: requireBigint('thresholds.mpc', artifact.thresholds?.mpc),
    },
    canonicalChainId: requireBigint('canonicalChainId', artifact.canonicalChainId),
    blockNumber: artifact.blockNumber,
    blockHash: artifact.blockHash,
  };
}
