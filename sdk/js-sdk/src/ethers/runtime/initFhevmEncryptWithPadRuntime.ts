import { initFhevmEncryptWithPadRuntime as initFhevmEncryptWithPadRuntime_ } from '../../core/runtime/initFhevmEncryptWithPadRuntime-p.js';
import { PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { getEthersRuntime } from '../internal/runtime.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmEncryptWithPadRuntime(): Promise<void> {
  return initFhevmEncryptWithPadRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
