import type { PublicClient } from 'viem';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmCleartextOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmBaseClient } from '../../core/types/fhevmClient.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { baseActions } from '../../core/clients/decorators/base.js';
import { PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { getCleartextViemRuntime } from '../internal/runtime-ct.js';
import { createCleartextFheEncryptionKeyPolicy } from '../../core/key/FheEncryptionKeyPolicy-p.js';
import {
  assertCleartextFheEncryptionKeyOptions,
  createFheEncryptionKeyProvider,
} from '../../core/key/FheEncryptionKeyProvider-p.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmCleartextBaseClient<
  chain extends FhevmChain,
  publicClient extends PublicClient,
>(parameters: {
  readonly publicClient: publicClient;
  readonly chain: chain;
  readonly options?: FhevmCleartextOptions | undefined;
}): FhevmBaseClient<chain, FhevmRuntime, publicClient> {
  assertCleartextFheEncryptionKeyOptions(parameters.options);
  const runtime = getCleartextViemRuntime();
  const c = createCoreFhevm(PRIVATE_VIEM_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.publicClient,
    options: parameters.options,
    fheEncryptionKeyProvider: createFheEncryptionKeyProvider({
      chain: parameters.chain,
      runtime,
      policy: createCleartextFheEncryptionKeyPolicy(),
    }),
  });
  return c.extend(baseActions);
}
