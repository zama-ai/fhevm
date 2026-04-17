import { getEthersRuntime, PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { initFhevmEncryptRuntime as initFhevmEncryptRuntime_ } from '../../core/runtime/initFhevmEncryptRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmEncryptRuntime(): Promise<void> {
  return initFhevmEncryptRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
