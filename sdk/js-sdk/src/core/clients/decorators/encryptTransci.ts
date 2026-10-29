import type { Fhevm, FhevmBase, FhevmExtension, WithTkmsVersion } from '../../types/coreFhevmClient.js';
import type { WithEncryptTransci } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptTransciModuleFactory } from '../../modules/encryptTransci/types.js';
import {
  encryptValueTransci,
  type EncryptValueTransciParameters,
  type EncryptValueTransciReturnType,
} from '../../actions/encrypt/encryptValueTransci.js';
import {
  encryptValuesTransci,
  type EncryptValuesTransciParameters,
  type EncryptValuesTransciReturnType,
} from '../../actions/encrypt/encryptValuesTransci.js';
import {
  generateTransportKeyPair,
  type GenerateTransportKeyPairReturnType,
} from '../../actions/decrypt/generateTransportKeyPair.js';
import { assertIsFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { _initEncryptTransci } from './encryptTransci-p.js';

////////////////////////////////////////////////////////////////////////////////

type EncryptTransciActionMethods = {
  readonly encryptValueTransci: (parameters: EncryptValueTransciParameters) => Promise<EncryptValueTransciReturnType>;
  readonly encryptValuesTransci: (
    parameters: EncryptValuesTransciParameters,
  ) => Promise<EncryptValuesTransciReturnType>;
  /** Generates the transport key pair whose public key the pad permit must carry. */
  readonly generateTransportKeyPair: () => Promise<GenerateTransportKeyPairReturnType>;
};

export type EncryptTransciActions = EncryptTransciActionMethods & WithTkmsVersion;

////////////////////////////////////////////////////////////////////////////////

function _encryptTransciActions(fhevm: Fhevm<FhevmChain, WithEncryptTransci>): EncryptTransciActionMethods {
  return {
    encryptValueTransci: (parameters) => encryptValueTransci(fhevm, parameters),
    encryptValuesTransci: (parameters) => encryptValuesTransci(fhevm, parameters),
    generateTransportKeyPair: () => generateTransportKeyPair(fhevm),
  };
}

////////////////////////////////////////////////////////////////////////////////

export function encryptTransciActionsWithModule(
  fhevm: FhevmBase<FhevmChain>,
  factory: EncryptTransciModuleFactory,
): FhevmExtension<EncryptTransciActions, WithEncryptTransci> {
  const runtime = fhevm.runtime.extend(factory);
  assertIsFhevmClientWith(fhevm, 'encryptTransci');
  return {
    actions: _encryptTransciActions(fhevm) as EncryptTransciActions,
    runtime,
    init: _initEncryptTransci,
  };
}
