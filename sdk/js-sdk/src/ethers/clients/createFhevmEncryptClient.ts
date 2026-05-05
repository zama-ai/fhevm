import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmEncryptClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { encryptActions } from '../decorators/encrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmEncryptClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmOptions | undefined;
}): FhevmEncryptClient<chain, WithEncrypt, provider> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(encryptActions);
}

////////////////////////////////////////////////////////////////////////////////
