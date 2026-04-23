import { PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { getViemRuntime } from '../internal/runtime.js';
import { initFhevmDecryptRuntime as initFhevmDecyptRuntime_ } from '../../core/runtime/initFhevmDecryptRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmDecryptRuntime(): Promise<void> {
  return initFhevmDecyptRuntime_(getViemRuntime(), PRIVATE_VIEM_TOKEN);
}
