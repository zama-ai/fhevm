import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithAll } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmCleartextOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmClient } from '../../core/types/fhevmClient.js';
import { createFhevmCleartextBaseClient } from './createFhevmCleartextBaseClient.js';
import { encryptActions } from './decorators/encrypt.js';
import { decryptActions } from './decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmCleartextClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmCleartextOptions | undefined;
}): FhevmClient<chain, WithAll, provider> {
  const c = createFhevmCleartextBaseClient(parameters);

  return c.extend(decryptActions).extend(encryptActions);
}
