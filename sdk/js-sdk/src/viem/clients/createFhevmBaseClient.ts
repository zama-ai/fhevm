import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions, FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmBaseClient } from '../../core/types/fhevmClient.js';
import { PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { getViemRuntime } from '../internal/runtime.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { baseActions } from '../../core/clients/decorators/base.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Ethereum chain definition (e.g. `mainnet`, `sepolia`) and a publicClient connected
 * to Ethereum, used only to read `KMSGeneration` and verify the FHE public key
 * and CRS. Required for encryption when the host chain is not Ethereum.
 */
export type FhevmViemKeyVerification = {
  readonly chain: FhevmChain;
  readonly publicClient: PublicClient;
};

export type FhevmViemOptions = FhevmOptions & {
  readonly keyVerification?: FhevmViemKeyVerification | undefined;
};

export type FhevmViemEncryptOptions = FhevmEncryptOptions & {
  readonly keyVerification?: FhevmViemKeyVerification | undefined;
};

////////////////////////////////////////////////////////////////////////////////

export function createFhevmBaseClient<chain extends FhevmChain, publicClient extends PublicClient>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmViemOptions | undefined;
}): FhevmBaseClient<chain, FhevmRuntime, publicClient> {
  const c = createCoreFhevm(PRIVATE_VIEM_TOKEN, {
    chain: parameters.chain,
    runtime: getViemRuntime(),
    client: parameters.publicClient,
    options: parameters.options,
    keyVerification: _toKeyVerification(parameters.options?.keyVerification),
  });
  return c.extend(baseActions);
}

function _toKeyVerification(
  keyVerification: FhevmViemKeyVerification | undefined,
): { readonly chain: FhevmChain; readonly client: PublicClient } | undefined {
  return keyVerification === undefined
    ? undefined
    : { chain: keyVerification.chain, client: keyVerification.publicClient };
}
