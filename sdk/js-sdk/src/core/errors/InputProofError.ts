import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// InputProofError
////////////////////////////////////////////////////////////////////////////////

export type InputProofErrorType = InputProofError & {
  name: 'InputProofError';
};

export type InputProofErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name'>
>;

export class InputProofError extends FhevmErrorBase {
  constructor(params: InputProofErrorParams) {
    super({
      ...params,
      message: params.message ?? `FHEVM InputProof is invalid.`,
      name: 'InputProofError',
    });
  }
}

////////////////////////////////////////////////////////////////////////////////
// TooManyHandlesError
////////////////////////////////////////////////////////////////////////////////

export type TooManyHandlesErrorType = TooManyHandlesError & {
  name: 'TooManyHandlesError';
};

export type TooManyHandlesErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name'> & { numberOfHandles: number }
>;

export class TooManyHandlesError extends FhevmErrorBase {
  constructor(params: TooManyHandlesErrorParams) {
    super({
      ...params,
      name: 'TooManyHandlesError',
      message: `Trying to pack ${params.numberOfHandles} handles. Packing more than 256 variables in a single input ciphertext is unsupported`,
    });
  }
}
