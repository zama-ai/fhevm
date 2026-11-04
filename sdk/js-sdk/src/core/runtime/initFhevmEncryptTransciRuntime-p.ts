import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { verifyFhevmRuntime } from './CoreFhevmRuntime-p.js';
import { encryptTransciModule } from '../modules/encryptTransci/module/index.js';

export async function initFhevmEncryptTransciRuntime(runtime: FhevmRuntime, ownerToken: symbol): Promise<void> {
  verifyFhevmRuntime(runtime, ownerToken);
  const encryptTransciRuntime = runtime.extend(encryptTransciModule);
  await encryptTransciRuntime.encryptTransci.initTkmsModule();
}
