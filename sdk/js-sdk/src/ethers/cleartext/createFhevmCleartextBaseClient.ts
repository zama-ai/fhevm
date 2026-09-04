import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmCleartextOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmBaseClient } from '../../core/types/fhevmClient.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { baseActions } from '../../core/clients/decorators/base.js';
import { PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { getCleartextEthersRuntime } from '../internal/runtime-ct.js';
import { createCleartextFheEncryptionKeyPolicy } from '../../core/key/FheEncryptionKeyPolicy-p.js';
import {
  assertCleartextFheEncryptionKeyOptions,
  createFheEncryptionKeyProvider,
} from '../../core/key/FheEncryptionKeyProvider-p.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmCleartextBaseClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmCleartextOptions | undefined;
}): FhevmBaseClient<chain, FhevmRuntime, provider> {
  assertCleartextFheEncryptionKeyOptions(parameters.options);
  const runtime = getCleartextEthersRuntime();
  const c = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: parameters.chain,
    runtime,
    client: parameters.provider,
    options: parameters.options,
    fheEncryptionKeyProvider: createFheEncryptionKeyProvider({
      chain: parameters.chain,
      runtime,
      policy: createCleartextFheEncryptionKeyPolicy(),
    }),
  });
  return c.extend(baseActions);
}
