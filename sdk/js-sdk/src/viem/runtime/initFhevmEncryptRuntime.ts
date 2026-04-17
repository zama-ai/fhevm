import { getViemRuntime, PRIVATE_VIEM_TOKEN } from '../internal/viem-p.js';
import { initFhevmEncryptRuntime as initFhevmEncryptRuntime_ } from '../../core/runtime/initFhevmEncryptRuntime-p.js';

////////////////////////////////////////////////////////////////////////////////

export function initFhevmEncryptRuntime(): Promise<void> {
  return initFhevmEncryptRuntime_(getViemRuntime(), PRIVATE_VIEM_TOKEN);
}
