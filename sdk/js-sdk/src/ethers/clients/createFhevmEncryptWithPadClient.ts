import type { ethers as EthersT } from 'ethers';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncryptWithPad } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmEncryptOptions } from '../../core/types/coreFhevmClient.js';
import type { FhevmEncryptWithPadClient } from '../../core/types/fhevmClient.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { encryptWithPadActions } from './decorators/encryptWithPad.js';

////////////////////////////////////////////////////////////////////////////////

export function createFhevmEncryptWithPadClient<
  chain extends FhevmChain,
  provider extends EthersT.ContractRunner,
>(parameters: {
  readonly provider: provider;
  readonly chain: chain;
  readonly options?: FhevmEncryptOptions | undefined;
}): FhevmEncryptWithPadClient<chain, WithEncryptWithPad, provider> {
  const c = createFhevmBaseClient(parameters);

  return c.extend(encryptWithPadActions);
}
