import type { Fhevm, FhevmBase, FhevmExtension, WithTfheVersion } from '../../types/coreFhevmClient.js';
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

type EncryptActionMethods = {
  readonly encryptValue: (parameters: EncryptValueParameters) => Promise<EncryptValueReturnType>;
  readonly encryptValues: (parameters: EncryptValuesParameters) => Promise<EncryptValuesReturnType>;
};

export type EncryptActions = EncryptActionMethods & WithTfheVersion;

////////////////////////////////////////////////////////////////////////////////

function _encryptActions(fhevm: Fhevm<FhevmChain, WithEncrypt>): EncryptActionMethods {
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
    actions: _encryptActions(fhevm) as EncryptActions,
    runtime,
    init: _initEncrypt,
  };
}
