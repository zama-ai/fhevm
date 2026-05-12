import { initFhevmRuntime as initFhevmRuntime_ } from '../../core/runtime/initFhevmRuntime-p.js';
import { PRIVATE_ETHERS_TOKEN } from '../internal/ethers-p.js';
import { getEthersRuntime } from '../internal/runtime.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmRuntime(): Promise<void> {
  return initFhevmRuntime_(getEthersRuntime(), PRIVATE_ETHERS_TOKEN);
}
