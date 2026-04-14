import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithDecrypt } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmBase, FhevmExtension } from '../../core/types/coreFhevmClient.js';
import type { DecryptActions } from '../../core/clients/decorators/decrypt.js';
import { decryptModule } from '../../core/modules/decrypt/module/index.js';
import { decryptActionsWithModule } from '../../core/clients/decorators/decrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function decryptActions(fhevm: FhevmBase<FhevmChain>): FhevmExtension<DecryptActions, WithDecrypt> {
  return decryptActionsWithModule(fhevm, decryptModule);
}
