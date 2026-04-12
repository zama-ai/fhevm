import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// EncryptionError
////////////////////////////////////////////////////////////////////////////////

export type EncryptionErrorType = EncryptionError & {
  name: 'EncryptionError';
};

export type EncryptionErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name'>
>;

export class EncryptionError extends FhevmErrorBase {
  constructor(params: EncryptionErrorParams) {
    super({
      ...params,
      name: 'EncryptionError',
    });
  }
}
