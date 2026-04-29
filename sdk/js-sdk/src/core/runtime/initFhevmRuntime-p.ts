import type { FhevmRuntime } from '../types/coreFhevmRuntime.js';
import { decryptModule } from '../modules/decrypt/module/index.js';
import { encryptModule } from '../modules/encrypt/module/index.js';
import { verifyFhevmRuntime } from './CoreFhevmRuntime-p.js';

export async function initFhevmRuntime(runtime: FhevmRuntime, ownerToken: symbol): Promise<void> {
  verifyFhevmRuntime(runtime, ownerToken);
  const fullRuntime = runtime.extend(decryptModule).extend(encryptModule);
  await Promise.all([fullRuntime.decrypt.initTkmsModule(), fullRuntime.encrypt.initTfheModule()]);
}
