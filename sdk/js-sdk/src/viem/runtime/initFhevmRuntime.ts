import { getViemRuntime, PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { initFhevmRuntime as initFhevmRuntime_ } from '../../core/runtime/initFhevmRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmRuntime(): Promise<void> {
  return initFhevmRuntime_(getViemRuntime(), PRIVATE_VIEM_TOKEN);
}
