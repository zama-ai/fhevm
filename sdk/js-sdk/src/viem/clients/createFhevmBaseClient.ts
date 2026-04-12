import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import { getViemRuntime, PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmBaseClient } from '../../core/types/fhevmClient.js';
import { baseActions } from '../../core/clients/decorators/base.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmBaseClient<
  chain extends FhevmChain,
  publicClient extends PublicClient,
>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmBaseClient<chain, FhevmRuntime, publicClient> {
  const c = createCoreFhevm(PRIVATE_VIEM_TOKEN, {
    chain: parameters.chain,
    runtime: getViemRuntime(),
    client: parameters.publicClient,
    options: parameters.options,
  });
  return c.extend(baseActions);
}
