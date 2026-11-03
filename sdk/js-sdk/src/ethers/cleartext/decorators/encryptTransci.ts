import type { FhevmChain } from '../../../core/types/fhevmChain.js';
import type { WithEncryptTransci } from '../../../core/types/coreFhevmRuntime.js';
import type { FhevmBase, FhevmExtension } from '../../../core/types/coreFhevmClient.js';
import type { EncryptTransciActions } from '../../../core/clients/decorators/encryptTransci.js';
import { encryptTransciModule } from '../../../core/modules/encryptTransci/mock.js';
import { encryptTransciActionsWithModule } from '../../../core/clients/decorators/encryptTransci.js';

////////////////////////////////////////////////////////////////////////////////

export function encryptTransciActions(
  fhevm: FhevmBase<FhevmChain>,
): FhevmExtension<EncryptTransciActions, WithEncryptTransci> {
  return encryptTransciActionsWithModule(fhevm, encryptTransciModule);
}
