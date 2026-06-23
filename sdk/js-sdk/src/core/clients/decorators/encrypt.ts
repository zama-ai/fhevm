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
import {
  encryptSeeded,
  type EncryptSeededParameters,
  type EncryptSeededReturnType,
} from '../../actions/encrypt/encryptSeeded.js';
import {
  verifySeededEncryption,
  type VerifySeededEncryptionParameters,
  type VerifySeededEncryptionReturnType,
} from '../../actions/encrypt/verifySeededEncryption.js';
import { assertIsFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { _initEncrypt } from './encrypt-p.js';

////////////////////////////////////////////////////////////////////////////////

type EncryptActionMethods = {
  readonly encryptValue: (parameters: EncryptValueParameters) => Promise<EncryptValueReturnType>;
  readonly encryptValues: (parameters: EncryptValuesParameters) => Promise<EncryptValuesReturnType>;
  readonly encryptSeeded: (parameters: EncryptSeededParameters) => Promise<EncryptSeededReturnType>;
  readonly verifySeededEncryption: (
    parameters: VerifySeededEncryptionParameters,
  ) => Promise<VerifySeededEncryptionReturnType>;
};

export type EncryptActions = EncryptActionMethods & WithTfheVersion;

////////////////////////////////////////////////////////////////////////////////

function _encryptActions(fhevm: Fhevm<FhevmChain, WithEncrypt>): EncryptActionMethods {
  return {
    encryptValue: (parameters) => encryptValue(fhevm, parameters),
    encryptValues: (parameters) => encryptValues(fhevm, parameters),
    encryptSeeded: (parameters) => encryptSeeded(fhevm, parameters),
    verifySeededEncryption: (parameters) => verifySeededEncryption(fhevm, parameters),
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
