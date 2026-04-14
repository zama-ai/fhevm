import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// UnknownSignerError
////////////////////////////////////////////////////////////////////////////////

export type UnknownSignerErrorType = UnknownSignerError & {
  name: 'UnknownSignerError';
};

export type UnknownSignerErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name'> & {
    unknownAddress: string;
    type: 'coprocessor' | 'kms';
  }
>;

export class UnknownSignerError extends FhevmErrorBase {
  constructor(params: UnknownSignerErrorParams) {
    super({
      ...params,
      name: 'UnknownSignerError',
      message: `Invalid address found: ${params.unknownAddress} is not in the list of ${params.type} signers`,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////
// ThresholdSignerError
////////////////////////////////////////////////////////////////////////////////

export type ThresholdSignerErrorType = ThresholdSignerError & {
  name: 'ThresholdSignerError';
};

export type ThresholdSignerErrorParams = Prettify<Omit<FhevmErrorBaseParams, 'name'> & { type: 'coprocessor' | 'kms' }>;

export class ThresholdSignerError extends FhevmErrorBase {
  constructor(params: ThresholdSignerErrorParams) {
    super({
      ...params,
      name: 'ThresholdSignerError',
      message: `${params.type} signers threshold is not reached`,
    });
  }
}

////////////////////////////////////////////////////////////////////////////////
// DuplicateSignerError
////////////////////////////////////////////////////////////////////////////////

export type DuplicateSignerErrorType = DuplicateSignerError & {
  name: 'DuplicateSignerError';
};

export type DuplicateSignerErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name'> & {
    duplicateAddress: string;
    type: 'coprocessor' | 'kms';
  }
>;

export class DuplicateSignerError extends FhevmErrorBase {
  constructor(params: DuplicateSignerErrorParams) {
    super({
      ...params,
      name: 'DuplicateSignerError',
      message: `Duplicate ${params.type} signer address found: ${params.duplicateAddress} appears multiple times in recovered addresses`,
    });
  }
}
