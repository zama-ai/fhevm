import { type Provider, getAddress } from 'ethers';
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
  currentEpochId: bigint;
  kmsNodes: KmsNode[];
  thresholds: KmsThresholds;
  canonicalChainId: bigint;
  blockNumber: number;
  blockHash: string;
};

// RPC providers must disable caching so this check re-fetches the block after the state reads.
export async function assertCanonicalSnapshotBlockHash(
  canonicalProvider: Provider,
  snapshot: Pick<CanonicalSnapshot, 'blockNumber' | 'blockHash'>,
): Promise<void> {
  const block = await canonicalProvider.getBlock(snapshot.blockNumber);
  if (block === null || block.hash === null || block.hash !== snapshot.blockHash) {
    throw new Error(`Canonical block ${snapshot.blockNumber} is missing or its hash changed during the read; retry.`);
  }
}

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
  if (options.blockNumber !== undefined && (!Number.isSafeInteger(options.blockNumber) || options.blockNumber < 0)) {
    throw new Error('Canonical block number must be a nonnegative safe integer.');
  }

  // Handshake before the identity check so a dead or mistyped RPC URL is reported as an RPC
  // problem, not as a contract identity failure. Require finality even for an explicit block:
  // a finalized block can't be reorged out, so the exported artifact stays
  // reproducible. We resolve the full block (not just its number) to capture the hash too, which
  // uniquely identifies the read's state across reorgs — a height alone is ambiguous.
  let canonicalChainId: bigint;
  let finalizedBlock: Awaited<ReturnType<Provider['getBlock']>>;
  try {
    canonicalChainId = (await canonicalProvider.getNetwork()).chainId;
    finalizedBlock = await canonicalProvider.getBlock('finalized');
  } catch (err) {
    throw new Error(`Canonical RPC handshake failed (${formatError(err)}).`);
  }
  if (finalizedBlock === null || finalizedBlock.hash === null) {
    throw new Error('Canonical RPC returned no finalized block.');
  }
  if (options.blockNumber !== undefined && options.blockNumber > finalizedBlock.number) {
    throw new Error(
      `Canonical block ${options.blockNumber} is above the latest finalized block ${finalizedBlock.number}; wait for finalization.`,
    );
  }
  const block =
    options.blockNumber === undefined || options.blockNumber === finalizedBlock.number
      ? finalizedBlock
      : await canonicalProvider.getBlock(options.blockNumber);
  if (block === null || block.hash === null) {
    throw new Error(`Canonical RPC returned no block or hash for "${options.blockNumber}".`);
  }
  const blockNumber = block.number;
  const blockHash = block.hash;
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

  await assertCanonicalSnapshotBlockHash(canonicalProvider, { blockNumber, blockHash });

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

// Builds the upgrade for a secondary ProtocolConfig proxy from a snapshot: deploys the
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

  // initializeFromCanonical takes KmsNodeParams (txSender/signer/ip/storageUrl plus MPC metadata:
  // partyId, mpcIdentity, caCert, storagePrefix). Only the first four are persisted in the on-chain
  // KmsNode struct, so the MPC metadata can't be read back from canonical and isn't part of the
  // mirrored state. _storeKmsContext neither stores nor validates those fields, so we fill them with
  // deterministic placeholders; the replica's stored node set still matches canonical exactly. The
  // fresh deploy path numbers party ids from 1 (KMS_NODE_PARTY_ID_0="1"), while the mirror path
  // numbers them from 0.
  const kmsNodeParams = snapshot.kmsNodes.map((node, index) => ({
    ...node,
    partyId: index,
    mpcIdentity: '',
    caCert: '0x',
    storagePrefix: '',
  }));

  return buildUpgradeProposal(hre, {
    proxyAddress,
    contractName: 'contracts/ProtocolConfig.sol:ProtocolConfig',
    innerFunctionName: 'initializeFromCanonical',
    decodedArgs: [snapshot.currentKmsContextId, snapshot.currentEpochId, kmsNodeParams, snapshot.thresholds],
  });
}
