import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { verifyFhevmRuntime } from './CoreFhevmRuntime-p.js';
import { encryptWithPadModule } from '../modules/encryptWithPad/module/index.js';

export async function initFhevmEncryptWithPadRuntime(runtime: FhevmRuntime, ownerToken: symbol): Promise<void> {
  verifyFhevmRuntime(runtime, ownerToken);
  const encryptWithPadRuntime = runtime.extend(encryptWithPadModule);
  await encryptWithPadRuntime.encryptWithPad.initTkmsModule();
}
