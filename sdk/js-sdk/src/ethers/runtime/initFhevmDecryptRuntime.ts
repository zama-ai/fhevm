import { initFhevmDecryptRuntime as initFhevmDecyptRuntime_ } from '../../core/runtime/initFhevmDecryptRuntime-p.js';
import { PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { getEthersRuntime } from '../internal/runtime.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmDecryptRuntime(): Promise<void> {
  return initFhevmDecyptRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
