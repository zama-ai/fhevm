import type { FhevmChain } from '../../../core/types/fhevmChain.js';
import type { WithEncryptWithPad } from '../../../core/types/coreFhevmRuntime.js';
import type { FhevmBase, FhevmExtension } from '../../../core/types/coreFhevmClient.js';
import type { EncryptWithPadActions } from '../../../core/clients/decorators/encryptWithPad.js';
import { encryptWithPadModule } from '../../../core/modules/encryptWithPad/mock.js';
import { encryptWithPadActionsWithModule } from '../../../core/clients/decorators/encryptWithPad.js';

////////////////////////////////////////////////////////////////////////////////

export function encryptWithPadActions(
  fhevm: FhevmBase<FhevmChain>,
): FhevmExtension<EncryptWithPadActions, WithEncryptWithPad> {
  return encryptWithPadActionsWithModule(fhevm, encryptWithPadModule);
}
