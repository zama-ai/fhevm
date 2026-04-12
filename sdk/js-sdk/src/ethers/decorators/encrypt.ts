import { encryptModule } from '../../core/modules/encrypt/module/index.js';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { WithEncrypt } from '../../core/types/coreFhevmRuntime.js';
import type {
  FhevmBase,
  FhevmExtension,
} from '../../core/types/coreFhevmClient.js';
import {
  encryptActionsWithModule,
  type EncryptActions,
} from '../../core/clients/decorators/encrypt.js';

////////////////////////////////////////////////////////////////////////////////

export function encryptActions(
  fhevm: FhevmBase<FhevmChain>,
): FhevmExtension<EncryptActions, WithEncrypt> {
  return encryptActionsWithModule(fhevm, encryptModule);
}
