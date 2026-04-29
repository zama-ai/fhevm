import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithAll } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { encryptActions } from '../decorators/encrypt.js';
import { decryptActions } from '../decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmClient<chain extends FhevmChain, provider extends EthersT.ContractRunner>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmClient<chain, WithAll, provider> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(decryptActions).extend(encryptActions);
}
