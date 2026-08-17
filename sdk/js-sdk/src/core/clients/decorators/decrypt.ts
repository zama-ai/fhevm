import type { Fhevm, FhevmBase, FhevmExtension, WithTkmsVersion } from '../../types/coreFhevmClient.js';
import type { WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { GenerateTransportKeyPairReturnType } from '../../actions/decrypt/generateTransportKeyPair.js';
import type { DecryptModuleFactory } from '../../modules/decrypt/types.js';
import { assertIsFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { generateTransportKeyPair } from '../../actions/decrypt/generateTransportKeyPair.js';
import {
  decryptValue,
  type DecryptValueParameters,
  type DecryptValueReturnType,
} from '../../actions/decrypt/decryptValue.js';
import {
  decryptValues,
  type DecryptValuesParameters,
  type DecryptValuesReturnType,
} from '../../actions/decrypt/decryptValues.js';
import {
  decryptValuesFromPairs,
  type DecryptValuesFromPairsParameters,
  type DecryptValuesFromPairsReturnType,
} from '../../actions/decrypt/decryptValuesFromPairs.js';
import { _initDecrypt } from './decrypt-p.js';

////////////////////////////////////////////////////////////////////////////////

type DecryptActionMethods = {
  readonly decryptValue: (parameters: DecryptValueParameters) => Promise<DecryptValueReturnType>;
  readonly decryptValues: (parameters: DecryptValuesParameters) => Promise<DecryptValuesReturnType>;
  readonly decryptValuesFromPairs: (
    parameters: DecryptValuesFromPairsParameters,
  ) => Promise<DecryptValuesFromPairsReturnType>;
  readonly generateTransportKeyPair: () => Promise<GenerateTransportKeyPairReturnType>;
};

export type DecryptActions = DecryptActionMethods & WithTkmsVersion;

////////////////////////////////////////////////////////////////////////////////

function _decryptActions(fhevm: Fhevm<FhevmChain, WithDecrypt>): DecryptActionMethods {
  // Preserve the original action overloads on the decorated client API.
  // Runtime behavior is unchanged: this is a direct pass-through wrapper.
  return {
    decryptValue: (parameters) => decryptValue(fhevm, parameters),
    decryptValues: (parameters) => decryptValues(fhevm, parameters),
    decryptValuesFromPairs: (parameters) => decryptValuesFromPairs(fhevm, parameters),
    generateTransportKeyPair: () => generateTransportKeyPair(fhevm),
  };
}

////////////////////////////////////////////////////////////////////////////////

export function decryptActionsWithModule(
  fhevm: FhevmBase<FhevmChain>,
  factory: DecryptModuleFactory,
): FhevmExtension<DecryptActions, WithDecrypt> {
  const runtime = fhevm.runtime.extend(factory);
  assertIsFhevmClientWith(fhevm, 'decrypt');
  return {
    actions: _decryptActions(fhevm) as DecryptActions,
    runtime,
    init: _initDecrypt,
  };
}
