import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// FhevmConfigError
////////////////////////////////////////////////////////////////////////////////

export type FhevmConfigErrorType = FhevmConfigError & {
  name: 'FhevmConfigError';
};

export type FhevmConfigErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name'>
>;

export class FhevmConfigError extends FhevmErrorBase {
  constructor(params: FhevmConfigErrorParams) {
    super({
      ...params,
      name: 'FhevmConfigError',
    });
  }
}
