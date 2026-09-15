import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { encryptModule } from '../modules/encrypt/module/index.js';
import { verifyFhevmRuntime } from './CoreFhevmRuntime-p.js';

export async function initFhevmEncryptRuntime(runtime: FhevmRuntime, ownerToken: symbol): Promise<void> {
  verifyFhevmRuntime(runtime, ownerToken);
  const encryptRuntime = runtime.extend(encryptModule);
  await encryptRuntime.encrypt.initTfheModule();
}
