import { assertIsFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import {
  encrypt,
  type EncryptSingleParameters,
  type EncryptSingleReturnType,
  type EncryptMultipleParameters,
  type EncryptMultipleReturnType,
} from '../../actions/encrypt/encrypt.js';
import type {
  Fhevm,
  FhevmBase,
  FhevmExtension,
} from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptModuleFactory } from '../../modules/encrypt/types.js';
import { _initEncrypt } from './encrypt-p.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptActions = {
  readonly encrypt: {
    (parameters: EncryptSingleParameters): Promise<EncryptSingleReturnType>;
    (parameters: EncryptMultipleParameters): Promise<EncryptMultipleReturnType>;
  };
};

////////////////////////////////////////////////////////////////////////////////

function _encryptActions(
  fhevm: Fhevm<FhevmChain, WithEncrypt>,
): EncryptActions {
  return {
    encrypt: ((
      parameters: EncryptSingleParameters | EncryptMultipleParameters,
    ) =>
      encrypt(
        fhevm,
        parameters as EncryptSingleParameters,
      )) as EncryptActions['encrypt'],
  };
}

////////////////////////////////////////////////////////////////////////////////

export function encryptActionsWithModule(
  fhevm: FhevmBase<FhevmChain>,
  factory: EncryptModuleFactory,
): FhevmExtension<EncryptActions, WithEncrypt> {
  const runtime = fhevm.runtime.extend(factory);
  assertIsFhevmClientWith(fhevm, 'encrypt');
  return {
    actions: _encryptActions(fhevm),
    runtime,
    init: _initEncrypt,
  };
}
