import { initFhevmEncryptRuntime as initFhevmEncryptRuntime_ } from '../../core/runtime/initFhevmEncryptRuntime-p.js';
import { PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { getEthersRuntime } from '../internal/runtime.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmEncryptRuntime(): Promise<void> {
  return initFhevmEncryptRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
