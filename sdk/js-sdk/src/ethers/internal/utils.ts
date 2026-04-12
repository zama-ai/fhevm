import { Contract, Interface } from 'ethers';
import type { ethers as EthersT } from 'ethers';
import type { ChecksummedAddress } from '../../core/types/primitives.js';
import { trustedClientToEthersContractRunner } from './ethers-p.js';
import type { TrustedClient } from '../../core/modules/ethereum/types.js';

/**
 * Get ethers Network from an unknown client.
 * Supports Provider (has getNetwork) and ContractRunner (via its provider).
 */
export async function getNetwork(
  hostPublicClient: TrustedClient<EthersT.ContractRunner>,
): Promise<EthersT.Network> {
  const runner = trustedClientToEthersContractRunner(hostPublicClient);

  if ((runner as unknown) === undefined || (runner as unknown) === null) {
    throw new Error('Cannot get network: client is null or undefined.');
  }

  if (
    typeof runner === 'object' &&
    'getNetwork' in runner &&
    typeof (runner as Record<string, unknown>).getNetwork === 'function'
  ) {
    return await (runner as unknown as EthersT.Provider).getNetwork();
  }

  if (runner.provider != null) {
    return await runner.provider.getNetwork();
  }

  throw new Error(
    'Cannot get network: client is neither a Provider nor a ContractRunner with a provider.',
  );
}

// eslint-disable-next-line @typescript-eslint/no-unnecessary-type-parameters
export function getEthersContract<C>(
  hostPublicClient: TrustedClient<EthersT.ContractRunner>,
  contractAddress: ChecksummedAddress,
  abi: ReadonlyArray<Record<string, unknown>>,
): C {
  const runner = trustedClientToEthersContractRunner(hostPublicClient);
  return new Contract(
    contractAddress,
    new Interface(abi),
    runner,
  ) as unknown as C;
}
