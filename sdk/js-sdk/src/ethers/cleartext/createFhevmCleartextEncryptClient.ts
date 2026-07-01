import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmEncryptClient } from '../../core/types/fhevmClient.js';
import { createFhevmCleartextBaseClient } from './createFhevmCleartextBaseClient.js';
import { encryptActions } from './decorators/encrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmCleartextEncryptClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmEncryptOptions | undefined;
}): FhevmEncryptClient<chain, WithEncrypt, provider> {
  const c = createFhevmCleartextBaseClient(parameters);

  return c.extend(encryptActions);
}
