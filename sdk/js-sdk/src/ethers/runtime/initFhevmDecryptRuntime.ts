import { getEthersRuntime, PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { initFhevmDecryptRuntime as initFhevmDecyptRuntime_ } from '../../core/runtime/initFhevmDecryptRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmDecryptRuntime(): Promise<void> {
  return initFhevmDecyptRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
