import type { Provider } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { getRequiredEnvVar } from './utils/loadVariables';

function formatError(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}

async function waitForProtocolConfigReady(
  hre: HardhatRuntimeEnvironment,
  proxyAddress: string,
  timeoutMs = 60_000,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  let lastError: unknown;
  const protocolConfig = new hre.ethers.Contract(
    proxyAddress,
    ['function getVersion() view returns (string)', 'function getCurrentKmsContextId() view returns (uint256)'],
    hre.ethers.provider,
  );

  while (true) {
    try {
      const version: string = await protocolConfig.getVersion();
      const currentKmsContextId: bigint = await protocolConfig.getCurrentKmsContextId();
      if (version.startsWith('ProtocolConfig') && currentKmsContextId !== 0n) {
        return;
      }
    } catch (err) {
      lastError = err;
    }
    if (Date.now() >= deadline) {
      throw new Error(
        `ProtocolConfig at ${proxyAddress} did not become ready after ${timeoutMs}ms: ${formatError(lastError)}`,
      );
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  }
}

export async function mirrorProtocolConfigFromCanonical(
  hre: HardhatRuntimeEnvironment,
  options: {
    canonicalProvider: Provider;
    canonicalProtocolConfigAddress: string;
    secondaryProxyAddress: string;
  },
): Promise<{
  currentContextId: bigint;
  kmsNodes: { txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }[];
  thresholds: { publicDecryption: bigint; userDecryption: bigint; kmsGen: bigint; mpc: bigint };
  canonicalChainId: bigint;
  canonicalBlockTag: number;
}> {
  const { ethers, upgrades } = hre;
  const { canonicalProvider, canonicalProtocolConfigAddress, secondaryProxyAddress } = options;

  const canonicalProtocolConfig = new ethers.Contract(
    canonicalProtocolConfigAddress,
    [
      'function getVersion() view returns (string)',
      'function getCurrentKmsContextId() view returns (uint256)',
      'function isValidKmsContext(uint256) view returns (bool)',
      'function getKmsNodesForContext(uint256) view returns (tuple(address txSenderAddress, address signerAddress, string ipAddress, string storageUrl)[])',
      'function getPublicDecryptionThresholdForContext(uint256) view returns (uint256)',
      'function getUserDecryptionThresholdForContext(uint256) view returns (uint256)',
      'function getKmsGenThresholdForContext(uint256) view returns (uint256)',
      'function getMpcThresholdForContext(uint256) view returns (uint256)',
    ],
    canonicalProvider,
  );

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
    canonicalBlockTag = await canonicalProvider.getBlockNumber();
  } catch (err) {
    throw new Error(`Canonical RPC handshake failed (${formatError(err)}).`);
  }
  const at = { blockTag: canonicalBlockTag };
  console.log(
    `Mirroring ProtocolConfig from canonical chain ${canonicalChainId} at block ${canonicalBlockTag} (contract ${canonicalProtocolConfigAddress}).`,
  );

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
  const kmsNodes = rawNodes.map(
    (node: { txSenderAddress: string; signerAddress: string; ipAddress: string; storageUrl: string }) => ({
      txSenderAddress: node.txSenderAddress,
      signerAddress: node.signerAddress,
      ipAddress: node.ipAddress,
      storageUrl: node.storageUrl,
    }),
  );
  const thresholds = { publicDecryption, userDecryption, kmsGen, mpc };
  console.log(
    `Canonical snapshot: contextId=${currentContextId}, kmsNodes=${kmsNodes.length}, thresholds={publicDecryption:${publicDecryption}, userDecryption:${userDecryption}, kmsGen:${kmsGen}, mpc:${mpc}}.`,
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
  await waitForProtocolConfigReady(hre, secondaryProxyAddress);

  return { currentContextId, kmsNodes, thresholds, canonicalChainId, canonicalBlockTag };
}
