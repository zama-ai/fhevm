import type { Fhevm, FhevmBase, FhevmExtension, WithTkmsVersion } from '../../types/coreFhevmClient.js';
import type { WithEncryptWithPad } from '../../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../types/fhevmChain.js';
import type { EncryptWithPadModuleFactory } from '../../modules/encryptWithPad/types.js';
import {
  encryptValueWithPad,
  type EncryptValueWithPadParameters,
  type EncryptValueWithPadReturnType,
} from '../../actions/encrypt/encryptValueWithPad.js';
import {
  encryptValuesWithPad,
  type EncryptValuesWithPadParameters,
  type EncryptValuesWithPadReturnType,
} from '../../actions/encrypt/encryptValuesWithPad.js';
import {
  generateTransportKeyPair,
  type GenerateTransportKeyPairReturnType,
} from '../../actions/decrypt/generateTransportKeyPair.js';
import { assertIsFhevmClientWith } from '../../runtime/CoreFhevm-p.js';
import { _initEncryptWithPad } from './encryptWithPad-p.js';

////////////////////////////////////////////////////////////////////////////////

type EncryptWithPadActionMethods = {
  readonly encryptValueWithPad: (parameters: EncryptValueWithPadParameters) => Promise<EncryptValueWithPadReturnType>;
  readonly encryptValuesWithPad: (
    parameters: EncryptValuesWithPadParameters,
  ) => Promise<EncryptValuesWithPadReturnType>;
  /** Generates the transport key pair whose public key the pad permit must carry. */
  readonly generateTransportKeyPair: () => Promise<GenerateTransportKeyPairReturnType>;
};

export type EncryptWithPadActions = EncryptWithPadActionMethods & WithTkmsVersion;

////////////////////////////////////////////////////////////////////////////////

function _encryptWithPadActions(fhevm: Fhevm<FhevmChain, WithEncryptWithPad>): EncryptWithPadActionMethods {
  return {
    encryptValueWithPad: (parameters) => encryptValueWithPad(fhevm, parameters),
    encryptValuesWithPad: (parameters) => encryptValuesWithPad(fhevm, parameters),
    generateTransportKeyPair: () => generateTransportKeyPair(fhevm),
  };
}

////////////////////////////////////////////////////////////////////////////////

export function encryptWithPadActionsWithModule(
  fhevm: FhevmBase<FhevmChain>,
  factory: EncryptWithPadModuleFactory,
): FhevmExtension<EncryptWithPadActions, WithEncryptWithPad> {
  const runtime = fhevm.runtime.extend(factory);
  assertIsFhevmClientWith(fhevm, 'encryptWithPad');
  return {
    actions: _encryptWithPadActions(fhevm) as EncryptWithPadActions,
    runtime,
    init: _initEncryptWithPad,
  };
}
