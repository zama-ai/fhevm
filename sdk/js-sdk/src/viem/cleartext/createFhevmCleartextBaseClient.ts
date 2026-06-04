import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmBaseClient } from '../../core/types/fhevmClient.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { baseActions } from '../../core/clients/decorators/base.js';
import { PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { getCleartextViemRuntime } from '../internal/runtime-ct.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmCleartextBaseClient<
  chain extends FhevmChain,
  publicClient extends PublicClient,
>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmBaseClient<chain, FhevmRuntime, publicClient> {
  const c = createCoreFhevm(PRIVATE_VIEM_TOKEN, {
    chain: parameters.chain,
    runtime: getCleartextViemRuntime(),
    client: parameters.publicClient,
    options: parameters.options,
  });
  return c.extend(baseActions);
}
