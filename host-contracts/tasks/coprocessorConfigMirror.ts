import { type Provider, getAddress } from 'ethers';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import type { CoprocessorConfig } from '../types';
import { assertCanonicalSnapshotBlockHash, resolveCanonicalBlock } from './protocolConfigMirror';
import { assertContractMatchesVersionPrefix } from './utils/contractVersion';
import { type UpgradeProposal, buildUpgradeProposal } from './utils/upgradeProposal';

export type Coprocessor = {
  txSenderAddress: string;
  signerAddress: string;
  storageUrl: string;
  prfEndpointUrl: string;
};

export type CanonicalCoprocessorSnapshot = {
  coprocessorConfigAddress: string;
  currentCoprocessorContextId: bigint;
  coprocessors: Coprocessor[];
  majorityThreshold: bigint;
  canonicalChainId: bigint;
  blockNumber: number;
  blockHash: string;
};

// Connected to `runner` when given (e.g. the canonical provider), otherwise to the local network.
export async function getCoprocessorConfig(
  hre: HardhatRuntimeEnvironment,
  address: string,
  runner?: Provider,
): Promise<CoprocessorConfig> {
  const coprocessorConfig = await hre.ethers.getContractAt('CoprocessorConfig', address);
  return (runner ? coprocessorConfig.connect(runner) : coprocessorConfig) as unknown as CoprocessorConfig;
}

// Reads the canonical CoprocessorConfig's current coprocessor context, pinned to one finalized block,
// for exports and mirror tasks. See `resolveCanonicalBlock` for `blockNumber`.
export async function readCanonicalCoprocessorSnapshot(
  hre: HardhatRuntimeEnvironment,
  options: { canonicalProvider: Provider; canonicalCoprocessorConfigAddress: string; blockNumber?: number },
): Promise<CanonicalCoprocessorSnapshot> {
  const { canonicalProvider, canonicalCoprocessorConfigAddress } = options;
  const { canonicalChainId, blockNumber, blockHash } = await resolveCanonicalBlock(
    canonicalProvider,
    options.blockNumber,
  );
  const at = { blockTag: blockNumber };

  await assertContractMatchesVersionPrefix(
    hre,
    canonicalCoprocessorConfigAddress,
    'CoprocessorConfig',
    canonicalProvider,
  );
  const canonicalCoprocessorConfig = await getCoprocessorConfig(
    hre,
    canonicalCoprocessorConfigAddress,
    canonicalProvider,
  );

  // The current coprocessor context is always valid: only past contexts can be destroyed.
  const currentCoprocessorContextId = await canonicalCoprocessorConfig.getCurrentCoprocessorContextId(at);
  const [rawCoprocessors, majorityThreshold] = await Promise.all([
    canonicalCoprocessorConfig.getCoprocessorsForContext(currentCoprocessorContextId, at),
    canonicalCoprocessorConfig.getCoprocessorMajorityThresholdForContext(currentCoprocessorContextId, at),
  ]);
  const coprocessors: Coprocessor[] = rawCoprocessors.map((coprocessor) => ({
    txSenderAddress: coprocessor.txSenderAddress,
    signerAddress: coprocessor.signerAddress,
    storageUrl: coprocessor.storageUrl,
    prfEndpointUrl: coprocessor.prfEndpointUrl,
  }));

  await assertCanonicalSnapshotBlockHash(canonicalProvider, { blockNumber, blockHash });

  return {
    coprocessorConfigAddress: getAddress(canonicalCoprocessorConfigAddress),
    currentCoprocessorContextId,
    coprocessors,
    majorityThreshold,
    canonicalChainId,
    blockNumber,
    blockHash,
  };
}

// Builds the CoprocessorConfigReplica upgrade for a non-canonical empty proxy from a snapshot: deploys
// the implementation and returns the upgradeToAndCall(initializeFromCanonical(...)) payload. The DAO
// path prints it for signers. The direct (devnet) path executes the very same payload with the
// deployer key (executeUpgradeProposal).
export async function buildCanonicalCoprocessorUpgradeProposal(
  hre: HardhatRuntimeEnvironment,
  options: { snapshot: CanonicalCoprocessorSnapshot; proxyAddress: string },
): Promise<UpgradeProposal> {
  const { snapshot, proxyAddress } = options;
  console.log(
    `Mirroring CoprocessorConfig ${snapshot.coprocessorConfigAddress} from canonical chain ${snapshot.canonicalChainId} at block ${snapshot.blockNumber} (${snapshot.blockHash}): coprocessorContextId=${snapshot.currentCoprocessorContextId}, coprocessors=${snapshot.coprocessors.length}.`,
  );
  return buildUpgradeProposal(hre, {
    proxyAddress,
    contractName: 'contracts/CoprocessorConfigReplica.sol:CoprocessorConfigReplica',
    innerFunctionName: 'initializeFromCanonical',
    decodedArgs: [snapshot.currentCoprocessorContextId, snapshot.coprocessors, snapshot.majorityThreshold],
  });
}
