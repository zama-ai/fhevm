import { initFhevmEncryptTransciRuntime as initFhevmEncryptTransciRuntime_ } from '../../core/runtime/initFhevmEncryptTransciRuntime-p.js';
import { PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { getEthersRuntime } from '../internal/runtime.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmEncryptTransciRuntime(): Promise<void> {
  return initFhevmEncryptTransciRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
