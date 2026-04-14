import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// TfheError
////////////////////////////////////////////////////////////////////////////////

export type TfheErrorType = TfheError & {
  name: 'TFHEError';
};

export type TfheErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name' | 'message'> & {
    readonly message: string;
  }
>;

export class TfheError extends FhevmErrorBase {
  constructor(params: TfheErrorParams) {
    super({
      ...params,
      name: 'TfheError',
    });
  }
}
