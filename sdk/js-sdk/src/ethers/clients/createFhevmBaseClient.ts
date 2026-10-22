import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions, FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmBaseClient } from '../../core/types/fhevmClient.js';
import { PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { baseActions } from '../../core/clients/decorators/base.js';
import { getEthersRuntime } from '../internal/runtime.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Ethereum chain definition (e.g. `mainnet`, `sepolia`) and a provider connected
 * to Ethereum, used only to read `KMSGeneration` and verify the FHE public key
 * and CRS. Required for encryption when the host chain is not Ethereum.
 */
export type FhevmEthersKeyVerification = {
  readonly chain: FhevmChain;
  readonly provider: EthersT.ContractRunner;
};

export type FhevmEthersOptions = FhevmOptions & {
  readonly keyVerification?: FhevmEthersKeyVerification | undefined;
};

export type FhevmEthersEncryptOptions = FhevmEncryptOptions & {
  readonly keyVerification?: FhevmEthersKeyVerification | undefined;
};

////////////////////////////////////////////////////////////////////////////////

export function createFhevmBaseClient<chain extends FhevmChain, provider extends EthersT.ContractRunner>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmEthersOptions | undefined;
}): FhevmBaseClient<chain, FhevmRuntime, provider> {
  const c = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: parameters.chain,
    runtime: getEthersRuntime(),
    client: parameters.provider,
    options: parameters.options,
    keyVerification: _toKeyVerification(parameters.options?.keyVerification),
  });
  return c.extend(baseActions);
}

function _toKeyVerification(
  keyVerification: FhevmEthersKeyVerification | undefined,
): { readonly chain: FhevmChain; readonly client: EthersT.ContractRunner } | undefined {
  return keyVerification === undefined ? undefined : { chain: keyVerification.chain, client: keyVerification.provider };
}
