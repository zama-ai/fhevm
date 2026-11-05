import { PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { getViemRuntime } from '../internal/runtime.js';
import { initFhevmEncryptWithPadRuntime as initFhevmEncryptWithPadRuntime_ } from '../../core/runtime/initFhevmEncryptWithPadRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmEncryptWithPadRuntime(): Promise<void> {
  return initFhevmEncryptWithPadRuntime_(getViemRuntime(), PRIVATE_VIEM_TOKEN);
}
