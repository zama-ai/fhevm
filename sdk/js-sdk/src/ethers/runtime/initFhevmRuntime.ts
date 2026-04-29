import { getEthersRuntime, PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { initFhevmRuntime as initFhevmRuntime_ } from '../../core/runtime/initFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmRuntime(): Promise<void> {
  return initFhevmRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
