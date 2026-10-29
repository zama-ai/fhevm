import { PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { getViemRuntime } from '../internal/runtime.js';
import { initFhevmEncryptTransciRuntime as initFhevmEncryptTransciRuntime_ } from '../../core/runtime/initFhevmEncryptTransciRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmEncryptTransciRuntime(): Promise<void> {
  return initFhevmEncryptTransciRuntime_(getViemRuntime(), PRIVATE_VIEM_TOKEN);
}
