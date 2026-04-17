import type { Fhevm, FhevmBase, FhevmExtension } from '../../types/coreFhevmClient.js';
import type { WithEncrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptModuleFactory } from '../../modules/encrypt/types.js';
import {
  encryptValue,
  type EncryptValueParameters,
  type EncryptValueReturnType,
} from '../../actions/encrypt/encryptValue.js';
import {
  encryptValues,
  type EncryptValuesParameters,
  type EncryptValuesReturnType,
} from '../../actions/encrypt/encryptValues.js';
import { assertIsFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { _initEncrypt } from './encrypt-p.js';

////////////////////////////////////////////////////////////////////////////////

export type EncryptActions = {
  readonly encryptValue: (parameters: EncryptValueParameters) => Promise<EncryptValueReturnType>;
  readonly encryptValues: (parameters: EncryptValuesParameters) => Promise<EncryptValuesReturnType>;
};

////////////////////////////////////////////////////////////////////////////////

function _encryptActions(fhevm: Fhevm<FhevmChain, WithEncrypt>): EncryptActions {
  return {
    encryptValue: (parameters) => encryptValue(fhevm, parameters),
    encryptValues: (parameters) => encryptValues(fhevm, parameters),
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
