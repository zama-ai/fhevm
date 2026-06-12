import type { Provider } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { ProtocolConfig } from '../types';
import { formatError } from './utils/formatError';
import { getRequiredEnvVar } from './utils/loadVariables';

export type SecondaryDeployArgs = {
  canonicalRpcUrl: string;
  canonicalProtocolConfigAddress: string;
};

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
  const { ethers, upgrades } = hre;
  const { canonicalProvider, canonicalProtocolConfigAddress, secondaryProxyAddress } = options;

  const snapshot = await readCanonicalSnapshot(hre, { canonicalProvider, canonicalProtocolConfigAddress });
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

  return snapshot;
}
