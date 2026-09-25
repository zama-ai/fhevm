import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { verifyFhevmRuntime } from './CoreFhevmRuntime-p.js';
import { decryptModule } from '../modules/decrypt/module/index.js';

export async function initFhevmDecryptRuntime(runtime: FhevmRuntime, ownerToken: symbol): Promise<void> {
  verifyFhevmRuntime(runtime, ownerToken);
  const decryptRuntime = runtime.extend(decryptModule);
  await decryptRuntime.decrypt.initTkmsModule();
}
