import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmBase, FhevmExtension } from '../../core/types/coreFhevmClient.js';
import type { EncryptActions } from '../../core/clients/decorators/encrypt.js';
import { encryptActionsWithModule } from '../../core/clients/decorators/encrypt.js';
import { encryptModule } from '../../core/modules/encrypt/module/index.js';

////////////////////////////////////////////////////////////////////////////////

export function encryptActions(fhevm: FhevmBase<FhevmChain>): FhevmExtension<EncryptActions, WithEncrypt> {
  return encryptActionsWithModule(fhevm, encryptModule);
}
