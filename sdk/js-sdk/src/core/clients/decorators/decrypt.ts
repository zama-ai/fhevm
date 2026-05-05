/* eslint-disable @typescript-eslint/unified-signatures */
import type { Fhevm, FhevmBase, FhevmExtension, OptionalNativeClient } from '../../types/coreFhevmClient.js';
import type { FhevmRuntime, WithDecrypt } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { GenerateTransportKeypairReturnType } from '../../actions/decrypt/generateTransportKeypair.js';
import type { DecryptModuleFactory } from '../../modules/decrypt/types.js';
import { asFhevmClientWith, assertIsFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { generateTransportKeypair } from '../../kms/TransportKeypair-p.js';
import {
  decryptValue,
  type DecryptSelfValueParameters,
  type DecryptDelegatedValueParameters,
  type DecryptValueReturnType,
} from '../../actions/decrypt/decryptValue.js';
import {
  decryptValues,
  type DecryptDelegatedValuesParameters,
  type DecryptSelfValuesParameters,
  type DecryptValuesReturnType,
} from '../../actions/decrypt/decryptValues.js';
import {
  decryptValuesFromPairs,
  type DecryptSelfValuesFromPairsParameters,
  type DecryptDelegatedValuesFromPairsParameters,
  type DecryptValuesFromPairsReturnType,
} from '../../actions/decrypt/decryptValuesFromPairs.js';

////////////////////////////////////////////////////////////////////////////////

export type DecryptActions = {
  readonly decryptValue: {
    (parameters: DecryptSelfValueParameters): Promise<DecryptValueReturnType>;
    (parameters: DecryptDelegatedValueParameters): Promise<DecryptValueReturnType>;
  };
  readonly decryptValues: {
    (parameters: DecryptSelfValuesParameters): Promise<DecryptValuesReturnType>;
    (parameters: DecryptDelegatedValuesParameters): Promise<DecryptValuesReturnType>;
  };
  readonly decryptValuesFromPairs: {
    (parameters: DecryptSelfValuesFromPairsParameters): Promise<DecryptValuesFromPairsReturnType>;
    (parameters: DecryptDelegatedValuesFromPairsParameters): Promise<DecryptValuesFromPairsReturnType>;
  };
  readonly generateTransportKeypair: () => Promise<GenerateTransportKeypairReturnType>;
};

////////////////////////////////////////////////////////////////////////////////

function _decryptActions(fhevm: Fhevm<FhevmChain, WithDecrypt>): DecryptActions {
  // Preserve the original action overloads on the decorated client API.
  // Runtime behavior is unchanged: this is a direct pass-through wrapper.
  return {
    decryptValue: ((parameters) =>
      decryptValue(fhevm, parameters as DecryptSelfValueParameters)) as DecryptActions['decryptValue'],
    decryptValues: ((parameters) =>
      decryptValues(fhevm, parameters as DecryptSelfValuesParameters)) as DecryptActions['decryptValues'],
    decryptValuesFromPairs: ((parameters) =>
      decryptValuesFromPairs(
        fhevm,
        parameters as DecryptSelfValuesFromPairsParameters,
      )) as DecryptActions['decryptValuesFromPairs'],
    generateTransportKeypair: () => generateTransportKeypair(fhevm),
  };
}

////////////////////////////////////////////////////////////////////////////////

function _initDecrypt(fhevm: FhevmBase<FhevmChain | undefined, FhevmRuntime, OptionalNativeClient>): Promise<void> {
  const f = asFhevmClientWith(fhevm, 'decrypt');
  return f.runtime.decrypt.initTkmsModule();
}

////////////////////////////////////////////////////////////////////////////////

export function decryptActionsWithModule(
  fhevm: FhevmBase<FhevmChain>,
  factory: DecryptModuleFactory,
): FhevmExtension<DecryptActions, WithDecrypt> {
  const runtime = fhevm.runtime.extend(factory);
  assertIsFhevmClientWith(fhevm, 'decrypt');
  return {
    actions: _decryptActions(fhevm),
    runtime,
    init: _initDecrypt,
  };
}
