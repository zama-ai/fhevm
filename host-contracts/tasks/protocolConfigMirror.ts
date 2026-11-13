import { type Provider, getAddress } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig } from '../types';
import { assertCanonicalBlockHash, resolveCanonicalBlock } from './utils/canonicalBlock';
import { assertContractMatchesVersionPrefix } from './utils/contractVersion';
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
  currentEpochId: bigint;
  kmsNodes: KmsNode[];
  thresholds: KmsThresholds;
  canonicalChainId: bigint;
  blockNumber: number;
  blockHash: string;
};

// Reads the canonical ProtocolConfig's current KMS context, pinned to one finalized block, for
// exports and mirror tasks. Pass blockNumber to pin to a finalized historical block (the export
// artifact's blockNumber) so a DAO signer can reproduce a snapshot byte-for-byte even after a later
// context rotation. Omit it to read the latest finalized block.
export async function readCanonicalSnapshot(
  hre: HardhatRuntimeEnvironment,
  options: { canonicalProvider: Provider; canonicalProtocolConfigAddress: string; blockNumber?: number },
): Promise<CanonicalSnapshot> {
  const { ethers } = hre;
  const { canonicalProvider, canonicalProtocolConfigAddress } = options;
  const { canonicalChainId, blockNumber, blockHash } = await resolveCanonicalBlock(
    canonicalProvider,
    options.blockNumber,
  );
  const at = { blockTag: blockNumber };

  // Reuse the shared version-prefix check, pointed at the canonical provider rather than the local
  // network so the identity check runs against the remote ProtocolConfig.
  await assertContractMatchesVersionPrefix(hre, canonicalProtocolConfigAddress, 'ProtocolConfig', canonicalProvider);

  const canonicalProtocolConfigBase = await ethers.getContractAt('ProtocolConfig', canonicalProtocolConfigAddress);
  const canonicalProtocolConfig = canonicalProtocolConfigBase.connect(canonicalProvider) as ProtocolConfig;

  const currentState = await canonicalProtocolConfig.getCurrentKmsContextAndEpoch(at);
  const currentKmsContextId = currentState[0];
  const currentEpochId = currentState[1];
  if (currentKmsContextId === 0n) {
    throw new Error(
      `Canonical ProtocolConfig at ${canonicalProtocolConfigAddress} has no active KMS context (currentKmsContextId=0); cannot mirror.`,
    );
  }
  if (currentEpochId === 0n) {
    throw new Error(
      `Canonical ProtocolConfig at ${canonicalProtocolConfigAddress} has no active KMS epoch (currentEpochId=0); cannot mirror.`,
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

  await assertCanonicalBlockHash(canonicalProvider, { blockNumber, blockHash });

  return {
    protocolConfigAddress: getAddress(canonicalProtocolConfigAddress),
    currentKmsContextId,
    currentEpochId,
    kmsNodes,
    thresholds,
    canonicalChainId,
    blockNumber,
    blockHash,
  };
}

// Builds the ProtocolConfigReplica upgrade for a non-canonical proxy from a snapshot: deploys the
// implementation and returns the upgradeToAndCall(initializeFromCanonical(... args ...)) payload.
// The DAO path prints it for signers. The direct (devnet) path executes the very same payload with
// the deployer key (executeUpgradeProposal). Mirror initialization lands the replica on canonical's
// active context and epoch rather than starting a fresh local lifecycle.
export async function buildCanonicalUpgradeProposal(
  hre: HardhatRuntimeEnvironment,
  options: { snapshot: CanonicalSnapshot; proxyAddress: string },
): Promise<UpgradeProposal> {
  const { snapshot, proxyAddress } = options;
  console.log(
    `Mirroring ProtocolConfig ${snapshot.protocolConfigAddress} from canonical chain ${snapshot.canonicalChainId} at block ${snapshot.blockNumber} (${snapshot.blockHash}): contextId=${snapshot.currentKmsContextId}, epochId=${snapshot.currentEpochId}, kmsNodes=${snapshot.kmsNodes.length}.`,
  );

  return buildUpgradeProposal(hre, {
    proxyAddress,
    contractName: 'contracts/ProtocolConfigReplica.sol:ProtocolConfigReplica',
    innerFunctionName: 'initializeFromCanonical',
    decodedArgs: [snapshot.currentKmsContextId, snapshot.currentEpochId, snapshot.kmsNodes, snapshot.thresholds],
  });
}
